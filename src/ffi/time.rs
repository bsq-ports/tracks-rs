use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
// seconds, nanos
pub struct CTimeUnit(u64, u32);

#[unsafe(no_mangle)]
pub extern "C" fn get_time() -> CTimeUnit {
    let t = SystemTime::now();

    t.into()
}

impl From<SystemTime> for CTimeUnit {
    fn from(t: SystemTime) -> Self {
        let unix_time = t.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        CTimeUnit(unix_time.as_secs(), unix_time.subsec_nanos())
    }
}

impl From<CTimeUnit> for SystemTime {
    fn from(t: CTimeUnit) -> Self {
        // Create a Duration from the provided seconds and nanoseconds
        let duration = Duration::new(t.0, t.1);

        UNIX_EPOCH + duration
    }
}
