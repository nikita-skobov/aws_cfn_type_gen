/// Creates an AWS Migration Hub Refactor Spaces application. The account that owns the environment also owns the    applications created inside the environment, regardless of the account that creates the    application. Refactor Spaces provisions an Amazon API Gateway, API Gateway VPC link, and     Network Load Balancer for the application proxy inside your account.
///
/// In environments created with a CreateEnvironment:NetworkFabricType of NONE you need to configure     VPC to VPC connectivity between your service VPC and the application proxy VPC to    route traffic through the application proxy to a service with a private URL endpoint. For more    information, see     Create an application in the Refactor Spaces User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {
    ///
    /// The endpoint URL of the Amazon API Gateway proxy.
    ///
    /// Required: No
    ///
    /// Type: ApiGatewayProxyInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiGatewayProxy")]
    pub api_gateway_proxy: Option<ApiGatewayProxyInput>,

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
    /// The name of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The proxy type of the proxy created within the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProxyType")]
    pub proxy_type: String,

    ///
    /// The tags assigned to the application.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the virtual private cloud (VPC).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::RefactorSpaces::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.api_gateway_proxy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A wrapper object holding the Amazon API Gateway endpoint input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApiGatewayProxyInput {
    ///
    /// The type of endpoint to use for the API Gateway proxy. If no value is specified in    the request, the value is set to REGIONAL by default.
    ///
    /// If the value is set to PRIVATE in the request, this creates a private API    endpoint that is isolated from the public internet. The private endpoint can only be accessed    by using Amazon Virtual Private Cloud (Amazon VPC) interface endpoints for the Amazon API Gateway that has been granted access.   For more information about creating a private connection with Refactor Spaces and interface   endpoint (AWS PrivateLink) availability, see Access    Refactor Spaces using an interface endpoint (AWS PrivateLink).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<String>,

    ///
    /// The name of the API Gateway stage. The name defaults to prod.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,
}

impl cfn_resources::CfnResource for ApiGatewayProxyInput {
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
