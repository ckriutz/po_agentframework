using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text.Json.Serialization;

[Description("Purchase order information including PO number, amounts, supplier, and buyer details")]
public class PurchaseOrder
{
    // Supplier fields.
    [JsonPropertyName("supplierName")]
    public string? SupplierName { get; set; }
    
    [JsonPropertyName("supplierAddressLine1")]
    public string? SupplierAddressLine1 { get; set; }
    
    [JsonPropertyName("supplierAddressLine2")]
    public string? SupplierAddressLine2 { get; set; }
    
    [JsonPropertyName("supplierCity")]
    public string? SupplierCity { get; set; }
    
    [JsonPropertyName("supplierState")]
    public string? SupplierState { get; set; }
    
    [JsonPropertyName("supplierPostalCode")]
    public string? SupplierPostalCode { get; set; }
    
    [JsonPropertyName("supplierCountry")]
    public string? SupplierCountry { get; set; }

    // Line items table
    [JsonPropertyName("items")]
    public List<PurchaseOrderItem> Items { get; set; } = new List<PurchaseOrderItem>();

    // Purchase order metadata
    [JsonPropertyName("poNumber")]
    public string? PoNumber { get; set; }
    
    [JsonPropertyName("createdBy")]
    public string? CreatedBy { get; set; }
    
    [JsonPropertyName("buyerDepartment")]
    public string? BuyerDepartment { get; set; }

    // Notes block
    [JsonPropertyName("notes")]
    public string? Notes { get; set; }

    // Tax information
    [JsonPropertyName("taxRate")]
    public decimal TaxRate { get; set; }

    // Computed totals
    [JsonPropertyName("subTotal")]
    public decimal SubTotal { get; set; }
    
    [JsonPropertyName("tax")]
    public decimal Tax { get; set; }
    
    [JsonPropertyName("grandTotal")]
    public decimal GrandTotal { get; set; }

    // Approval flow
    [JsonPropertyName("isApproved")]
    public bool IsApproved { get; set; }
    
    [JsonPropertyName("approvalReason")]
    public string? ApprovalReason { get; set; }
}

[Description("Purchase order approval decision including PO number, approval status, and reason")]
public class PurchaseOrderApproval
{
    [JsonPropertyName("poNumber")]
    public string? PoNumber { get; set; }

    [JsonPropertyName("isApproved")]
    public bool IsApproved { get; set; }

    [JsonPropertyName("approvalReason")]
    public string? ApprovalReason { get; set; }
}

[Description("Individual line item in a purchase order")]
public class PurchaseOrderItem
{
    // Table columns: Item Code, Description, Quantity, Unit Price, Line Total
    [JsonPropertyName("itemCode")]
    public string? ItemCode { get; set; }
    
    [JsonPropertyName("description")]
    public string? Description { get; set; }
    
    [JsonPropertyName("quantity")]
    public int Quantity { get; set; }
    
    [JsonPropertyName("unitPrice")]
    public decimal UnitPrice { get; set; }
    
    [JsonPropertyName("lineTotal")]
    public decimal LineTotal { get; set; }
}