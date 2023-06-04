/// A key group.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnKeyGroup {
    ///
    /// The key group configuration.
    ///
    /// Required: Yes
    ///
    /// Type: KeyGroupConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyGroupConfig")]
    pub key_group_config: KeyGroupConfig,

    #[serde(skip_serializing)]
    pub att_id: CfnKeyGroupid,

    #[serde(skip_serializing)]
    pub att_last_modified_time: CfnKeyGrouplastmodifiedtime,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnKeyGroupid;
impl CfnKeyGroupid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnKeyGrouplastmodifiedtime;
impl CfnKeyGrouplastmodifiedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedTime"#
    }
}

impl cfn_resources::CfnResource for CfnKeyGroup {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::KeyGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.key_group_config.validate()?;

        Ok(())
    }
}

/// A key group configuration.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct KeyGroupConfig {
    ///
    /// A comment to describe the key group. The comment cannot be longer than 128 			characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<cfn_resources::StrVal>,

    ///
    /// A list of the identifiers of the public keys in the key group.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<String>,

    ///
    /// A name to identify the key group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KeyGroupConfig {
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
