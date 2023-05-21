

/// Defines a resiliency policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResiliencyPolicy {


    /// 
    /// The tags assigned to the resource. A tag is a label that you assign to an AWS resource. Each tag consists of a key/value pair.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The description for the policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDescription")]
    pub policy_description: Option<String>,


    /// 
    /// The resiliency policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of FailurePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: std::collections::HashMap<String, FailurePolicy>,


    /// 
    /// The name of the policy
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: String,


    /// 
    /// Specifies a high-level geographical location constraint for where your resilience policy    data can be stored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLocationConstraint")]
    pub data_location_constraint: Option<String>,


    /// 
    /// The tier for this resiliency policy, ranging from the highest severity     (MissionCritical) to lowest (NonCritical).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    pub tier: String,

}



impl cfn_resources::CfnResource for CfnResiliencyPolicy {
    fn type_string() -> &'static str {
        "AWS::ResilienceHub::ResiliencyPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Defines a failure policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FailurePolicy {


    /// 
    /// The Recovery Point Objective (RPO), in seconds.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RpoInSecs")]
    pub rpo_in_secs: i64,


    /// 
    /// The Recovery Time Objective (RTO), in seconds.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RtoInSecs")]
    pub rto_in_secs: i64,

}


