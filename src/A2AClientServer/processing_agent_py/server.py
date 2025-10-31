from a2a.server.apps import A2AStarletteApplication  
from a2a.server.request_handlers import DefaultRequestHandler  
from a2a.server.tasks import InMemoryTaskStore
from a2a.types import AgentCapabilities, AgentCard, AgentSkill

from agents import create_processing_agent_executor  


if __name__ == '__main__':  
    # 2. Define the agent card  
    agent_card = AgentCard(  
        name='Processing Agent',  
        description='Just a processing agent',  
        url='http://localhost:9998/',  
        version='1.0.0',  
        defaultInputModes=['text'],  
        defaultOutputModes=['text'],  
        capabilities=AgentCapabilities(streaming=True),  
        skills=[],  
    )  

    # 3. Define the request handler  
    request_handler = DefaultRequestHandler(  
        agent_executor=create_processing_agent_executor(),
        task_store=InMemoryTaskStore(),
    )  

    # 4. Build the server  
    server = A2AStarletteApplication(  
        agent_card=agent_card, http_handler=request_handler  
    )  

    # 5. Start the server with Uvicorn  
    import uvicorn  
    uvicorn.run(server.build(), host='0.0.0.0', port=9998)  