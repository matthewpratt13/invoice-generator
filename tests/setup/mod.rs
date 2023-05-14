pub mod test_data;
pub use invoice_generator::data::{self, DailyData, Data, Period};
pub use invoice_generator::data::{
    daily_schedule::{self, DailySchedule},
    invoice_entry::{self, InvoiceEntry},
    invoice_totals::{self, InvoiceTotals},
    power_totals::PowerTotals,
};

pub use chrono::{NaiveDate, Weekday};
pub use csv::Reader;

use std::path::PathBuf;
pub use std::{error::Error, fs::File, path::Path};

pub struct SetupData {
    pub xls_path: PathBuf,
    pub daily_schedule: DailySchedule,
    pub daily_data: Vec<DailyData>,
}

impl SetupData {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let xls_path =
            PathBuf::from("res/summaries/plant-01/combined/plant_01_combined_without_title.xlsx");

        let hours_rdr: Reader<File> = setup_hours()?;

        let daily_schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;
        let daily_data: Vec<DailyData> = data::from_xls(&xls_path, &daily_schedule)?;

        Ok(Self {
            xls_path,
            daily_schedule,
            daily_data,
        })
    }
}

pub fn setup_hours() -> Result<Reader<File>, Box<dyn Error>> {
    let hours_path = Path::new("res/hours.csv");

    let hours_data: File = match hours_path.exists() {
        true => std::fs::OpenOptions::new().read(true).open(hours_path)?,
        false => panic!("hours_path does not exist"),
    };

    let hours_rdr: Reader<File> = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(hours_data);

    Ok(hours_rdr)
}

#[allow(dead_code)]
pub fn setup_entries() -> Result<Vec<InvoiceEntry>, Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let data: Vec<DailyData> = setup_data.daily_data;

    let invoice_entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(data)?;

    Ok(invoice_entries)
}
