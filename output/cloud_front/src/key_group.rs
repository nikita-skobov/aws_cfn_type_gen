

/// A key group.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

}



impl cfn_resources::CfnResource for CfnKeyGroup {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::KeyGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.key_group_config.validate()?;

        Ok(())
    }
}

/// A key group configuration.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub comment: Option<String>,


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
    pub name: String,

}



impl cfn_resources::CfnResource for KeyGroupConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}