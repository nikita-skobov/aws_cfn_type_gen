

/// A key group.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Default, serde::Serialize)]
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


/// A key group configuration.
///
/// A key group contains a list of public keys that you can use with CloudFront signed URLs and signed cookies.
#[derive(Default, serde::Serialize)]
pub struct KeyGroupConfig {


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
    /// A comment to describe the key group. The comment cannot be longer than 128 			characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}
