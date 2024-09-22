use crate::blackout::Blackout;
use crate::ztoe::retriever::load_schedule;
use crate::ztoe::scraper::scrape_blackouts;
use chrono::NaiveDate;

pub async fn check_blackouts(date: NaiveDate) -> Vec<Blackout> {
    let html = load_schedule(date).await;
    scrape_blackouts(&html, date).await

}