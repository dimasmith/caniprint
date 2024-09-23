//! Telegram bot command handlers.

use crate::subscriptions::subscribers::FileStorage;
use crate::subscriptions::Subscribers;
use crate::telegram::bot::Command;
use crate::telegram::messages::*;
use crate::{load_daily_forecast, load_forecast_digest};
use chrono::{Local, NaiveDate, TimeDelta};
use std::ops::Add;
use std::sync::Arc;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use tokio::sync::Mutex;
use tracing::warn;

pub async fn subscribe(
    bot: Bot,
    msg: Message,
    subscribers: Arc<Mutex<Subscribers<FileStorage>>>,
) -> ResponseResult<()> {
    {
        // separate block to release the lock as soon as possible
        let mut subscribers = subscribers.lock().await;
        subscribers.subscribe(msg.chat.id).await;
    }

    let digest = load_forecast_digest(3).await;
    match digest {
        Ok(d) => send_digest(bot, msg.chat.id, &d).await,
        Err(e) => {
            warn!("Failed to load forecast digest: {}", e);
            send_digest_unavailable(bot, msg.chat.id).await
        }
    }?;
    Ok(())
}

pub async fn today(bot: Bot, msg: Message) -> ResponseResult<()> {
    let today = Local::now().date_naive();
    forecast_for_date(bot, msg, today).await
}

pub async fn tomorrow(bot: Bot, msg: Message) -> ResponseResult<()> {
    let tomorrow = Local::now().date_naive().add(TimeDelta::days(1));
    forecast_for_date(bot, msg, tomorrow).await
}

async fn forecast_for_date(bot: Bot, msg: Message, date: NaiveDate) -> ResponseResult<()> {
    let forecast = load_daily_forecast(date).await;
    match forecast {
        Ok(f) => send_forecast(bot, msg.chat.id, &f).await,
        Err(e) => {
            warn!("Failed to load forecast for {}: {}", date, e);
            send_unavailable_forecast(bot, msg.chat.id, date).await
        }
    }
}

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
