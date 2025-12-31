use alpm::Alpm;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn time_since_last_pacman_update() -> Result<Duration, Box<dyn Error>> {
    let handle = Alpm::new("/", "/var/lib/pacman/")?;
    let local_db = handle.localdb();

    let latest_update_ts: i64 = local_db
        .pkgs()
        .iter()
        .filter_map(|pkg| pkg.install_date())
        .max()
        .ok_or("no installed packages found")?;

    let target = UNIX_EPOCH + Duration::from_secs(latest_update_ts as u64);
    Ok(SystemTime::now().duration_since(target)?)
}
