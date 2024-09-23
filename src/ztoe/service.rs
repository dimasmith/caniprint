use crate::blackout::{Digest, Forecast};
use crate::ztoe::retriever::{load_schedule, LoadError};
use crate::ztoe::scraper::scrape_blackouts;
use chrono::{Local, NaiveDate, TimeDelta};
use futures::future::join_all;
use std::ops::Add;
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum ForecastError {
    #[error("Failed to load forecast for {0} due to {1}")]
    ForecastNotAvailable(NaiveDate, LoadError),
    #[error("Failed to load digest")]
    DigestNotAvailable,
}

pub async fn load_daily_forecast(date: NaiveDate) -> Result<Forecast, ForecastError> {
    let html = load_schedule(date)
        .await
        .map_err(|e| ForecastError::ForecastNotAvailable(date, e))?;
    let blackouts = scrape_blackouts(&html, date).await;
    Ok(Forecast::new(date, blackouts))
}

pub async fn load_forecast_digest(days: u32) -> Result<Digest, ForecastError> {
    let today = Local::now().date_naive();
    let dates: Vec<NaiveDate> = (0..days)
        .map(|i| today.add(TimeDelta::days(i as i64)))
        .collect();
    let forecasts: Vec<_> = join_all(dates.into_iter().map(load_daily_forecast)).await;

    let mut available_forecasts = vec![];
    for forecast in forecasts {
        match forecast {
            Ok(f) => available_forecasts.push(f),
            Err(e) => {
                warn!("Failed to load forecast: {}", e)
            }
        }
    }
    if available_forecasts.is_empty() {
        return Err(ForecastError::DigestNotAvailable);
    }

    Ok(Digest::now(available_forecasts))
}
