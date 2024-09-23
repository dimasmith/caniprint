use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Failed to load schedule for {0} due to {1}")]
    CommunicationError(NaiveDate, reqwest::Error),
}

const REM_ID: usize = 19;
pub async fn load_schedule(date: NaiveDate) -> Result<String, LoadError> {
    let url = format!(
        "https://ztoe.com.ua/unhooking.php?rem_id={}&date={}",
        REM_ID, date
    );
    let response = reqwest::get(url)
        .await
        .map_err(|e| LoadError::CommunicationError(date, e))?;
    response
        .text()
        .await
        .map_err(|e| LoadError::CommunicationError(date, e))
}
