use crate::blackout::Blackout;
use chrono::{NaiveDate, NaiveDateTime};
use scraper::{Html, Selector};

pub async fn parse_forecast(html: &str, date: NaiveDate) -> Vec<Blackout> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("div#data_result table tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let rows = document.select(&row_selector).collect::<Vec<_>>();
    let mut blackouts = Vec::new();
    for chunk in rows.chunks(3) {
        if let [_, meta, address] = chunk {
            let meta_cells = &meta.select(&cell_selector).collect::<Vec<_>>()[0..6];
            let [_, _, start, end, _, notified] = &meta_cells else { continue };
            let start_date = parse_date_time_cell(start);
            let end_date = parse_date_time_cell(end);
            let notified_date = parse_date_time_cell(notified);

            let address_cells = address.select(&cell_selector);
            let places = address_cells.flat_map(|cell| cell.text()).collect();
            let blackout = Blackout::new(date, start_date, end_date, notified_date, places);
            blackouts.push(blackout);
        }
    }
    blackouts
}

fn parse_date_time_cell(cell: &scraper::ElementRef) -> NaiveDateTime {
    let text = cell.text().collect::<Vec<_>>().join(" ");
    NaiveDateTime::parse_from_str(&text, "%d.%m.%Y %H %M").unwrap()
}
