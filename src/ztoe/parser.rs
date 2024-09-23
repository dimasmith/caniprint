use crate::blackout::Blackout;
use chrono::NaiveDate;
use scraper::{Html, Selector};

pub async fn parse_forecast(html: &str, date: NaiveDate) -> Vec<Blackout> {
    let document = Html::parse_document(html);
    let table_cell_selector = Selector::parse("div#data_result table td").unwrap();
    let cells = document.select(&table_cell_selector);
    let found_blackouts = cells.flat_map(|e| e.text())
        .map(|t| t.trim().to_lowercase())
        .filter(|t| t.contains("корнин "))
        .count();

    if found_blackouts > 0 {
        vec![Blackout::new(date)]
    } else {
        vec![]
    }
}