

/// The AWS::RDS::DBClusterParameterGroup resource creates a new Amazon RDS       DB cluster parameter group.
///
/// For information about configuring parameters for Amazon Aurora DB clusters, see       Working with parameter         groups in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBClusterParameterGroup {


    /// 
    /// A friendly description for this DB cluster parameter group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,


    /// 
    /// The DB cluster parameter group family name. A DB cluster parameter group can be       associated with one and only one DB cluster parameter group family, and can be applied       only to a DB cluster running a DB engine and engine version compatible with that DB       cluster parameter group family.
    /// 
    /// NoteThe DB cluster parameter group family can't be changed when updating a DB cluster         parameter group.
    /// 
    /// To list all of the available parameter group families, use the following       command:
    /// 
    /// aws rds describe-db-engine-versions --query         "DBEngineVersions[].DBParameterGroupFamily"
    /// 
    /// The output contains duplicates.
    /// 
    /// For more information, see CreateDBClusterParameterGroup.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Family")]
    pub family: String,


    /// 
    /// Provides a list of parameters for the DB cluster parameter group.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: serde_json::Value,


    /// 
    /// An optional array of key-value pairs to apply to this DB cluster parameter group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the DB cluster parameter group.
    /// 
    /// Constraints:
    /// 
    /// Must not match the name of an existing DB cluster parameter group.
    /// 
    /// If you don't specify a value for DBClusterParameterGroupName property, a name is automatically created for the DB cluster parameter group.
    /// 
    /// NoteThis value is stored as a lowercase string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,

}

impl cfn_resources::CfnResource for CfnDBClusterParameterGroup {
    fn type_string() -> &'static str {
        "AWS::RDS::DBClusterParameterGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}
