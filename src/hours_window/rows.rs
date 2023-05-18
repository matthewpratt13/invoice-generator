use super::hours_grid::HoursGrid;
use super::{HoursRow, RowHeaders};

use adw::prelude::*;

use std::error::Error;

#[derive(Debug, Clone)]
pub struct Rows {
    mon_row: HoursRow,
    tue_row: HoursRow,
    wed_row: HoursRow,
    thu_row: HoursRow,
    fri_row: HoursRow,
    sat_row: HoursRow,
    sun_row: HoursRow,
}

impl Rows {
    pub fn new(hours_rows: Vec<HoursRow>) -> Self {
        let mon_row: HoursRow = hours_rows[0].clone();
        let tue_row: HoursRow = hours_rows[1].clone();
        let wed_row: HoursRow = hours_rows[2].clone();
        let thu_row: HoursRow = hours_rows[3].clone();
        let fri_row: HoursRow = hours_rows[4].clone();
        let sat_row: HoursRow = hours_rows[5].clone();
        let sun_row: HoursRow = hours_rows[6].clone();

        Self {
            mon_row,
            tue_row,
            wed_row,
            thu_row,
            fri_row,
            sat_row,
            sun_row,
        }
    }

    fn hours_rows(&self) -> Vec<HoursRow> {
        let mut hours_rows: Vec<HoursRow> = Vec::new();

        hours_rows.push(self.mon_row.clone());
        hours_rows.push(self.tue_row.clone());
        hours_rows.push(self.wed_row.clone());
        hours_rows.push(self.thu_row.clone());
        hours_rows.push(self.fri_row.clone());
        hours_rows.push(self.sat_row.clone());
        hours_rows.push(self.sun_row.clone());

        hours_rows
    }

    pub fn to_schedule(&self, row_headers: RowHeaders) -> Result<HoursGrid, Box<dyn Error>> {
        let row_headers_strings: Vec<String> = row_headers.to_headers_strings();

        let hours_rows: Vec<HoursRow> = self.hours_rows();

        let mut rows_strings: Vec<Option<Vec<String>>> = Vec::new();

        for hr in hours_rows.iter() {
            let mut row: Vec<String> = Vec::new();
            let weekday_string: String = hr.weekday_label().label().to_string();

            row.push(weekday_string);

            hr.hours_entries()
                .snapshot()
                .iter()
                .filter_map(Cast::downcast_ref::<gtk::Entry>)
                .map(|e| e.text().to_string())
                .for_each(|s| row.push(s));

            rows_strings.push(Some(row));
        }

        Ok(HoursGrid::new(row_headers_strings, rows_strings))
    }
}
