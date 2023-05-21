

/// Creates a Data Store that can ingest and export FHIR formatted data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFHIRDatastore {


    /// 
    /// The FHIR version of the Data Store. The only supported version is R4.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: R4
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatastoreTypeVersion")]
    pub datastore_type_version: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The user generated name for the Data Store.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-%@]*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatastoreName")]
    pub datastore_name: Option<String>,


    /// 
    /// The server-side encryption key configuration for a customer provided encryption key specified for creating a Data Store.
    /// 
    /// Required: No
    ///
    /// Type: SseConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "SseConfiguration")]
    pub sse_configuration: Option<SseConfiguration>,


    /// 
    /// The preloaded data configuration for the Data Store. Only data preloaded from Synthea is supported.
    /// 
    /// Required: No
    ///
    /// Type: PreloadDataConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreloadDataConfig")]
    pub preload_data_config: Option<PreloadDataConfig>,

}

impl cfn_resources::CfnResource for CfnFHIRDatastore {
    fn type_string() -> &'static str {
        "AWS::HealthLake::FHIRDatastore"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The CreatedAt property type specifies Property description not available. for an AWS::HealthLake::FHIRDatastore.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreatedAt {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Seconds")]
    pub seconds: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Nanos")]
    pub nanos: i64,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// Optional parameter to preload data upon creation of the Data Store. Currently, the only     supported preloaded data is synthetic data generated from Synthea.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PreloadDataConfig {


    /// 
    /// The type of preloaded data. Only Synthea preloaded data is supported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SYNTHEA
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreloadDataType")]
    pub preload_data_type: String,

}


/// The server-side encryption key configuration for a customer provided encryption key.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SseConfiguration {


    /// 
    /// The server-side encryption key configuration for a customer provided encryption key (CMK).
    /// 
    /// Required: Yes
    ///
    /// Type: KmsEncryptionConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsEncryptionConfig")]
    pub kms_encryption_config: KmsEncryptionConfig,

}


/// The customer-managed-key(CMK) used when creating a Data Store. If a customer owned key is not specified, an    Amazon owned key will be used for encryption.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KmsEncryptionConfig {


    /// 
    /// The type of customer-managed-key(CMK) used for encryption. The two types of supported CMKs are customer owned    CMKs and Amazon owned CMKs. For more information on CMK types, see KmsEncryptionConfig.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CmkType")]
    pub cmk_type: String,


    /// 
    /// The KMS encryption key id/alias used to encrypt the Data Store contents at rest.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 400
    ///
    /// Pattern: (arn:aws((-us-gov)|(-iso)|(-iso-b)|(-cn))?:kms:)?([a-z]{2}-[a-z]+(-[a-z]+)?-\d:)?(\d{12}:)?(((key/)?[a-zA-Z0-9-_]+)|(alias/[a-zA-Z0-9:/_-]+))
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}
