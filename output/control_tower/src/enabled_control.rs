

/// The resource represents an enabled control. It specifies an asynchronous operation       that creates AWS resources on the specified organizational unit and the       accounts it contains. The resources created will vary according to the control that you       specify.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnabledControl {


    /// 
    /// The ARN of the control. Only Strongly recommended and Elective controls are permitted,     with the exception of the Region deny guardrail.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ControlIdentifier")]
    pub control_identifier: String,


    /// 
    /// The ARN of the organizational unit.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetIdentifier")]
    pub target_identifier: String,

}

impl cfn_resources::CfnResource for CfnEnabledControl {
    fn type_string() -> &'static str {
        "AWS::ControlTower::EnabledControl"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
