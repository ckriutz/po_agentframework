# Processing Agent Python Module

A simplified Python implementation of the processing agent for Microsoft Agent Framework that provides A2A (Application-to-Application) processing of purchase orders using **Azure OpenAI**.

This is a streamlined Python port of the C# processing agent found in `AFPurchaseOrderClient/Agents.cs`, with **Agent Card** support and **FastAPI** for modern web APIs.

## Overview

The processing agent evaluates purchase orders for approval using Azure OpenAI based on these business rules:

1. **Grand Total Limit**: Must be less than $1000
2. **Supplier Validation**: Supplier Name must not be empty
3. **Department Validation**: Buyer Department must be one of: "Travel", "Marketing", "IT", "HR"

## Architecture

```
processing_agent_py/
├── models.py          # Data models (PurchaseOrder, PurchaseOrderApproval)
├── agents.py          # ProcessingAgent using Azure OpenAI
├── agent_card.py      # Agent Card for A2A discovery
├── server.py          # FastAPI web server
├── test_simple.py     # Simple test suite
└── requirements.txt   # Dependencies
```

## Installation

1. Install dependencies:
```bash
pip install -r requirements.txt
```

2. Set environment variables:
```bash
export ENDPOINT="https://your-resource.openai.azure.com/"
export API_KEY="your-azure-openai-api-key"
export DEPLOYMENT_NAME="your-model-deployment-name"
```

## Usage

### Basic Usage

```python
from processing_agent_py import create_processing_agent, PurchaseOrder

# Create purchase order
po_data = {
    "poNumber": "PO-2024-001",
    "supplierName": "Contoso Corporation",
    "buyerDepartment": "IT", 
    "grandTotal": 750.00
}

purchase_order = PurchaseOrder.from_dict(po_data)

# Process with Azure OpenAI
processing_agent = create_processing_agent()
approval = processing_agent.process_purchase_order(purchase_order)

print(f"Approved: {approval.is_approved}")
print(f"Reason: {approval.approval_reason}")
```

### Web Server

```bash
# Run FastAPI server
python server.py --host localhost --port 5001 --reload
```

### A2A Endpoints

- `GET /.well-known/agent` - Agent card for discovery
- `GET /health` - Health check
- `POST /process` - Process purchase order
- `GET /` - Agent information

### Example API Usage

```bash
# Get agent card
curl http://localhost:5001/.well-known/agent

# Process purchase order
curl -X POST http://localhost:5001/process \
  -H "Content-Type: application/json" \
  -d '{
    "poNumber": "PO-001",
    "supplierName": "Contoso",
    "buyerDepartment": "IT",
    "grandTotal": 500
  }'
```

## Agent Card

```json
{
  "Name": "The Purchase Order Processing Agent",
  "Description": "An agent that processes purchase order JSON data and validates against business rules for approval decisions.",
  "Url": "http://localhost:5001",
  "Version": "1.0.0",
  "DefaultInputModes": ["application/json"],
  "DefaultOutputModes": ["application/json", "text"],
  "Capabilities": {
    "Streaming": true,
    "PushNotifications": false
  },
  "Skills": [
    {
      "Id": "purchase-order-processing-agent",
      "Name": "PurchaseOrderProcessingAgent",
      "Description": "A skill that processes Purchase Order JSON data and determines approval based on business rules.",
      "Tags": ["purchase-order", "approval-processing", "business-rules", "validation"],
      "Examples": [
        "Evaluate this purchase order for approval based on business rules.",
        "Analyze the attached PO JSON and determine if it meets approval criteria.",
        "Process this purchase order data and check: Grand Total < $1000, valid supplier, valid department."
      ]
    }
  ]
}
```

## Testing

```bash
python test_simple.py
```

## Dependencies

- `azure-ai-openai>=1.0.0` - Azure OpenAI client
- `fastapi>=0.104.0` - Modern web framework
- `uvicorn>=0.24.0` - ASGI server

## Key Features

✅ **Azure OpenAI Integration**: Uses `azure-ai-openai` package  
✅ **Simple Architecture**: Minimal, focused codebase  
✅ **FastAPI**: Modern Python web framework with auto-docs  
✅ **Agent Card**: A2A discovery support  
✅ **Type Safety**: Pydantic models for validation  
✅ **Processing Only**: No data storage - just approval logic  

## Comparison with C# Version

| Feature | C# Version | Python Version |
|---------|------------|----------------|
| AI Integration | Microsoft.Agents.AI | Azure AI OpenAI |
| Web Framework | ASP.NET Core | FastAPI |
| Agent Card | Built-in | Custom implementation |
| Data Models | C# classes | Python dataclasses |
| Validation | JSON schema | Pydantic models |

This implementation provides a clean, simple, and modern Python alternative to the C# processing agent with full Microsoft Agent Framework compatibility.