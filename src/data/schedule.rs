#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    pub peak_hours: Option<Vec<u32>>,
    pub off_peak_hours: Vec<u32>,
    pub sec_peak_hours: Option<Vec<u32>>,
    pub sec_off_peak_hours: Option<Vec<u32>>,
}

impl Schedule {
    pub fn new(
        peak_hours: Option<Vec<u32>>,
        off_peak_hours: Vec<u32>,
        sec_peak_hours: Option<Vec<u32>>,
        sec_off_peak_hours: Option<Vec<u32>>,
    ) -> Self {
        Self {
            peak_hours,
            off_peak_hours,
            sec_peak_hours,
            sec_off_peak_hours,
        }
    }
}
