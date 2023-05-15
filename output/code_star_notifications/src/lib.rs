
pub mod cfn_notification_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnNotificationRule {
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EventTypeIds")]
    pub event_type_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DetailType")]
    pub detail_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "EventTypeId")]
    pub event_type_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetAddress")]
    pub target_address: Option<String>,
    /// List of Target
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    /// No documentation provided by AWS
    #[serde(rename = "Resource")]
    pub resource: String,

}


#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "TargetType")]
    pub target_type: String,
    #[serde(rename = "TargetAddress")]
    pub target_address: String,

}


}
