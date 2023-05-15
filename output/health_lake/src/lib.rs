
pub mod cfn_fhirdatastore {

#[derive(serde::Serialize, Default)]
pub struct CfnFHIRDatastore {
    /// The preloaded data configuration for the Data Store. Only data preloaded from Synthea is supported.
    #[serde(rename = "PreloadDataConfig")]
    pub preload_data_config: Option<PreloadDataConfig>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The server-side encryption key configuration for a customer provided encryption key.
    #[serde(rename = "SseConfiguration")]
    pub sse_configuration: Option<SseConfiguration>,
    /// The user-generated name for the Data Store.
    #[serde(rename = "DatastoreName")]
    pub datastore_name: Option<DatastoreName>,
    /// The FHIR version. Only R4 version data is supported.
    #[serde(rename = "DatastoreTypeVersion")]
    pub datastore_type_version: DatastoreTypeVersion,

}


#[derive(serde::Serialize, Default)]
pub struct PreloadDataConfig {
    #[serde(rename = "PreloadDataType")]
    pub preload_data_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct SseConfiguration {
    #[serde(rename = "KmsEncryptionConfig")]
    pub kms_encryption_config: KmsEncryptionConfig,

}
pub type DatastoreName = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type DatastoreTypeVersion = String;
#[derive(serde::Serialize, Default)]
pub struct KmsEncryptionConfig {
    #[serde(rename = "CmkType")]
    pub cmk_type: String,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}
pub type DatastoreStatus = String;
#[derive(serde::Serialize, Default)]
pub struct CreatedAt {
    #[serde(rename = "Nanos")]
    pub nanos: usize,
    #[serde(rename = "Seconds")]
    pub seconds: String,

}
pub type DatastoreArn = String;pub type DatastoreEndpoint = String;pub type DatastoreId = String;

}
