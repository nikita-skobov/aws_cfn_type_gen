

/// The AWS::AppRunner::VpcConnector resource is an AWS App Runner resource type that specifies an App Runner VPC connector.
///
/// App Runner requires this resource when you want to associate your App Runner service to a custom Amazon Virtual Private Cloud (Amazon VPC).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVpcConnector {


    /// 
    /// A list of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the    default security group of the Amazon VPC. The default security group allows all outbound traffic.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,


    /// 
    /// A list of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single     Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
    /// 
    /// Note     App Runner currently only provides support for IPv4.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,


    /// 
    /// A list of metadata items that you can associate with your VPC connector resource. A tag is a key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A name for the VPC connector.
    /// 
    /// If you don't specify a name, AWS CloudFormation generates a name for your VPC connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9\-_]{3,39}
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConnectorName")]
    pub vpc_connector_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnVpcConnector {
    fn type_string() -> &'static str {
        "AWS::AppRunner::VpcConnector"
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


