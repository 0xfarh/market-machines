use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use binary_primitives_core::{Market, Outcome, Position};

#[derive(Clone)]
struct AppState {
    market: Arc<Mutex<Market>>,
}

#[tokio::main]
async fn main() {
    let market = Market::new("Will it rain tomorrow?", 0.6);
    let state = AppState {
        market: Arc::new(Mutex::new(market)),
    };

    let app = Router::new()
        .route("/market", get(get_market))
        .route("/buy", post(buy))
        .route("/resolve", post(resolve))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_market(axum::extract::State(state): axum::extract::State<AppState>) -> Json<Market> {
    let market = state.market.lock().unwrap();
    Json(market.clone())
}

#[derive(Deserialize)]
struct BuyRequest {
    outcome: String,
    shares: u64,
}

#[derive(Serialize)]
struct BuyResponse {
    cost: f64,
}

async fn buy(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<BuyRequest>,
) -> Json<BuyResponse> {
    let market = state.market.lock().unwrap();
    let outcome = match payload.outcome.to_lowercase().as_str() {
        "yes" => Outcome::Yes,
        "no" => Outcome::No,
        _ => Outcome::No, // fallback
    };

    let position = market.buy(outcome, payload.shares);
    Json(BuyResponse { cost: position.cost })
}

#[derive(Deserialize)]
struct ResolveRequest {
    outcome: String,
}

async fn resolve(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<ResolveRequest>,
) -> &'static str {
    let mut market = state.market.lock().unwrap();
    let outcome = match payload.outcome.to_lowercase().as_str() {
        "yes" => Outcome::Yes,
        "no" => Outcome::No,
        _ => Outcome::No,
    };
    market.resolve(outcome);
    "Market resolved"
}
