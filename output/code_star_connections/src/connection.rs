

/// The AWS::CodeStarConnections::Connection resource can be used to connect external       source providers with services like AWS CodePipeline.
///
/// Note: A connection created through AWS CloudFormation is in         PENDING status by default. You can make its status         AVAILABLE by updating the connection in the console.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnection {


    /// 
    /// The Amazon Resource Name (ARN) of the host associated with the connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws(-[\w]+)*:codestar-connections:.+:[0-9]{12}:host\/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostArn")]
    pub host_arn: Option<String>,


    /// 
    /// Specifies the tags applied to the resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the external provider where your third-party code repository is    configured.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Bitbucket | GitHub | GitHubEnterpriseServer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProviderType")]
    pub provider_type: Option<ConnectionProviderTypeEnum>,


    /// 
    /// The name of the connection. Connection names must be unique in an AWS user account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectionProviderTypeEnum {

    /// Bitbucket
    #[serde(rename = "Bitbucket")]
    Bitbucket,

    /// GitHub
    #[serde(rename = "GitHub")]
    Github,

    /// GitHubEnterpriseServer
    #[serde(rename = "GitHubEnterpriseServer")]
    Githubenterpriseserver,

}

impl Default for ConnectionProviderTypeEnum {
    fn default() -> Self {
        ConnectionProviderTypeEnum::Bitbucket
    }
}


impl cfn_resources::CfnResource for CfnConnection {
    fn type_string() -> &'static str {
        "AWS::CodeStarConnections::Connection"
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


