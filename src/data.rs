pub mod daily_schedule;
pub mod invoice_entry;
pub mod invoice_totals;
pub mod power_totals;

mod hours;
mod record;

use self::daily_schedule::DailySchedule;
use self::record::Record;

use calamine::*; // crate to work with excel files

use chrono::{Datelike, NaiveDate, Timelike, Weekday};

use itertools::Itertools;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;

#[allow(dead_code)]
pub type DailyData = HashMap<NaiveDate, Vec<Data>>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Period {
    Peak,
    Standard, // automatically calculated
    OffPeak,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Data {
    pub weekday: Weekday,    // Mon, Tue, Wed …
    pub date: NaiveDate,     // yyyy-mm-dd
    pub hour: u32,           // 0 is midnight … 23 is 11 p.m.
    pub inverter_yield: f64, // summed to `total_produced` in `invoice_entry.rs`
    pub export: f64,         // summed to `to_grid` in `invoice_entry.rs`
    pub period: Period,      // Peak, Standard or Off-peak
}

#[allow(dead_code)]
impl Data {
    fn build(
        weekday: Weekday,
        record: Record, // raw data from original summaries
        // Weekday -> ( Vec<start_hour, end_hour> per period )
        schedule: DailySchedule,
    ) -> Result<Self, Box<dyn Error>> {
        let date: NaiveDate = record.naive_date_time()?.date();
        let hour: u32 = record.naive_date_time()?.hour();
        let inverter_yield: f64 = record.inverter_yield();
        let export: f64 = record.export();

        let period: Period = self::period(hour, weekday, &schedule)?;

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

#[allow(dead_code)]
pub fn from_xls(
    path: &PathBuf, // path to summaries
    schedule: &DailySchedule,
) -> Result<Vec<DailyData>, Box<dyn Error>> {
    let file: File = OpenOptions::new().read(true).open(path)?;
    let rdr: BufReader<File> = BufReader::new(file);
    let mut workbook: Xlsx<BufReader<File>> = Xlsx::new(rdr)?;

    let mut data_vec: Vec<Data> = Vec::new();

    let mut range: Range<DataType> = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("cannot find 'Sheet1'"))??;

    let start: &DataType = range
        .get_value(range.start().ok_or("missing start value")?)
        .ok_or("unable to get start value")?;

    let end_index: (u32, u32) = range.end().ok_or("unable to get workbook end cell index")?;

    let new_start_index: (u32, u32) = (1, 0);

    if let Some(t) = start.get_string() {
        if t != "Statistical Period" {
            range = range.range(new_start_index, end_index);
        }
    }

    let iter: RangeDeserializer<DataType, Record> = RangeDeserializerBuilder::with_headers(&[
        "Statistical Period",
        "Inverter Yield (kWh)",
        "Export (kWh)",
    ])
    .from_range(&range)?;

    for result in iter {
        let record: Record = result?;
        let weekday: Weekday = record.naive_date_time()?.weekday();

        let data = Data::build(weekday, record, schedule.clone())?;

        data_vec.push(data);
    }

    let mut days: Vec<DailyData> = Vec::new();

    let day: DailyData = data_vec.into_iter().into_group_map_by(|data| data.date);

    days.push(day);

    Ok(days)
}

fn period(hour: u32, weekday: Weekday, schedule: &DailySchedule) -> Result<Period, Box<dyn Error>> {
    let msg: String = format!(
        "Format error: expected schedule for {}, but got none",
        &weekday
    );

    let err = Box::<dyn Error>::from(msg.as_str());

    let times: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = schedule.get(&weekday).ok_or(err)?.clone();

    let period: Period = match weekday {
        Weekday::Mon => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Tue => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Wed => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Thu => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Fri => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Sat => match_hour(&weekday.to_string(), hour, times)?,
        Weekday::Sun => match_hour(&weekday.to_string(), hour, times)?,
    };

    Ok(period)
}

fn match_hour(
    day: &str,
    hour: u32,
    times: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ),
) -> Result<Period, Box<dyn Error>> {
    let peak_hours: Option<Vec<u32>> = times.0;
    let off_peak_hours: Vec<u32> = times.1;
    let sec_peak_hours: Option<Vec<u32>> = times.2;
    let sec_off_peak_hours: Option<Vec<u32>> = times.3;

    match hour {
        _ if is_in_range(hour, peak_hours, "peak", day)? => Ok(Period::Peak),
        _ if is_in_range(hour, Some(off_peak_hours), "off_peak", day)? => Ok(Period::OffPeak),
        _ if is_in_range(hour, sec_peak_hours, "secondary_peak", day)? => Ok(Period::Peak),
        _ if is_in_range(hour, sec_off_peak_hours, "secondary_off_peak", day)? => {
            Ok(Period::OffPeak)
        }
        _ => Ok(Period::Standard),
    }
}

fn is_in_range(
    val: u32,
    vec: Option<Vec<u32>>,
    period: &str,
    day: &str,
) -> Result<bool, Box<dyn Error>> {
    let msg: String = format!("Format error: invalid {}_start and/or {}_end hour value in schedule for {}. Hours must be within range 0–23 (inclusive)", period, period, day);

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
