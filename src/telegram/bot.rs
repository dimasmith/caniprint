use crate::subscriptions::subscribers::FileStorage;
use crate::subscriptions::Subscribers;
use crate::telegram::handlers;
use std::sync::Arc;
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;
#[derive(BotCommands, Clone, PartialEq)]
#[command(rename_rule = "lowercase", description = "Бот підтримує такі команди")]
pub(super) enum Command {
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
                .branch(case![Command::Today].endpoint(handlers::today))
                .branch(case![Command::Tomorrow].endpoint(handlers::tomorrow))
                .branch(case![Command::Subscribe].endpoint(handlers::subscribe))
                .endpoint(handlers::help),
        )
        .endpoint(handlers::help);

    Dispatcher::builder(bot, command_handler)
        .dependencies(dptree::deps![subscribers])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
