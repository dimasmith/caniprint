//! Methods to send telegram messages to bot subscribers.

use chrono::NaiveDate;
use crate::blackout::{Digest, Forecast};
use crate::telegram::markdown::*;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::SubscriberId;

/// Send a digest to a chat.
pub async fn send_digest(bot: &Bot, subscriber_id: SubscriberId, digest: &Digest) -> ResponseResult<()> {
    let message = display_digest(digest);
    bot.send_message(ChatId::from(subscriber_id), message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

/// Send a message about an unavailable digest to a chat.
pub async fn send_digest_unavailable(bot: &Bot, chat_id: SubscriberId) -> ResponseResult<()> {
    bot.send_message(ChatId::from(chat_id), display_unavailable_digest())
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub(super) async fn send_forecast(bot: &Bot, chat_id: ChatId, forecast: &Forecast) -> ResponseResult<()> {
    let message = display_forecast(forecast);
    bot.send_message(chat_id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub(super) async fn send_unavailable_forecast(bot: &Bot, chat_id: ChatId, date: NaiveDate) -> ResponseResult<()> {
    bot.send_message(chat_id, display_unavailable_forecast(date))
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}