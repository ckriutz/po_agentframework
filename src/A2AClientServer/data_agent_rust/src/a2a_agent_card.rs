use serde::{Deserialize, Serialize};

/// A2A Protocol compliant AgentCard structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2AAgentCard {
    /// Human-readable name for the Agent
    pub name: String,
    /// Human-readable description of the Agent's function
    pub description: String,
    /// URL address where the Agent is hosted
    pub url: String,
    /// Service provider information for the Agent
    pub provider: Option<ProviderInfo>,
    /// Version of the Agent
    pub version: String,
    /// URL for the Agent's documentation
    pub documentation_url: Option<String>,
    /// Optional capabilities supported by the Agent
    pub capabilities: Capabilities,
    /// Authentication requirements for the Agent
    pub authentication: Authentication,
    /// Default interaction modes supported by the Agent across all skills
    pub default_input_modes: Vec<String>,
    /// Default interaction modes supported by the Agent across all skills
    pub default_output_modes: Vec<String>,
    /// Collection of capability units the Agent can perform
    pub skills: Vec<Skill>,
}

/// Provider information for the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub organization: String,
    pub url: String,
}

/// Capabilities supported by the Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// If the Agent supports Server-Sent Events
    pub streaming: Option<bool>,
    /// If the Agent can push update notifications to the client
    pub push_notifications: Option<bool>,
    /// If the Agent exposes task state change history
    pub state_transition_history: Option<bool>,
}

/// Authentication requirements for the Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authentication {
    /// Authentication schemes (e.g., Basic, Bearer)
    pub schemes: Vec<String>,
    /// Credentials for the client to use for private Cards
    pub credentials: Option<String>,
}

/// Individual skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    /// Unique identifier for the skill
    pub id: String,
    /// Human-readable name for the skill
    pub name: String,
    /// Skill description
    pub description: String,
    /// Tags describing the skill's capability category
    pub tags: Vec<String>,
    /// Example scenarios or prompts the skill can execute
    pub examples: Option<Vec<String>>,
    /// Input MIME types supported by the skill (if different from default)
    pub input_modes: Option<Vec<String>>,
    /// Output MIME types supported by the skill (if different from default)
    pub output_modes: Option<Vec<String>>,
}

impl A2AAgentCard {
    /// Create a new A2A compliant AgentCard for the Purchase Order Processing Agent
    pub fn new_purchase_order_agent(base_url: &str) -> Self {
        Self {
            name: "Purchase Order Processing Agent".to_string(),
            description: "Specialized A2A agent for processing, validating, and managing purchase orders with comprehensive business rules checking, financial validation, and approval workflows.".to_string(),
            url: base_url.to_string(),
            provider: Some(ProviderInfo {
                organization: "A2A Protocol Framework".to_string(),
                url: "https://agent2agent.info".to_string(),
            }),
            version: "1.0.0".to_string(),
            documentation_url: Some(format!("{}/docs", base_url)),
            capabilities: Capabilities {
                streaming: Some(false),
                push_notifications: Some(false),
                state_transition_history: Some(true),
            },
            authentication: Authentication {
                schemes: vec!["none".to_string()], // No authentication required for this demo
                credentials: None,
            },
            default_input_modes: vec![
                "application/json".to_string(),
                "text/plain".to_string(),
            ],
            default_output_modes: vec![
                "text/csv".to_string(),
                "application/json".to_string(),
                "text/plain".to_string(),
            ],
            skills: vec![
                Skill {
                    id: "purchase-order-processing".to_string(),
                    name: "Purchase Order Processing".to_string(),
                    description: "Process and validate purchase orders with comprehensive business rules checking, financial calculations verification, and approval status determination.".to_string(),
                    tags: vec![
                        "finance".to_string(),
                        "procurement".to_string(),
                        "validation".to_string(),
                        "business-rules".to_string(),
                        "approval-workflow".to_string(),
                    ],
                    examples: Some(vec![
                        "Process a purchase order for office supplies totaling $500".to_string(),
                        "Validate a marketing department purchase order with tax calculations".to_string(),
                        "Check approval status for a high-value IT equipment purchase order".to_string(),
                        "Generate CSV report from purchase order data".to_string(),
                    ]),
                    input_modes: None, // Uses default input modes
                    output_modes: None, // Uses default output modes
                },
                Skill {
                    id: "purchase-order-validation".to_string(),
                    name: "Purchase Order Validation".to_string(),
                    description: "Validate purchase order data including required fields, financial calculations, line item verification, and business rules compliance.".to_string(),
                    tags: vec![
                        "validation".to_string(),
                        "data-integrity".to_string(),
                        "business-rules".to_string(),
                        "compliance".to_string(),
                    ],
                    examples: Some(vec![
                        "Validate that all required fields are present in a purchase order".to_string(),
                        "Check that line totals match quantity × unit price calculations".to_string(),
                        "Verify that tax calculations are correct based on tax rate".to_string(),
                        "Ensure buyer department is authorized for purchases".to_string(),
                    ]),
                    input_modes: None,
                    output_modes: None,
                },
                Skill {
                    id: "purchase-order-reporting".to_string(),
                    name: "Purchase Order Reporting".to_string(),
                    description: "Generate structured reports and summaries from purchase order data in various formats including CSV, JSON, and text.".to_string(),
                    tags: vec![
                        "reporting".to_string(),
                        "data-export".to_string(),
                        "csv".to_string(),
                        "analytics".to_string(),
                    ],
                    examples: Some(vec![
                        "Generate CSV report with PO number, totals, supplier, and department".to_string(),
                        "Create JSON summary with validation status and key metrics".to_string(),
                        "Export purchase order details for accounting system integration".to_string(),
                    ]),
                    input_modes: None,
                    output_modes: Some(vec![
                        "text/csv".to_string(),
                        "application/json".to_string(),
                    ]),
                },
            ],
        }
    }

    /// Create a custom A2A AgentCard with provided parameters
    pub fn new_custom(
        name: &str,
        description: &str,
        url: &str,
        version: &str,
        provider_org: Option<&str>,
        provider_url: Option<&str>,
    ) -> Self {
        let provider = if let (Some(org), Some(purl)) = (provider_org, provider_url) {
            Some(ProviderInfo {
                organization: org.to_string(),
                url: purl.to_string(),
            })
        } else {
            None
        };

        Self {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
            provider,
            version: version.to_string(),
            documentation_url: Some(format!("{}/docs", url)),
            capabilities: Capabilities {
                streaming: Some(false),
                push_notifications: Some(false),
                state_transition_history: Some(true),
            },
            authentication: Authentication {
                schemes: vec!["none".to_string()],
                credentials: None,
            },
            default_input_modes: vec![
                "application/json".to_string(),
                "text/plain".to_string(),
            ],
            default_output_modes: vec![
                "text/csv".to_string(),
                "application/json".to_string(),
                "text/plain".to_string(),
            ],
            skills: vec![
                Skill {
                    id: "purchase-order-processing".to_string(),
                    name: "Purchase Order Processing".to_string(),
                    description: "Process and validate purchase orders".to_string(),
                    tags: vec!["finance".to_string(), "procurement".to_string()],
                    examples: Some(vec![
                        "Process a purchase order".to_string(),
                    ]),
                    input_modes: None,
                    output_modes: None,
                },
            ],
        }
    }
}