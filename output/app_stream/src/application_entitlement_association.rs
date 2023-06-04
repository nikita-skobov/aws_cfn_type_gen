/// Associates an application to an entitlement.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApplicationEntitlementAssociation {
    /// The identifier of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: cfn_resources::StrVal,

    /// The name of the entitlement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntitlementName")]
    pub entitlement_name: cfn_resources::StrVal,

    /// The name of the stack.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackName")]
    pub stack_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnApplicationEntitlementAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::ApplicationEntitlementAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
