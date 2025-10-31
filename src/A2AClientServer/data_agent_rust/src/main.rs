mod agent;

use agent::{PurchaseOrderAgent, PurchaseOrderWrapper, PurchaseOrder, PurchaseOrderItem};
use a2a::{A2AProtocol, Message, Part};
use std::error::Error;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Starting Purchase Order Processing Agent (A2A Protocol)");
    
    // Create the agent
    let agent = PurchaseOrderAgent::new();
    
    // Display agent information
    let card = agent.get_agent_card();
    println!("📋 Agent Name: {}", card.name);
    println!("📋 Version: {}", card.version);
    println!("📋 URL: {}", card.url);
    if let Some(desc) = &card.description {
        println!("📋 Description: {}", desc);
    }
    
    // Demo: Create the exact purchase order from your specification
    println!("\n🧪 Running demo with your purchase order...");
    
    let sample_po = PurchaseOrderWrapper {
        purchase_order: PurchaseOrder {
            supplier_name: "Marketing Masters Supplies".to_string(),
            supplier_address_line1: "1234 Creative Avenue, Suite 567".to_string(),
            supplier_address_line2: None,
            supplier_city: "Imagination City".to_string(),
            supplier_state: "CA".to_string(),
            supplier_postal_code: "90210".to_string(),
            supplier_country: "USA".to_string(),
            items: vec![
                PurchaseOrderItem {
                    item_code: "bk-2345".to_string(),
                    description: "Marketing Strategy Guidebook".to_string(),
                    quantity: 3,
                    unit_price: 29.99,
                    line_total: 89.97,
                },
                PurchaseOrderItem {
                    item_code: "Bk-1311".to_string(),
                    description: "Promotional Materials Handbook".to_string(),
                    quantity: 3,
                    unit_price: 34.99,
                    line_total: 104.97,
                },
            ],
            po_number: "MMS-80085".to_string(),
            created_by: "J.J. Schmidt".to_string(),
            buyer_department: "Marketing".to_string(),
            notes: Some("thanks for the order! Happy learning!! :)".to_string()),
            tax_rate: 0.07,
            sub_total: 194.94,
            tax: 13.65,
            grand_total: 208.59,
            is_approved: true,
            approval_reason: Some("Approved: Grand Total $208.59 is below $1000, Supplier Name is provided, and Buyer Department 'Marketing' is an authorized department.".to_string()),
        }
    };
    
    let po_data = serde_json::to_value(&sample_po)?;
    let sample_message = Message {
        role: "user".to_string(),
        parts: vec![
            Part::Data { data: po_data },
        ],
    };
    
    // Send the task
    match agent.send_task(sample_message).await {
        Ok(task) => {
            println!("📋 Purchase order task created with ID: {}", task.id);
            
            // Display the task status
            println!("📋 Task status: {:?}", task.status.state);
            
            if let Some(message) = &task.status.message {
                println!("💬 Response role: {}", message.role);
                
                // Display text response
                for (i, part) in message.parts.iter().enumerate() {
                    match part {
                        Part::Text { text } => {
                            println!("� CSV Output: {}", text);
                        }
                        Part::Data { data } => {
                            println!("📊 Detailed Processing Result (part {}):", i + 1);
                            println!("{}", serde_json::to_string_pretty(data)?);
                        }
                        _ => {}
                    }
                }
            }
            
            // Retrieve the task to demonstrate get_task
            println!("\n🔍 Retrieving task to verify storage...");
            match agent.get_task(&task.id).await {
                Ok(retrieved_task) => {
                    println!("✅ Successfully retrieved task: {}", retrieved_task.id);
                    println!("📋 Retrieved task status: {:?}", retrieved_task.status.state);
                }
                Err(e) => println!("❌ Error retrieving task: {}", e),
            }
        }
        Err(e) => println!("❌ Error processing purchase order: {}", e),
    }
    
    println!("\n✨ Purchase Order Processing Agent ready for operation!");
    println!("🛒 Send purchase order JSON data to process orders, validate them, and track approval status.");
    
    Ok(())
}
