
pub mod cfn_enabled_control {

#[derive(serde::Serialize, Default)]
pub struct CfnEnabledControl {
    /// Arn of the control.
    #[serde(rename = "ControlIdentifier")]
    pub control_identifier: String,
    /// Arn for Organizational unit to which the control needs to be applied
    #[serde(rename = "TargetIdentifier")]
    pub target_identifier: String,

}



}
