use data_agent_rust::{PurchaseOrderAgent, create_router};
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,data_agent_rust=debug")
        .init();

    // Create the Purchase Order Agent
    let agent = Arc::new(PurchaseOrderAgent::new());
    info!("🚀 Purchase Order Processing Agent initialized");

    // Create the router
    let app = create_router(agent);

    // Define the server address
    let addr = "0.0.0.0:8080";
    info!("🌐 Starting server on http://{}", addr);

    // Create listener
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("✅ Server listening on {}", addr);
            listener
        }
        Err(e) => {
            error!("❌ Failed to bind to {}: {}", addr, e);
            return;
        }
    };

    // Print startup information
    println!("\n🦀 Purchase Order Processing Agent Server");
    println!("==========================================");
    println!("🌐 Server URL: http://localhost:8080");
    println!("📋 Available endpoints:");
    println!("   GET  /                         - Agent info and API documentation");
    println!("   GET  /.well-known/agent.json   - A2A compliant agent card (standard)");
    println!("   GET  /health                   - Health check");
    println!("   POST /agent/task               - Submit purchase order for processing");
    println!("   GET  /agent/task/{{id}}         - Get task status and results");
    println!("   POST /agent/task/{{id}}/cancel  - Cancel a task");
    println!("\n📝 Example usage:");
    println!("   curl http://localhost:8080/health");
    println!("   curl http://localhost:8080/.well-known/agent.json");
    println!("   curl http://localhost:8080/agent/info");
    println!("\n🔗 A2A Agent Card: http://localhost:8080/.well-known/agent.json");
    println!("🔗 Full API documentation available at: http://localhost:8080");
    println!("==========================================\n");

    // Start the server
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Server error: {}", e);
    }
}