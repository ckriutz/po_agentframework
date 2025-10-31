using Microsoft.Agents.AI;
using Microsoft.Extensions.AI;
using Microsoft.Agents.AI.A2A;
using A2A;
using System.Text.Json;
using System.Formats.Tar;

// Create a cancellation token source for the entire program
var cts = new CancellationTokenSource();
var token = cts.Token;

// First things first, lets see if can cwn get the agent, and it's card.
var IntakeAgentUrl = new Uri("http://localhost:5000");
var processingAgentUrl = new Uri("http://localhost:9998");
var dataAgentUrl = new Uri("http://localhost:8080");
var httpClient = new HttpClient
{
    Timeout = TimeSpan.FromMinutes(2)
};

// First lets set up the intake agent.
var intakeAgentCardResolver = new A2ACardResolver(IntakeAgentUrl, httpClient);

var intakeAgentCard = await intakeAgentCardResolver.GetAgentCardAsync(token);
Console.WriteLine($"Intake Agent Name: {intakeAgentCard.Name}");
Console.WriteLine($"Intake Agent Description: {intakeAgentCard.Description}");

var intakeAgent = await intakeAgentCardResolver.GetAIAgentAsync();
Console.WriteLine($"Intake Agent Type: {intakeAgent.GetType().Name}");

// Now let's set up the processing agent.
var processingAgentCardResolver = new A2ACardResolver(processingAgentUrl, httpClient);
var processingAgentCard = await processingAgentCardResolver.GetAgentCardAsync(token);
Console.WriteLine($"Processing Agent Name: {processingAgentCard.Name}");
Console.WriteLine($"Processing Agent Description: {processingAgentCard.Description}");

var processingAgent = await processingAgentCardResolver.GetAIAgentAsync();
Console.WriteLine($"Processing Agent Type: {processingAgent.GetType().Name}");

//var dataAgentCardResolver = new A2ACardResolver(dataAgentUrl, httpClient);
//var dataAgentCard = await dataAgentCardResolver.GetAgentCardAsync(token);
//Console.WriteLine($"Data Agent Name: {dataAgentCard.Name}");
//Console.WriteLine($"Data Agent Description: {dataAgentCard.Description}");

//var dataAgent = await dataAgentCardResolver.GetAIAgentAsync();
//Console.WriteLine($"Data Agent Type: {dataAgent.GetType().Name}");

// Now to start the first agent.
var purchaseOrder = await MessageIntakeAgent(intakeAgent);
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

await MessageProcessingAgent(processingAgent, purchaseOrder);

async Task<PurchaseOrder> MessageIntakeAgent(AIAgent agent)
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

async Task MessageProcessingAgent(AIAgent agent, PurchaseOrder po)
{
    Console.WriteLine("Invoking Message Processing Agent...");
    AgentThread thread = agent.GetNewThread();

    ChatMessage message = new ChatMessage();
    message.Role = ChatRole.User;
    string poJson = JsonSerializer.Serialize(po, JsonSerializerOptions.Web);
    message.Contents.Add(new TextContent($"Here is a purchase order in JSON format: {poJson}. Please process it accordingly."));

    var response = await agent.RunAsync(message);
    Console.WriteLine("Processing Agent Response:");
    Console.WriteLine(response.ToString());
}