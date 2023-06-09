/// Use the AWS::EventSchemas::Discoverer resource to specify a         discoverer that is associated with an event bus. A discoverer       allows the Amazon EventBridge Schema Registry to automatically generate schemas based on       events on an event bus.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDiscoverer {
    ///
    /// Allows for the discovery of the event schemas that are sent to the event bus from another account.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account: Option<bool>,

    ///
    /// A description for the discoverer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the event bus.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceArn")]
    pub source_arn: cfn_resources::StrVal,

    ///
    /// Tags associated with the resource.
    ///
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagsEntry>>,

    #[serde(skip_serializing)]
    pub att_discoverer_arn: CfnDiscovererdiscovererarn,

    #[serde(skip_serializing)]
    pub att_discoverer_id: CfnDiscovererdiscovererid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiscovererdiscovererarn;
impl CfnDiscovererdiscovererarn {
    pub fn att_name(&self) -> &'static str {
        r#"DiscovererArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiscovererdiscovererid;
impl CfnDiscovererdiscovererid {
    pub fn att_name(&self) -> &'static str {
        r#"DiscovererId"#
    }
}

impl cfn_resources::CfnResource for CfnDiscoverer {
    fn type_string(&self) -> &'static str {
        "AWS::EventSchemas::Discoverer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Tags to associate with the discoverer.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub key: cfn_resources::StrVal,

    ///
    /// They value of a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TagsEntry {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
