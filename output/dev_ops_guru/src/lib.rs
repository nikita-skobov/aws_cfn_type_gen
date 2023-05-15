
pub mod cfn_log_anomaly_detection_integration {

#[derive(serde::Serialize, Default)]
pub struct CfnLogAnomalyDetectionIntegration {

}

pub type AccountId = String;

}

pub mod cfn_notification_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnNotificationChannel {
    /// No documentation provided by AWS
    #[serde(rename = "Config")]
    pub config: NotificationChannelConfig,

}


#[derive(serde::Serialize, Default)]
pub struct NotificationMessageTypesFilterList {

}
pub type InsightSeverity = String;
#[derive(serde::Serialize, Default)]
pub struct SnsChannelConfig {
    #[serde(rename = "TopicArn")]
    pub topic_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightSeveritiesFilterList {

}
pub type NotificationMessageType = String;
#[derive(serde::Serialize, Default)]
pub struct NotificationChannelConfig {
    #[serde(rename = "Filters")]
    pub filters: Option<NotificationFilterConfig>,
    #[serde(rename = "Sns")]
    pub sns: Option<SnsChannelConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationFilterConfig {
    #[serde(rename = "MessageTypes")]
    pub message_types: Option<NotificationMessageTypesFilterList>,
    #[serde(rename = "Severities")]
    pub severities: Option<InsightSeveritiesFilterList>,

}


}

pub mod cfn_resource_collection {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceCollection {
    /// Information about a filter used to specify which AWS resources are analyzed for anomalous behavior by DevOps Guru.
    #[serde(rename = "ResourceCollectionFilter")]
    pub resource_collection_filter: ResourceCollectionFilter,

}


#[derive(serde::Serialize, Default)]
pub struct TagCollections {

}

#[derive(serde::Serialize, Default)]
pub struct ResourceCollectionFilter {
    #[serde(rename = "CloudFormation")]
    pub cloud_formation: Option<CloudFormationCollectionFilter>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagCollections>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudFormationCollectionFilter {
    #[serde(rename = "StackNames")]
    pub stack_names: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct TagCollection {
    #[serde(rename = "AppBoundaryKey")]
    pub app_boundary_key: Option<String>,
    #[serde(rename = "TagValues")]
    pub tag_values: Option<Vec<String>>,

}


}
