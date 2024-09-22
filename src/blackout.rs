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
