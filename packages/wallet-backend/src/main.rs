mod contexts;
mod shared;

use rustls;

use std::sync::Arc;
use axum::Router;
use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
use contexts::wallet::infrastructure::repositories::postgres_balance_repo::PostgresBalanceRepo;
use contexts::wallet::application::use_cases::get_balance::GetBalanceUseCase;
use contexts::wallet::application::use_cases::convert_balance::ConvertBalanceUseCase;
use contexts::wallet::{
    infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo,
    application::use_cases::{
        configure_wallet::ConfigureWalletUseCase,
        get_wallet::GetWalletUseCase,
    },
    presentation::routes::wallet_routes::wallet_router,
};
use contexts::lightning::infrastructure::lnd::LndClient;
use contexts::lightning::presentation::routes::lnurlp_routes::lnurlp_router;
use contexts::merchant::application::use_cases::create_payment::CreatePaymentUseCase;
use contexts::merchant::presentation::routes::merchant_routes::merchant_router;
use contexts::lightning::presentation::handlers::lnurlp_handler::LnurlpState;
use contexts::lightning::application::use_cases::auto_convert_listener::AutoConvertListener;

fn install_crypto() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install ring crypto provider");
}

#[tokio::main]
async fn main() {
    install_crypto();
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "warn,wallet_backend=info,sqlx=info".into()
        }))
        .init();

    let base_url = std::env::var("FLASH_BASE_URL").expect("FLASH_BASE_URL must be set");
    let user_id = std::env::var("FLASH_USER_ID").expect("FLASH_USER_ID must be set");
    let lightning_address = std::env::var("FLASH_LIGHTNING_ADDRESS").expect("FLASH_LIGHTNING_ADDRESS must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let lnd_host = std::env::var("LND_HOST").expect("LND_HOST must be set");
    let lnd_tls_cert = std::env::var("LND_TLS_CERT").expect("LND_TLS_CERT must be set");
    let lnd_macaroon = std::env::var("LND_MACAROON").expect("LND_MACAROON must be set");

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
    let balance_repo = Arc::new(PostgresBalanceRepo::new(pool.clone()));

    // LND Client
    // Client LND pour les invoices (LNURL-pay)
    let lnd_invoice_client = LndClient::connect(
        &lnd_host,
        &lnd_tls_cert,
        &lnd_macaroon,
    ).await.expect("Failed to connect to LND");

    let lnd_invoice_client = Arc::new(tokio::sync::Mutex::new(lnd_invoice_client));

    // Client LND séparé pour le listener
    let lnd_listener_client = LndClient::connect(
        &lnd_host,
        &lnd_tls_cert,
        &lnd_macaroon,
    ).await.expect("Failed to connect to LND listener");
    let lnd_listener_client = Arc::new(tokio::sync::Mutex::new(lnd_listener_client));
    // Client LND pour les conversions de balance
    let lnd_convert_client = LndClient::connect(
        &lnd_host,
        &lnd_tls_cert,
        &lnd_macaroon,
    ).await.expect("Failed to connect to LND convert");
    let lnd_convert_client = Arc::new(tokio::sync::Mutex::new(lnd_convert_client));
    tracing::info!(" Connected to LND (carol)");

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
    let get_wallet_use_case = Arc::new(GetWalletUseCase::new(wallet_repo.clone()));
    let get_balance_use_case = Arc::new(GetBalanceUseCase::new(wallet_repo.clone(), transaction_repo.clone(), lnd_convert_client.clone()));
    let convert_balance_use_case = Arc::new(ConvertBalanceUseCase::new(
        wallet_repo.clone(),
        flash_gateway.clone(),
        transaction_repo.clone(),
        lnd_convert_client.clone(),
    ));

    let create_payment_use_case = Arc::new(CreatePaymentUseCase::new(
        wallet_repo.clone(),
        lnd_invoice_client.clone(),
    ));

    // Auto-convert listener — tourne en arrière-plan
    let listener = AutoConvertListener::new(
        auto_convert_use_case.clone(),
        wallet_repo.clone(),
        balance_repo.clone(),
        lnd_invoice_client.clone(),
    );
    let listener = Arc::new(listener);
    let lnd_for_listener = lnd_listener_client.clone();

    tokio::spawn(async move {
        tracing::info!(" Starting invoice listener...");
        let mut client = lnd_for_listener.lock().await;
        if let Err(e) = client.subscribe_invoices(|invoice| {
            let listener = listener.clone();
            let amount_sats = invoice.value as u64;
            let memo = invoice.memo.clone();
            tokio::spawn(async move {
                listener.on_invoice_settled(amount_sats, &memo).await;
            });
        }).await {
            tracing::error!("Invoice listener error: {}", e);
        }
    });

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .merge(transaction_router(
            auto_convert_use_case,
            get_status_use_case,
            buy_bitcoin_use_case,
            list_transactions_use_case,
        ))
        .merge(wallet_router(configure_wallet_use_case, get_wallet_use_case, get_balance_use_case, convert_balance_use_case))
        .merge(merchant_router(create_payment_use_case))
        .merge(lnurlp_router(LnurlpState {
            wallet_repo: wallet_repo.clone(),
            lnd_client: lnd_invoice_client.clone(),
            base_url: std::env::var("SERVER_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!(" Flash Wallet Backend running on http://localhost:{}", port);
    // Job expiration des transactions PENDING
    let expire_repo = transaction_repo.clone();
    tokio::spawn(async move {
        use contexts::conversion::application::use_cases::expire_pending::ExpirePendingUseCase;
        let expire_use_case = ExpirePendingUseCase::new(expire_repo);
        expire_use_case.run_forever().await;
    });

    axum::serve(listener, app).await.unwrap();
}
