use crate::blackout::{Digest, Forecast};

pub fn display_forecast(forecast: &Forecast) -> String {
    if forecast.is_empty() {
        format!("✅ `{}` відключення не прогнозуються\\.", forecast.date())
    } else {
        format!("⚠️ `{}` можливі відключення\\. Деталі на [сайті ЖТОЕ](https://ztoe.com.ua/unhooking.php?rem_id=19&date={})", forecast.date(), forecast.date())
    }
}

pub fn display_forecasts(forecasts: &[Forecast]) -> String {
    let messages: Vec<String> = forecasts.iter().map(display_forecast).collect();
    messages.join("\n")
}

pub fn display_digest(digest: &Digest) -> String {
    let message = display_forecasts(digest.forecasts());
    let last_update = digest.last_update().format("%Y-%m-%d %H:%M:%S");
    format!("{}\n\n_Оновлено:_ `{}`", message, last_update)
}
