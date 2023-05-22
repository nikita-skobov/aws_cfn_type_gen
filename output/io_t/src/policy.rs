/// Use the AWS::IoT::Policy resource to declare an AWS IoT policy. For more     information about working with AWS IoT policies, see Authorization in the       AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPolicy {
    ///
    /// The JSON document that describes the policy.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

    ///
    /// The policy name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnPolicyarn,

    #[serde(skip_serializing)]
    pub att_id: CfnPolicyid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPolicyarn;
impl CfnPolicyarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPolicyid;
impl CfnPolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::Policy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
