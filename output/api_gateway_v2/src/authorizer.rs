/// The AWS::ApiGatewayV2::Authorizer resource creates an authorizer for a       WebSocket API or an HTTP API. To learn more, see Controlling and managing access to a WebSocket API in API Gateway and         Controlling and         managing access to an HTTP API in API Gateway in the API Gateway         Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAuthorizer {
    ///
    /// The API identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the format of the payload sent to an HTTP API Lambda authorizer. Required for HTTP API Lambda authorizers. Supported values are 1.0 and 2.0. To learn more, see Working with AWS Lambda authorizers for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerPayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<cfn_resources::StrVal>,

    ///
    /// The time to live (TTL) for cached authorizer results, in seconds. If it equals 0, authorization         caching is disabled. If it is greater than 0, API Gateway caches authorizer         responses. The maximum value is 3600, or 1 hour. Supported only for HTTP API Lambda authorizers.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,

    ///
    /// The authorizer type. Specify REQUEST for a Lambda function using incoming request parameters. Specify JWT to use JSON Web Tokens (supported only for HTTP APIs).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerType")]
    pub authorizer_type: cfn_resources::StrVal,

    ///
    /// The authorizer's Uniform Resource Identifier (URI). For REQUEST          authorizers, this must be a well-formed Lambda function URI, for example,                arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.          In general, the URI has this form:                arn:aws:apigateway:{region}:lambda:path/{service_api}          , where {region} is the same as the region hosting the          Lambda function, path indicates that the remaining substring in the URI should be          treated as the path to the resource, including the initial /. For          Lambda functions, this is usually of the form             /2015-03-31/functions/[FunctionARN]/invocations.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether a Lambda authorizer returns a response in a simple format. By default, a Lambda authorizer must return an IAM policy. If enabled, the Lambda authorizer can return a boolean value instead of an IAM policy. Supported only for HTTP APIs. To learn more, see Working with AWS Lambda authorizers for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableSimpleResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,

    ///
    /// The identity source for which authorization is requested.
    ///
    /// For a REQUEST authorizer, this is optional. The value is a set of         one or more mapping expressions of the specified request parameters. The         identity source can be headers, query string parameters, stage variables, and         context parameters. For example, if an Auth header and a Name query string         parameter are defined as identity sources, this value is         route.request.header.Auth, route.request.querystring.Name for WebSocket APIs.         For HTTP APIs, use selection expressions prefixed with $, for         example, $request.header.Auth,         $request.querystring.Name. These parameters are used to perform         runtime validation for Lambda-based authorizers by verifying all of the         identity-related request parameters are present in the request, not null, and         non-empty. Only when this is true does the authorizer invoke the authorizer         Lambda function. Otherwise, it returns a 401 Unauthorized response without         calling the Lambda function. For HTTP APIs, identity sources are also used as the cache key when caching is enabled. To learn more, see Working with AWS Lambda authorizers for HTTP APIs.
    ///
    /// For JWT, a single entry that specifies where to extract the JSON         Web Token (JWT) from inbound requests. Currently only header-based and query         parameter-based selections are supported, for example         $request.header.Authorization.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,

    ///
    /// This parameter is not used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<cfn_resources::StrVal>,

    ///
    /// The JWTConfiguration property specifies the configuration of a JWT          authorizer. Required for the JWT authorizer type. Supported only for          HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: JWTConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,

    ///
    /// The name of the authorizer.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_authorizer_id: CfnAuthorizerauthorizerid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAuthorizerauthorizerid;
impl CfnAuthorizerauthorizerid {
    pub fn att_name(&self) -> &'static str {
        r#"AuthorizerId"#
    }
}

impl cfn_resources::CfnResource for CfnAuthorizer {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::Authorizer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.jwt_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The JWTConfiguration property specifies the configuration of a JWT          authorizer. Required for the JWT authorizer type. Supported only for          HTTP APIs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JWTConfiguration {
    ///
    /// A list of the intended recipients of the JWT. A valid JWT must provide an          aud that matches at least one entry in this list. See RFC 7519.        Required for the JWT authorizer type. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Audience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<String>>,

    ///
    /// The base domain of the identity provider that issues JSON Web Tokens. For example,        an Amazon Cognito user pool has the following format:           https://cognito-idp.{region}.amazonaws.com/{userPoolId}           .        Required for the JWT authorizer type. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JWTConfiguration {
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
