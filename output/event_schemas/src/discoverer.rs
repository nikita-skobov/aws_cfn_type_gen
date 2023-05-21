

/// Use the AWS::EventSchemas::Discoverer resource to specify a         discoverer that is associated with an event bus. A discoverer       allows the Amazon EventBridge Schema Registry to automatically generate schemas based on       events on an event bus.
#[derive(Default, serde::Serialize)]
pub struct CfnDiscoverer {


    /// 
    /// A description for the discoverer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Allows for the discovery of the event schemas that are sent to the event bus from another account.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossAccount")]
    pub cross_account: Option<bool>,


    /// 
    /// Tags associated with the resource.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,


    /// 
    /// The ARN of the event bus.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceArn")]
    pub source_arn: String,

}


/// Tags to associate with the discoverer.
#[derive(Default, serde::Serialize)]
pub struct TagsEntry {


    /// 
    /// They key of a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// They value of a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}
