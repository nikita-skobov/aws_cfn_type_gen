

/// Detailed data of an AWS Proton environment account connection resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentAccountConnection {


    /// 
    /// The Amazon Resource Name (ARN) of an IAM service role in the environment account. AWS Proton uses this role to provision infrastructure resources    using CodeBuild-based provisioning in the associated environment account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):iam::\d{12}:role/([\w+=,.@-]{1,512}[/:])*([\w+=,.@-]{1,64})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodebuildRoleArn")]
    pub codebuild_role_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM service role that AWS Proton uses when provisioning directly defined components in the associated    environment account. It determines the scope of infrastructure that a component can provision in the account.
    /// 
    /// The environment account connection must have a componentRoleArn to allow directly defined components to be associated with any    environments running in the account.
    /// 
    /// For more information about components, see  AWS Proton components in the  AWS Proton User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):iam::\d{12}:role/([\w+=,.@-]{1,512}[/:])*([\w+=,.@-]{1,64})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentRoleArn")]
    pub component_role_arn: Option<String>,


    /// 
    /// The environment account that's connected to the environment account connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^\d{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentAccountId")]
    pub environment_account_id: Option<String>,


    /// 
    /// The name of the environment that's associated with the environment account connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z]+[0-9A-Za-z_\-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,


    /// 
    /// The ID of the management account that's connected to the environment account connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^\d{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagementAccountId")]
    pub management_account_id: Option<String>,


    /// 
    /// The IAM service role that's associated with the environment account connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):[a-zA-Z0-9-]+:[a-zA-Z0-9-]*:\d{12}:([\w+=,.@-]+[/:])*[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// An optional list of metadata items that you can associate with the AWS Proton environment account connection. A tag is a key-value pair.
    /// 
    /// For more information, see AWS Proton resources and tagging in the     AWS Proton User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnEnvironmentAccountConnection {
    fn type_string() -> &'static str {
        "AWS::Proton::EnvironmentAccountConnection"
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


