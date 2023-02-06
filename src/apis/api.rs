use crate::controllers::init_noneed_auth_router;
use axum::Router;

//api
pub fn routers() -> Router {
    Router::new().merge(init_noneed_auth_router())
}
