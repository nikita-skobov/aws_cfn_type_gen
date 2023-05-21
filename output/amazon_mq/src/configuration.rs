

/// Creates a new configuration for the specified configuration name. Amazon MQ uses        the default configuration (the engine type and version).
#[derive(Default, serde::Serialize)]
pub struct CfnConfiguration {


    /// 
    /// The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,


    /// 
    /// The description of the configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the configuration. This value can contain only alphanumeric characters,    dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters    long.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Create tags when creating the configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,


    /// 
    /// The type of broker engine. Note: Currently, Amazon MQ only supports ACTIVEMQ for creating and editing broker configurations.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineType")]
    pub engine_type: String,


    /// 
    /// Optional. The authentication strategy associated with the configuration. The        default is SIMPLE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationStrategy")]
    pub authentication_strategy: Option<String>,


    /// 
    /// The base64-encoded XML configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Data")]
    pub data: String,

}


/// A key-value pair to associate with the configuration.
#[derive(Default, serde::Serialize)]
pub struct TagsEntry {


    /// 
    /// The key in a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value in a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}
