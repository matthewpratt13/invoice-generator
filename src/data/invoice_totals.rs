use serde::{Deserialize, Serialize};

use super::power_totals::PowerTotals;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct InvoiceTotals {
    total_produced: PowerTotals, // inverter yield per period
    to_grid: PowerTotals,        // export per period
    total_consumed: PowerTotals, // (inverter yield - export) per period
    consumed: f64,               // sum of `total_produced`
    produced: f64,               // sum of `total_consumed`
}

#[allow(dead_code)]
impl InvoiceTotals {
    pub fn from_power(
        total_produced: PowerTotals,
        to_grid: PowerTotals,
        total_consumed: PowerTotals,
        consumed: f64,
        produced: f64,
    ) -> Self {
        Self {
            total_produced,
            to_grid,
            total_consumed,
            consumed,
            produced,
        }
    }

    // { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn total_produced(&self) -> PowerTotals {
        self.total_produced
    }

    // { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn to_grid(&self) -> PowerTotals {
        self.to_grid
    }

    // { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn total_consumed(&self) -> PowerTotals {
        self.total_consumed
    }

    // sum of consumption (total_produced - to_grid)
    pub fn consumed(&self) -> f64 {
        self.consumed
    }

    // sum of production (inverter yield)
    pub fn produced(&self) -> f64 {
        self.produced
    }
}
