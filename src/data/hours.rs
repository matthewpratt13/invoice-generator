use chrono::Weekday;
use serde::Deserialize;

use std::error::Error;

#[derive(Clone, Deserialize)]
pub struct Hours {
    #[serde(rename = "Weekday")]
    weekday_string: String,

    // can be None
    #[serde(deserialize_with = "csv::invalid_option", rename = "Peak Start")]
    peak_start: Option<u32>,

    // can be None
    #[serde(deserialize_with = "csv::invalid_option", rename = "Peak End")]
    peak_end: Option<u32>,

    // CANNOT be None
    #[serde(rename = "Off-peak Start")]
    off_peak_start: u32,

    // CANNOT be None
    #[serde(rename = "Off-peak End")]
    off_peak_end: u32,

    // can be None
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename = "Secondary Peak Start"
    )]
    secondary_peak_start: Option<u32>,

    // can be None
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename = "Secondary Peak End"
    )]
    secondary_peak_end: Option<u32>,

    // can be None
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename = "Secondary Off-peak Start"
    )]
    secondary_off_peak_start: Option<u32>,

    // can be None
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename = "Secondary Off-peak End"
    )]
    secondary_off_peak_end: Option<u32>,
}

impl Hours {
    // convert to type that can be manipulated
    pub(crate) fn weekday_enum(&self) -> Result<Weekday, Box<dyn Error>> {
        let weekday_string: String = self.weekday_string.to_lowercase();

        let msg: String = format!(
        "Format error: unable to parse `{}` into Weekday. Expected weekday_string that is either written in full (i.e., `Monday`, `Tuesday`, `Wednesday`, etc.) or in its shortest form (i.e., `Mon`, `Tue`, `Wed`, etc.)",
        self.weekday_string
    );

        if let Ok(d) = weekday_string.parse::<Weekday>() {
            Ok(d)
        } else {
            Err(From::from(msg))
        }
    }

    // getters:

    pub(crate) fn peak_hours(&self) -> Result<Option<Vec<u32>>, Box<dyn Error>> {
        let peak_start: Option<u32> = self.peak_start;
        let peak_end: Option<u32> = self.peak_end;

        let period = "peak";
        let weekday: Weekday = self.weekday_enum()?;

        let peak_hours: Option<Vec<u32>> = hours(peak_start, peak_end, period, weekday)?;
        Ok(peak_hours)
    }

    // return vector with off-peak start and end hours
    pub(crate) fn off_peak_hours(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        let off_peak_start: Option<u32> = Some(self.off_peak_start);
        let off_peak_end: Option<u32> = Some(self.off_peak_end);

        let period = "off_peak";
        let weekday: Weekday = self.weekday_enum()?;

        let off_peak_hours: Vec<u32> =
            hours(off_peak_start, off_peak_end, period, weekday)?.unwrap();

        Ok(off_peak_hours)
    }

    pub(crate) fn sec_peak_hours(&self) -> Result<Option<Vec<u32>>, Box<dyn Error>> {
        let secondary_peak_start: Option<u32> = self.secondary_peak_start;
        let secondary_peak_end: Option<u32> = self.secondary_peak_end;

        let period = "secondary_peak";
        let weekday: Weekday = self.weekday_enum()?;

        let secondary_peak_hours: Option<Vec<u32>> =
            hours(secondary_peak_start, secondary_peak_end, period, weekday)?;

        Ok(secondary_peak_hours)
    }

    pub(crate) fn sec_off_peak_hours(&self) -> Result<Option<Vec<u32>>, Box<dyn Error>> {
        let secondary_off_peak_start: Option<u32> = self.secondary_off_peak_start;
        let secondary_off_peak_end: Option<u32> = self.secondary_off_peak_end;

        let period = "secondary_off_peak";
        let weekday: Weekday = self.weekday_enum()?;

        let secondary_off_peak_hours: Option<Vec<u32>> = hours(
            secondary_off_peak_start,
            secondary_off_peak_end,
            period,
            weekday,
        )?;

        Ok(secondary_off_peak_hours)
    }
}

// check if hours are valid and return vector with start and end hours
fn hours(
    start: Option<u32>,
    end: Option<u32>,
    period: &str,
    weekday: Weekday,
) -> Result<Option<Vec<u32>>, Box<dyn Error>> {
    let msg: String = format!("Format error: invalid {}_start and/or {}_end hour value in schedule for {}. Hours must be within range 0–23 (inclusive)", period, period, weekday);

    let mut hours: Vec<u32> = Vec::new();

    if let Some(s) = start {
        if s > 23 {
            return Err(From::from(msg));
        }

        if let Some(e) = end {
            if e > 23 {
                return Err(From::from(msg));
            } else {
                hours.push(s);
                hours.push(e);
            }
        }
    } else {
        return Ok(None);
    }

    Ok(Some(hours))
}
