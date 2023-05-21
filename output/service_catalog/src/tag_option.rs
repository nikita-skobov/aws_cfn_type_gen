

/// Specifies a TagOption. A TagOption is a key-value pair managed by AWS Service Catalog     that serves as a template for creating an AWS tag.
#[derive(Default, serde::Serialize)]
pub struct CfnTagOption {


    /// 
    /// The TagOption key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The TagOption active state.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Active")]
    pub active: Option<bool>,


    /// 
    /// The TagOption value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,

}
