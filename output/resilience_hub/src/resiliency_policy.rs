/// Defines a resiliency policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResiliencyPolicy {
    ///
    /// Specifies a high-level geographical location constraint for where your resilience policy    data can be stored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLocationConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_constraint: Option<cfn_resources::StrVal>,

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
    /// The description for the policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the policy
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: cfn_resources::StrVal,

    ///
    /// The tags assigned to the resource. A tag is a label that you assign to an AWS resource. Each tag consists of a key/value pair.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The tier for this resiliency policy, ranging from the highest severity     (MissionCritical) to lowest (NonCritical).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    pub tier: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_policy_arn: CfnResiliencyPolicypolicyarn,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResiliencyPolicypolicyarn;
impl CfnResiliencyPolicypolicyarn {
    pub fn att_name(&self) -> &'static str {
        r#"PolicyArn"#
    }
}

impl cfn_resources::CfnResource for CfnResiliencyPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::ResilienceHub::ResiliencyPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
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

impl cfn_resources::CfnResource for FailurePolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
