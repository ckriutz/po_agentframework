using Microsoft.Agents.AI;
using Microsoft.Extensions.AI;
using Microsoft.Agents.AI.A2A;
using A2A;
using System.Text.Json;

// Create a cancellation token source for the entire program
var cts = new CancellationTokenSource();
var token = cts.Token;

// First things first, lets see if can cwn get the agent, and it's card.
var url = new Uri("http://localhost:5000");
var httpClient = new HttpClient
{
    Timeout = TimeSpan.FromMinutes(2)
};

var agentCardResolver = new A2ACardResolver(url, httpClient);

var agentCard = await agentCardResolver.GetAgentCardAsync(token);
Console.WriteLine($"Agent Name: {agentCard.Name}");
Console.WriteLine($"Agent Description: {agentCard.Description}");

var agent = await agentCardResolver.GetAIAgentAsync();
Console.WriteLine($"Agent Type: {agent.GetType().Name}");

var purchaseOrder = await MessageIntakeAgent();
if (purchaseOrder != null)
{
    Console.WriteLine("Extracted Purchase Order:");
    Console.WriteLine($"Order ID: {purchaseOrder.PoNumber}");
    Console.WriteLine($"Order Date: {purchaseOrder.BuyerDepartment}");
    Console.WriteLine($"Supplier: {purchaseOrder.SupplierName}");
    Console.WriteLine($"Total Amount: {purchaseOrder.SubTotal}");
    Console.WriteLine($"Total Amount: {purchaseOrder.Tax}");
    Console.WriteLine($"Total Amount: {purchaseOrder.GrandTotal}");
}
else
{
    Console.WriteLine("No purchase order extracted.");
}

async Task<PurchaseOrder> MessageIntakeAgent()
{
    Console.WriteLine("Invoking Message Intake Agent...");
    AgentThread thread = agent.GetNewThread();

    ChatMessage message = new ChatMessage();
    message.Role = ChatRole.User;
    //message.Contents.Add(new TextContent("Here is a purchase order image. Please extract the relevant information in JSON format."));

    // Use a local purchase order image - read as byte array and use DataContent
    string imagePath = Path.Combine(Directory.GetCurrentDirectory(), "PurchaseOrders", "AdventureWorksPO_MKTOrder.png");
    byte[] imageBytes = File.ReadAllBytes(imagePath);
    //message.Contents.Add(new DataContent(imageBytes, "image/png"));

    string base64Image = Convert.ToBase64String(imageBytes);
    message.Contents.Add(new TextContent($"Here is a purchase order image. data:image/png;base64,{base64Image}"));

    var response = await agent.RunAsync(message, thread);
    Console.WriteLine("Raw Response:");
    Console.WriteLine(response.ToString());

    // Deserialize the response into the PurchaseOrder class
    var purchaseOrder = response.Deserialize<PurchaseOrder>(JsonSerializerOptions.Web);

    if (purchaseOrder == null)
    {
        Console.WriteLine("Failed to parse purchase order from response.");
        return null;
    }
    return purchaseOrder;
}