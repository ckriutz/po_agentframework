using System;
using System.ClientModel;
using System.ComponentModel;
using System.Text.Json;
using System.Text.Json.Serialization;
using Azure.AI.OpenAI;
using Azure.Identity;
using Microsoft.Agents.AI;
using Microsoft.Extensions.AI;
using OpenAI;


var intakeAgent = Agents.CreateIntakeAgent();



ChatMessage message = new ChatMessage();
message.Role = ChatRole.User;
message.Contents.Add(new TextContent("Here is a purchase order image. Please extract the relevant information in JSON format."));

// Use a local purchase order image - read as byte array and use DataContent
string imagePath = Path.Combine(Directory.GetCurrentDirectory(), "PurchaseOrders", "AdventureWorksPO_HROrder.png");
byte[] imageBytes = File.ReadAllBytes(imagePath);
message.Contents.Add(new DataContent(imageBytes, "image/png"));

var intakeThread = intakeAgent.GetNewThread();

// Run the agent and get structured output
var response = await intakeAgent.RunAsync(message, intakeThread);

// Deserialize the response into the PurchaseOrder class
var purchaseOrder = response.Deserialize<PurchaseOrder>(JsonSerializerOptions.Web);

Console.WriteLine("Purchase Order Information:");
Console.WriteLine($"PO Number: {purchaseOrder.PoNumber}");
Console.WriteLine($"Subtotal: {purchaseOrder.SubTotal}");
Console.WriteLine($"Tax: {purchaseOrder.Tax}");
Console.WriteLine($"Grand Total: {purchaseOrder.GrandTotal}");
Console.WriteLine($"Supplier Name: {purchaseOrder.SupplierName}");
Console.WriteLine($"Buyer Department: {purchaseOrder.BuyerDepartment}");
Console.WriteLine($"Notes: {purchaseOrder.Notes}");

var processingAgent = Agents.CreateProcessingAgent();
var processingThread = processingAgent.GetNewThread();
var processingMessage = new ChatMessage();
processingMessage.Contents.Add(new TextContent($"Here is the purchase order data in JSON format: {JsonSerializer.Serialize(purchaseOrder, JsonSerializerOptions.Web)}. Please determine if it can be approved based on the criteria provided."));
var processingResponse = await processingAgent.RunAsync(processingMessage, processingThread);

Console.WriteLine("\nProcessing Agent Response:");
Console.WriteLine(processingResponse.ToString());

// Deserialize the approval response
var approval = processingResponse.Deserialize<PurchaseOrderApproval>(JsonSerializerOptions.Web);

// Update the purchase order with approval information
purchaseOrder.IsApproved = approval.IsApproved;
purchaseOrder.ApprovalReason = approval.ApprovalReason;

Console.WriteLine("\nFinal Purchase Order with Approval:");
Console.WriteLine($"PO Number: {purchaseOrder.PoNumber}");
Console.WriteLine($"Grand Total: {purchaseOrder.GrandTotal}");
Console.WriteLine($"Supplier: {purchaseOrder.SupplierName}");
Console.WriteLine($"Department: {purchaseOrder.BuyerDepartment}");
Console.WriteLine($"Is Approved: {purchaseOrder.IsApproved}");
Console.WriteLine($"Approval Reason: {purchaseOrder.ApprovalReason}");