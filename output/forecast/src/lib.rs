
pub mod cfn_dataset {

#[derive(serde::Serialize, Default)]
pub struct CfnDataset {
    /// No documentation provided by AWS
    #[serde(rename = "Schema")]
    pub schema: (),
    /// The domain associated with the dataset
    #[serde(rename = "Domain")]
    pub domain: String,
    /// A name for the dataset
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionConfig")]
    pub encryption_config: Option<()>,
    /// The dataset type
    #[serde(rename = "DatasetType")]
    pub dataset_type: String,
    /// Frequency of data collection. This parameter is required for RELATED_TIME_SERIES
    #[serde(rename = "DataFrequency")]
    pub data_frequency: Option<String>,

}

pub type KmsKeyArn = String;pub type Key = String;pub type Value = String;
#[derive(serde::Serialize, Default)]
pub struct Attributes {

}
pub type RoleArn = String;

}

pub mod cfn_dataset_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDatasetGroup {
    /// An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the dataset group.
    #[serde(rename = "DatasetArns")]
    pub dataset_arns: Option<Vec<Arn>>,
    /// The domain associated with the dataset group. When you add a dataset to a dataset group, this value and the value specified for the Domain parameter of the CreateDataset operation must match.
    #[serde(rename = "Domain")]
    pub domain: String,
    /// A name for the dataset group.
    #[serde(rename = "DatasetGroupName")]
    pub dataset_group_name: String,
    /// The tags of Application Insights application.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

pub type MaxResults = usize;pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type NextToken = String;

}
