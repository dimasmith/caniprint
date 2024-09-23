//! Methods to send telegram messages to bot subscribers.

use chrono::NaiveDate;
use crate::blackout::{Digest, Forecast};
use crate::telegram::markdown::{display_digest, display_forecast, display_unavailable_digest, display_unavailable_forecast};
use teloxide::prelude::*;
use teloxide::types::ParseMode;

/// Send a digest to a chat.
pub async fn send_digest(bot: Bot, chat_id: ChatId, digest: &Digest) -> ResponseResult<()> {
    let message = display_digest(digest);
    bot.send_message(chat_id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

/// Send a message about an unavailable digest to a chat.
pub async fn send_digest_unavailable(bot: Bot, chat_id: ChatId) -> ResponseResult<()> {
    bot.send_message(chat_id, display_unavailable_digest())
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub async fn send_forecast(bot: Bot, chat_id: ChatId, forecast: &Forecast) -> ResponseResult<()> {
    let message = display_forecast(forecast);
    bot.send_message(chat_id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub async fn send_unavailable_forecast(bot: Bot, chat_id: ChatId, date: NaiveDate) -> ResponseResult<()> {
    bot.send_message(chat_id, display_unavailable_forecast(date))
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}