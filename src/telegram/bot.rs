use crate::blackout::Forecast;
use crate::subscriptions::Subscribers;
use crate::ztoe::retrieve_forecast;
use chrono::{Local, NaiveDate, TimeDelta};
use std::ops::Add;
use std::sync::Arc;
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;

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

pub async fn start_bot(bot: Bot, subscribers: Arc<Mutex<Subscribers>>) {
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
    subscribers: Arc<Mutex<Subscribers>>,
    forecasts: &[Forecast],
) -> ResponseResult<()> {
    let clients = subscribers.lock().await.subscribers().await;
    let message = forecasts_message(forecasts);
    for client in clients {
        bot.send_message(client, message.clone())
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
    let message = forecast_for(today).await;
    bot.parse_mode(ParseMode::MarkdownV2)
        .send_message(msg.chat.id, message)
        .await?;
    Ok(())
}

async fn tomorrow(bot: Bot, msg: Message) -> ResponseResult<()> {
    let tomorrow = Local::now().date_naive().add(TimeDelta::days(1));
    let message = forecast_for(tomorrow).await;
    bot.send_message(msg.chat.id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

async fn subscribe(
    bot: Bot,
    msg: Message,
    subscribers: Arc<Mutex<Subscribers>>,
) -> ResponseResult<()> {
    {
        // separate block to release the lock as soon as possible
        let mut subscribers = subscribers.lock().await;
        subscribers.subscribe(msg.chat.id).await;
    }

    let message = forecast_digest().await;
    bot.send_message(msg.chat.id, message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

async fn forecast_for(date: NaiveDate) -> String {
    let forecast = retrieve_forecast(date).await;
    forecast_message(&forecast)
}

async fn forecast_digest() -> String {
    let dates = [
        Local::now().date_naive(),
        Local::now().date_naive().add(TimeDelta::days(1)),
    ];
    let mut forecasts = vec![];
    for date in dates.iter() {
        let forecast = retrieve_forecast(*date).await;
        forecasts.push(forecast);
    }
    forecasts_message(&forecasts)
}

fn forecast_message(forecast: &Forecast) -> String {
    if forecast.is_empty() {
        format!("✅ `{}` відключення не прогнозуються\\.", forecast.date())
    } else {
        format!("⚠️ `{}` можливі відключення\\. Деталі на [сайті ЖТОЕ](https://ztoe.com.ua/unhooking.php?rem_id=19&date={})", forecast.date(), forecast.date())
    }
}

fn forecasts_message(forecasts: &[Forecast]) -> String {
    let messages: Vec<String> = forecasts.iter().map(forecast_message).collect();
    messages.join("\n")
}
