# Purchase Order Processing Agent (A2A Protocol)

A specialized Rust implementation of the [A2A (Agent-to-Agent) Protocol](https://modelcontextprotocol.io/a2a-protocol) for processing and validating purchase orders. This agent is designed to handle purchase order data with comprehensive validation, business rules checking, and approval tracking.

## 🚀 Features

- **A2A Protocol Compliance**: Full implementation of the Agent-to-Agent protocol specification
- **Purchase Order Validation**: Comprehensive validation of purchase order data including:
  - Required fields validation (supplier info, PO number, items, etc.)
  - Financial calculations verification (subtotals, taxes, grand totals)
  - Business rules checking (authorized departments, approval limits)
  - Item-level validation (quantities, prices, line totals)
- **Async Processing**: Built with Tokio for high-performance async operations
- **Thread-Safe**: Concurrent task processing with thread-safe storage
- **Error Handling**: Detailed error reporting and validation feedback
- **Task Management**: Complete CRUD operations for task tracking
- **Flexible Input**: Accepts purchase orders as structured data or JSON text

## 📋 Purchase Order Schema

The agent processes purchase orders with this exact structure:

```json
{
  "purchaseOrder": {
    "supplier_name": "Marketing Masters Supplies",
    "supplier_address_line1": "1234 Creative Avenue, Suite 567",
    "supplier_address_line2": null,
    "supplier_city": "Imagination City",
    "supplier_state": "CA",
    "supplier_postal_code": "90210",
    "supplier_country": "USA",
    "items": [
      {
        "item_code": "bk-2345",
        "description": "Marketing Strategy Guidebook",
        "quantity": 3,
        "unit_price": 29.99,
        "line_total": 89.97
      }
    ],
    "po_number": "MMS-80085",
    "created_by": "J.J. Schmidt",
    "buyer_department": "Marketing",
    "notes": "thanks for the order! Happy learning!! :)",
    "tax_rate": 0.07,
    "sub_total": 194.94,
    "tax": 13.65,
    "grand_total": 208.59,
    "is_approved": true,
    "approval_reason": "Approved: Grand Total $208.59 is below $1000..."
  }
}
```

## 🛠 Installation & Setup

### Prerequisites
- Rust 1.70+ (uses 2024 edition)
- Tokio async runtime

### Dependencies
- `a2a = "0.1.0"` - Core A2A protocol implementation
- `tokio = { version = "1.0", features = ["full"] }` - Async runtime
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_json = "1.0"` - JSON handling
- `uuid = { version = "1.0", features = ["v4"] }` - Unique ID generation
- `async-trait = "0.1"` - Async trait support
- `chrono = { version = "0.4", features = ["serde"] }` - Date/time handling

### Build & Run

```bash
# Clone and navigate to the project
cd data_agent_rust

# Build the project
cargo build

# Run the basic demo
cargo run --bin demo

# Run comprehensive examples
cargo run --example enhanced_demo

# Test A2A agent card implementation
cargo run --example test_agent_card

# Test server A2A endpoints
cargo run --example test_server_endpoint

# Run tests
cargo test
```

## 📖 Usage Examples

### Basic Purchase Order Processing

```rust
use data_agent_rust::PurchaseOrderAgent;
use a2a::{A2AProtocol, Message, Part};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the agent
    let agent = PurchaseOrderAgent::new();
    
    // Prepare your purchase order data
    let po_json = r#"{"purchaseOrder": {...}}"#;
    
    // Send processing task
    let message = Message {
        role: "user".to_string(),
        parts: vec![Part::Text { text: po_json.to_string() }],
    };
    
    let task = agent.send_task(message).await?;
    println!("Task ID: {}", task.id);
    println!("Status: {:?}", task.status.state);
    
    Ok(())
}
```

### Custom Agent Configuration

```rust
let agent = PurchaseOrderAgent::with_config(
    "Custom PO Agent",
    "Specialized purchase order processor",
    "http://my-po-agent.example.com",
    "2.0.0"
);
```

## 📊 Processing Results

The agent returns results in **CSV format** as the primary response:

### CSV Output Format
```
PONumber,Subtotal,Tax,GrandTotal,SupplierName,BuyerDepartment,Notes
```

**Example CSV output:**
```
MMS-80085,194.94,13.65,208.59,Marketing Masters Supplies,Marketing,"thanks for the order! Happy learning!! :)"
```

The agent also provides detailed processing metadata including:
- **Validation Status**: APPROVED, VALIDATION_FAILED, or PROCESSING_ERROR
- **Summary Data**: Key metrics (total items, quantities, amounts, department)
- **Validation Errors**: Specific issues found during validation
- **Warnings**: Non-critical issues (unauthorized departments, etc.)
- **Processing Metadata**: Timestamps, approval reasons

## ✅ Validation Rules

The agent enforces these business rules:

### Required Fields
- Supplier name and complete address
- Purchase order number
- At least one line item
- Created by (user name)
- Buyer department

### Financial Validation
- Line totals must equal quantity × unit price
- Subtotal must equal sum of all line totals
- Tax must equal subtotal × tax rate
- Grand total must equal subtotal + tax

### Business Rules
- Authorized departments: IT, Marketing, Finance, Operations, HR
- Auto-approval for orders under $1000 with valid supplier and authorized department
- Warnings for unauthorized departments (still processes but flags)

## 🏗 Architecture

```
src/
├── lib.rs              # Library exports
├── main.rs             # Basic demo with your exact PO data
├── main_server.rs      # A2A compliant web server
├── agent.rs            # Core PurchaseOrderAgent implementation
├── server.rs           # HTTP endpoints and routing
├── a2a_agent_card.rs   # A2A compliant AgentCard structures
└── examples/
    ├── enhanced_demo.rs        # Advanced usage examples
    ├── test_agent_card.rs      # A2A agent card testing
    └── test_server_endpoint.rs # A2A server endpoint testing
```

### Key Components

- **PurchaseOrderAgent**: Main agent implementing A2AProtocol for PO processing
- **A2AAgentCard**: Full A2A compliant agent card with capabilities, skills, and metadata
- **Web Server**: HTTP endpoints for A2A protocol compliance (/.well-known/agent.json)
- **PurchaseOrder**: Core data structure matching your schema exactly
- **PurchaseOrderItem**: Individual line item structure
- **ProcessingResult**: Complete processing outcome with validation details
- **PurchaseOrderSummary**: Key metrics and status summary

## 🔧 API Reference

### PurchaseOrderAgent

- `new()` - Create agent with default configuration
- `with_config(name, description, url, version)` - Create with custom config
- `get_agent_card()` - Get basic agent metadata (legacy)
- `get_a2a_agent_card()` - Get full A2A compliant agent card
- `send_task(message)` - Process purchase order
- `get_task(task_id)` - Retrieve task by ID
- `cancel_task(task_id)` - Cancel processing task

### A2A Web Server Endpoints

- `GET /.well-known/agent.json` - **A2A standard agent card endpoint**
- `GET /` - Agent information and API documentation
- `GET /health` - Health check
- `POST /agent/task` - Submit purchase order for processing
- `GET /agent/task/{id}` - Get task status and results
- `POST /agent/task/{id}/cancel` - Cancel a task

### Data Structures

- `PurchaseOrder` - Main purchase order structure
- `PurchaseOrderItem` - Individual line item
- `PurchaseOrderWrapper` - Root wrapper for JSON parsing
- `ProcessingResult` - Complete processing outcome
- `PurchaseOrderSummary` - Key metrics summary

## 🧪 Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Test results show validation, processing, and error handling
running 9 tests
test agent::tests::test_agent_creation ... ok
test agent::tests::test_custom_agent_creation ... ok
test agent::tests::test_process_valid_purchase_order ... ok
test agent::tests::test_invalid_purchase_order_validation ... ok
test agent::tests::test_process_purchase_order_from_json_text ... ok
test agent::tests::test_get_task ... ok
test agent::tests::test_cancel_task ... ok
test agent::tests::test_get_nonexistent_task ... ok
test agent::tests::test_invalid_message_format ... ok
```

## 🎯 A2A Protocol Implementation

This agent implements the full A2A protocol specification with a compliant AgentCard:

### ✅ A2A Agent Card Compliance

The agent exposes its capabilities through a **fully compliant A2A AgentCard** at the standard endpoint:

**🔗 Standard A2A Endpoint:** `http://localhost:8080/.well-known/agent.json`

#### Agent Card Structure

```json
{
  "name": "Purchase Order Processing Agent",
  "description": "Specialized A2A agent for processing, validating, and managing purchase orders...",
  "url": "http://localhost:8080",
  "provider": {
    "organization": "A2A Protocol Framework",
    "url": "https://agent2agent.info"
  },
  "version": "1.0.0",
  "documentationUrl": "http://localhost:8080/docs",
  "capabilities": {
    "streaming": false,
    "pushNotifications": false,
    "stateTransitionHistory": true
  },
  "authentication": {
    "schemes": ["none"],
    "credentials": null
  },
  "defaultInputModes": ["application/json", "text/plain"],
  "defaultOutputModes": ["text/csv", "application/json", "text/plain"],
  "skills": [
    {
      "id": "purchase-order-processing",
      "name": "Purchase Order Processing",
      "description": "Process and validate purchase orders with comprehensive business rules checking...",
      "tags": ["finance", "procurement", "validation", "business-rules", "approval-workflow"],
      "examples": [
        "Process a purchase order for office supplies totaling $500",
        "Validate a marketing department purchase order with tax calculations",
        "Check approval status for a high-value IT equipment purchase order",
        "Generate CSV report from purchase order data"
      ]
    },
    {
      "id": "purchase-order-validation",
      "name": "Purchase Order Validation",
      "description": "Validate purchase order data including required fields, financial calculations...",
      "tags": ["validation", "data-integrity", "business-rules", "compliance"],
      "examples": [
        "Validate that all required fields are present in a purchase order",
        "Check that line totals match quantity × unit price calculations",
        "Verify that tax calculations are correct based on tax rate",
        "Ensure buyer department is authorized for purchases"
      ]
    },
    {
      "id": "purchase-order-reporting",
      "name": "Purchase Order Reporting",
      "description": "Generate structured reports and summaries from purchase order data...",
      "tags": ["reporting", "data-export", "csv", "analytics"],
      "examples": [
        "Generate CSV report with PO number, totals, supplier, and department",
        "Create JSON summary with validation status and key metrics",
        "Export purchase order details for accounting system integration"
      ],
      "outputModes": ["text/csv", "application/json"]
    }
  ]
}
```

#### A2A Compliance Features

- ✅ **Standard Agent Discovery**: Agent card available at `/.well-known/agent.json`
- ✅ **Complete Agent Metadata**: All required and optional A2A fields present
- ✅ **Provider Information**: Organization and URL for agent attribution
- ✅ **Capability Declaration**: Streaming, notifications, and state history support
- ✅ **Authentication Specification**: Clear authentication requirements (none for demo)
- ✅ **Input/Output Modes**: MIME types for supported data formats
- ✅ **Structured Skills**: Detailed skill definitions with IDs, tags, and examples
- ✅ **Task-based Interactions**: Full A2A protocol for send_task, get_task, cancel_task
- ✅ **Structured Data Exchange**: Message with multiple Part types
- ✅ **Error Handling**: Comprehensive validation and processing error responses
- ✅ **Status Tracking**: Task states and completion status monitoring

### Testing A2A Compliance

```bash
# Get the A2A agent card
curl http://localhost:8080/.well-known/agent.json

# Test agent capabilities
curl http://localhost:8080/agent/info

# Health check
curl http://localhost:8080/health

# Test A2A agent card structure
cargo run --example test_agent_card

# Test server endpoints
cargo run --example test_server_endpoint
```

## 🎯 A2A Protocol Implementation

This agent implements the full A2A protocol:

- ✅ **Agent capability discovery** (via AgentCard)
- ✅ **Task-based interactions** (send_task, get_task, cancel_task)
- ✅ **Structured data exchange** (Message with multiple Part types)
- ✅ **Error handling** (validation errors, processing failures)
- ✅ **Status tracking** (task states and completion status)

## 🔮 Future Enhancements

- [ ] REST API endpoint for web integration
- [ ] Database persistence for task history
- [ ] Email notifications for approvals
- [ ] Multiple approval workflows
- [ ] Integration with accounting systems
- [ ] Real-time processing status updates
- [ ] Bulk purchase order processing
- [ ] Advanced reporting and analytics

## 📚 Resources

- [A2A Protocol Specification](https://modelcontextprotocol.io/a2a-protocol)
- [A2A Crate Documentation](https://docs.rs/a2a/latest/a2a/)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.