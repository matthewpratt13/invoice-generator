pub mod daily_schedule;
pub mod invoice_entry;
pub mod invoice_totals;
pub mod power_totals;
pub mod schedule;

mod hours;
mod record;

use self::daily_schedule::DailySchedule;
use self::record::Record;
use self::schedule::Schedule;

use calamine::*; // to work with excel files

// to work with date and time value types (instead of string references)
use chrono::{Datelike, NaiveDate, Timelike, Weekday};

use itertools::Itertools; // to iterate through, map and sort data

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;

#[allow(dead_code)] // remove rust-analyzer warnings
pub type DailyData = HashMap<NaiveDate, Vec<Data>>; // for convenience

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Period {
    Peak,
    Standard,
    OffPeak,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Data {
    pub weekday: Weekday,    // Mon, Tue, Wed … (enum)
    pub date: NaiveDate,     // yyyy-mm-dd
    pub hour: u32,           // 0 is midnight … 23 is 11 p.m.
    pub inverter_yield: f64, // summed to `total_produced` in `invoice_entry.rs`
    pub export: f64,         // summed to `to_grid` in `invoice_entry.rs`
    pub period: Period,      // Peak, Standard or Off-peak
}

#[allow(dead_code)] // remove rust-analyzer warnings
impl Data {
    fn build(
        weekday: Weekday,
        record: Record,                // raw data from original summaries
        daily_schedule: DailySchedule, // { Weekday -> Schedule }
    ) -> Result<Self, Box<dyn Error>> {
        let date: NaiveDate = record.naive_date_time()?.date();
        let hour: u32 = record.naive_date_time()?.hour();
        let inverter_yield: f64 = record.inverter_yield();
        let export: f64 = record.export();

        let period: Period = period(hour, weekday, &daily_schedule)?;

        Ok(Self {
            weekday,
            date,
            hour,
            inverter_yield,
            export,
            period,
        })
    }
}

#[allow(dead_code)] // remove rust-analyzer warnings
pub fn from_xls(
    path: &PathBuf, // path to summaries
    daily_schedule: &DailySchedule,
) -> Result<Vec<DailyData>, Box<dyn Error>> {
    let file: File = OpenOptions::new().read(true).open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut workbook: Xlsx<BufReader<File>> = Xlsx::new(reader)?;

    let mut data_vec: Vec<Data> = Vec::new();

    let mut range: Range<DataType> = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("cannot find 'Sheet1'"))??;

    let start_index: (u32, u32) = range
        .start()
        .ok_or("unable to get workbook start cell index")?;

    let end_index: (u32, u32) = range.end().ok_or("unable to get workbook end cell index")?;

    let start_value: &DataType = range
        .get_value(start_index)
        .ok_or("unable to get start value")?;

    let new_start_index: (u32, u32) = (start_index.0 + 1, start_index.1); // A2

    if start_value.to_string() != "Statistical Period" {
        range = range.range(new_start_index, end_index);
    }

    let iter: RangeDeserializer<DataType, Record> = RangeDeserializerBuilder::with_headers(&[
        "Statistical Period",
        "Inverter Yield (kWh)",
        "Export (kWh)",
    ])
    .from_range(&range)?;

    for result in iter {
        let record: Record = result?; // { date_time_string, inverter_yield, export }
        let weekday: Weekday = record.naive_date_time()?.weekday();

        let data = Data::build(weekday, record, daily_schedule.clone())?;

        data_vec.push(data);
        println!("{:?}", &data_vec);
    }

    let mut days: Vec<DailyData> = Vec::new();

    // map Vec<Data> to a NaiveDate and sort by date
    let day: DailyData = data_vec.into_iter().into_group_map_by(|data| data.date);

    days.push(day);

    Ok(days)
}

// determine period (Peak, Standard, OffPeak) for a given hour on a given day,
// according to the schedule
fn period(
    hour: u32,
    weekday: Weekday,
    daily_schedule: &DailySchedule,
) -> Result<Period, Box<dyn Error>> {
    let msg: String = format!(
        "Format error: expected schedule for {}, but got none",
        &weekday
    );

    let err = Box::<dyn Error>::from(msg.as_str());

    let schedule: Schedule = daily_schedule.get(&weekday).ok_or(err)?.clone();

    let period: Period = match weekday {
        Weekday::Mon => match_hour(weekday, hour, schedule)?,
        Weekday::Tue => match_hour(weekday, hour, schedule)?,
        Weekday::Wed => match_hour(weekday, hour, schedule)?,
        Weekday::Thu => match_hour(weekday, hour, schedule)?,
        Weekday::Fri => match_hour(weekday, hour, schedule)?,
        Weekday::Sat => match_hour(weekday, hour, schedule)?,
        Weekday::Sun => match_hour(weekday, hour, schedule)?,
    };

    Ok(period)
}

// helper
fn match_hour(weekday: Weekday, hour: u32, schedule: Schedule) -> Result<Period, Box<dyn Error>> {
    let peak_hours: Option<Vec<u32>> = schedule.peak_hours;
    let off_peak_hours: Vec<u32> = schedule.off_peak_hours;
    let sec_peak_hours: Option<Vec<u32>> = schedule.sec_peak_hours;
    let sec_off_peak_hours: Option<Vec<u32>> = schedule.sec_off_peak_hours;

    match hour {
        _ if is_in_range(hour, peak_hours, "peak", weekday)? => Ok(Period::Peak),
        _ if is_in_range(hour, Some(off_peak_hours), "off_peak", weekday)? => Ok(Period::OffPeak),
        _ if is_in_range(hour, sec_peak_hours, "secondary_peak", weekday)? => Ok(Period::Peak),
        _ if is_in_range(hour, sec_off_peak_hours, "secondary_off_peak", weekday)? => {
            Ok(Period::OffPeak)
        }
        _ => Ok(Period::Standard), // default variant
    }
}

fn is_in_range(
    val: u32,
    vec: Option<Vec<u32>>,
    period: &str,
    weekday: Weekday,
) -> Result<bool, Box<dyn Error>> {
    let msg: String = format!("Format error: invalid {}_start and/or {}_end hour value in schedule for {}. Hours must be within range 0–23 (inclusive)", period, period, weekday);

    if let Some(v) = &vec {
        let first: u32 = v[0];
        let last: u32 = v[1];

        if first < last {
            if val >= first && val < last {
                Ok(true)
            } else {
                Ok(false)
            }
        } else if first >= last {
            if val >= first || val < last {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(From::from(msg.as_str()))
        }
    } else {
        Ok(false)
    }
}
