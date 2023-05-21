

/// Creates an Amazon Forecast dataset. The information about the dataset that you provide helps    Forecast understand how to consume the data for model training. This includes the    following:
///
/// After creating a dataset, you import your training data into it and add the dataset to a    dataset group. You use the dataset group to create a predictor. For more information, see     Importing datasets.
///
/// To get a list of all your datasets, use the ListDatasets operation.
///
/// For example Forecast datasets, see the Amazon Forecast Sample GitHub     repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataset {


    /// 
    /// The frequency of data collection. This parameter is required for RELATED_TIME_SERIES    datasets.
    /// 
    /// Valid intervals are an integer followed by Y (Year), M (Month), W (Week), D (Day), H (Hour), and min (Minute). For example,    "1D" indicates every day and "15min" indicates every 15 minutes. You cannot specify a value that would overlap with the next larger frequency. That means, for example, you cannot specify a frequency of 60 minutes, because that is equivalent to 1 hour. The valid values for each frequency are the following:
    /// 
    /// Minute - 1-59               Hour - 1-23               Day - 1-6               Week - 1-4               Month - 1-11               Year - 1
    /// 
    /// Thus, if you want every other week forecasts, specify "2W". Or, if you want quarterly forecasts, you specify "3M".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5
    ///
    /// Pattern: ^Y|M|W|D|H|30min|15min|10min|5min|1min$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataFrequency")]
    pub data_frequency: Option<String>,


    /// 
    /// The name of the dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,


    /// 
    /// The dataset type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ITEM_METADATA | RELATED_TIME_SERIES | TARGET_TIME_SERIES
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetType")]
    pub dataset_type: DatasetDatasetTypeEnum,


    /// 
    /// The domain associated with the dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | EC2_CAPACITY | INVENTORY_PLANNING | METRICS | RETAIL | WEB_TRAFFIC | WORK_FORCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: DatasetDomainEnum,


    /// 
    /// A Key Management Service (KMS) key and the Identity and Access Management (IAM) role that Amazon Forecast can assume to access    the key.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfig")]
    pub encryption_config: Option<EncryptionConfig>,


    /// 
    /// The schema for the dataset. The schema attributes and their order must match the fields in    your data. The dataset Domain and DatasetType that you choose    determine the minimum required fields in your training data. For information about the    required fields for a specific dataset domain and type, see Dataset Domains and Dataset     Types.
    /// 
    /// Required: Yes
    ///
    /// Type: Schema
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: Schema,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsItems
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsItems>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DatasetDatasetTypeEnum {

    /// ITEM_METADATA
    #[serde(rename = "ITEM_METADATA")]
    Itemmetadata,

    /// RELATED_TIME_SERIES
    #[serde(rename = "RELATED_TIME_SERIES")]
    Relatedtimeseries,

    /// TARGET_TIME_SERIES
    #[serde(rename = "TARGET_TIME_SERIES")]
    Targettimeseries,

}

impl Default for DatasetDatasetTypeEnum {
    fn default() -> Self {
        DatasetDatasetTypeEnum::Itemmetadata
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DatasetDomainEnum {

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// EC2_CAPACITY
    #[serde(rename = "EC2_CAPACITY")]
    Ec2capacity,

    /// INVENTORY_PLANNING
    #[serde(rename = "INVENTORY_PLANNING")]
    Inventoryplanning,

    /// METRICS
    #[serde(rename = "METRICS")]
    Metrics,

    /// RETAIL
    #[serde(rename = "RETAIL")]
    Retail,

    /// WEB_TRAFFIC
    #[serde(rename = "WEB_TRAFFIC")]
    Webtraffic,

    /// WORK_FORCE
    #[serde(rename = "WORK_FORCE")]
    Workforce,

}

impl Default for DatasetDomainEnum {
    fn default() -> Self {
        DatasetDomainEnum::Custom
    }
}


impl cfn_resources::CfnResource for CfnDataset {
    fn type_string() -> &'static str {
        "AWS::Forecast::Dataset"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AttributesItems property type specifies Property description not available. for an AWS::Forecast::Dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AttributesItems {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeType")]
    pub attribute_type: Option<String>,

}




/// An AWS Key Management Service (KMS) key and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to    access the key. You can specify this optional object in the    CreateDataset and CreatePredictor requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfig {


    /// 
    /// The Amazon Resource Name (ARN) of the KMS key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws:kms:.*:key/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,


    /// 
    /// The ARN of the IAM role that Amazon Forecast can assume to access the AWS KMS key.
    /// 
    /// Passing a role across AWS accounts is not allowed. If you pass a role that isn't in your    account, you get an InvalidInputException error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):forecast:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}




/// Defines the fields of a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schema {


    /// 
    /// An array of attributes specifying the name and type of each field in a dataset.
    /// 
    /// Required: No
    ///
    /// Type: List of AttributesItems
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<Vec<AttributesItems>>,

}




/// The TagsItems property type specifies Property description not available. for an AWS::Forecast::Dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagsItems {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


