use std::{convert::Infallible, time::Duration};

use anyhow::Context;
use async_openai::{config::OpenAIConfig, Client};

use sqlx::{postgres::PgPoolOptions, PgPool};
use teloxide::{
    dispatching::Dispatcher,
    dptree,
    error_handlers::LoggingErrorHandler,
    update_listeners::{webhooks, UpdateListener},
    Bot,
};

use crate::{
    bot::{bot_handler, init_static_bot_details},
    routes::app_router,
    settings::{database::DatabaseSettings, environment::Environment, Settings},
};

async fn start_server(
    bot: Bot,
    settings: &Settings,
    env: Environment,
) -> impl UpdateListener<Err = std::convert::Infallible> {
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    )
    .parse()
    .with_context(|| {
        format!(
            "{}:{}",
            settings.application.host, settings.application.port,
        )
    })
    .expect("unable to parse into address url");

    let url = format!("{}/webhook", settings.application.public_url)
        .parse()
        .context(settings.application.public_url.to_string())
        .expect("unable to parse into webhook url");

    let mut options = webhooks::Options::new(address, url);

    if env == Environment::Local {
        options = options.drop_pending_updates();
    }

    let (mut listener, stop_flag, router) = webhooks::axum_to_router(bot, options)
        .await
        .expect("unable to get listener");

    let app = app_router(router);

    let stop_token = listener.stop_token();

    tokio::spawn(async move {
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|_| stop_token.stop())
            .expect("axum server error");
    });

    listener
}

fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}

pub async fn start_app(settings: Settings, env: Environment) {
    let tele_bot = Bot::from_env();
    let chatgpt = Client::new();
    let connection_pool = get_connection_pool(&settings.database);

    if let Environment::Production = env {
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:#?}");
                e
            })
            .expect("migration failed.");
    };

    let listener = start_server(tele_bot.clone(), &settings, env).await;

    Box::pin(start_bot(
        tele_bot,
        listener,
        settings,
        chatgpt,
        connection_pool,
    ))
    .await;
}

async fn start_bot(
    bot: Bot,
    listener: impl UpdateListener<Err = Infallible>,
    settings: Settings,
    chatgpt: Client<OpenAIConfig>,
    pool: PgPool,
) {
    init_static_bot_details(&bot).await;

    let handler = bot_handler();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![settings.stickers, chatgpt, pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::new())
        .await;
}
