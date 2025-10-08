using A2A;
using Microsoft.Agents.AI;
using Azure.AI.OpenAI;
using Azure.Identity;
using Microsoft.Extensions.AI;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Configuration;

public class IntakeAgent
{
    private readonly ILogger _logger;
    private readonly AIAgent _agent;
    private ITaskManager? _taskManager;

    public IntakeAgent(ILogger logger)
    {
        _logger = logger;

        // Initialize the agent
        _agent = InitializeAgent();
    }

    private AIAgent InitializeAgent()
    {
        var deploymentName = Environment.GetEnvironmentVariable("DEPLOYMENT_NAME") ?? throw new ArgumentException("DEPLOYMENT_NAME must be provided");
        var endpoint = Environment.GetEnvironmentVariable("ENDPOINT") ?? throw new ArgumentException("ENDPOINT must be provided");
        var apiKey = Environment.GetEnvironmentVariable("API_KEY") ?? throw new ArgumentException("API_KEY must be provided");

        _logger.LogInformation("Initializing AgentFramework agent with model {deploymentName}", deploymentName);

        var client = new AzureOpenAIClient(new Uri(endpoint), new System.ClientModel.ApiKeyCredential(apiKey))
            .GetChatClient(deploymentName)
            .AsIChatClient();

        var agentName = "IntakeAgent";
        var agentInstructions =
         """
            You are a specialized document processor, and your task is to read the image of a purchase order (PO) and extract the relevant information in a structured format.
            
            Analyze this purchase order image and extract ALL the following details:
            
            Supplier Information:
            - supplierName
            - supplierAddressLine1, supplierAddressLine2
            - supplierCity, supplierState, supplierPostalCode, supplierCountry
            
            Line Items (extract each item from the table):
            - itemCode, description, quantity, unitPrice, lineTotal
            
            Purchase Order Metadata:
            - poNumber, createdBy, buyerDepartment
            
            Additional Information:
            - notes, taxRate
            
            Computed Totals:
            - subTotal, tax, grandTotal
            
            Approval Information:
            - isApproved, approvalReason
        """;

        ChatClientAgentOptions agentOptions = new(name: agentName, instructions: agentInstructions)
        {
            ChatOptions = new()
            {
                ResponseFormat = ChatResponseFormat.Json
            }
        };

        var agent = client.CreateAIAgent(agentOptions);

        _logger.LogInformation("Purchase Order Intake Agent initialized successfully.");
        return agent;
    }

    public void Attach(ITaskManager taskManager)
    {
        _taskManager = taskManager;
        _taskManager.OnAgentCardQuery = GetAgentCardAsync;
        _taskManager.OnMessageReceived = ProcessMessageAsync;
    }

    private Task<AgentCard> GetAgentCardAsync(string agentUrl, CancellationToken cancellationToken)
    {
        if (cancellationToken.IsCancellationRequested)
        {
            return Task.FromCanceled<AgentCard>(cancellationToken);
        }

        var capabilities = new AgentCapabilities()
        {
            Streaming = true,
            PushNotifications = false,
        };

        var orderAgentSkill = new AgentSkill()
        {
            Id = "purchase-order-intake-agent",
            Name = "PurchaseOrderIntakeAgent",
            Description = "A skill that processes Purchase Order Images and extracts key details.",
            Tags = ["purchase-order", "image-processing", "data-extraction"],
            Examples =
            [
                "Extract key details from this purchase order image.",
                "Analyze the attached PO image and return the data as JSON.",
                "Process this purchase order image and provide the following: PO number, total amount, supplier name, and buyer department.",
                "Verify this purchase order image is a valid PO and extract the relevant information.",
                "Scan the provided PO image and return the extracted details in JSON format."
            ],
        };

        return Task.FromResult(new AgentCard()
        {
            Name = "The Purchase Order Intake Agent",
            Description = "An agent that manages purchase order images.",
            Url = agentUrl,
            Version = "1.0.0",
            DefaultInputModes = ["image/png"],
            DefaultOutputModes = ["text"],
            Capabilities = capabilities,
            Skills = [orderAgentSkill],
        });
    }

    private async Task<A2AResponse> ProcessMessageAsync(MessageSendParams messageSendParams, CancellationToken cancellationToken)
    {
        _logger.LogInformation("Processing message.");
        if (cancellationToken.IsCancellationRequested)
        {
            return new AgentMessage
            {
                Role = MessageRole.Agent,
                MessageId = Guid.NewGuid().ToString(),
                ContextId = messageSendParams.Message.ContextId,
                Parts = [new TextPart { Text = "Request was cancelled." }]
            };
        }

        var textPart = messageSendParams.Message.Parts.OfType<TextPart>().FirstOrDefault();
        if (textPart == null || string.IsNullOrEmpty(textPart.Text))
        {
            _logger.LogWarning("No valid TextPart found in message");
            return new AgentMessage
            {
                Role = MessageRole.Agent,
                MessageId = Guid.NewGuid().ToString(),
                ContextId = messageSendParams.Message.ContextId,
                Parts = [new TextPart { Text = "Error: No text content found in the message." }]
            };
        }

        // Process image from TextPart
        var imageContent = ProcessImageFromTextPart(textPart);
        if (imageContent == null)
        {
            return new AgentMessage
            {
                Role = MessageRole.Agent,
                MessageId = Guid.NewGuid().ToString(),
                ContextId = messageSendParams.Message.ContextId,
                Parts = [new TextPart { Text = "Error: Could not process the image data from the message." }]
            };
        }

        var prompt = "Please analyze this purchase order image and extract the key details in JSON format.";
        var message = new ChatMessage
        {
            Role = ChatRole.User,
            Contents = new List<AIContent> { new TextContent(prompt), imageContent }
        };

        _logger.LogInformation("Calling AI agent for image analysis");
        var response = await _agent.RunAsync(message, cancellationToken: cancellationToken);
        _logger.LogInformation("AI agent call completed");

        var artifact = new Artifact();
        artifact.Parts.Add(new TextPart { Text = response.Text ?? "" });

        var agentMessage = new AgentMessage
        {
            Role = MessageRole.Agent,
            MessageId = Guid.NewGuid().ToString(),
            ContextId = messageSendParams.Message.ContextId,
            Parts = artifact.Parts
        };

        return agentMessage;
    }

    /// <summary>
    /// Processes a TextPart that might contain image data as base64 and returns an AIContent object
    /// </summary>
    /// <param name="textPart">The TextPart containing the text with potential image data</param>
    /// <returns>AIContent object ready for AI processing, or null if processing failed</returns>
    private AIContent? ProcessImageFromTextPart(TextPart textPart)
    {
        try
        {
            if (string.IsNullOrEmpty(textPart.Text))
            {
                _logger.LogWarning("TextPart is empty");
                return null;
            }

            // Look for data URL pattern: data:image/png;base64,<base64data>
            var dataUrlPattern = @"data:image/[^;]+;base64,([A-Za-z0-9+/=]+)";
            var match = System.Text.RegularExpressions.Regex.Match(textPart.Text, dataUrlPattern);
            if (match.Success)
            {
                var base64String = match.Groups[1].Value;
                var imageBytes = Convert.FromBase64String(base64String);
                var binaryData = BinaryData.FromBytes(imageBytes);

                _logger.LogInformation("Successfully extracted image from TextPart: {Size} bytes", imageBytes.Length);

                return new DataContent(binaryData, "image/png");
            }

            _logger.LogWarning("TextPart does not contain valid base64 image data");
            return null;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to process image from TextPart");
            return null;
        }
    }
}