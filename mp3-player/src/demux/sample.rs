#[derive(Debug)]
pub struct MP4Sample {
    pub start_time: u64,
    pub duration: u32,
    pub range: (usize, usize),
}

impl MP4Sample {
    pub(super) fn new(start_time: u64, duration: u32, range: (usize, usize)) -> Self {
        Self {
            start_time,
            duration,
            range,
        }
    }
}
