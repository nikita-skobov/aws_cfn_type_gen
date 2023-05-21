

/// Turns on Resource Explorer in the AWS Region in which you called this       operation by creating an index. Resource Explorer begins discovering the resources in       this Region and stores the details about the resources in the index so that they can be       queried by using the Search       operation.
///
/// You can create either a local index that returns search results from only the AWS Region in which the index exists, or you can create an aggregator index       that returns search results from all AWS Regions in the AWS account.
///
/// For more details about what happens when you turn on Resource Explorer in an AWS Region, see Turning on         Resource Explorer to index your resources in an AWS Region       in the AWS Resource Explorer User Guide.
///
/// If this is the first AWS Region in which you've created an index for       Resource Explorer, this operation also creates a service-linked role in your AWS account that allows Resource Explorer to search for your resources and       populate the index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIndex {


    /// 
    /// The specified tags are attached to only the index created in this AWS Region. The tags don't attach to any of the resources listed in the       index.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Specifies the type of the index in this Region. For information about the aggregator       index and how it differs from a local index, see Turning on         cross-Region search by creating an aggregator index in the AWS Resource Explorer User Guide..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AGGREGATOR | LOCAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: IndexTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum IndexTypeEnum {

    /// AGGREGATOR
    #[serde(rename = "AGGREGATOR")]
    Aggregator,

    /// LOCAL
    #[serde(rename = "LOCAL")]
    Local,

}

impl Default for IndexTypeEnum {
    fn default() -> Self {
        IndexTypeEnum::Aggregator
    }
}


impl cfn_resources::CfnResource for CfnIndex {
    fn type_string() -> &'static str {
        "AWS::ResourceExplorer2::Index"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}