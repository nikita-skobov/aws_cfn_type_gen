

/// The AWS::DocDB::DBClusterParameterGroup Amazon DocumentDB (with MongoDB compatibility) resource describes a DBClusterParameterGroup.     For more information, see DBClusterParameterGroup      in the Amazon DocumentDB Developer Guide.
///
/// Parameters in a cluster parameter group apply to all of the instances in a cluster.
///
/// A cluster parameter group is initially created with the default parameters for the     database engine used by instances in the cluster. To provide custom values for any of     the parameters, you must modify the group after you create it. After you create a DB     cluster parameter group, you must associate it with your cluster. For the new cluster     parameter group and associated settings to take effect, you must then reboot the DB     instances in the cluster without failover.
#[derive(Default, serde::Serialize)]
pub struct CfnDBClusterParameterGroup {


    /// 
    /// The cluster parameter group family name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Family")]
    pub family: String,


    /// 
    /// Provides a list of parameters for the cluster parameter group.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: serde_json::Value,


    /// 
    /// The tags to be assigned to the cluster parameter group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The description for the cluster parameter group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,


    /// 
    /// The name of the DB cluster parameter group.
    /// 
    /// Constraints:
    /// 
    /// Must not match the name of an existing           DBClusterParameterGroup.
    /// 
    /// NoteThis value is stored as a lowercase string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}