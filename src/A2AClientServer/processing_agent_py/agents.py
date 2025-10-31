import os
import json
from typing import Optional, Any
from a2a.server.agent_execution import AgentExecutor, RequestContext
from a2a.server.events.event_queue import EventQueue
from a2a.utils import new_agent_text_message
from openai import AzureOpenAI

# Load environment variables from .env file
try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    # dotenv not installed, environment variables should be set manually
    pass

class ProcessingAgent: 
    def __init__(self, endpoint: Optional[str] = None, api_key: Optional[str] = None, deployment_name: Optional[str] = None, api_version: Optional[str] = None):
        self.endpoint = endpoint or os.getenv("ENDPOINT")
        self.api_key = api_key or os.getenv("API_KEY")
        self.api_version = api_version or os.getenv("API_VERSION")
        self.deployment_name = deployment_name or os.getenv("DEPLOYMENT_NAME")

        if not all([self.endpoint, self.api_key, self.deployment_name]):
            raise ValueError("Missing required configuration: ENDPOINT, API_KEY, and DEPLOYMENT_NAME must be provided")

        # Initialize OpenAI client
        self.client = AzureOpenAI(
            azure_endpoint=self.endpoint,
            api_key=self.api_key,
            api_version=self.api_version
        )
        
        self.agent_instructions = """
        You are a specialized document processor that evaluates purchase orders for approval.

        Analyze the purchase order data and check these business rules:
        1. The Grand Total must be less than $1000
        2. The Supplier Name must not be empty
        3. The Buyer Department must be one of: "Travel", "Marketing", "IT", "HR"

        Respond with JSON containing:
        {
            "poNumber": "string",
            "isApproved": boolean,
            "approvalReason": "string explaining the decision"
        }
        """

    async def invoke(self, message: str = "") -> str:
        """Process a message and return a response"""
        try:
            response = self.client.chat.completions.create(
                model=self.deployment_name,
                messages=[
                    {"role": "system", "content": self.agent_instructions},
                    {"role": "user", "content": message or "Hello, please introduce yourself."}
                ]
            )
            return response.choices[0].message.content
        except Exception as e:
            return f"Error processing request: {str(e)}"
    

class ProcessingAgentExecutor(AgentExecutor):
    def __init__(self):
        self.agent = ProcessingAgent()
        self._cancelled = False

    async def execute(self, context: RequestContext, event_queue: EventQueue) -> None:
        """
        Execute the agent processing.
        
        Note: Returns None because results are communicated through the event_queue.
        This is the standard pattern for the A2A framework where:
        - Results are streamed through events
        - Multiple responses can be sent
        - Long-running operations are supported
        """
        try:
            if self._cancelled:
                return
            
            # Get the input message from context
            input_message = ""
            if hasattr(context, 'content') and context.content:
                input_message = str(context.content)
            elif hasattr(context, 'message') and context.message:
                input_message = str(context.message)
            
            # Process the message
            result = await self.agent.invoke(input_message)
            
            if not self._cancelled:
                # Send result back through event queue using A2A message format
                message = new_agent_text_message(
                    text=result,
                    context_id=getattr(context, 'context_id', None),
                    task_id=getattr(context, 'task_id', None)
                )
                await event_queue.enqueue_event(message)
            
        except Exception as e:
            if not self._cancelled:
                # Send error response using A2A message format
                error_message = new_agent_text_message(
                    text=f"Processing failed: {str(e)}",
                    context_id=getattr(context, 'context_id', None),
                    task_id=getattr(context, 'task_id', None)
                )
                await event_queue.enqueue_event(error_message)

    async def cancel(self) -> None:
        """Cancel the execution"""
        self._cancelled = True

def create_processing_agent_executor() -> ProcessingAgentExecutor:
    """Factory function to create a ProcessingAgentExecutor instance"""
    return ProcessingAgentExecutor()