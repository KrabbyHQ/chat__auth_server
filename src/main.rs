use chat__auth_server::{create_app, AppState};
use chat__auth_server::db::connect_postgres::connect_pg;
use chat__auth_server::utils::load_config::load_config;
use chat__auth_server::utils::load_env::load_env;
use tracing::{error, info};
use tracing_subscriber::fmt::time::SystemTime;
use std::net::SocketAddr;
use std::sync::Arc;

fn initialize_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_timer(SystemTime)
        .with_level(true)
        .init();
}

#[tokio::main]
async fn main() {
    load_env();
    initialize_logging();

    let app_config = load_config();

    // println!("{:?}", app_config);

    let clean_config = match app_config {
        Ok(config) => {
            if let Err(e) = config.validate() {
                let error = format!("SERVER START-UP ERROR: FAILED TO LOAD SERVER CONFIGURATIONS, {}", e);
                error!("{}", error);
                return;
            }

            config
        }
        Err(e) => {
            let error = format!("SERVER START-UP ERROR: FAILED TO LOAD SERVER CONFIGURATIONS, {}", e);
            error!("{}", error);
            return;
        }
    };

    let db_config = clean_config.database.as_ref().expect("SERVER START-UP ERROR: DATABASE CONFIGURATION IS MISSING!");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user.as_deref().expect("SERVER START-UP ERROR: DATABASE USER IS MISSING!"),
        db_config.password.as_deref().expect("SERVER START-UP ERROR: DATABASE PASSWORD IS MISSING!"),
        db_config.host,
        db_config.port,
        db_config.name
    );
    
    let db_pool = connect_pg(
        database_url.clone(),
        db_config.max_connections,
        db_config.connect_timeout_secs,
    ).await;

    let state = AppState {
        config: Arc::new(clean_config),
        db: db_pool,
    };

    let app = create_app(state.clone());

    let host = state.config.server.as_ref().map(|s| s.host.as_str()).unwrap_or("127.0.0.1");
    let port = state.config.server.as_ref().map(|s| s.port).unwrap_or(8000);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    let slice_db_url = format!("{}...", &database_url[0..25]);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            print!(
                "
                .................................................
                Connected to DB: {}
                Environment: {}
                Status: DB connected successfully
                .................................................

                Server running on http://{}
                ",
                slice_db_url,
                state.config.app.environment.as_deref().unwrap_or("unknown"),
                addr
            );
            listener
        }
        Err(e) => {
            error!("SERVER INITIALIZATION ERROR: {}!", e);
            return;
        }
    };

    let server_result = axum::serve(listener, app).await;

    match server_result {
        Ok(_) => {
            info!("Graceful server shutdown!");
        }
        Err(e) => {
            error!("SERVER SHUTDOWN ERROR: {}!", e);
        }
    }
}
