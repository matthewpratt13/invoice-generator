// import inner module containing exports
mod setup;

// use modules from `setup` module
use crate::setup::*;

#[test]
fn it_calculates_invoice_totals() -> Result<(), Box<dyn Error>> {
    let entry_01: InvoiceEntry = test_data::_dummy_entry();
    let entry_02: InvoiceEntry = test_data::_dummy_entry_02();
    let entry_03: InvoiceEntry = test_data::_dummy_entry_03();
    let entry_04: InvoiceEntry = test_data::_dummy_entry_04();
    let entry_05: InvoiceEntry = test_data::_dummy_entry_05();
    let entry_06: InvoiceEntry = test_data::_dummy_entry_06();
    let entry_07: InvoiceEntry = test_data::_dummy_entry_07();
    let entry_08: InvoiceEntry = test_data::_dummy_entry_08();
    let entry_09: InvoiceEntry = test_data::_dummy_entry_09();
    let entry_10: InvoiceEntry = test_data::_dummy_entry_10();

    let test_entries: Vec<InvoiceEntry> = vec![
        entry_01, entry_02, entry_03, entry_04, entry_05, entry_06, entry_07, entry_08, entry_09,
        entry_10,
    ];

    let invoice_totals: InvoiceTotals = invoice_entry::invoice_totals(&test_entries)?;

    let total_produced: PowerTotals = invoice_totals.total_produced();
    let to_grid: PowerTotals = invoice_totals.to_grid();
    let total_consumed: PowerTotals = invoice_totals.total_consumed();
    
    let consumed: f64 = invoice_totals.consumed();
    let produced: f64 = invoice_totals.produced();

    let test_total_produced = PowerTotals::from_periods(193.0, 1420.0, 916.0);
    let test_to_grid = PowerTotals::from_periods(77.0, 962.0, 889.0);
    let test_total_consumed = PowerTotals::from_periods(116.0, 458.0, 27.0);

    let test_consumed = 601.0;
    let test_produced = 2529.0;

    assert_eq!(
        total_produced, test_total_produced,
        "miscalculated total_produced"
    );
    assert_eq!(to_grid, test_to_grid, "miscalculated to_grid");

    assert_eq!(
        total_consumed, test_total_consumed,
        "miscalculated total_consumed"
    );
    assert_eq!(consumed, test_consumed, "miscalculated consumed");
    assert_eq!(produced, test_produced, "miscalculated produced");

    Ok(())
}
