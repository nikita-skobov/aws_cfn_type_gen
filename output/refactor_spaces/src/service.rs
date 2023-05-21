

/// Creates an AWS Migration Hub Refactor Spaces service. The account owner of the service is always the    environment owner, regardless of which account in the environment creates the service.    Services have either a URL endpoint in a virtual private cloud (VPC), or a Lambda    function endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnService {


    /// 
    /// The unique identifier of the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,


    /// 
    /// A description of the service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The endpoint type of the service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,


    /// 
    /// The unique identifier of the environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: String,


    /// 
    /// A summary of the configuration for the AWS Lambda endpoint type.
    /// 
    /// Required: No
    ///
    /// Type: LambdaEndpointInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaEndpoint")]
    pub lambda_endpoint: Option<LambdaEndpointInput>,


    /// 
    /// The name of the service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The tags assigned to the service.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The summary of the configuration for the URL endpoint type.
    /// 
    /// Required: No
    ///
    /// Type: UrlEndpointInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "UrlEndpoint")]
    pub url_endpoint: Option<UrlEndpointInput>,


    /// 
    /// The ID of the virtual private cloud (VPC).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnService {
    fn type_string() -> &'static str {
        "AWS::RefactorSpaces::Service"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The input for the AWS Lambda endpoint type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaEndpointInput {


    /// 
    /// The Amazon Resource Name (ARN) of the Lambda function or alias.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: String,

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




/// The configuration for the URL endpoint type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UrlEndpointInput {


    /// 
    /// The health check URL of the URL endpoint type. If the URL is a public endpoint, the     HealthUrl must also be a public endpoint. If the URL is a private endpoint    inside a virtual private cloud (VPC), the health URL must also be a private endpoint, and the    host must be the same as the URL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HealthUrl")]
    pub health_url: Option<String>,


    /// 
    /// The URL to route traffic to. The URL must be an rfc3986-formatted URL. If the    host is a domain name, the name must be resolvable over the public internet. If the scheme is     https, the top level domain of the host must be listed in the IANA root zone database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Url")]
    pub url: String,

}


