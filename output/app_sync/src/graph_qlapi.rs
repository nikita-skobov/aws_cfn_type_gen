/// The AWS::AppSync::GraphQLApi resource creates a new AWS AppSync GraphQL API. This is     the top-level construct for your application. For more information, see Quick Start in the AWS AppSync       Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGraphQLApi {
    ///
    /// A list of additional authentication providers for the GraphqlApi     API.
    ///
    /// Required: No
    ///
    /// Type: List of AdditionalAuthenticationProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalAuthenticationProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_authentication_providers: Option<Vec<AdditionalAuthenticationProvider>>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-apitype
    #[serde(rename = "ApiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,

    ///
    /// Security configuration for your GraphQL API. For allowed values (such as API_KEY,       AWS_IAM, AMAZON_COGNITO_USER_POOLS, OPENID_CONNECT, or       AWS_LAMBDA), see Security in the AWS AppSync Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,

    ///
    /// A LambdaAuthorizerConfig holds configuration on how to authorize AWS AppSync API     access when using the AWS_LAMBDA authorizer mode. Be aware that an AWS AppSync API     may have only one Lambda authorizer configured at a time.
    ///
    /// Required: No
    ///
    /// Type: LambdaAuthorizerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaAuthorizerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,

    ///
    /// The Amazon CloudWatch Logs configuration.
    ///
    /// Required: No
    ///
    /// Type: LogConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-mergedapiexecutionrolearn
    #[serde(rename = "MergedApiExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_execution_role_arn: Option<String>,

    ///
    /// The API name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The OpenID Connect configuration.
    ///
    /// Required: No
    ///
    /// Type: OpenIDConnectConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenIDConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_idconnect_config: Option<OpenIDConnectConfig>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-ownercontact
    #[serde(rename = "OwnerContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,

    ///
    /// An arbitrary set of tags (key-value pairs) for this GraphQL API.
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
    /// Optional authorization configuration for using Amazon Cognito user pools with your GraphQL endpoint.
    ///
    /// Required: No
    ///
    /// Type: UserPoolConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,

    ///
    /// Sets the scope of the GraphQL API to public (GLOBAL) or private (PRIVATE). By     default, the scope is set to Global if no value is provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,

    ///
    /// A flag indicating whether to use AWS X-Ray tracing for this       GraphqlApi.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "XrayEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for CfnGraphQLApi {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::GraphQLApi"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_authorizer_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.log_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.open_idconnect_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.user_pool_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an additional authentication provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdditionalAuthenticationProvider {
    ///
    /// The authentication type for API key, AWS Identity and Access Management, OIDC, Amazon Cognito user pools, or AWS Lambda.
    ///
    /// Valid Values: API_KEY | AWS_IAM | OPENID_CONNECT |       AMAZON_COGNITO_USER_POOLS | AWS_LAMBDA
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AdditionalAuthenticationProviderAuthenticationTypeEnum,

    ///
    /// Configuration for AWS Lambda function authorization.
    ///
    /// Required: No
    ///
    /// Type: LambdaAuthorizerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaAuthorizerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,

    ///
    /// The OIDC configuration.
    ///
    /// Required: No
    ///
    /// Type: OpenIDConnectConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenIDConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_idconnect_config: Option<OpenIDConnectConfig>,

    ///
    /// The Amazon Cognito user pool configuration.
    ///
    /// Required: No
    ///
    /// Type: CognitoUserPoolConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<CognitoUserPoolConfig>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AdditionalAuthenticationProviderAuthenticationTypeEnum {
    /// API_KEY
    #[serde(rename = "API_KEY")]
    Apikey,

    /// AWS_IAM
    #[serde(rename = "AWS_IAM")]
    Awsiam,

    /// OPENID_CONNECT
    #[serde(rename = "OPENID_CONNECT")]
    Openidconnect,

    /// AMAZON_COGNITO_USER_POOLS
    #[serde(rename = "AMAZON_COGNITO_USER_POOLS")]
    Amazoncognitouserpools,

    /// AWS_LAMBDA
    #[serde(rename = "AWS_LAMBDA")]
    Awslambda,
}

impl Default for AdditionalAuthenticationProviderAuthenticationTypeEnum {
    fn default() -> Self {
        AdditionalAuthenticationProviderAuthenticationTypeEnum::Apikey
    }
}

impl cfn_resources::CfnResource for AdditionalAuthenticationProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_authorizer_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.open_idconnect_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.user_pool_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an Amazon Cognito user pool configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CognitoUserPoolConfig {
    ///
    /// A regular expression for validating the incoming Amazon Cognito user pool app client     ID. If this value isn't set, no filtering is applied.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppIdClientRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,

    ///
    /// The AWS Region in which the user pool was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,

    ///
    /// The user pool ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

impl cfn_resources::CfnResource for CognitoUserPoolConfig {
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

/// Configuration for AWS Lambda function authorization.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaAuthorizerConfig {
    ///
    /// The number of seconds a response should be cached for. The default is 0 seconds, which disables caching. If     you don't specify a value for authorizerResultTtlInSeconds, the default value is used. The maximum     value is one hour (3600 seconds). The Lambda function can override this by returning a       ttlOverride key in its response.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<f64>,

    ///
    /// The ARN of the Lambda function to be called for authorization. This may be a standard Lambda ARN, a version     ARN (.../v3) or alias ARN.
    ///
    /// Note: This Lambda function must have the following resource-based policy assigned to     it. When configuring Lambda authorizers in the console, this is done for you. To do so with the AWS CLI, run the following:
    ///
    /// aws lambda add-permission --function-name       "arn:aws:lambda:us-east-2:111122223333:function:my-function" --statement-id "appsync" --principal       appsync.amazonaws.com --action lambda:InvokeFunction
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,

    ///
    /// A regular expression for validation of tokens before the Lambda function is called.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
}

impl cfn_resources::CfnResource for LambdaAuthorizerConfig {
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

/// The LogConfig property type specifies the logging configuration when writing GraphQL operations     and tracing to Amazon CloudWatch for an AWS AppSync GraphQL API.
///
/// LogConfig is a property of the AWS::AppSync::GraphQLApi property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogConfig {
    ///
    /// The service role that AWS AppSync will assume to publish to Amazon CloudWatch Logs in your     account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,

    ///
    /// Set to TRUE to exclude sections that contain information such as headers, context, and     evaluated mapping templates, regardless of logging level.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeVerboseContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_verbose_content: Option<bool>,

    ///
    /// The field logging level. Values can be NONE, ERROR, or ALL.
    ///
    /// NONE: No field-level logs are        captured.                        ERROR: Logs the following information only for        the fields that are in error:                                                         The error section in the server response.                     Field-level errors.                     The generated request/response functions that got resolved for error           fields.                                  ALL: The following information is logged for        all fields in the query:                                               Field-level tracing information.                     The generated request/response functions that got resolved for each           field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_log_level: Option<String>,
}

impl cfn_resources::CfnResource for LogConfig {
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

/// The OpenIDConnectConfig property type specifies the optional authorization configuration for     using an OpenID Connect compliant service with your GraphQL endpoint for an AWS AppSync GraphQL     API.
///
/// OpenIDConnectConfig is a property of the AWS::AppSync::GraphQLApi property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenIDConnectConfig {
    ///
    /// The number of milliseconds that a token is valid after being authenticated.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_ttl: Option<f64>,

    ///
    /// The client identifier of the Relying party at the OpenID identity provider. This identifier is typically     obtained when the Relying party is registered with the OpenID identity provider. You can specify a regular     expression so that AWS AppSync can validate against multiple client identifiers at a time.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    ///
    /// The number of milliseconds that a token is valid after it's issued to a user.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "IatTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat_ttl: Option<f64>,

    ///
    /// The issuer for the OIDC configuration. The issuer returned by discovery must exactly     match the value of iss in the ID token.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl cfn_resources::CfnResource for OpenIDConnectConfig {
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

/// The UserPoolConfig property type specifies the optional authorization configuration for using       Amazon Cognito user pools with your GraphQL endpoint for an AWS AppSync GraphQL API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserPoolConfig {
    ///
    /// A regular expression for validating the incoming Amazon Cognito user pool app client     ID. If this value isn't set, no filtering is applied.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppIdClientRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,

    ///
    /// The AWS Region in which the user pool was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,

    ///
    /// The action that you want your GraphQL API to take when a request that uses Amazon Cognito user pool     authentication doesn't match the Amazon Cognito user pool configuration.
    ///
    /// When specifying Amazon Cognito user pools as the default authentication, you must set the value for       DefaultAction to ALLOW if specifying       AdditionalAuthenticationProviders.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,

    ///
    /// The user pool ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

impl cfn_resources::CfnResource for UserPoolConfig {
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
