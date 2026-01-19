use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use price_by_flow_core::{Market, Outcome};

#[derive(Clone)]
struct AppState {
    market: Arc<Mutex<Market>>,
}

#[tokio::main]
async fn main() {
    let market = Market::new("Will stock X go up tomorrow?", 0.5);
    let state = AppState {
        market: Arc::new(Mutex::new(market)),
    };

    let app = Router::new()
        .route("/market", get(get_market))
        .route("/buy", post(buy))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Server running on http://127.0.0.1:3001");
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
    let mut market = state.market.lock().unwrap();
    let outcome = match payload.outcome.to_lowercase().as_str() {
        "yes" => Outcome::Yes,
        "no" => Outcome::No,
        _ => Outcome::No,
    };
    let cost = market.buy(outcome, payload.shares);
    Json(BuyResponse { cost })
}
