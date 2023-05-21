

/// Sets the specified view as the default for the AWS Region in which       you call this operation. If a user makes a search query that doesn't explicitly specify       the view to use, Resource Explorer chooses this default view automatically for searches       performed in this AWS Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDefaultViewAssociation {


    /// 
    /// The ARN of the view to set as the default for the AWS Region and         AWS account in which you call this operation. The specified view       must already exist in the specified Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewArn")]
    pub view_arn: String,

}



impl cfn_resources::CfnResource for CfnDefaultViewAssociation {
    fn type_string() -> &'static str {
        "AWS::ResourceExplorer2::DefaultViewAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
