

/// Associate the specified TagOption with the specified portfolio or product.
#[derive(Default, serde::Serialize)]
pub struct CfnTagOptionAssociation {


    /// 
    /// The resource identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: String,


    /// 
    /// The TagOption identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,

}
