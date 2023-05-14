use super::*;

pub(crate) fn _dummy_data() -> Vec<Data> {
    let weekday = Weekday::Fri;
    let date = NaiveDate::from_ymd_opt(2023, 2, 17).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 2.33,
            export: 2.13,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 6.79,
            export: 6.57,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 18.02,
            export: 17.76,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 2.12,
            export: 1.93,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 32.11,
            export: 31.83,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 46.78,
            export: 46.47,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 44.51,
            export: 44.2,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 28.07,
            export: 27.83,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 5.59,
            export: 5.37,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 0.66,
            export: 0.49,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 0.01,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry() -> InvoiceEntry {
    let weekday = Weekday::Fri;
    let date = NaiveDate::from_ymd_opt(2023, 2, 17).unwrap();

    let total_produced = PowerTotals::from_periods(27.0, 160.0, 0.0);

    let to_grid = PowerTotals::from_periods(26.0, 158.0, 0.0);

    let total_consumed = PowerTotals::from_periods(1.0, 2.0, 0.0);

    let consumed = 3.0;
    let produced = 187.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

///////////////////////////

pub(crate) fn _dummy_data_02() -> Vec<Data> {
    let weekday = Weekday::Sat;
    let date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();

    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 0.77,
            export: 0.52,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 5.94,
            export: 5.65,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 12.96,
            export: 12.64,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 3.23,
            export: 2.69,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 9.7,
            export: 9.42,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 42.09,
            export: 41.63,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 49.68,
            export: 49.27,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 49.4,
            export: 48.97,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 36.72,
            export: 36.33,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 11.01,
            export: 10.71,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 0.58,
            export: 0.43,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_02() -> InvoiceEntry {
    let weekday = Weekday::Sat;
    let date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();

    let total_produced = PowerTotals::from_periods(0.0, 32.0, 190.0);

    let to_grid = PowerTotals::from_periods(0.0, 30.0, 188.0);

    let total_consumed = PowerTotals::from_periods(0.0, 2.0, 2.0);

    let consumed = 4.0;
    let produced = 222.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

/////////////////////////

pub(crate) fn _dummy_data_03() -> Vec<Data> {
    let weekday = Weekday::Sun;
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    let off_peak = Period::OffPeak;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 0.82,
            export: 0.58,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 4.45,
            export: 4.13,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 16.32,
            export: 15.99,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 8.01,
            export: 7.85,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 36.46,
            export: 35.94,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 42.99,
            export: 42.57,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 50.56,
            export: 41.95,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 30.11,
            export: 25.51,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 31.48,
            export: 31.11,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 22.16,
            export: 21.83,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 3.37,
            export: 1.68,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_03() -> InvoiceEntry {
    let weekday = Weekday::Sun;
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();

    let total_produced = PowerTotals::from_periods(0.0, 0.0, 247.0);

    let to_grid = PowerTotals::from_periods(0.0, 0.0, 229.0);

    // total_export = 229.14

    let total_consumed = PowerTotals::from_periods(0.0, 0.0, 18.0);

    let consumed = 18.0;
    let produced = 247.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

// /////////////////////////////

pub(crate) fn _dummy_data_04() -> Vec<Data> {
    let weekday = Weekday::Mon;
    let date = NaiveDate::from_ymd_opt(2023, 2, 20).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.01,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 1.09,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 8.48,
            export: 3.51,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 25.49,
            export: 19.78,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 1.02,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 43.82,
            export: 38.66,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 39.56,
            export: 34.01,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 34.27,
            export: 28.79,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 27.66,
            export: 22.18,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 4.41,
            export: 1.83,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_04() -> InvoiceEntry {
    let weekday = Weekday::Mon;
    let date = NaiveDate::from_ymd_opt(2023, 2, 20).unwrap();

    let total_produced = PowerTotals::from_periods(35.0, 151.0, 0.0);
    let to_grid = PowerTotals::from_periods(23.0, 125.0, 0.0);

    // total_export = 148.76

    let total_consumed = PowerTotals::from_periods(12.0, 26.0, 0.0);

    let consumed = 38.0;
    let produced = 186.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

// /////////////////////////////

pub(crate) fn _dummy_data_05() -> Vec<Data> {
    let weekday = Weekday::Tue;
    let date = NaiveDate::from_ymd_opt(2023, 2, 21).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 52.97,
            export: 47.48,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 51.96,
            export: 46.5,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 48.86,
            export: 43.42,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 28.19,
            export: 24.8,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 0.31,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 1.2,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 1.69,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_05() -> InvoiceEntry {
    let weekday = Weekday::Tue;
    let date = NaiveDate::from_ymd_opt(2023, 2, 21).unwrap();

    let total_produced = PowerTotals::from_periods(0.0, 185.0, 0.0);

    let to_grid = PowerTotals::from_periods(0.0, 162.0, 0.0);

    // total_export = 162.2

    let total_consumed = PowerTotals::from_periods(0.0, 23.0, 0.0);

    let consumed = 23.0;
    let produced = 185.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

/////////////////////////////

pub(crate) fn _dummy_data_06() -> Vec<Data> {
    let weekday = Weekday::Wed;
    let date = NaiveDate::from_ymd_opt(2023, 2, 22).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 51.26,
            export: 49.83,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 51.99,
            export: 35.71,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 50.7,
            export: 34.2,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 47.89,
            export: 31.75,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 33.15,
            export: 21.38,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 46.85,
            export: 17.8,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 1.05,
            export: 0.04,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 0.5,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 3.29,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_06() -> InvoiceEntry {
    let weekday = Weekday::Wed;
    let date = NaiveDate::from_ymd_opt(2023, 2, 22).unwrap();

    let total_produced = PowerTotals::from_periods(34.0, 253.0, 0.0);

    let to_grid = PowerTotals::from_periods(21.0, 169.0, 0.0);

    // total_export = 190.71

    let total_consumed = PowerTotals::from_periods(13.0, 84.0, 0.0);

    let consumed = 97.0;
    let produced = 287.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

/////////////////////////////

pub(crate) fn _dummy_data_07() -> Vec<Data> {
    let weekday = Weekday::Thu;
    let date = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 50.94,
            export: 21.4,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 49.91,
            export: 20.46,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 49.96,
            export: 20.43,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 46.47,
            export: 16.83,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 45.76,
            export: 16.38,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 29.62,
            export: 6.71,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 10.0,
            export: 2.54,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 10.08,
            export: 0.89,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 1.01,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.01,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.01,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 1.8,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 0.78,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_07() -> InvoiceEntry {
    let weekday = Weekday::Thu;
    let date = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();

    let total_produced = PowerTotals::from_periods(32.0, 264.0, 0.0);

    let to_grid = PowerTotals::from_periods(7.0, 99.0, 0.0);

    // total_export = 105.64

    let total_consumed = PowerTotals::from_periods(25.0, 165.0, 0.0);

    let consumed = 190.0;
    let produced = 296.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

/////////////////////////////

pub(crate) fn _dummy_data_08() -> Vec<Data> {
    let weekday = Weekday::Fri;
    let date = NaiveDate::from_ymd_opt(2023, 3, 10).unwrap();

    let peak = Period::Peak;
    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 27.56,
            export: 27.28,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 24.62,
            export: 24.27,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 10.44,
            export: 10.16,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.61,
            export: 0.43,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 45.71,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 36.7,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 21.39,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 49.83,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 7.67,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 6.25,
            export: 0.0,
            period: peak,
        },
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 50.66,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_08() -> InvoiceEntry {
    let weekday = Weekday::Fri;
    let date = NaiveDate::from_ymd_opt(2023, 3, 10).unwrap();

    let total_produced = PowerTotals::from_periods(65.0, 216.0, 0.0);

    let to_grid = PowerTotals::from_periods(0.0, 62.0, 0.0);

    // total_export = 62.14

    let total_consumed = PowerTotals::from_periods(65.0, 154.0, 0.0);

    let consumed = 219.0;
    let produced = 281.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

// /////////////////////////////

pub(crate) fn _dummy_data_09() -> Vec<Data> {
    let weekday = Weekday::Sat;
    let date = NaiveDate::from_ymd_opt(2023, 3, 11).unwrap();

    let off_peak = Period::OffPeak;
    let standard = Period::Standard;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.34,
            export: 0.22,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 5.94,
            export: 5.7,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 5.98,
            export: 5.74,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 20.48,
            export: 20.2,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 23.27,
            export: 23.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 36.3,
            export: 36.0,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 36.68,
            export: 36.41,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 45.23,
            export: 44.93,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 45.31,
            export: 44.97,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 49.87,
            export: 49.56,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 50.4,
            export: 50.11,
            period: standard,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 50.46,
            export: 50.19,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_09() -> InvoiceEntry {
    let weekday = Weekday::Sat;
    let date = NaiveDate::from_ymd_opt(2023, 3, 11).unwrap();

    let total_produced = PowerTotals::from_periods(0.0, 159.0, 211.0);

    let to_grid = PowerTotals::from_periods(0.0, 157.0, 210.0);

    // total_export = 367.03

    let total_consumed = PowerTotals::from_periods(0.0, 2.0, 1.0);

    let consumed = 3.0;
    let produced = 370.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}

// /////////////////////////////

pub(crate) fn _dummy_data_10() -> Vec<Data> {
    let weekday = Weekday::Sun;
    let date = NaiveDate::from_ymd_opt(2023, 3, 12).unwrap();

    let off_peak = Period::OffPeak;

    let data: Vec<Data> = vec![
        Data {
            weekday,
            date,
            hour: 22,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 1,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 20,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 21,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 2,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 14,
            inverter_yield: 0.01,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 4,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 13,
            inverter_yield: 1.27,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 5,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 23,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 3,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 19,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 0,
            inverter_yield: 0.0,
            export: 0.0,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 18,
            inverter_yield: 0.5,
            export: 0.16,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 6,
            inverter_yield: 0.49,
            export: 0.33,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 7,
            inverter_yield: 5.24,
            export: 4.92,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 17,
            inverter_yield: 9.66,
            export: 8.96,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 8,
            inverter_yield: 23.28,
            export: 22.91,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 16,
            inverter_yield: 23.94,
            export: 23.17,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 15,
            inverter_yield: 32.35,
            export: 31.83,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 10,
            inverter_yield: 33.9,
            export: 33.48,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 9,
            inverter_yield: 37.01,
            export: 36.55,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 11,
            inverter_yield: 49.11,
            export: 48.69,
            period: off_peak,
        },
        Data {
            weekday,
            date,
            hour: 12,
            inverter_yield: 51.37,
            export: 50.91,
            period: off_peak,
        },
    ];

    data
}

pub(crate) fn _dummy_entry_10() -> InvoiceEntry {
    let weekday = Weekday::Sun;
    let date = NaiveDate::from_ymd_opt(2023, 3, 12).unwrap();

    let total_produced = PowerTotals::from_periods(0.0, 0.0, 268.0);

    let to_grid = PowerTotals::from_periods(0.0, 0.0, 262.0);

    // total_export = 261.91

    let total_consumed = PowerTotals::from_periods(0.0, 0.0, 6.0);

    let consumed = 6.0;
    let produced = 268.0;

    let entry = InvoiceEntry::new(
        weekday,
        date,
        total_produced,
        to_grid,
        total_consumed,
        consumed,
        produced,
    );

    entry
}
