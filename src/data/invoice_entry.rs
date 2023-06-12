use super::invoice_totals::InvoiceTotals;
use super::power_totals::PowerTotals;
use super::DailyData; // HashMap<NaiveDate, Vec<Data>>
use super::Data; // struct
use super::Period; // enum

use chrono::{Datelike, NaiveDate, Weekday};

use itertools::Itertools; // to sort by date

use serde::Deserialize;

use std::error::Error;

#[allow(dead_code)] // remove rust-analyzer warnings
enum Parameter {
    InverterYield,
    Export,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct InvoiceEntry {
    weekday: Weekday,            // Mon, Tue, Wed … (enum)
    date: NaiveDate,             // yyyy-mm-dd
    total_produced: PowerTotals, // inverter yield per period
    to_grid: PowerTotals,        // export per period
    total_consumed: PowerTotals, // (inverter yield - export) per period
    consumed: f64,               // sum of `total_consumed`
    produced: f64,               // sum of `total_produced`
}

#[allow(dead_code)] // remove rust-analyzer warnings
impl InvoiceEntry {
    pub fn new(
        weekday: Weekday,
        date: NaiveDate,
        total_produced: PowerTotals,
        to_grid: PowerTotals,
        total_consumed: PowerTotals,
        consumed: f64,
        produced: f64,
    ) -> Self {
        Self {
            weekday,
            date,
            total_produced,
            to_grid,
            total_consumed,
            consumed,
            produced,
        }
    }

    // getters:

    pub fn weekday(&self) -> Weekday {
        self.weekday
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    // return { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn total_produced(&self) -> PowerTotals {
        self.total_produced
    }

    // return { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn to_grid(&self) -> PowerTotals {
        self.to_grid
    }

    // return { peak_kwh, standard_kwh, off_peak_kwh }
    pub fn total_consumed(&self) -> PowerTotals {
        self.total_consumed
    }

    pub fn consumed(&self) -> f64 {
        self.consumed
    }

    pub fn produced(&self) -> f64 {
        self.produced
    }
}

// DailyData = { HashMap<NaiveDate, Vec<Data>> }
// Data = { weekday, naive_date, hour, inverter_yield, export, period }
#[allow(dead_code)] // remove rust-analyzer warnings
pub fn from_daily_data(data: Vec<DailyData>) -> Result<Vec<InvoiceEntry>, Box<dyn Error>> {
    if data.is_empty() {
        return Err(From::from(
            "Input error: expected data, but received empty vector",
        ));
    }

    let mut invoice_entries: Vec<InvoiceEntry> = Vec::new();

    for d in data {
        for (date, data) in d {
            let weekday: Weekday = date.weekday();

            let total_produced: PowerTotals = power_totals(&data, Parameter::InverterYield)?;
            let to_grid: PowerTotals = power_totals(&data, Parameter::Export)?;

            let peak_consumed: f64 = total_produced.peak_kwh() - to_grid.peak_kwh();
            let standard_consumed: f64 = total_produced.standard_kwh() - to_grid.standard_kwh();
            let off_peak_consumed: f64 = total_produced.off_peak_kwh() - to_grid.off_peak_kwh();

            let total_consumed =
                PowerTotals::build(peak_consumed, standard_consumed, off_peak_consumed);

            let consumed: f64 = total_consumed.peak_kwh()
                + total_consumed.standard_kwh()
                + total_consumed.off_peak_kwh();

            let produced: f64 = total_produced.peak_kwh()
                + total_produced.standard_kwh()
                + total_produced.off_peak_kwh();

            let consumed: f64 = consumed.round();
            let produced: f64 = produced.round();

            let entry = InvoiceEntry {
                weekday,
                date,
                total_produced,
                to_grid,
                total_consumed,
                consumed,
                produced,
            };

            invoice_entries.push(entry);
        }
    }

    // sort invoice entries by date
    let sorted: Vec<InvoiceEntry> = invoice_entries
        .into_iter()
        .sorted_by_key(|k| k.date)
        .collect();

    Ok(sorted)
}

// match data to periods in a new collection/struct
fn power_totals(data: &Vec<Data>, param: Parameter) -> Result<PowerTotals, Box<dyn Error>> {
    if data.is_empty() {
        return Err(From::from(
            "Input error: expected data, but received empty vector",
        ));
    }

    let mut peak_kwh = 0.0;
    let mut standard_kwh = 0.0;
    let mut off_peak_kwh = 0.0;

    for d in data {
        let p: f64 = match param {
            Parameter::InverterYield => d.inverter_yield,
            Parameter::Export => d.export,
        };

        match d.period {
            Period::Peak => peak_kwh += p,
            Period::Standard => standard_kwh += p,
            Period::OffPeak => off_peak_kwh += p,
        }
    }

    let peak_kwh: f64 = peak_kwh.round();
    let standard_kwh: f64 = standard_kwh.round();
    let off_peak_kwh: f64 = off_peak_kwh.round();

    Ok(PowerTotals::from_periods(
        peak_kwh,
        standard_kwh,
        off_peak_kwh,
    ))
}

// calculate final row in invoice
#[allow(dead_code)] // remove rust-analyzer warnings
pub fn invoice_totals(
    invoice_entries: &Vec<InvoiceEntry>,
) -> Result<InvoiceTotals, Box<dyn Error>> {
    if invoice_entries.is_empty() {
        return Err(From::from(
            "Input error: expected entries, but received empty vector",
        ));
    }

    let mut inverter_yield_peak_kwh = 0.0;
    let mut inverter_yield_standard_kwh = 0.0;
    let mut inverter_yield_off_peak_kwh = 0.0;

    let mut export_peak_kwh = 0.0;
    let mut export_standard_kwh = 0.0;
    let mut export_off_peak_kwh = 0.0;

    let mut consumption_peak_kwh = 0.0;
    let mut consumption_standard_kwh = 0.0;
    let mut consumption_off_peak_kwh = 0.0;

    // sum all values for the month per column
    for e in invoice_entries {
        inverter_yield_peak_kwh += e.total_produced().peak_kwh();
        inverter_yield_standard_kwh += e.total_produced().standard_kwh();
        inverter_yield_off_peak_kwh += e.total_produced().off_peak_kwh();

        export_peak_kwh += e.to_grid().peak_kwh();
        export_standard_kwh += e.to_grid().standard_kwh();
        export_off_peak_kwh += e.to_grid().off_peak_kwh();

        consumption_peak_kwh += e.total_consumed().peak_kwh();
        consumption_standard_kwh += e.total_consumed().standard_kwh();
        consumption_off_peak_kwh += e.total_consumed().off_peak_kwh();
    }

    // round values to nearest whole number as float (i.e., x.0)
    let inverter_yield_peak_kwh: f64 = inverter_yield_peak_kwh.round();
    let inverter_yield_standard_kwh: f64 = inverter_yield_standard_kwh.round();
    let inverter_yield_off_peak_kwh: f64 = inverter_yield_off_peak_kwh.round();

    let export_peak_kwh: f64 = export_peak_kwh.round();
    let export_standard_kwh: f64 = export_standard_kwh.round();
    let export_off_peak_kwh: f64 = export_off_peak_kwh.round();

    let consumption_peak_kwh: f64 = consumption_peak_kwh.round();
    let consumption_standard_kwh: f64 = consumption_standard_kwh.round();
    let consumption_off_peak_kwh: f64 = consumption_off_peak_kwh.round();

    let produced: f64 =
        &inverter_yield_peak_kwh + &inverter_yield_standard_kwh + &inverter_yield_off_peak_kwh;
    let consumed: f64 =
        &consumption_peak_kwh + &consumption_standard_kwh + &consumption_off_peak_kwh;

    let produced: f64 = produced.round();
    let consumed: f64 = consumed.round();

    let total_produced = PowerTotals::from_periods(
        inverter_yield_peak_kwh,
        inverter_yield_standard_kwh,
        inverter_yield_off_peak_kwh,
    );

    let to_grid =
        PowerTotals::from_periods(export_peak_kwh, export_standard_kwh, export_off_peak_kwh);

    let total_consumed = PowerTotals::from_periods(
        consumption_peak_kwh,
        consumption_standard_kwh,
        consumption_off_peak_kwh,
    );

    Ok(InvoiceTotals::from_power(
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    ))
}
