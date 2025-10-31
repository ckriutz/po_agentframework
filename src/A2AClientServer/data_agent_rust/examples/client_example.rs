use reqwest;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Purchase Order Agent Client Example");
    println!("=====================================\n");

    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    // 1. Check if the agent is running
    println!("1. 🏥 Checking agent health...");
    let health_response = client
        .get(&format!("{}/health", base_url))
        .send()
        .await?;
    
    if health_response.status().is_success() {
        let health_data: serde_json::Value = health_response.json().await?;
        println!("   ✅ Agent is healthy: {}", health_data["status"]);
    } else {
        println!("   ❌ Agent health check failed");
        return Ok(());
    }

    // 2. Get agent information
    println!("\n2. ℹ️  Getting agent information...");
    let info_response = client
        .get(&format!("{}/agent/info", base_url))
        .send()
        .await?;
    
    let agent_info: serde_json::Value = info_response.json().await?;
    println!("   Agent: {}", agent_info["name"]);
    println!("   Version: {}", agent_info["version"]);
    println!("   Description: {}", agent_info["description"]);

    // 3. Prepare a sample purchase order
    println!("\n3. 📦 Preparing sample purchase order...");
    let purchase_order = serde_json::json!({
        "purchaseOrder": {
            "supplierName": "Marketing Masters Supplies",
            "supplierAddressLine1": "1234 Creative Avenue, Suite 567",
            "supplierAddressLine2": null,
            "supplierCity": "Imagination City",
            "supplierState": "CA",
            "supplierPostalCode": "90210",
            "supplierCountry": "USA",
            "items": [
                {
                    "itemCode": "bk-2345",
                    "description": "Marketing Strategy Guidebook",
                    "quantity": 3,
                    "unitPrice": 29.99,
                    "lineTotal": 89.97
                },
                {
                    "itemCode": "Bk-1311",
                    "description": "Promotional Materials Handbook",
                    "quantity": 3,
                    "unitPrice": 34.99,
                    "lineTotal": 104.97
                }
            ],
            "poNumber": "MMS-80085",
            "createdBy": "J.J. Schmidt",
            "buyerDepartment": "Marketing",
            "notes": "thanks for the order! Happy learning!! :)",
            "taxRate": 0.07,
            "subTotal": 194.94,
            "tax": 13.65,
            "grandTotal": 208.59,
            "isApproved": true,
            "approvalReason": "Approved: Grand Total $208.59 is below $1000, Supplier Name is provided, and Buyer Department 'Marketing' is an authorized department."
        }
    });

    // 4. Submit the purchase order for processing
    println!("\n4. 🚀 Submitting purchase order for processing...");
    let task_request = serde_json::json!({
        "message": {
            "role": "user",
            "parts": [
                {
                    "type": "data",
                    "data": purchase_order
                }
            ]
        }
    });

    let task_response = client
        .post(&format!("{}/agent/task", base_url))
        .header("Content-Type", "application/json")
        .json(&task_request)
        .send()
        .await?;

    if task_response.status().is_success() {
        let task_result: serde_json::Value = task_response.json().await?;
        let task_id = task_result["task_id"].as_str().unwrap_or("unknown");
        
        println!("   ✅ Task submitted successfully!");
        println!("   📋 Task ID: {}", task_id);
        println!("   📊 Status: {}", task_result["status"]);
        
        if let Some(csv_output) = task_result["csv_output"].as_str() {
            println!("   💾 CSV Output: {}", csv_output);
        }

        // 5. Retrieve the task details
        println!("\n5. 🔍 Retrieving task details...");
        let get_task_response = client
            .get(&format!("{}/agent/task/{}", base_url, task_id))
            .send()
            .await?;

        if get_task_response.status().is_success() {
            let task_details: serde_json::Value = get_task_response.json().await?;
            println!("   📋 Task Status: {}", task_details["status"]);
            
            if let Some(detailed_result) = task_details["detailed_result"].as_object() {
                println!("   📊 Processing Results:");
                println!("      PO Number: {}", detailed_result["po_number"]);
                println!("      Status: {}", detailed_result["status"]);
                if let Some(errors) = detailed_result["validation_errors"].as_array() {
                    if !errors.is_empty() {
                        println!("      ❌ Validation Errors: {:?}", errors);
                    }
                }
                if let Some(warnings) = detailed_result["warnings"].as_array() {
                    if !warnings.is_empty() {
                        println!("      ⚠️  Warnings: {:?}", warnings);
                    }
                }
                println!("      💰 Grand Total: ${}", detailed_result["grand_total"]);
                println!("      🏢 Supplier: {}", detailed_result["supplier_name"]);
                println!("      🏬 Department: {}", detailed_result["buyer_department"]);
            }
        } else {
            println!("   ❌ Failed to retrieve task details");
        }

    } else {
        println!("   ❌ Failed to submit task: {}", task_response.status());
        let error_text = task_response.text().await?;
        println!("   Error details: {}", error_text);
    }

    println!("\n🎉 Client example completed!");
    Ok(())
}