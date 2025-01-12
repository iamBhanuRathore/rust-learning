use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use http_server::configure_cors;
use tower::ServiceBuilder;

use crate::{
    controllers::{
        home_controller::home_controller,
        user_controller::{
            body_and_param, body_and_query_and_param, body_and_query_and_param_and_header,
            get_all_users, product_controller, return_same,
        },
    },
    middleware::auth_middleware::auth_middleware,
};

pub fn users_routes() -> Router {
    let cors = configure_cors();
    Router::new()
        .route("/", get(home_controller))
        .route("/users", get(get_all_users))
        .route("/return", post(return_same))
        .route("/product/{id}", get(product_controller))
        .route("/other/{id}", get(body_and_param))
        .route(
            "/body_and_query_and_param/{id}",
            get(body_and_query_and_param),
        )
        .route(
            "/body_and_query_and_param_and_header/{id}",
            post(body_and_query_and_param_and_header),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(auth_middleware)))
        .layer(cors)
}
