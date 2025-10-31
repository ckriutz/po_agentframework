// See https://aka.ms/new-console-template for more information
using System.Text.Json;
using Microsoft.Agents.AI;
using Microsoft.Agents.AI.Workflows;
using Microsoft.Extensions.AI;

// First lets create the agents.
AIAgent intakeAgent = Agents.CreateIntakeAgent();
AIAgent processingAgent = Agents.CreateProcessingAgent();
AIAgent dataAgent = Agents.CreateDataAgent();


ChatMessage message = new ChatMessage();
message.Role = ChatRole.User;
message.Contents.Add(new TextContent("Here is a purchase order image. Please extract the relevant information in JSON format."));

Workflow workflow = AgentWorkflowBuilder.BuildSequential(intakeAgent, processingAgent, dataAgent);

// Use a local purchase order image - read as byte array and use DataContent
string imagePath = Path.Combine(Directory.GetCurrentDirectory(), "PurchaseOrders", "AdventureWorksPO_MKTOrder.png");
byte[] imageBytes = File.ReadAllBytes(imagePath);
message.Contents.Add(new DataContent(imageBytes, "image/png"));

 // Execute the workflow
StreamingRun run = await InProcessExecution.StreamAsync(workflow, message);
await run.TrySendMessageAsync(new TurnToken(emitEvents: true));


//Must send the turn token to trigger the agents.
//The agents are wrapped as executors. When they receive messages,
//they will cache the messages and only start processing when they receive a TurnToken.
string? lastExecutorId = null;
await foreach (WorkflowEvent evt in run.WatchStreamAsync().ConfigureAwait(false))
{
    // All this down here is more-or-less to show the ouutput of the workflow in the
    if (evt is AgentRunUpdateEvent e)
    {
        if (e.ExecutorId != lastExecutorId)
        {
            lastExecutorId = e.ExecutorId;
            Console.WriteLine();
            Console.WriteLine(e.ExecutorId);
        }

        Console.Write(e.Update.Text);
        if (e.Update.Contents.OfType<FunctionCallContent>().FirstOrDefault() is FunctionCallContent call)
        {
            Console.WriteLine();
            Console.WriteLine($"  [Calling function '{call.Name}' with arguments: {JsonSerializer.Serialize(call.Arguments)}]");
        }
    }
    if (evt is WorkflowOutputEvent output)
    {
        Console.WriteLine();
        Console.WriteLine("Workflow completed with output:");
        Console.WriteLine(output.As<List<ChatMessage>>()!);
        foreach (ChatMessage msg in output.As<List<ChatMessage>>()!)
        {
            Console.WriteLine($"**Message from {msg.Role}**");
            if (msg.Contents != null)
            {
                Console.Write("Contents:");
                foreach (var content in msg.Contents)
                {
                    if (content is TextContent text)
                    {
                        Console.WriteLine($"  {text.Text}");
                    }
                    else if (content is DataContent data)
                    {
                        Console.WriteLine($"  [Data content with {data.Data.Length} bytes]");
                    }
                    else if (content is FunctionCallContent functionCall)
                    {
                        Console.WriteLine($"  [Function call to '{functionCall.Name}' with arguments: {JsonSerializer.Serialize(functionCall.Arguments)}]");
                    }
                }
            }
        }
    }
}