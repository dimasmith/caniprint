use crate::blackout::{Digest, Forecast};
use crate::ztoe::retriever::load_schedule;
use crate::ztoe::scraper::scrape_blackouts;
use chrono::{Local, NaiveDate, TimeDelta};
use futures::future::join_all;
use std::ops::Add;

pub async fn retrieve_forecast(date: NaiveDate) -> Forecast {
    let html = load_schedule(date).await;
    let blackouts = scrape_blackouts(&html, date).await;
    Forecast::new(date, blackouts)
}

pub async fn digest_forecasts(days: u32) -> Digest {
    let today = Local::now().date_naive();
    let dates: Vec<NaiveDate> = (0..days)
        .map(|i| today.add(TimeDelta::days(i as i64)))
        .collect();
    let forecasts: Vec<Forecast> = join_all(dates.into_iter().map(retrieve_forecast)).await;
    Digest::now(forecasts)
}
