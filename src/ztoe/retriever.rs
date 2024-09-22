use chrono::NaiveDate;

const REM_ID: usize = 19;
pub async fn load_schedule(date: NaiveDate) -> String {
    let url = format!(
        "https://ztoe.com.ua/unhooking.php?rem_id={}&date={}",
        REM_ID, date
    );
    let response = reqwest::get(url).await.unwrap();
    response.text().await.unwrap()
}
