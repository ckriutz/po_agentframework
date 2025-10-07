using System;
using System.ClientModel;
using System.ComponentModel;
using Azure.AI.OpenAI;
using Azure.Identity;
using Microsoft.Agents.AI;
using Microsoft.Extensions.AI;
using OpenAI;

// In the recent past, creating an Agent, was done by creating a class
// for the entire agent, and passing in a Kernel.
// With the new Agent Framework, you can create an agent a lot easier.
// So I'm going to create all of them here.

public static class Agents
{
    // Use environment variables if available, otherwise fall back to hardcoded values for development
    public static string endpoint = Environment.GetEnvironmentVariable("ENDPOINT");
    public static string apiKey = Environment.GetEnvironmentVariable("API_KEY");
    public static string deploymentName = Environment.GetEnvironmentVariable("DEPLOYMENT_NAME");

    [Description("Function to write Purchase Order data to a CSV file.")]
    public static string WriteToCSVFile([Description("Function to write Purchase Order data to a CSV file.")] PurchaseOrder purchaseOrder)
    {

        // Define the CSV file path
        string filePath = "purchase_orders.csv";

        // Create the CSV header if the file does not exist
        if (!System.IO.File.Exists(filePath))
        {
            var header = "PONumber,Subtotal,Tax,GrandTotal,SupplierName,BuyerDepartment,Notes";
            System.IO.File.WriteAllText(filePath, header + Environment.NewLine);
        }

        // Append the purchase order data to the CSV file
        var csvLine = $"{purchaseOrder.PoNumber},{purchaseOrder.SubTotal},{purchaseOrder.Tax},{purchaseOrder.GrandTotal},{purchaseOrder.SupplierName},{purchaseOrder.BuyerDepartment},{purchaseOrder.Notes}";
        System.IO.File.AppendAllText(filePath, csvLine + Environment.NewLine);

        return $"Purchase order {purchaseOrder.PoNumber} written to CSV file.";
    }

    public static AIAgent CreateIntakeAgent()
    {
        var AgentName = "IntakeAgent";
        var AgentInstructions =
         """
            You are a specialized document processor, and your task is to read the image of a purchase order (PO) and extract the relevant information in a structured format.
            Analyze this purchase order image and extract the key details: PO Number, Subtotal, Tax, Grand Total, Supplier Name, Buyer Department, and Notes.
            """;

        // Try using structured output with vision - this should work with newer API versions
        ChatClientAgentOptions agentOptions = new(name: AgentName, instructions: AgentInstructions)
        {
            ChatOptions = new()
            {
                ResponseFormat = ChatResponseFormat.ForJsonSchema<PurchaseOrder>()
            }
        };

        AIAgent agent = new AzureOpenAIClient(new Uri(endpoint), new ApiKeyCredential(apiKey))
            .GetChatClient(deploymentName)
            .CreateAIAgent(agentOptions);

        return agent;
    }

    public static AIAgent CreateProcessingAgent()
    {
        var AgentName = "ProcessingAgent";
        var AgentInstructions =
         """
            You are a specialized document processor, and your task is to read the JSON data of a purchase order (PO) and determine if it can be approved.
            Analyze this purchase order data and check the following:
            1. The Grand Total must be less than $1000.
            2. The Supplier Name must not be empty.
            3. The Buyer Department must be one of the following: "Travel", "Marketing", "IT", "HR".
            Evaluate the rules and provide a clear reason in the approvalReason field.
        """;

        // Create the agent options, specifying the response format to use a JSON schema based on the PurchaseOrderApproval class.
        ChatClientAgentOptions agentOptions = new(name: AgentName, instructions: AgentInstructions)
        {
            ChatOptions = new()
            {
                ResponseFormat = ChatResponseFormat.ForJsonSchema<PurchaseOrderApproval>()
            }
        };

        AIAgent agent = new AzureOpenAIClient(new Uri(endpoint), new ApiKeyCredential(apiKey))
            .GetChatClient(deploymentName)
            .CreateAIAgent(agentOptions);

        return agent;
    }

    public static AIAgent CreateDataAgent()
    {
        var AgentName = "DataAgent";
        var AgentInstructions =
         """
            You are a specialized document processor, and your task is to read the JSON data of a purchase order (PO) and determine if it was approved or not.
            If the document was approved, write the purchase order data to a CSV file using the provided function.
            If it was not approved, do not write to the CSV file, and return a message indicating the PO was not approved, and the reason why.
        """;

        // Create the agent options, specifying the response format to use a JSON schema based on the SQLInsertStatement class.
        ChatClientAgentOptions agentOptions = new(name: AgentName, instructions: AgentInstructions, tools: [AIFunctionFactory.Create(WriteToCSVFile)]) { };

        AIAgent agent = new AzureOpenAIClient(new Uri(endpoint), new ApiKeyCredential(apiKey))
            .GetChatClient(deploymentName)
            .CreateAIAgent(agentOptions);

        return agent;
    }
}