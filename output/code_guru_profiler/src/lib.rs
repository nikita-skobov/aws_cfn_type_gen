
pub mod cfn_profiling_group {

#[derive(serde::Serialize, Default)]
pub struct CfnProfilingGroup {
    /// Configuration for Notification Channels for Anomaly Detection feature in CodeGuru Profiler which enables customers to detect anomalies in the application profile for those methods that represent the highest proportion of CPU time or latency
    #[serde(rename = "AnomalyDetectionNotificationConfiguration")]
    pub anomaly_detection_notification_configuration: Option<Vec<Channel>>,
    /// The name of the profiling group.
    #[serde(rename = "ProfilingGroupName")]
    pub profiling_group_name: String,
    /// The tags associated with a profiling group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The agent permissions attached to this profiling group.
    #[serde(rename = "AgentPermissions")]
    pub agent_permissions: Option<()>,
    /// The compute platform of the profiling group.
    #[serde(rename = "ComputePlatform")]
    pub compute_platform: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type ProfilingGroupArn = String;pub type ChannelId = String;pub type IamArn = String;
#[derive(serde::Serialize, Default)]
pub struct Channel {
    #[serde(rename = "channelUri")]
    pub channel_uri: ChannelUri,
    #[serde(rename = "channelId")]
    pub channel_id: Option<ChannelId>,

}
pub type ChannelUri = String;

}
