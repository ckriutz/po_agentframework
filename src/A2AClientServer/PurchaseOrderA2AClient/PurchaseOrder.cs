using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text.Json.Serialization;
using System.Text.Json;
using System.Globalization;

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
    [JsonPropertyName("lineItems")]
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
    [JsonConverter(typeof(PercentageStringToDecimalConverter))]
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

public class PercentageStringToDecimalConverter : JsonConverter<decimal>
{
    public override decimal Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String)
        {
            string value = reader.GetString()!;
            // Handle percentage strings like "7%"
            if (value.EndsWith("%"))
            {
                string numericPart = value.TrimEnd('%');
                if (decimal.TryParse(numericPart, NumberStyles.Number, CultureInfo.InvariantCulture, out decimal result))
                {
                    return result / 100; // Convert percentage to decimal (7% -> 0.07)
                }
            }
            // Handle regular numeric strings
            if (decimal.TryParse(value, NumberStyles.Number, CultureInfo.InvariantCulture, out decimal directResult))
            {
                return directResult;
            }
            throw new JsonException($"Unable to convert \"{value}\" to decimal.");
        }
        else if (reader.TokenType == JsonTokenType.Number)
        {
            return reader.GetDecimal();
        }
        
        throw new JsonException($"Unexpected token type: {reader.TokenType}");
    }

    public override void Write(Utf8JsonWriter writer, decimal value, JsonSerializerOptions options)
    {
        writer.WriteNumberValue(value);
    }
}