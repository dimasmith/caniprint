use chrono::{Local, NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Blackout {
    date: NaiveDate,
}

impl Blackout {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
}

#[derive(Debug, Clone)]
pub struct Forecast {
    date: NaiveDate,
    blackouts: Vec<Blackout>,
}

impl Forecast {
    pub fn new(date: NaiveDate, blackouts: Vec<Blackout>) -> Self {
        Self { date, blackouts }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn blackouts(&self) -> &[Blackout] {
        &self.blackouts
    }

    pub fn is_empty(&self) -> bool {
        self.blackouts.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Digest {
    last_update: NaiveDateTime,
    forecasts: Vec<Forecast>,
}

impl Digest {
    pub fn new(last_update: NaiveDateTime, forecasts: Vec<Forecast>) -> Self {
        Self {
            last_update,
            forecasts,
        }
    }

    pub fn now(forecasts: Vec<Forecast>) -> Self {
        let now = Local::now().naive_local();
        Self::new(now, forecasts)
    }

    pub fn last_update(&self) -> NaiveDateTime {
        self.last_update
    }

    pub fn forecasts(&self) -> &[Forecast] {
        &self.forecasts
    }

    pub fn is_empty(&self) -> bool {
        self.forecasts.is_empty()
    }
}
