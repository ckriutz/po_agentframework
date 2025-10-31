"""
Purchase Order data models for the Python Agent Framework implementation.
"""
from dataclasses import dataclass
from typing import List, Optional
import json


@dataclass
class PurchaseOrderItem:
    """Individual line item in a purchase order"""
    item_code: Optional[str] = None
    description: Optional[str] = None
    quantity: int = 0
    unit_price: float = 0.0
    line_total: float = 0.0

    def to_dict(self) -> dict:
        return {
            "itemCode": self.item_code,
            "description": self.description,
            "quantity": self.quantity,
            "unitPrice": self.unit_price,
            "lineTotal": self.line_total
        }

    @classmethod
    def from_dict(cls, data: dict) -> 'PurchaseOrderItem':
        return cls(
            item_code=data.get("itemCode"),
            description=data.get("description"),
            quantity=data.get("quantity", 0),
            unit_price=data.get("unitPrice", 0.0),
            line_total=data.get("lineTotal", 0.0)
        )


@dataclass
class PurchaseOrder:
    """Purchase order information including PO number, amounts, supplier, and buyer details"""
    # Supplier fields
    supplier_name: Optional[str] = None
    supplier_address_line1: Optional[str] = None
    supplier_address_line2: Optional[str] = None
    supplier_city: Optional[str] = None
    supplier_state: Optional[str] = None
    supplier_postal_code: Optional[str] = None
    supplier_country: Optional[str] = None
    
    # Line items
    items: List[PurchaseOrderItem] = None
    
    # Purchase order metadata
    po_number: Optional[str] = None
    created_by: Optional[str] = None
    buyer_department: Optional[str] = None
    
    # Notes
    notes: Optional[str] = None
    
    # Tax information
    tax_rate: float = 0.0
    
    # Computed totals
    sub_total: float = 0.0
    tax: float = 0.0
    grand_total: float = 0.0
    
    # Approval flow
    is_approved: bool = False
    approval_reason: Optional[str] = None

    def __post_init__(self):
        if self.items is None:
            self.items = []

    def to_dict(self) -> dict:
        return {
            "supplierName": self.supplier_name,
            "supplierAddressLine1": self.supplier_address_line1,
            "supplierAddressLine2": self.supplier_address_line2,
            "supplierCity": self.supplier_city,
            "supplierState": self.supplier_state,
            "supplierPostalCode": self.supplier_postal_code,
            "supplierCountry": self.supplier_country,
            "items": [item.to_dict() for item in self.items],
            "poNumber": self.po_number,
            "createdBy": self.created_by,
            "buyerDepartment": self.buyer_department,
            "notes": self.notes,
            "taxRate": self.tax_rate,
            "subTotal": self.sub_total,
            "tax": self.tax,
            "grandTotal": self.grand_total,
            "isApproved": self.is_approved,
            "approvalReason": self.approval_reason
        }

    @classmethod
    def from_dict(cls, data: dict) -> 'PurchaseOrder':
        items = [PurchaseOrderItem.from_dict(item) for item in data.get("items", [])]
        return cls(
            supplier_name=data.get("supplierName"),
            supplier_address_line1=data.get("supplierAddressLine1"),
            supplier_address_line2=data.get("supplierAddressLine2"),
            supplier_city=data.get("supplierCity"),
            supplier_state=data.get("supplierState"),
            supplier_postal_code=data.get("supplierPostalCode"),
            supplier_country=data.get("supplierCountry"),
            items=items,
            po_number=data.get("poNumber"),
            created_by=data.get("createdBy"),
            buyer_department=data.get("buyerDepartment"),
            notes=data.get("notes"),
            tax_rate=data.get("taxRate", 0.0),
            sub_total=data.get("subTotal", 0.0),
            tax=data.get("tax", 0.0),
            grand_total=data.get("grandTotal", 0.0),
            is_approved=data.get("isApproved", False),
            approval_reason=data.get("approvalReason")
        )

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), indent=2)

    @classmethod
    def from_json(cls, json_str: str) -> 'PurchaseOrder':
        data = json.loads(json_str)
        return cls.from_dict(data)


@dataclass
class PurchaseOrderApproval:
    """Purchase order approval decision including PO number, approval status, and reason"""
    po_number: Optional[str] = None
    is_approved: bool = False
    approval_reason: Optional[str] = None

    def to_dict(self) -> dict:
        return {
            "poNumber": self.po_number,
            "isApproved": self.is_approved,
            "approvalReason": self.approval_reason
        }

    @classmethod
    def from_dict(cls, data: dict) -> 'PurchaseOrderApproval':
        return cls(
            po_number=data.get("poNumber"),
            is_approved=data.get("isApproved", False),
            approval_reason=data.get("approvalReason")
        )

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), indent=2)

    @classmethod
    def from_json(cls, json_str: str) -> 'PurchaseOrderApproval':
        data = json.loads(json_str)
        return cls.from_dict(data)