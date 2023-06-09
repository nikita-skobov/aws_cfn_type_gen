/// A public key that you can use with signed URLs and signed cookies, or with field-level encryption.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPublicKey {
    ///
    /// Configuration information about a public key that you can use with signed URLs and signed cookies, or with field-level encryption.
    ///
    /// Required: Yes
    ///
    /// Type: PublicKeyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKeyConfig")]
    pub public_key_config: PublicKeyConfig,

    #[serde(skip_serializing)]
    pub att_created_time: CfnPublicKeycreatedtime,

    #[serde(skip_serializing)]
    pub att_id: CfnPublicKeyid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublicKeycreatedtime;
impl CfnPublicKeycreatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublicKeyid;
impl CfnPublicKeyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnPublicKey {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::PublicKey"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.public_key_config.validate()?;

        Ok(())
    }
}

/// Configuration information about a public key that you can use with signed URLs and signed cookies, or with field-level encryption.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublicKeyConfig {
    ///
    /// A string included in the request to help make sure that the request can't be 			replayed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CallerReference")]
    pub caller_reference: cfn_resources::StrVal,

    ///
    /// A comment to describe the public key. The comment cannot be longer than 128 			characters.
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
    /// The public key that you can use with signed URLs and signed cookies, or with field-level encryption.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncodedKey")]
    pub encoded_key: cfn_resources::StrVal,

    ///
    /// A name to help identify the public key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PublicKeyConfig {
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
