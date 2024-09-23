use chrono::NaiveDate;

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