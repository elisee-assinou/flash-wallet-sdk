mod contexts;
mod shared;

use std::sync::Arc;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use contexts::conversion::{
    infrastructure::{
        api::flash_api_gateway::FlashApiGateway,
        repositories::postgres_transaction_repo::PostgresTransactionRepo,
    },
    application::use_cases::{
        auto_convert::AutoConvertUseCase,
        get_transaction_status::GetTransactionStatusUseCase,
        buy_bitcoin::BuyBitcoinUseCase,
        list_transactions::ListTransactionsUseCase,
    },
    presentation::routes::transaction_routes::transaction_router,
};
use contexts::wallet::{
    infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo,
    application::use_cases::{
        configure_wallet::ConfigureWalletUseCase,
        get_wallet::GetWalletUseCase,
    },
    presentation::routes::wallet_routes::wallet_router,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let base_url = std::env::var("FLASH_BASE_URL").expect("FLASH_BASE_URL must be set");
    let user_id = std::env::var("FLASH_USER_ID").expect("FLASH_USER_ID must be set");
    let lightning_address = std::env::var("FLASH_LIGHTNING_ADDRESS").expect("FLASH_LIGHTNING_ADDRESS must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    // Database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!(" Connected to PostgreSQL");

    // Infrastructure
    let flash_gateway = Arc::new(FlashApiGateway::new(base_url, user_id, lightning_address));
    let transaction_repo = Arc::new(PostgresTransactionRepo::new(pool.clone()));
    let wallet_repo = Arc::new(PostgresWalletRepo::new(pool.clone()));

    // Conversion use cases
    let auto_convert_use_case = Arc::new(AutoConvertUseCase::new(
        transaction_repo.clone(),
        flash_gateway.clone(),
    ));
    let get_status_use_case = Arc::new(GetTransactionStatusUseCase::new(
        flash_gateway.clone(),
        transaction_repo.clone(),
        
    ));
    let buy_bitcoin_use_case = Arc::new(BuyBitcoinUseCase::new(
        transaction_repo.clone(),
        flash_gateway.clone(),
    ));
    let list_transactions_use_case = Arc::new(ListTransactionsUseCase::new(
        transaction_repo.clone(),
    ));

    // Wallet use cases
    let configure_wallet_use_case = Arc::new(ConfigureWalletUseCase::new(wallet_repo.clone()));
    let get_wallet_use_case = Arc::new(GetWalletUseCase::new(wallet_repo));

    let app = Router::new()
        .merge(transaction_router(
            auto_convert_use_case,
            get_status_use_case,
            buy_bitcoin_use_case,
            list_transactions_use_case,
        ))
        .merge(wallet_router(configure_wallet_use_case, get_wallet_use_case))
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!(" Flash Wallet Backend running on http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}