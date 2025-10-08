# Use the official .NET 9 SDK image to build the app
FROM mcr.microsoft.com/dotnet/sdk:9.0-preview AS build
WORKDIR /src

# Copy the project file and restore dependencies
COPY IntakeAgentA2A.csproj .
RUN dotnet restore IntakeAgentA2A.csproj

# Copy the rest of the source code
COPY . .

# Build and publish the app
RUN dotnet publish IntakeAgentA2A.csproj -c Release -o /app/publish

# Use the official .NET 9 ASP.NET runtime image
FROM mcr.microsoft.com/dotnet/aspnet:9.0-preview AS runtime
WORKDIR /app

# Copy the published app from the build stage
COPY --from=build /app/publish .

# Set environment variables
ENV ASPNETCORE_URLS=http://+:8080

# Expose the port the app runs on
EXPOSE 8080

# Set the entry point
ENTRYPOINT ["dotnet", "IntakeAgentA2A.dll"]