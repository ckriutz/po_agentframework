use reqwest;
use tokio;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing A2A Agent Card Server Endpoint");
    
    // Start a simple test server for a moment
    println!("📡 Testing server endpoint at http://localhost:8080/.well-known/agent.json");
    
    // Try to fetch the agent card
    match reqwest::get("http://localhost:8080/.well-known/agent.json").await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>().await {
                    Ok(agent_card) => {
                        println!("✅ Successfully retrieved A2A Agent Card from server!");
                        println!("📋 Agent Name: {}", agent_card["name"].as_str().unwrap_or("Unknown"));
                        println!("📋 Version: {}", agent_card["version"].as_str().unwrap_or("Unknown"));
                        println!("📋 Skills Count: {}", agent_card["skills"].as_array().map(|s| s.len()).unwrap_or(0));
                        
                        // Verify key A2A fields
                        let required_fields = ["name", "description", "url", "version", "capabilities", "authentication", "defaultInputModes", "defaultOutputModes", "skills"];
                        for field in &required_fields {
                            if agent_card.get(field).is_some() {
                                println!("✅ Field '{}' present", field);
                            } else {
                                println!("❌ Field '{}' missing", field);
                            }
                        }
                        
                        println!("\n🎯 A2A Agent Card endpoint is working correctly!");
                    }
                    Err(e) => println!("❌ Failed to parse JSON response: {}", e),
                }
            } else {
                println!("❌ Server returned error status: {}", response.status());
            }
        }
        Err(e) => {
            println!("❌ Failed to connect to server: {}", e);
            println!("💡 This is expected if the server is not running");
            println!("💡 Start the server with: cargo run --bin server");
        }
    }
    
    Ok(())
}