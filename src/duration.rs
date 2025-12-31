use std::{fmt, time::Duration};

pub struct HumanDuration(pub Duration);

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        let (value, unit) = if secs >= 86400 {
            (secs / 86400, "d")
        } else if secs >= 3600 {
            (secs / 3600, "h")
        } else if secs >= 60 {
            (secs / 60, "m")
        } else {
            (secs, "s")
        };

        write!(f, "{}{}", value, unit)
    }
}
