

/// The AWS::LakeFormation::Tag resource represents an LF-tag, which consists of a key and one or more possible values for the key.    During a stack operation, AWS CloudFormation calls the AWS Lake Formation CreateLFTag API to create a tag, and UpdateLFTag API to update a tag resource, and a DeleteLFTag to delete it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTag {


    /// 
    /// Catalog id string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    /// 
    /// The identifier for the Data Catalog. By default, the account ID.       The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// UTF-8 string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    /// 
    /// The key-name for the LF-tag.
    /// 
    /// For more information about using the Reffunction, see Ref.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagKey")]
    pub tag_key: String,


    /// 
    /// An array of UTF-8 strings, not less than 1 or more than 50 strings.
    /// 
    /// A list of possible values of the corresponding TagKey of an LF-tag key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,

}



impl cfn_resources::CfnResource for CfnTag {
    fn type_string() -> &'static str {
        "AWS::LakeFormation::Tag"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
