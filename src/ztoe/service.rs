use crate::blackout::Forecast;
use crate::ztoe::retriever::load_schedule;
use crate::ztoe::scraper::scrape_blackouts;
use chrono::NaiveDate;

pub async fn retrieve_forecast(date: NaiveDate) -> Forecast {
    let html = load_schedule(date).await;
    let blackouts = scrape_blackouts(&html, date).await;
    Forecast::new(date, blackouts)
}