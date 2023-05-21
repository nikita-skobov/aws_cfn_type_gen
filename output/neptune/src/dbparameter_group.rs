

/// AWS::Neptune::DBParameterGroup creates a new DB parameter group.      This type can be declared in a template and referenced in the DBParameterGroupName      parameter of AWS::Neptune::DBInstance.
///
/// A DB parameter group is initially created with the default parameters for the database    engine used by the DB instance. To provide custom values for any of the parameters, you must    modify the group after creating it using ModifyDBParameterGroup. Once    you've created a DB parameter group, you need to associate it with your DB instance using    ModifyDBInstance. When you associate a new DB parameter group with a    running DB instance, you need to reboot the DB instance without failover for the new DB    parameter group and associated settings to take effect.
#[derive(Default, serde::Serialize)]
pub struct CfnDBParameterGroup {


    /// 
    /// The parameters to set for this DB parameter group.
    /// 
    /// The parameters are expressed as a JSON object consisting of key-value pairs.
    /// 
    /// Changes to dynamic parameters are applied immediately. During an update, if     you have static parameters (whether they were changed or not), it triggers AWS CloudFormation     to reboot the associated DB instance without failover.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: serde_json::Value,


    /// 
    /// Provides the customer-specified description for this DB parameter group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,


    /// 
    /// The tags that you want to attach to this parameter group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Must be neptune1 for engine versions prior to 1.2.0.0, or    neptune1.2 for engine version 1.2.0.0 and higher.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Family")]
    pub family: String,


    /// 
    /// Provides the name of the DB parameter group.
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
