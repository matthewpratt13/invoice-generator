use chrono::NaiveDateTime;
use serde::Deserialize;

use std::error::Error;

#[derive(Clone, PartialEq, Deserialize)]
pub(crate) struct Record {
    #[serde(rename = "Statistical Period")] // original date and time
    pub(crate) date_time_string: String,

    #[serde(rename = " Inverter Yield (kWh)")] // original heading (has initial space)
    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) inverter_yield: Option<f64>, // produced

    #[serde(rename = "Export (kWh)")] // original heading
    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) export: Option<f64>, // to grid
}

impl Record {
    // convert to type that can be manipulated
    pub(crate) fn naive_date_time(&self) -> Result<NaiveDateTime, Box<dyn Error>> {
        let dt_str: &str = self.date_time_string.as_str();
        let msg: String = format!("Format error: unable to parse `{}` into NaiveDateTime. Expected a valid date_time_string in format `%Y-%m-%d %H:%M:%S`", dt_str);

        if let Ok(dt) = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt)
        } else {
            Err(From::from(msg.as_str()))
        }
    }

    // get inverter yield or return 0.0 if value is None
    pub(crate) fn inverter_yield(&self) -> f64 {
        let inverter_yield: f64 = self.inverter_yield.unwrap_or_else(|| 0.0);
        inverter_yield
    }

    // get export or return if value in None
    pub(crate) fn export(&self) -> f64 {
        let export: f64 = self.export.unwrap_or_else(|| 0.0);
        export
    }
}
