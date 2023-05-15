
pub mod cfn_ledger {

#[derive(serde::Serialize, Default)]
pub struct CfnLedger {
    /// No documentation provided by AWS
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionsMode")]
    pub permissions_mode: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnStream {
    /// No documentation provided by AWS
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ExclusiveEndTime")]
    pub exclusive_end_time: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LedgerName")]
    pub ledger_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,
    /// No documentation provided by AWS
    #[serde(rename = "InclusiveStartTime")]
    pub inclusive_start_time: String,
    /// No documentation provided by AWS
    #[serde(rename = "KinesisConfiguration")]
    pub kinesis_configuration: KinesisConfiguration,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisConfiguration {
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<Arn>,
    #[serde(rename = "AggregationEnabled")]
    pub aggregation_enabled: Option<bool>,

}


}
