pub mod data; // expose back end

pub use crate::data::daily_schedule::{self, DailySchedule};
pub use crate::data::invoice_entry::{self, InvoiceEntry};
pub use crate::data::invoice_totals::InvoiceTotals;
pub use crate::data::DailyData; // HashMap<NaiveDate, Vec<Data>>

use excel::*; // `simple_excel_writer` crate

use std::error::Error;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub fn invoice_data(
    xls_path: PathBuf,
    hours_path: PathBuf,
) -> Result<(Vec<InvoiceEntry>, InvoiceTotals), Box<dyn Error>> {
    let hours_rdr: csv::Reader<File> = self::read_file(&hours_path)?;

    // hours collected a new data type
    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;
    // records collected into a HashMap mapping dates to data
    let data: Vec<DailyData> = data::from_xls(&xls_path, &schedule)?;

    // printed data types
    let invoice_entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(data)?;
    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&invoice_entries)?; // last row

    let invoice_data: (Vec<InvoiceEntry>, InvoiceTotals) =
        (invoice_entries.clone(), invoice_totals.clone());

    Ok(invoice_data)
}

pub fn read_file(path: &PathBuf) -> Result<csv::Reader<File>, std::io::Error> {
    let file: File = OpenOptions::new().read(true).open(path)?; // open summaries

    let rdr: csv::Reader<File> = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    Ok(rdr)
}

pub fn write_to_xls(
    invoice_entries: &Vec<InvoiceEntry>,
    invoice_totals: &InvoiceTotals,
    mut path: PathBuf,
) {
    if path.as_path().extension() != Some(&OsString::from("xlsx")) {
        path.set_extension("xlsx");
    }

    let file: &str = path.as_path().to_str().expect("invalid path");

    let mut wb: excel::Workbook = Workbook::create(&file);
    let mut sheet: excel::Sheet = wb.create_sheet("Sheet1");

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw: &mut excel::SheetWriter = sheet_writer;

        sw.append_row(row![
            "".to_string(),
            "".to_string(),
            String::from("Total Produced"),
            "".to_string(),
            "".to_string(),
            String::from("To Grid"),
            "".to_string(),
            "".to_string(),
            String::from("Total Consumed"),
            "".to_string(),
            "".to_string(),
            String::from("Consumed"),
            String::from("Produced")
        ])?;

        sw.append_row(row![
            // blank cells
            "".to_string(),
            "".to_string(),
            String::from("Peak (kWh)"),
            String::from("Std (kWh)"),
            String::from("Off-Peak (kWh)"),
            String::from("Peak (kWh)"),
            String::from("Std (kWh)"),
            String::from("Off-Peak (kWh)"),
            String::from("Peak (kWh)"),
            String::from("Std (kWh)"),
            String::from("Off-Peak (kWh)"),
            // blank cells
            "".to_string(),
            "".to_string()
        ])?;

        sw.merge_range("A1".to_string(), "A2".to_string())?;
        sw.merge_range("B1".to_string(), "B2".to_string())?;
        sw.merge_range("C1".to_string(), "E1".to_string())?;
        sw.merge_range("F1".to_string(), "H1".to_string())?;
        sw.merge_range("I1".to_string(), "K1".to_string())?;
        sw.merge_range("L1".to_string(), "L2".to_string())?;
        sw.merge_range("M1".to_string(), "M2".to_string())?;

        for e in invoice_entries {
            sw.append_row(row![
                e.weekday().to_string(),
                e.date().to_string(),
                e.total_produced().peak_kwh(),
                e.total_produced().standard_kwh(),
                e.total_produced().off_peak_kwh(),
                e.to_grid().peak_kwh(),
                e.to_grid().standard_kwh(),
                e.to_grid().off_peak_kwh(),
                e.total_consumed().peak_kwh(),
                e.total_consumed().standard_kwh(),
                e.total_consumed().off_peak_kwh(),
                e.consumed(),
                e.produced()
            ])
            .expect("unable to serialize invoice entry");
        }

        sw.append_row(row![
            "".to_string(),
            "".to_string(),
            invoice_totals.total_produced().peak_kwh(),
            invoice_totals.total_produced().standard_kwh(),
            invoice_totals.total_produced().off_peak_kwh(),
            invoice_totals.to_grid().peak_kwh(),
            invoice_totals.to_grid().standard_kwh(),
            invoice_totals.to_grid().off_peak_kwh(),
            invoice_totals.total_consumed().peak_kwh(),
            invoice_totals.total_consumed().standard_kwh(),
            invoice_totals.total_consumed().off_peak_kwh(),
            invoice_totals.consumed(),
            invoice_totals.produced()
        ])
    })
    .expect("unable to write to Excel workbook");

    wb.close().expect("unable to close Excel workbook");
}
