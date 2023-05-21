

/// Creates a new subnet group.
#[derive(Default, serde::Serialize)]
pub struct CfnSubnetGroup {


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
