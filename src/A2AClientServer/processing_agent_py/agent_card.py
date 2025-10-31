from a2a.types import AgentCard, AgentCapabilities  

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