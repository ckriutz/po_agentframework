#!/usr/bin/env python3
"""
Simple test script for the Azure OpenAI Processing Agent.
"""
import sys
import os

# Add the current directory to the Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def test_models():
    """Test data models."""
    print("=== Testing Data Models ===")
    
    from models import PurchaseOrder, PurchaseOrderApproval
    
    # Test PurchaseOrder
    po_data = {
        'poNumber': 'TEST-001',
        'supplierName': 'Test Supplier',
        'buyerDepartment': 'IT',
        'grandTotal': 500.0
    }
    
    po = PurchaseOrder.from_dict(po_data)
    print(f"✓ PurchaseOrder: {po.po_number}")
    
    # Test JSON serialization
    json_str = po.to_json()
    po_restored = PurchaseOrder.from_json(json_str)
    print(f"✓ JSON serialization: {po_restored.po_number}")
    
    # Test PurchaseOrderApproval
    approval_data = {
        'poNumber': 'TEST-001',
        'isApproved': True,
        'approvalReason': 'Test approval'
    }
    
    approval = PurchaseOrderApproval.from_dict(approval_data)
    print(f"✓ PurchaseOrderApproval: {approval.po_number} - {approval.is_approved}")
    
    return True

def test_agent_card():
    """Test agent card creation."""
    print("\n=== Testing Agent Card ===")
    
    from agent_card import create_processing_agent_card
    
    card = create_processing_agent_card("http://localhost:5001")
    print(f"✓ Agent Card: {card.name}")
    print(f"✓ Skills: {len(card.skills)}")
    
    # Test JSON serialization
    card_json = card.to_json()
    import json
    parsed = json.loads(card_json)
    print(f"✓ JSON: {len(parsed['Skills'])} skills")
    
    return True

def test_agent_creation():
    """Test agent creation with mock config."""
    print("\n=== Testing Agent Creation ===")
    
    from agents import ProcessingAgent
    
    try:
        # Should fail without config
        agent = ProcessingAgent()
        print("✗ Expected failure without config")
        return False
    except ValueError as e:
        print("✓ Proper config validation")
    
    try:
        # Should succeed with mock config
        agent = ProcessingAgent(
            endpoint="https://mock.openai.azure.com/",
            api_key="mock-key",
            deployment_name="mock-deployment"
        )
        print("✓ Agent creation with config")
        return True
    except Exception as e:
        print(f"✗ Unexpected error: {e}")
        return False

def test_server_imports():
    """Test server imports."""
    print("\n=== Testing Server Imports ===")
    
    try:
        from server import ProcessingAgentServer, create_app
        print("✓ Server imports")
        
        # Test app creation (will fail at agent init without config, but that's expected)
        try:
            app = create_app()
            print("✗ Expected app creation to fail without config")
            return False
        except ValueError:
            print("✓ App properly validates config")
            return True
        except Exception as e:
            print(f"✗ Unexpected error: {e}")
            return False
            
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        return False

def main():
    """Run all tests."""
    print("Azure OpenAI Processing Agent Test Suite")
    print("=" * 50)
    
    success = True
    
    try:
        success &= test_models()
        success &= test_agent_card()
        success &= test_agent_creation()
        success &= test_server_imports()
        
        if success:
            print("\n🎉 All tests passed!")
            print("\nTo run the server:")
            print("  export ENDPOINT='your-azure-openai-endpoint'")
            print("  export API_KEY='your-api-key'") 
            print("  export DEPLOYMENT_NAME='your-deployment'")
            print("  python server.py")
        else:
            print("\n❌ Some tests failed.")
            
    except Exception as e:
        print(f"\n❌ Test suite failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()