using System;
using System.ClientModel;
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
    public static string endpoint = "";
    public static string apiKey = "";
    public static string deploymentName = "";

    public static AIAgent CreateIntakeAgent()
    {
        var AgentName = "IntakeAgent";
        var AgentInstructions =
         """
            You are a specialized document processor, and your task is to read the image of a purchase order (PO) and extract the relevant information in a structured format.
            Analyze this purchase order image and extract the key details: PO Number, Subtotal, Tax, Grand Total, Supplier Name, Buyer Department, and Notes.
            """;

        // Create the agent options, specifying the response format to use a JSON schema based on the PurchaseOrder class.
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
}