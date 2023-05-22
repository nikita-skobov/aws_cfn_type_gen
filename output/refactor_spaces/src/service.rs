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
    pub application_identifier: cfn_resources::StrVal,

    ///
    /// A description of the service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The endpoint type of the service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointType")]
    pub endpoint_type: cfn_resources::StrVal,

    ///
    /// The unique identifier of the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: cfn_resources::StrVal,

    ///
    /// A summary of the configuration for the AWS Lambda endpoint type.
    ///
    /// Required: No
    ///
    /// Type: LambdaEndpointInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub name: cfn_resources::StrVal,

    ///
    /// The tags assigned to the service.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnServicearn,

    #[serde(skip_serializing)]
    pub att_service_identifier: CfnServiceserviceidentifier,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServicearn;
impl CfnServicearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceserviceidentifier;
impl CfnServiceserviceidentifier {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceIdentifier"#
    }
}

impl cfn_resources::CfnResource for CfnService {
    fn type_string(&self) -> &'static str {
        "AWS::RefactorSpaces::Service"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.url_endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
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
    pub arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LambdaEndpointInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_url: Option<cfn_resources::StrVal>,

    ///
    /// The URL to route traffic to. The URL must be an rfc3986-formatted URL. If the    host is a domain name, the name must be resolvable over the public internet. If the scheme is     https, the top level domain of the host must be listed in the IANA root zone database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Url")]
    pub url: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for UrlEndpointInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
