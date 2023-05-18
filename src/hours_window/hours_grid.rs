use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HoursGrid {
    pub row_headers: Vec<String>,
    pub mon_hours: Vec<String>,
    pub tue_hours: Vec<String>,
    pub wed_hours: Vec<String>,
    pub thu_hours: Vec<String>,
    pub fri_hours: Vec<String>,
    pub sat_hours: Vec<String>,
    pub sun_hours: Vec<String>,
}

impl HoursGrid {
    pub fn new(row_headers: Vec<String>, rows: Vec<Option<Vec<String>>>) -> Self {
        let mon_hours: Vec<String> = rows[0]
            .as_ref()
            .expect("no hours (Vec<String>) at index 0")
            .clone();
        let tue_hours: Vec<String> = rows[1]
            .as_ref()
            .expect("no hours (Vec<String>) at index 1")
            .clone();
        let wed_hours: Vec<String> = rows[2]
            .as_ref()
            .expect("no hours (Vec<String>) at index 2")
            .clone();
        let thu_hours: Vec<String> = rows[3]
            .as_ref()
            .expect("no hours (Vec<String>) at index 3")
            .clone();
        let fri_hours: Vec<String> = rows[4]
            .as_ref()
            .expect("no hours (Vec<String>) at index 4")
            .clone();
        let sat_hours: Vec<String> = rows[5]
            .as_ref()
            .expect("no hours (Vec<String>) at index 5")
            .clone();
        let sun_hours: Vec<String> = rows[6]
            .as_ref()
            .expect("no hours (Vec<String>) at index 6")
            .clone();

        Self {
            row_headers,
            mon_hours,
            tue_hours,
            wed_hours,
            thu_hours,
            fri_hours,
            sat_hours,
            sun_hours,
        }
    }
}
