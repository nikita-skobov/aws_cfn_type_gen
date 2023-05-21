

/// The AWS::CodeDeploy::Application resource creates an AWS CodeDeploy    application. In CodeDeploy, an application is a name that functions as a container    to ensure that the correct combination of revision, deployment configuration, and deployment    group are referenced during a deployment. You can use the     AWS::CodeDeploy::DeploymentGroup resource to associate the application with a     CodeDeploy deployment group. For more information, see CodeDeploy     Deployments in the AWS CodeDeploy User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {


    /// 
    /// The metadata that you apply to CodeDeploy applications to help you organize and       categorize them. Each tag consists of a key and an optional value, both of which you       define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A name for the application. If you don't specify a name, AWS CloudFormation generates a    unique physical ID and uses that ID for the application name. For more information, see Name     Type.
    /// 
    /// Note Updates to ApplicationName are not supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,


    /// 
    /// The compute platform that CodeDeploy deploys the application to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECS | Lambda | Server
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComputePlatform")]
    pub compute_platform: Option<ApplicationComputePlatformEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationComputePlatformEnum {

    /// ECS
    #[serde(rename = "ECS")]
    Ecs,

    /// Lambda
    #[serde(rename = "Lambda")]
    Lambda,

    /// Server
    #[serde(rename = "Server")]
    Server,

}

impl Default for ApplicationComputePlatformEnum {
    fn default() -> Self {
        ApplicationComputePlatformEnum::Ecs
    }
}


impl cfn_resources::CfnResource for CfnApplication {
    fn type_string() -> &'static str {
        "AWS::CodeDeploy::Application"
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


