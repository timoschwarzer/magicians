use std::sync::Arc;
use axum::routing::{any, post};
use axum::{Router};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use game_logic::{ApplicationState};

mod protocol;
mod game_logic;
mod http;
mod timestamp;

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let application_state = ApplicationState {
        games: Arc::new(Mutex::new(Default::default())),
    };

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/game", post(http::create_game::handler))
        .route("/api/game/{game_id}/websocket", any(http::websocket::handler))
        .with_state(application_state.clone())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback(http::ui::handler);

    let mut tasks = JoinSet::new();

    tasks.spawn(async move {
        let mut interval = time::interval(Duration::from_hours(24));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    tracing::debug!("Cleaning up beacons");
                    application_state.clean_old_games().await;
                }
                _ = shutdown_signal() => {
                    break;
                }
            }
        }
    });

    tasks.spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        tracing::info!("Server is listening on 0.0.0.0:8080");
        axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
    });

    tasks.join_all().await;
}
