

/// Creates a new subnet group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubnetGroup {


    /// 
    /// A list of VPC subnet IDs for the subnet group.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The name of the subnet group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: Option<String>,


    /// 
    /// The description of the subnet group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnSubnetGroup {
    fn type_string() -> &'static str {
        "AWS::DAX::SubnetGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
