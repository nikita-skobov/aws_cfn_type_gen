/// Creates a channel to receive content.
///
/// After it's created, a channel provides static input URLs. These URLs remain the same throughout the lifetime of the channel, regardless of any failures or upgrades that might     occur. Use these URLs to configure the outputs of your upstream encoder.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnChannel {
    ///
    /// Any descriptive information that you want to add to the channel for future identification purposes.
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
    /// Configures egress access logs.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<LogConfiguration>,

    ///
    /// The input URL where the source stream should be sent.
    ///
    /// Required: No
    ///
    /// Type: HlsIngest
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,

    ///
    /// Unique identifier that you assign to the channel.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// Configures ingress access logs.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<LogConfiguration>,

    ///
    /// The tags to assign to the channel.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnChannel {
    fn type_string(&self) -> &'static str {
        "AWS::MediaPackage::Channel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.egress_access_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_ingest
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ingress_access_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// HLS ingest configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsIngest {
    ///
    /// The input URL where the source stream should be sent.
    ///
    /// Required: No
    ///
    /// Type: List of IngestEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "ingestEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
}

impl cfn_resources::CfnResource for HlsIngest {
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

/// An endpoint for ingesting source content for a channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IngestEndpoint {
    ///
    /// The endpoint identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The system-generated password for WebDAV input authentication.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// The input URL where the source stream should be sent.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: cfn_resources::StrVal,

    ///
    /// The system-generated username for WebDAV input authentication.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IngestEndpoint {
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

/// The access log configuration parameters for your channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogConfiguration {
    ///
    /// Sets a custom Amazon CloudWatch log group name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LogConfiguration {
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
