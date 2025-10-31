use data_agent_rust::{PurchaseOrderAgent, PurchaseOrderWrapper, PurchaseOrder, PurchaseOrderItem};
use a2a::{A2AProtocol, Message, Part};
use std::error::Error;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Purchase Order Processing Agent - Enhanced Examples");
    println!("========================================================\n");
    
    // Create the agent
    let agent = PurchaseOrderAgent::with_config(
        "Enhanced Purchase Order Agent",
        "Advanced A2A agent for comprehensive purchase order processing",
        "http://localhost:8080",
        "1.5.0"
    );
    
    // Display agent information
    let card = agent.get_agent_card();
    println!("📋 Agent Information:");
    println!("   Name: {}", card.name);
    println!("   Version: {}", card.version);
    println!("   URL: {}", card.url);
    if let Some(desc) = &card.description {
        println!("   Description: {}", desc);
    }
    println!();
    
    // Example 1: Process your exact purchase order
    println!("🧪 Example 1: Processing the specified purchase order");
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
    
    let po_message = Message {
        role: "user".to_string(),
        parts: vec![
            Part::Data { data: serde_json::to_value(&sample_po)? },
        ],
    };
    
    let task1 = agent.send_task(po_message).await?;
    println!("   Task ID: {}", task1.id);
    println!("   Status: {:?}", task1.status.state);
    
    if let Some(msg) = &task1.status.message {
        if let Part::Text { text } = &msg.parts[0] {
            println!("   CSV Output: {}", text);
        }
        if msg.parts.len() > 1 {
            if let Part::Data { data } = &msg.parts[1] {
                println!("   Detailed Result:");
                println!("{}", serde_json::to_string_pretty(data)?);
            }
        }
    }
    println!();
    
    // Example 2: Process an invalid purchase order
    println!("🧪 Example 2: Processing invalid purchase order (validation test)");
    let invalid_po = PurchaseOrderWrapper {
        purchase_order: PurchaseOrder {
            supplier_name: "".to_string(), // Invalid: empty
            supplier_address_line1: "123 Test St".to_string(),
            supplier_address_line2: None,
            supplier_city: "Test City".to_string(),
            supplier_state: "CA".to_string(),
            supplier_postal_code: "12345".to_string(),
            supplier_country: "USA".to_string(),
            items: vec![], // Invalid: no items
            po_number: "".to_string(), // Invalid: empty
            created_by: "Test User".to_string(),
            buyer_department: "InvalidDept".to_string(), // Warning: unauthorized dept
            notes: None,
            tax_rate: 0.07,
            sub_total: 0.0,
            tax: 0.0,
            grand_total: 0.0,
            is_approved: false,
            approval_reason: None,
        }
    };
    
    let invalid_message = Message {
        role: "user".to_string(),
        parts: vec![
            Part::Data { data: serde_json::to_value(&invalid_po)? },
        ],
    };
    
    let task2 = agent.send_task(invalid_message).await?;
    println!("   Task ID: {}", task2.id);
    println!("   Status: {:?} (Expected: Failed due to validation errors)", task2.status.state);
    
    if let Some(msg) = &task2.status.message {
        if let Part::Text { text } = &msg.parts[0] {
            println!("   CSV Output: {}", text);
        }
    }
    println!();
    
    // Example 3: Process purchase order from JSON string
    println!("🧪 Example 3: Processing purchase order from JSON text");
    let json_string = serde_json::to_string(&sample_po)?;
    
    let text_message = Message {
        role: "user".to_string(),
        parts: vec![
            Part::Text { text: json_string },
        ],
    };
    
    let task3 = agent.send_task(text_message).await?;
    println!("   Task ID: {}", task3.id);
    println!("   Status: {:?}", task3.status.state);
    println!();
    
    // Example 4: Task retrieval and cancellation
    println!("🧪 Example 4: Task management operations");
    
    // Retrieve a task
    match agent.get_task(&task1.id).await {
        Ok(retrieved_task) => {
            println!("   ✅ Retrieved task: {}", retrieved_task.id);
            println!("   Status: {:?}", retrieved_task.status.state);
        }
        Err(e) => println!("   ❌ Error retrieving task: {}", e),
    }
    
    // Cancel a task
    match agent.cancel_task(&task3.id).await {
        Ok(cancelled_task) => {
            println!("   ✅ Cancelled task: {}", cancelled_task.id);
            println!("   New status: {:?}", cancelled_task.status.state);
        }
        Err(e) => println!("   ❌ Error cancelling task: {}", e),
    }
    println!();
    
    println!("✨ All purchase order processing examples completed successfully!");
    println!("🛒 The agent can handle:");
    println!("   • Purchase order validation");
    println!("   • Financial calculations verification");
    println!("   • Business rules checking");
    println!("   • Approval status tracking");
    println!("   • Comprehensive error reporting");
    
    Ok(())
}