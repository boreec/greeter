use std::error::Error;
use std::fmt;
use std::time::Duration;
use zbus::{Connection, proxy::Proxy};

#[derive(Debug, Default)]
pub struct BootTimeRecord {
    firmware: Option<Duration>,
    loader: Option<Duration>,
    kernel: Option<Duration>,
    initrd: Option<Duration>,
    userspace: Option<Duration>,
    total: Option<Duration>,
}

pub async fn retrieve_boot_time() -> std::result::Result<BootTimeRecord, Box<dyn Error>> {
    let connection = Connection::system().await?;
    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
    )
    .await?;

    let firmware_ts: u64 = proxy.get_property("FirmwareTimestampMonotonic").await?;
    let loader_ts: u64 = proxy.get_property("LoaderTimestampMonotonic").await?;
    let initrd_ts: u64 = proxy.get_property("InitRDTimestampMonotonic").await?;
    let userspace_ts: u64 = proxy.get_property("UserspaceTimestampMonotonic").await?;
    let finish_ts: u64 = proxy.get_property("FinishTimestampMonotonic").await?;

    let usec = |us: u64| Duration::from_micros(us);
    let mut record = BootTimeRecord::default();

    let kernel_done_time = if initrd_ts > 0 {
        initrd_ts
    } else {
        userspace_ts
    };

    if firmware_ts > 0 && loader_ts > 0 {
        record.firmware = Some(usec(firmware_ts.saturating_sub(loader_ts)));
    }

    if loader_ts > 0 {
        record.loader = Some(usec(loader_ts));
    }

    record.kernel = Some(usec(kernel_done_time));

    if initrd_ts > 0 && userspace_ts > 0 {
        record.initrd = Some(usec(userspace_ts.saturating_sub(initrd_ts)));
    }

    if finish_ts > 0 && userspace_ts > 0 {
        record.userspace = Some(usec(finish_ts.saturating_sub(userspace_ts)));
    }

    if firmware_ts > 0 && finish_ts > 0 {
        record.total = Some(usec(firmware_ts + finish_ts));
    } else if finish_ts > 0 {
        record.total = Some(usec(finish_ts));
    }

    Ok(record)
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
            fmt_dur(self.total),
        )
    }
}
