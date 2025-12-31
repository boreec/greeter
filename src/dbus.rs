use std::error::Error;
use std::fmt;
use std::fs;
use std::time::Duration;
use zbus::{Connection, proxy::Proxy};

#[derive(Debug, Default)]
pub struct BootTimeRecord {
    firmware: Option<Duration>,
    loader: Option<Duration>,
    kernel: Option<Duration>,
    initrd: Option<Duration>,
    userspace: Option<Duration>,
}

impl BootTimeRecord {
    pub fn total_duration(&self) -> Duration {
        let mut total = Duration::from_millis(0);
        total += self.firmware.unwrap_or_default();
        total += self.loader.unwrap_or_default();
        total += self.kernel.unwrap_or_default();
        total += self.initrd.unwrap_or_default();
        total += self.userspace.unwrap_or_default();
        total
    }
}

impl fmt::Display for BootTimeRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_dur = |opt: Option<Duration>| -> String {
            match opt {
                Some(d) => format!("{}ms", d.as_millis()),
                None => "?".to_string(),
            }
        };
        write!(
            f,
            "{} (firmware) + {} (loader) + {} (kernel) + {} (initrd) + {} (userspace) = {}",
            fmt_dur(self.firmware),
            fmt_dur(self.loader),
            fmt_dur(self.kernel),
            fmt_dur(self.initrd),
            fmt_dur(self.userspace),
            format!("{}ms", self.total_duration().as_millis()),
        )
    }
}

pub async fn retrieve_boot_time() -> std::result::Result<BootTimeRecord, Box<dyn Error>> {
    let connection = Connection::system().await?;

    let p = Proxy::new(
        &connection,
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
    )
    .await?;

    let firmware_ts: u64 = p
        .get_property("FirmwareTimestampMonotonic")
        .await
        .unwrap_or(0);
    let loader_ts: u64 = p
        .get_property("LoaderTimestampMonotonic")
        .await
        .unwrap_or(0);
    let initrd_ts: u64 = p
        .get_property("InitRDTimestampMonotonic")
        .await
        .unwrap_or(0);
    let mut userspace_ts: u64 = p
        .get_property("UserspaceTimestampMonotonic")
        .await
        .unwrap_or(0);
    let mut finish_ts: u64 = p
        .get_property("FinishTimestampMonotonic")
        .await
        .unwrap_or(0);

    // fallack if userspace is missing from dbus (unfinished boot)
    if userspace_ts == 0 {
        if let Some(ts) = get_pid1_starttime_us() {
            userspace_ts = ts;
        }
    }

    // fallback if finish is missing from dbus (unfinished boot)
    if finish_ts == 0 {
        finish_ts = get_current_monotonic_us();
    }

    let usec = |us: u64| Duration::from_micros(us);
    let mut record = BootTimeRecord::default();

    let kernel_end_ts = if initrd_ts > 0 {
        initrd_ts
    } else {
        userspace_ts
    };

    if firmware_ts > 0 && loader_ts > 0 {
        record.firmware = Some(usec(firmware_ts - loader_ts));
    }

    if loader_ts > 0 {
        record.loader = Some(usec(loader_ts));
    }

    if kernel_end_ts > 0 {
        record.kernel = Some(usec(kernel_end_ts));
    }

    if initrd_ts > 0 && userspace_ts > 0 {
        record.initrd = Some(usec(userspace_ts.saturating_sub(initrd_ts)));
    }

    if userspace_ts > 0 {
        record.userspace = Some(usec(finish_ts.saturating_sub(userspace_ts)));
    }

    Ok(record)
}

fn get_pid1_starttime_us() -> Option<u64> {
    let content = fs::read_to_string("/proc/1/stat").ok()?;
    let fields: Vec<&str> = content.split_whitespace().collect();

    let start_ticks: u64 = fields.get(21)?.parse().ok()?;

    let clk_tck = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
    if clk_tck == 0 {
        return None;
    }

    let seconds = start_ticks / clk_tck;
    let remainder = start_ticks % clk_tck;
    let micros = (remainder * 1_000_000) / clk_tck;

    Some((seconds * 1_000_000) + micros)
}

/// Retrieve the current time.
fn get_current_monotonic_us() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    (ts.tv_sec as u64) * 1_000_000 + (ts.tv_nsec as u64) / 1_000
}
