use a2a::{A2AProtocol, AgentCard, Message, Task, TaskStatus, TaskState, Part, InMemoryTaskStore};
use async_trait::async_trait;
use std::error::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Purchase Order Item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseOrderItem {
    pub item_code: String,
    pub description: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub line_total: f64,
}

/// Purchase Order structure matching the expected format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseOrder {
    pub supplier_name: String,
    pub supplier_address_line1: String,
    pub supplier_address_line2: Option<String>,
    pub supplier_city: String,
    pub supplier_state: String,
    pub supplier_postal_code: String,
    pub supplier_country: String,
    pub items: Vec<PurchaseOrderItem>,
    pub po_number: String,
    pub created_by: String,
    pub buyer_department: String,
    pub notes: Option<String>,
    pub tax_rate: f64,
    pub sub_total: f64,
    pub tax: f64,
    pub grand_total: f64,
    pub is_approved: bool,
    pub approval_reason: Option<String>,
}

/// Wrapper for the incoming purchase order data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseOrderWrapper {
    pub purchase_order: PurchaseOrder,
}

/// Complete processing result for a purchase order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub status: String,
    pub po_number: String,
    pub validation_errors: Vec<String>,
    pub warnings: Vec<String>,
    pub summary: PurchaseOrderSummary,
    pub processed_at: DateTime<Utc>,
    // Added fields for CSV output
    pub supplier_name: String,
    pub buyer_department: String,
    pub notes: Option<String>,
    pub sub_total: f64,
    pub tax: f64,
    pub grand_total: f64,
}

/// Summary information about the processed purchase order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderSummary {
    pub total_items: usize,
    pub total_quantity: u32,
    pub sub_total: f64,
    pub tax: f64,
    pub grand_total: f64,
    pub supplier: String,
    pub department: String,
    pub is_approved: bool,
}

/// A specialized A2A agent for processing Purchase Orders
pub struct PurchaseOrderAgent {
    agent_card: AgentCard,
    task_store: Arc<Mutex<InMemoryTaskStore>>,
}

impl PurchaseOrderAgent {
    /// Create a new PurchaseOrderAgent
    pub fn new() -> Self {
        let agent_card = AgentCard {
            name: "Purchase Order Processing Agent".to_string(),
            description: Some("Specialized A2A agent for processing and validating purchase orders".to_string()),
            url: "http://localhost:8080".to_string(),
            version: "1.0.0".to_string(),
        };

        Self {
            agent_card,
            task_store: Arc::new(Mutex::new(InMemoryTaskStore::new())),
        }
    }

    /// Create a new PurchaseOrderAgent with custom configuration
    pub fn with_config(name: &str, description: &str, url: &str, version: &str) -> Self {
        let agent_card = AgentCard {
            name: name.to_string(),
            description: Some(description.to_string()),
            url: url.to_string(),
            version: version.to_string(),
        };

        Self {
            agent_card,
            task_store: Arc::new(Mutex::new(InMemoryTaskStore::new())),
        }
    }

    /// Get the agent's card information
    pub fn get_agent_card(&self) -> &AgentCard {
        &self.agent_card
    }

    /// Validate a purchase order and return any errors or warnings
    fn validate_purchase_order(&self, po: &PurchaseOrder) -> (Vec<String>, Vec<String>) {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Required field validation
        if po.supplier_name.trim().is_empty() {
            errors.push("Supplier name is required".to_string());
        }
        if po.po_number.trim().is_empty() {
            errors.push("PO number is required".to_string());
        }
        if po.created_by.trim().is_empty() {
            errors.push("Created by field is required".to_string());
        }
        if po.buyer_department.trim().is_empty() {
            errors.push("Buyer department is required".to_string());
        }

        // Items validation
        if po.items.is_empty() {
            errors.push("Purchase order must contain at least one item".to_string());
        } else {
            for (index, item) in po.items.iter().enumerate() {
                if item.item_code.trim().is_empty() {
                    errors.push(format!("Item {} is missing item code", index + 1));
                }
                if item.description.trim().is_empty() {
                    errors.push(format!("Item {} is missing description", index + 1));
                }
                if item.quantity == 0 {
                    errors.push(format!("Item {} has zero quantity", index + 1));
                }
                if item.unit_price <= 0.0 {
                    errors.push(format!("Item {} has invalid unit price", index + 1));
                }
                
                // Verify line total calculation
                let expected_total = item.quantity as f64 * item.unit_price;
                if (item.line_total - expected_total).abs() > 0.01 {
                    warnings.push(format!(
                        "Item {} line total mismatch: expected {:.2}, got {:.2}",
                        index + 1, expected_total, item.line_total
                    ));
                }
            }
        }

        // Financial calculations validation
        let calculated_subtotal: f64 = po.items.iter().map(|item| item.line_total).sum();
        if (po.sub_total - calculated_subtotal).abs() > 0.01 {
            warnings.push(format!(
                "Subtotal mismatch: expected {:.2}, got {:.2}",
                calculated_subtotal, po.sub_total
            ));
        }

        let calculated_tax = po.sub_total * po.tax_rate;
        if (po.tax - calculated_tax).abs() > 0.01 {
            warnings.push(format!(
                "Tax calculation mismatch: expected {:.2}, got {:.2}",
                calculated_tax, po.tax
            ));
        }

        let calculated_grand_total = po.sub_total + po.tax;
        if (po.grand_total - calculated_grand_total).abs() > 0.01 {
            warnings.push(format!(
                "Grand total mismatch: expected {:.2}, got {:.2}",
                calculated_grand_total, po.grand_total
            ));
        }

        // Business rules warnings
        if po.grand_total > 10000.0 {
            warnings.push("High value purchase order - may require additional approval".to_string());
        }

        if po.tax_rate < 0.0 || po.tax_rate > 0.2 {
            warnings.push("Unusual tax rate detected".to_string());
        }

        // Department validation (example authorized departments)
        let authorized_departments = ["Marketing", "Sales", "IT", "Finance", "Operations", "HR"];
        if !authorized_departments.contains(&po.buyer_department.as_str()) {
            warnings.push(format!("Department '{}' may not be authorized for purchases", po.buyer_department));
        }

        (errors, warnings)
    }

    /// Create a summary of the purchase order
    fn create_summary(&self, po: &PurchaseOrder) -> PurchaseOrderSummary {
        PurchaseOrderSummary {
            total_items: po.items.len(),
            total_quantity: po.items.iter().map(|item| item.quantity).sum(),
            sub_total: po.sub_total,
            tax: po.tax,
            grand_total: po.grand_total,
            supplier: po.supplier_name.clone(),
            department: po.buyer_department.clone(),
            is_approved: po.is_approved,
        }
    }

    /// Process a purchase order message
    async fn process_purchase_order(&self, message: &Message) -> Result<ProcessingResult, Box<dyn Error>> {
        println!("🛒 Processing purchase order message from role: {}", message.role);

        // Look for purchase order data in message parts
        let mut purchase_order: Option<PurchaseOrder> = None;

        for part in &message.parts {
            match part {
                Part::Data { data } => {
                    // Try to parse as PurchaseOrderWrapper first
                    if let Ok(wrapper) = serde_json::from_value::<PurchaseOrderWrapper>(data.clone()) {
                        purchase_order = Some(wrapper.purchase_order);
                        break;
                    }
                    // Try to parse as direct PurchaseOrder
                    else if let Ok(po) = serde_json::from_value::<PurchaseOrder>(data.clone()) {
                        purchase_order = Some(po);
                        break;
                    }
                }
                Part::Text { text } => {
                    // Try to parse JSON from text
                    if let Ok(wrapper) = serde_json::from_str::<PurchaseOrderWrapper>(text) {
                        purchase_order = Some(wrapper.purchase_order);
                        break;
                    }
                    else if let Ok(po) = serde_json::from_str::<PurchaseOrder>(text) {
                        purchase_order = Some(po);
                        break;
                    }
                }
                _ => continue,
            }
        }

        let po = purchase_order.ok_or("No valid purchase order found in message")?;

        // Validate the purchase order
        let (validation_errors, warnings) = self.validate_purchase_order(&po);

        // Create summary
        let summary = self.create_summary(&po);

        // Determine processing status
        let status = if !validation_errors.is_empty() {
            "VALIDATION_FAILED".to_string()
        } else if po.is_approved {
            "APPROVED".to_string()
        } else {
            "PENDING_APPROVAL".to_string()
        };

        let result = ProcessingResult {
            status,
            po_number: po.po_number.clone(),
            validation_errors,
            warnings,
            summary,
            processed_at: Utc::now(),
            // Include original data for CSV output
            supplier_name: po.supplier_name.clone(),
            buyer_department: po.buyer_department.clone(),
            notes: po.notes.clone(),
            sub_total: po.sub_total,
            tax: po.tax,
            grand_total: po.grand_total,
        };

        println!("✅ Purchase order {} processed with status: {}", result.po_number, result.status);

        Ok(result)
    }

    /// Get current timestamp as string
    fn current_timestamp(&self) -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
    }
}

#[async_trait]
impl A2AProtocol for PurchaseOrderAgent {
    /// Send a task to the agent for processing
    async fn send_task(
        &self,
        message: Message,
    ) -> Result<Task, Box<dyn Error>> {
        println!("📨 Received purchase order processing task from role: {}", message.role);
        
        // Generate a unique task ID
        let task_id = Uuid::new_v4().to_string();
        
        // Process the purchase order
        let processing_result = self.process_purchase_order(&message).await?;
        
        // Create CSV format response as requested
        let notes_escaped = processing_result.notes.as_ref()
            .map(|n| n.replace("\"", "\"\""))  // Escape quotes in CSV
            .unwrap_or_else(|| "".to_string());
        
        let csv_response = format!(
            "{},{},{},{},{},{},\"{}\"",
            processing_result.po_number,
            processing_result.sub_total,
            processing_result.tax,
            processing_result.grand_total,
            processing_result.supplier_name,
            processing_result.buyer_department,
            notes_escaped
        );

        // Create response message with CSV format and detailed result
        let response_message = Message {
            role: "assistant".to_string(),
            parts: vec![
                Part::Text { 
                    text: csv_response
                },
                Part::Data { data: serde_json::to_value(&processing_result)? }
            ],
        };

        // Create task status
        let status = TaskStatus {
            state: if processing_result.validation_errors.is_empty() {
                TaskState::Completed
            } else {
                TaskState::Failed
            },
            message: Some(response_message),
            timestamp: self.current_timestamp(),
        };

        // Create the task
        let task = Task {
            id: task_id.clone(),
            session_id: None,
            status,
            artifacts: None,
        };

        // Store the task
        {
            let mut store = self.task_store.lock().map_err(|_| "Failed to acquire task store lock")?;
            store.store_task(task.clone());
        }
        
        println!("✅ Purchase order task {} completed", task_id);
        
        Ok(task)
    }

    /// Retrieve a task by its ID
    async fn get_task(&self, task_id: &str) -> Result<Task, Box<dyn Error>> {
        println!("🔍 Looking up task: {}", task_id);
        
        let store = self.task_store.lock().map_err(|_| "Failed to acquire task store lock")?;
        match store.get_task(task_id) {
            Some(task) => {
                println!("✅ Found task: {}", task_id);
                Ok(task)
            }
            None => {
                println!("❌ Task not found: {}", task_id);
                Err(format!("Task {} not found", task_id).into())
            }
        }
    }

    /// Cancel a task by its ID
    async fn cancel_task(&self, task_id: &str) -> Result<Task, Box<dyn Error>> {
        println!("🛑 Attempting to cancel task: {}", task_id);
        
        let mut store = self.task_store.lock().map_err(|_| "Failed to acquire task store lock")?;
        
        // Retrieve the existing task
        let existing_task = match store.get_task(task_id) {
            Some(task) => task,
            None => return Err(format!("Task {} not found", task_id).into()),
        };
        
        // Update task status to cancelled
        let mut updated_task = existing_task.clone();
        updated_task.status.state = TaskState::Failed;
        updated_task.status.message = Some(Message {
            role: "system".to_string(),
            parts: vec![Part::Text { text: "Purchase order processing task was cancelled by user request".to_string() }],
        });
        updated_task.status.timestamp = self.current_timestamp();

        // Store the updated task
        store.store_task(updated_task.clone());
        
        println!("✅ Task {} cancelled successfully", task_id);
        
        Ok(updated_task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use a2a::{Message, Part, A2AProtocol, TaskState};

    fn create_sample_purchase_order() -> PurchaseOrderWrapper {
        PurchaseOrderWrapper {
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
        }
    }

    #[tokio::test]
    async fn test_agent_creation() {
        let agent = PurchaseOrderAgent::new();
        let card = agent.get_agent_card();
        
        assert_eq!(card.name, "Purchase Order Processing Agent");
        assert_eq!(card.version, "1.0.0");
        assert_eq!(card.url, "http://localhost:8080");
        assert!(card.description.is_some());
        assert!(card.description.as_ref().unwrap().contains("purchase orders"));
    }

    #[tokio::test]
    async fn test_custom_agent_creation() {
        let agent = PurchaseOrderAgent::with_config(
            "Test PO Agent",
            "Test description",
            "http://test.com",
            "2.0.0"
        );
        let card = agent.get_agent_card();
        
        assert_eq!(card.name, "Test PO Agent");
        assert_eq!(card.version, "2.0.0");
        assert_eq!(card.url, "http://test.com");
        assert_eq!(card.description.as_ref().unwrap(), "Test description");
    }

    #[tokio::test]
    async fn test_process_valid_purchase_order() {
        let agent = PurchaseOrderAgent::new();
        let po_wrapper = create_sample_purchase_order();
        
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Data { data: serde_json::to_value(&po_wrapper).unwrap() }
            ],
        };
        
        let result = agent.send_task(message).await;
        assert!(result.is_ok());
        
        let task = result.unwrap();
        assert!(!task.id.is_empty());
        assert!(matches!(task.status.state, TaskState::Completed));
        assert!(task.status.message.is_some());
        
        // Verify response contains processing result
        let response_msg = task.status.message.unwrap();
        assert_eq!(response_msg.role, "assistant");
        assert_eq!(response_msg.parts.len(), 2); // Text + Data parts
        
        if let Part::Text { text } = &response_msg.parts[0] {
            // Verify CSV format: PONumber,Subtotal,Tax,GrandTotal,SupplierName,BuyerDepartment,Notes
            assert!(text.contains("MMS-80085"));
            assert!(text.contains("194.94"));
            assert!(text.contains("13.65"));
            assert!(text.contains("208.59"));
            assert!(text.contains("Marketing Masters Supplies"));
            assert!(text.contains("Marketing"));
        }
    }

    #[tokio::test]
    async fn test_process_purchase_order_from_json_text() {
        let agent = PurchaseOrderAgent::new();
        let po_wrapper = create_sample_purchase_order();
        let json_text = serde_json::to_string(&po_wrapper).unwrap();
        
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Text { text: json_text }
            ],
        };
        
        let result = agent.send_task(message).await;
        assert!(result.is_ok());
        
        let task = result.unwrap();
        assert!(matches!(task.status.state, TaskState::Completed));
    }

    #[tokio::test]
    async fn test_get_task() {
        let agent = PurchaseOrderAgent::new();
        let po_wrapper = create_sample_purchase_order();
        
        // First, create a task
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Data { data: serde_json::to_value(&po_wrapper).unwrap() }
            ],
        };
        
        let created_task = agent.send_task(message).await.unwrap();
        
        // Then retrieve it
        let retrieved_task = agent.get_task(&created_task.id).await.unwrap();
        
        assert_eq!(created_task.id, retrieved_task.id);
        assert!(matches!(retrieved_task.status.state, TaskState::Completed));
    }

    #[tokio::test]
    async fn test_cancel_task() {
        let agent = PurchaseOrderAgent::new();
        let po_wrapper = create_sample_purchase_order();
        
        // Create a task
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Data { data: serde_json::to_value(&po_wrapper).unwrap() }
            ],
        };
        
        let task = agent.send_task(message).await.unwrap();
        
        // Cancel it
        let cancelled_task = agent.cancel_task(&task.id).await.unwrap();
        
        assert_eq!(task.id, cancelled_task.id);
        assert!(matches!(cancelled_task.status.state, TaskState::Failed));
        
        // Verify the cancellation message
        let cancel_msg = cancelled_task.status.message.unwrap();
        assert_eq!(cancel_msg.role, "system");
        if let Part::Text { text } = &cancel_msg.parts[0] {
            assert!(text.contains("cancelled"));
        }
    }

    #[tokio::test]
    async fn test_get_nonexistent_task() {
        let agent = PurchaseOrderAgent::new();
        
        let result = agent.get_task("nonexistent-task-id").await;
        assert!(result.is_err());
        
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not found"));
    }

    #[tokio::test]
    async fn test_invalid_purchase_order_validation() {
        let agent = PurchaseOrderAgent::new();
        
        // Create invalid PO (missing required fields)
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
                created_by: "Tester".to_string(),
                buyer_department: "Marketing".to_string(),
                notes: None,
                tax_rate: 0.07,
                sub_total: 0.0,
                tax: 0.0,
                grand_total: 0.0,
                is_approved: false,
                approval_reason: None,
            }
        };
        
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Data { data: serde_json::to_value(&invalid_po).unwrap() }
            ],
        };
        
        let result = agent.send_task(message).await;
        assert!(result.is_ok());
        
        let task = result.unwrap();
        // Should fail due to validation errors
        assert!(matches!(task.status.state, TaskState::Failed));
    }

    #[tokio::test]
    async fn test_invalid_message_format() {
        let agent = PurchaseOrderAgent::new();
        
        let message = Message {
            role: "user".to_string(),
            parts: vec![
                Part::Text { text: "This is not a purchase order".to_string() }
            ],
        };
        
        let result = agent.send_task(message).await;
        assert!(result.is_err());
        
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("No valid purchase order"));
    }
}