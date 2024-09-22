use crate::ztoe::check_blackouts;
use chrono::{Local, NaiveDate, TimeDelta};
use std::ops::Add;
use teloxide::prelude::*;

pub async fn init_bot() {
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(command) = msg.text() {
            match command {
                "/today" => {
                    let message = forecast_for(Local::now().date_naive()).await;
                    bot.send_message(msg.chat.id, message)
                }
                "/tomorrow" => {
                    let message =
                        forecast_for(Local::now().date_naive().add(TimeDelta::days(1))).await;
                    bot.send_message(msg.chat.id, message)
                }
                _ => bot.send_message(msg.chat.id, "unknown command. supported commands are /today and /tomorrow"),
            }
            .await
            .unwrap();
        };
        Ok(())
    })
    .await;
}

async fn forecast_for(date: NaiveDate) -> String {
    let blackouts = check_blackouts(date).await;
    if blackouts.is_empty() {
        format!("No blackouts for {}", date)
    } else {
        format!("Potential blackout scheduled for {}. Check the [ZTOE Website](https://ztoe.com.ua/unhooking.php?rem_id=19&date={})", date, date)
    }
}
