use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Schedule {
    pub row_headers: Vec<String>,
    pub mon_hours: Vec<String>,
    pub tue_hours: Vec<String>,
    pub wed_hours: Vec<String>,
    pub thu_hours: Vec<String>,
    pub fri_hours: Vec<String>,
    pub sat_hours: Vec<String>,
    pub sun_hours: Vec<String>,
}

impl Schedule {
    pub fn new(row_headers: Vec<String>, rows: Vec<Option<Vec<String>>>) -> Self {
        let mon_hours: &Vec<String> = rows[0].as_ref().expect("no hours (Vec<String>) at index 0");
        let tue_hours: &Vec<String> = rows[1].as_ref().expect("no hours (Vec<String>) at index 1");
        let wed_hours: &Vec<String> = rows[2].as_ref().expect("no hours (Vec<String>) at index 2");
        let thu_hours: &Vec<String> = rows[3].as_ref().expect("no hours (Vec<String>) at index 3");
        let fri_hours: &Vec<String> = rows[4].as_ref().expect("no hours (Vec<String>) at index 4");
        let sat_hours: &Vec<String> = rows[5].as_ref().expect("no hours (Vec<String>) at index 5");
        let sun_hours: &Vec<String> = rows[6].as_ref().expect("no hours (Vec<String>) at index 6");

        Self {
            row_headers,
            mon_hours: mon_hours.clone(),
            tue_hours: tue_hours.clone(),
            wed_hours: wed_hours.clone(),
            thu_hours: thu_hours.clone(),
            fri_hours: fri_hours.clone(),
            sat_hours: sat_hours.clone(),
            sun_hours: sun_hours.clone(),
        }
    }
}
