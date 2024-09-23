use crate::blackout::Digest;
use crate::subscriptions::subscribers::FileStorage;
use crate::subscriptions::Subscribers;
use crate::telegram::messages::{
    display_digest, display_forecast, display_unavailable_digest, display_unavailable_forecast,
};
use crate::ztoe::load_daily_forecast;
use crate::ztoe::service::load_forecast_digest;
use chrono::{Local, NaiveDate, TimeDelta};
use std::ops::Add;
use std::sync::Arc;
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;
use tracing::warn;

#[derive(BotCommands, Clone, PartialEq)]
#[command(rename_rule = "lowercase", description = "Бот підтримує такі команди")]
enum Command {
    #[command(description = "Показати список команд")]
    Help,
    #[command(description = "Перевірити відключення на сьогодні")]
    Today,
    #[command(description = "Перевірити відключення на завтра")]
    Tomorrow,
    #[command(description = "Підписатися на повідомлення про відключення")]
    Subscribe,
}

pub async fn start_bot(bot: Bot, subscribers: Arc<Mutex<Subscribers<FileStorage>>>) {
    let command_handler = Update::filter_message()
        .branch(
            teloxide::filter_command::<Command, _>()
                .branch(case![Command::Today].endpoint(today))
                .branch(case![Command::Tomorrow].endpoint(tomorrow))
                .branch(case![Command::Subscribe].endpoint(subscribe))
                .endpoint(help),
        )
        .endpoint(help);

    Dispatcher::builder(bot, command_handler)
        .dependencies(dptree::deps![subscribers])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn send_forecast_digest(
    bot: Bot,
    clients: &[ChatId],
    digest: &Digest,
) -> ResponseResult<()> {
    let message = display_digest(digest);
    for client in clients {
        bot.send_message(*client, message.clone())
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn today(bot: Bot, msg: Message) -> ResponseResult<()> {
    let today = Local::now().date_naive();
    forecast_for_date(bot, msg, today).await
}

async fn tomorrow(bot: Bot, msg: Message) -> ResponseResult<()> {
    let tomorrow = Local::now().date_naive().add(TimeDelta::days(1));
    forecast_for_date(bot, msg, tomorrow).await
}

async fn forecast_for_date(bot: Bot, msg: Message, date: NaiveDate) -> ResponseResult<()> {
    let forecast = load_daily_forecast(date).await;
    let message = match forecast {
        Ok(f) => display_forecast(&f),
        Err(e) => {
            warn!("Failed to load forecast for {}: {}", date, e);
            display_unavailable_forecast(date)
        },
    };
    bot.parse_mode(ParseMode::MarkdownV2)
        .send_message(msg.chat.id, message)
        .await?;
    Ok(())
}

async fn subscribe(
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
    let message = match digest {
        Ok(d) => display_digest(&d),
        Err(e) => {
            warn!("Failed to load digest: {}", e);
            display_unavailable_digest()
        }
    };
    bot.send_message(msg.chat.id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}
