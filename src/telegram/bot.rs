use crate::ztoe::check_blackouts;
use chrono::{Local, NaiveDate, TimeDelta};
use std::ops::Add;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, PartialEq)]
#[command(rename_rule = "lowercase", description = "Бот підтримує такі команди")]
enum Command {
    #[command(description = "Показати список команд")]
    Help,
    #[command(description = "Перевірити відключення на сьогодні")]
    Today,
    #[command(description = "Перевірити відключення на завтра")]
    Tomorrow,
}

pub async fn init_bot() {
    let bot = Bot::from_env();
    teloxide::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message) -> ResponseResult<()> {
    // parse command
    let cmd = match msg.text() {
        None => Command::Help,
        Some(text) => Command::parse(text, "").unwrap_or(Command::Help),
    };
    handle_command(bot, msg, cmd).await
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()),
        Command::Today => {
            let message = forecast_for(Local::now().date_naive()).await;
            bot.parse_mode(ParseMode::MarkdownV2)
                .send_message(msg.chat.id, message)
        }
        Command::Tomorrow => {
            let message = forecast_for(Local::now().date_naive().add(TimeDelta::days(1))).await;
            bot.send_message(msg.chat.id, message)
                .parse_mode(ParseMode::MarkdownV2)
        }
    }
    .await?;
    Ok(())
}

async fn forecast_for(date: NaiveDate) -> String {
    let blackouts = check_blackouts(date).await;
    if blackouts.is_empty() {
        format!("✅ `{}` відключення не прогнозуються\\.", date)
    } else {
        format!("⚠️ `{}` можливі відключення\\. Деталі на [сайті ЖТОЕ](https://ztoe.com.ua/unhooking.php?rem_id=19&date={})", date, date)
    }
}
