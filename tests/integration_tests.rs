mod setup;

use crate::setup::*;

#[test]
#[ignore]
fn test_hours_setup() {
    if let Ok(r) = setup_hours() {
        println!("{:?}", r)
    } else {
        panic!("unable to initiate hours reader")
    }
}

#[test]
fn it_returns_correct_hours() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;
    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let friday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (Some(vec![7, 10]), vec![22, 6], Some(vec![18, 20]), None);

    let friday_weekday = Weekday::Fri;

    let test_hours = schedule.get(&friday_weekday).unwrap();

    assert_eq!(
        *test_hours, friday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 2, 17).unwrap();

    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 17).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[0];
    let test_entry: InvoiceEntry = test_data::_dummy_entry();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(27.0, 160.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(26.0, 158.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(1.0, 2.0, 0.0);

    let test_consumed = 3.0;
    let test_produced = 187.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_hours_02() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let saturday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (None, vec![20, 7], None, Some(vec![12, 18]));

    let saturday_weekday = Weekday::Sat;

    let test_hours = schedule.get(&saturday_weekday).unwrap();

    assert_eq!(
        *test_hours, saturday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data_02() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_02();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_02() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[1];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_02();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_02() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_02();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(0.0, 32.0, 190.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 30.0, 188.0);

    let test_total_consumed = PowerTotals::from_periods(0.0, 2.0, 2.0);

    let test_consumed = 4.0;
    let test_produced = 222.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_hours_03() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let sunday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (None, vec![0, 0], None, None);

    let sunday_weekday = Weekday::Sun;

    let test_hours = schedule.get(&sunday_weekday).unwrap();

    assert_eq!(
        *test_hours, sunday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data_03() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;
    let date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_03();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_03() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[2];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_03();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_03() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_03();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(0.0, 0.0, 247.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 0.0, 229.0);

    let test_total_consumed = PowerTotals::from_periods(0.0, 0.0, 18.0);

    let test_consumed = 18.0;
    let test_produced = 247.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////
#[test]
fn it_returns_correct_hours_04() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let monday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (Some(vec![7, 10]), vec![22, 6], Some(vec![18, 20]), None);

    let monday_weekday = Weekday::Mon;

    let test_hours = schedule.get(&monday_weekday).unwrap();

    assert_eq!(
        *test_hours, monday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data_04() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;
    let date = NaiveDate::from_ymd_opt(2023, 2, 20).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 20).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_04();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_04() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[3];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_04();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_04() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_04();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(35.0, 151.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(23.0, 125.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(12.0, 26.0, 0.0);

    let test_consumed = 38.0;
    let test_produced = 186.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );

    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////
#[test]
fn it_returns_correct_hours_05() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let tuesday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (Some(vec![7, 10]), vec![22, 6], Some(vec![18, 20]), None);

    let tuesday_weekday = Weekday::Tue;

    let test_hours = schedule.get(&tuesday_weekday).unwrap();

    assert_eq!(
        *test_hours, tuesday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data_05() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 2, 21).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 21).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_05();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]

fn it_returns_correct_entry_05() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[4];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_05();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );

    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_05() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_05();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(0.0, 185.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 162.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(0.0, 23.0, 0.0);

    let test_consumed = 23.0;
    let test_produced = 185.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );

    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

#[test]
fn it_returns_correct_hours_06() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let wednesday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (Some(vec![7, 10]), vec![22, 6], Some(vec![18, 20]), None);

    let wednesday_weekday = Weekday::Wed;

    let test_hours = schedule.get(&wednesday_weekday).unwrap();

    assert_eq!(
        *test_hours, wednesday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_data_06() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 2, 22).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 22).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_06();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_06() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[5];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_06();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );

    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_06() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_06();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(34.0, 253.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(21.0, 169.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(13.0, 84.0, 0.0);

    let test_consumed = 97.0;
    let test_produced = 287.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );

    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

///////////////////////////

#[test]
fn it_returns_correct_hours_07() -> Result<(), Box<dyn Error>> {
    let hours_rdr: Reader<File> = setup_hours()?;

    let schedule: DailySchedule = daily_schedule::from_hours(hours_rdr)?;

    let thursday_hours: (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ) = (Some(vec![7, 10]), vec![22, 6], Some(vec![18, 20]), None);

    let thursday_weekday = Weekday::Thu;

    let test_hours = schedule.get(&thursday_weekday).unwrap();

    assert_eq!(
        *test_hours, thursday_hours,
        "returned incorrect schedule – check hours"
    );

    Ok(())
}

#[test]
fn it_returns_correct_data_07() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_07();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_07() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[6];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_07();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_07() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_07();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(32.0, 264.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(7.0, 99.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(25.0, 165.0, 0.0);

    let test_consumed = 190.0;
    let test_produced = 296.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_data_08() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 3, 10).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 3, 10).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_08();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_08() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[21];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_08();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_08() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_08();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(65.0, 216.0, 0.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 62.0, 0.0);

    let test_total_consumed = PowerTotals::from_periods(65.0, 154.0, 0.0);

    let test_consumed = 219.0;
    let test_produced = 281.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_data_09() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 3, 11).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 3, 11).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_09();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_09() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[22];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_09();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );

    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_09() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_09();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(0.0, 159.0, 211.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 157.0, 210.0);

    let test_total_consumed = PowerTotals::from_periods(0.0, 2.0, 1.0);

    let test_consumed = 3.0;
    let test_produced = 370.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );
    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}

/////////////////////////////

#[test]
fn it_returns_correct_data_10() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let date = NaiveDate::from_ymd_opt(2023, 3, 12).unwrap();
    let data: &Vec<Data> = setup_data.daily_data[0].get(&date).unwrap();

    let test_date = NaiveDate::from_ymd_opt(2023, 3, 12).unwrap();
    let test_data: Vec<Data> = test_data::_dummy_data_10();

    assert_eq!(date, test_date, "incorrectly calculated date");
    assert_eq!(*data, test_data, "incorrectly deserialized record");

    Ok(())
}

#[test]
fn it_returns_correct_entry_10() -> Result<(), Box<dyn Error>> {
    let setup_data = SetupData::new()?;

    let daily_data: Vec<DailyData> = setup_data.daily_data;

    let entries: Vec<InvoiceEntry> = invoice_entry::from_daily_data(daily_data)?;

    let entry: InvoiceEntry = entries[23];
    let test_entry: InvoiceEntry = test_data::_dummy_entry_10();

    assert_eq!(entry.weekday(), test_entry.weekday(), "weekday");
    assert_eq!(entry.weekday(), test_entry.weekday(), "incorrect date");
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated inverter_yield"
    );
    assert_eq!(
        entry.to_grid(),
        test_entry.to_grid(),
        "miscalculated export"
    );
    assert_eq!(
        entry.total_consumed(),
        test_entry.total_consumed(),
        "miscalculated consumption"
    );
    assert_eq!(
        entry.total_produced(),
        test_entry.total_produced(),
        "miscalculated total_produced"
    );
    assert_eq!(
        entry.consumed(),
        test_entry.consumed(),
        "miscalculated total_consumed"
    );

    Ok(())
}

#[test]
fn it_returns_correct_entry_total_10() -> Result<(), Box<dyn Error>> {
    let entry: InvoiceEntry = test_data::_dummy_entry_10();

    let mut entry_vec: Vec<InvoiceEntry> = Vec::new();
    entry_vec.push(entry);

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&entry_vec)?;

    let test_total_produced = PowerTotals::from_periods(0.0, 0.0, 268.0);

    let test_to_grid = PowerTotals::from_periods(0.0, 0.0, 262.0);

    let test_total_consumed = PowerTotals::from_periods(0.0, 0.0, 6.0);

    let test_consumed = 6.0;
    let test_produced = 268.0;

    assert_eq!(
        invoice_totals.total_produced(),
        test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(
        invoice_totals.to_grid(),
        test_to_grid,
        "miscalculated to_grid"
    );

    assert_eq!(
        invoice_totals.total_consumed(),
        test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(
        invoice_totals.consumed(),
        test_consumed,
        "miscalculated `consumed`"
    );
    assert_eq!(
        invoice_totals.produced(),
        test_produced,
        "miscalculated `produced`"
    );

    Ok(())
}
