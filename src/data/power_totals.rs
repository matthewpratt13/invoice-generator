use serde::Deserialize;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub struct PowerTotals {
    peak_kwh: f64,
    standard_kwh: f64,
    off_peak_kwh: f64,
}

impl PowerTotals {
    pub fn build(peak_kwh: f64, standard_kwh: f64, off_peak_kwh: f64) -> Self {
        let peak_kwh: f64 = peak_kwh.round();
        let standard_kwh: f64 = standard_kwh.round();
        let off_peak_kwh: f64 = off_peak_kwh.round();

        Self {
            peak_kwh,
            standard_kwh,
            off_peak_kwh,
        }
    }

    pub fn from_periods(peak_kwh: f64, standard_kwh: f64, off_peak_kwh: f64) -> Self {
        Self {
            peak_kwh,
            standard_kwh,
            off_peak_kwh,
        }
    }

    pub fn peak_kwh(&self) -> f64 {
        self.peak_kwh
    }

    pub fn standard_kwh(&self) -> f64 {
        self.standard_kwh
    }

    pub fn off_peak_kwh(&self) -> f64 {
        self.off_peak_kwh
    }
}

impl fmt::Display for PowerTotals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.peak_kwh, self.standard_kwh, self.off_peak_kwh,
        )
    }
}
