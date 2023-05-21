

/// The AWS::ApiGateway::UsagePlanKey resource associates an API key with a usage plan. This association determines which users the usage plan is applied to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUsagePlanKey {


    /// 
    /// The Id of the UsagePlanKey resource to be deleted.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyId")]
    pub key_id: String,


    /// 
    /// The type of a UsagePlanKey resource for a plan customer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyType")]
    pub key_type: String,


    /// 
    /// The Id of the UsagePlan resource representing the usage plan containing the to-be-deleted UsagePlanKey resource representing a plan customer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UsagePlanId")]
    pub usage_plan_id: String,

}



impl cfn_resources::CfnResource for CfnUsagePlanKey {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::UsagePlanKey"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}