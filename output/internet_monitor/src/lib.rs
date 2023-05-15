
pub mod cfn_monitor {

#[derive(serde::Serialize, Default)]
pub struct CfnMonitor {
    /// No documentation provided by AWS
    #[serde(rename = "MonitorName")]
    pub monitor_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ResourcesToAdd")]
    pub resources_to_add: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourcesToRemove")]
    pub resources_to_remove: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxCityNetworksToMonitor")]
    pub max_city_networks_to_monitor: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "InternetMeasurementsLogDelivery")]
    pub internet_measurements_log_delivery: Option<InternetMeasurementsLogDelivery>,
    /// No documentation provided by AWS
    #[serde(rename = "TrafficPercentageToMonitor")]
    pub traffic_percentage_to_monitor: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InternetMeasurementsLogDelivery {
    #[serde(rename = "S3Config")]
    pub s3_config: Option<S3Config>,

}
pub type MonitorProcessingStatusCode = String;
#[derive(serde::Serialize, Default)]
pub struct S3Config {
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "LogDeliveryStatus")]
    pub log_delivery_status: Option<String>,

}
pub type MonitorConfigState = String;pub type iso8601UTC = String;

}
