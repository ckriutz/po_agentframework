#!/usr/bin/env python3
"""
Python client example for the Purchase Order Processing Agent
"""

import requests
import json
import sys

def main():
    print("🐍 Python Client for Purchase Order Agent")
    print("=========================================\n")
    
    base_url = "http://localhost:8080"
    
    try:
        # 1. Health check
        print("1. 🏥 Checking agent health...")
        health_response = requests.get(f"{base_url}/health")
        health_response.raise_for_status()
        health_data = health_response.json()
        print(f"   ✅ Agent is healthy: {health_data['status']}")
        
        # 2. Get agent info
        print("\n2. ℹ️  Getting agent information...")
        info_response = requests.get(f"{base_url}/agent/info")
        info_response.raise_for_status()
        agent_info = info_response.json()
        print(f"   Agent: {agent_info['name']}")
        print(f"   Version: {agent_info['version']}")
        
        # 3. Prepare purchase order
        print("\n3. 📦 Preparing sample purchase order...")
        purchase_order = {
            "purchaseOrder": {
                "supplierName": "Tech Supply Co",
                "supplierAddressLine1": "456 Tech Street",
                "supplierAddressLine2": None,
                "supplierCity": "Silicon Valley",
                "supplierState": "CA",
                "supplierPostalCode": "94000",
                "supplierCountry": "USA",
                "items": [
                    {
                        "itemCode": "LAPTOP-001",
                        "description": "Business Laptop",
                        "quantity": 2,
                        "unitPrice": 999.99,
                        "lineTotal": 1999.98
                    }
                ],
                "poNumber": "TECH-2024-001",
                "createdBy": "Python Client",
                "buyerDepartment": "IT",
                "notes": "Urgent order for new employees",
                "taxRate": 0.08,
                "subTotal": 1999.98,
                "tax": 159.998,
                "grandTotal": 2159.978,
                "isApproved": False,
                "approvalReason": None
            }
        }
        
        # 4. Submit for processing
        print("\n4. 🚀 Submitting purchase order...")
        task_request = {
            "message": {
                "role": "user",
                "parts": [
                    {
                        "Data": {
                            "data": purchase_order
                        }
                    }
                ]
            }
        }
        
        task_response = requests.post(
            f"{base_url}/agent/task",
            json=task_request,
            headers={"Content-Type": "application/json"}
        )
        task_response.raise_for_status()
        task_result = task_response.json()
        
        task_id = task_result.get('task_id', 'unknown')
        print(f"   ✅ Task submitted successfully!")
        print(f"   📋 Task ID: {task_id}")
        print(f"   📊 Status: {task_result['status']}")
        
        if 'csv_output' in task_result and task_result['csv_output']:
            print(f"   💾 CSV Output: {task_result['csv_output']}")
        
        # 5. Get detailed results
        print("\n5. 🔍 Retrieving detailed results...")
        details_response = requests.get(f"{base_url}/agent/task/{task_id}")
        details_response.raise_for_status()
        details = details_response.json()
        
        if 'detailed_result' in details and details['detailed_result']:
            result = details['detailed_result']
            print(f"   📊 Processing Results:")
            print(f"      PO Number: {result.get('po_number', 'N/A')}")
            print(f"      Status: {result.get('status', 'N/A')}")
            print(f"      💰 Grand Total: ${result.get('grand_total', 0)}")
            print(f"      🏢 Supplier: {result.get('supplier_name', 'N/A')}")
            
            if result.get('validation_errors'):
                print(f"      ❌ Validation Errors: {result['validation_errors']}")
            if result.get('warnings'):
                print(f"      ⚠️  Warnings: {result['warnings']}")
        
        print("\n🎉 Python client example completed!")
        
    except requests.exceptions.RequestException as e:
        print(f"❌ Error connecting to agent: {e}")
        print("Make sure the agent server is running on http://localhost:8080")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()