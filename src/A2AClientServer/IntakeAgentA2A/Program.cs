using A2A;
using A2A.AspNetCore;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.AspNetCore.Http;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddLogging();
builder.Services.AddHttpClient();

var app = builder.Build();
var httpClient = app.Services.GetRequiredService<HttpClient>();
var taskManager = new TaskManager();
var logger = app.Logger;

var poIntakeAgent = new IntakeAgent(logger);
poIntakeAgent.Attach(taskManager);
app.MapA2A(taskManager, "/");
app.MapWellKnownAgentCard(taskManager, "/");
app.MapHttpA2A(taskManager, "/");

app.MapGet("/health", () => Results.Ok(new { status = "healthy" }));

await app.RunAsync();
