// apps/server/src/main.rs
//
// INTEGRATION TEMPLATE
// Replace this with your existing main.rs and merge the relay_physics router

mod relay_physics;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Initialize Relay Physics state
    let relay_state = relay_physics::main_integration::init_relay_physics_state();

    // Build router
    let app = Router::new()
        // --- ADD YOUR EXISTING GIT ROUTES HERE ---
        // .route("/git-pull", post(git_pull_handler))
        // .route("/config", get(config_handler))
        // etc...
        
        // Merge Relay Physics routes
        .merge(relay_physics::main_integration::relay_router(relay_state))
        
        // CORS (allow all for dev)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    // Bind and serve
    let addr = "0.0.0.0:3002".parse().unwrap();
    println!("🚀 Relay server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
