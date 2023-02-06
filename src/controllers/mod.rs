use axum::{routing::post, Router};
pub mod cut_controller;
use cut_controller::*;

pub fn init_noneed_auth_router() -> Router {
    Router::new()
    .route("/stocks_1d_by_weight", post(solve_cut_optimize_by_weight))
    .route("/stocks_1d_by_len", post(solve_cut_optimize_by_len))
}
