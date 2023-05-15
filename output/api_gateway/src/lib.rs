
pub mod cfn_account {

#[derive(serde::Serialize, Default)]
pub struct CfnAccount {
    /// The Amazon Resource Name (ARN) of an IAM role that has write access to CloudWatch Logs in your account.
    #[serde(rename = "CloudWatchRoleArn")]
    pub cloud_watch_role_arn: Option<String>,

}



}

pub mod cfn_api_key {

#[derive(serde::Serialize, Default)]
pub struct CfnApiKey {
    /// An AWS Marketplace customer identifier to use when integrating with the AWS SaaS Marketplace.
    #[serde(rename = "CustomerId")]
    pub customer_id: Option<String>,
    /// Specifies whether the key identifier is distinct from the created API key value. This parameter is deprecated and should not be used.
    #[serde(rename = "GenerateDistinctId")]
    pub generate_distinct_id: Option<bool>,
    /// A list of stages to associate with this API key.
    #[serde(rename = "StageKeys")]
    pub stage_keys: Option<Vec<StageKey>>,
    /// An array of arbitrary tags (key-value pairs) to associate with the API key.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The value of the API key. Must be at least 20 characters long.
    #[serde(rename = "Value")]
    pub value: Option<String>,
    /// A name for the API key. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the API key name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Indicates whether the API key can be used by clients.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// A description of the purpose of the API key.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct StageKey {
    #[serde(rename = "RestApiId")]
    pub rest_api_id: Option<String>,
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_authorizer {

#[derive(serde::Serialize, Default)]
pub struct CfnAuthorizer {
    /// Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer.
    #[serde(rename = "AuthorizerCredentials")]
    pub authorizer_credentials: Option<String>,
    /// The identity source for which authorization is requested.
    #[serde(rename = "IdentitySource")]
    pub identity_source: Option<String>,
    /// Optional customer-defined field, used in OpenAPI imports and exports without functional impact.
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,
    /// The identifier of the API.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// A validation expression for the incoming identity token.
    #[serde(rename = "IdentityValidationExpression")]
    pub identity_validation_expression: Option<String>,
    /// The name of the authorizer.
    #[serde(rename = "Name")]
    pub name: String,
    /// A list of the Amazon Cognito user pool ARNs for the COGNITO_USER_POOLS authorizer.
    #[serde(rename = "ProviderARNs")]
    pub provider_arns: Option<Vec<String>>,
    /// The authorizer type.
    #[serde(rename = "Type")]
    pub ty: String,
    /// Specifies the authorizer's Uniform Resource Identifier (URI).
    #[serde(rename = "AuthorizerUri")]
    pub authorizer_uri: Option<String>,
    /// The TTL in seconds of cached authorizer results.
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: Option<usize>,

}



}

pub mod cfn_base_path_mapping {

#[derive(serde::Serialize, Default)]
pub struct CfnBasePathMapping {
    /// The DomainName of an AWS::ApiGateway::DomainName resource.
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// The name of the API's stage.
    #[serde(rename = "Stage")]
    pub stage: Option<String>,
    /// The base path name that callers of the API must provide in the URL after the domain name.
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,
    /// The ID of the API.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: Option<String>,

}



}

pub mod cfn_client_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnClientCertificate {
    /// A description of the client certificate.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// An array of arbitrary tags (key-value pairs) to associate with the client certificate.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_deployment {

#[derive(serde::Serialize, Default)]
pub struct CfnDeployment {
    /// The ID of the RestApi resource to deploy.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// Configures the stage that API Gateway creates with this deployment.
    #[serde(rename = "StageDescription")]
    pub stage_description: Option<StageDescription>,
    /// A description of the purpose of the API Gateway deployment.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// A name for the stage that API Gateway creates with this deployment. Use only alphanumeric characters.
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,
    /// Specifies settings for the canary deployment.
    #[serde(rename = "DeploymentCanarySettings")]
    pub deployment_canary_settings: Option<DeploymentCanarySettings>,

}


#[derive(serde::Serialize, Default)]
pub struct CanarySetting {
    #[serde(rename = "PercentTraffic")]
    pub percent_traffic: Option<f64>,
    #[serde(rename = "UseStageCache")]
    pub use_stage_cache: Option<bool>,
    #[serde(rename = "StageVariableOverrides")]
    pub stage_variable_overrides: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct MethodSetting {
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "CacheTtlInSeconds")]
    pub cache_ttl_in_seconds: Option<usize>,
    #[serde(rename = "CachingEnabled")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    #[serde(rename = "ResourcePath")]
    pub resource_path: Option<String>,
    #[serde(rename = "CacheDataEncrypted")]
    pub cache_data_encrypted: Option<bool>,
    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: Option<bool>,
    #[serde(rename = "HttpMethod")]
    pub http_method: Option<String>,
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct StageDescription {
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "MethodSettings")]
    pub method_settings: Option<Vec<MethodSetting>>,
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: Option<String>,
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<usize>,
    #[serde(rename = "CacheClusterEnabled")]
    pub cache_cluster_enabled: Option<bool>,
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "CachingEnabled")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: Option<bool>,
    #[serde(rename = "CanarySetting")]
    pub canary_setting: Option<CanarySetting>,
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    #[serde(rename = "AccessLogSetting")]
    pub access_log_setting: Option<AccessLogSetting>,
    #[serde(rename = "TracingEnabled")]
    pub tracing_enabled: Option<bool>,
    #[serde(rename = "CacheDataEncrypted")]
    pub cache_data_encrypted: Option<bool>,
    #[serde(rename = "Variables")]
    pub variables: Option<()>,
    #[serde(rename = "CacheTtlInSeconds")]
    pub cache_ttl_in_seconds: Option<usize>,
    #[serde(rename = "CacheClusterSize")]
    pub cache_cluster_size: Option<String>,
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentCanarySettings {
    #[serde(rename = "StageVariableOverrides")]
    pub stage_variable_overrides: Option<()>,
    #[serde(rename = "UseStageCache")]
    pub use_stage_cache: Option<bool>,
    #[serde(rename = "PercentTraffic")]
    pub percent_traffic: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessLogSetting {
    #[serde(rename = "Format")]
    pub format: Option<String>,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}


}

pub mod cfn_documentation_part {

#[derive(serde::Serialize, Default)]
pub struct CfnDocumentationPart {
    /// Identifier of the targeted API entity
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// The location of the API entity that the documentation applies to.
    #[serde(rename = "Location")]
    pub location: Location,
    /// The documentation content map of the targeted API entity.
    #[serde(rename = "Properties")]
    pub properties: String,

}


#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Method")]
    pub method: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,

}


}

pub mod cfn_documentation_version {

#[derive(serde::Serialize, Default)]
pub struct CfnDocumentationVersion {
    /// The version identifier of the API documentation snapshot.
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: String,
    /// The identifier of the API.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// The description of the API documentation snapshot.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



}

pub mod cfn_domain_name {

#[derive(serde::Serialize, Default)]
pub struct CfnDomainName {
    /// No documentation provided by AWS
    #[serde(rename = "MutualTlsAuthentication")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "OwnershipVerificationCertificateArn")]
    pub ownership_verification_certificate_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityPolicy")]
    pub security_policy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RegionalCertificateArn")]
    pub regional_certificate_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EndpointConfiguration {
    #[serde(rename = "Types")]
    pub types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct MutualTlsAuthentication {
    #[serde(rename = "TruststoreUri")]
    pub truststore_uri: Option<String>,
    #[serde(rename = "TruststoreVersion")]
    pub truststore_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_gateway_response {

#[derive(serde::Serialize, Default)]
pub struct CfnGatewayResponse {
    /// The response templates for the response.
    #[serde(rename = "ResponseTemplates")]
    pub response_templates: Option<()>,
    /// The identifier of the API.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// The type of the Gateway Response.
    #[serde(rename = "ResponseType")]
    pub response_type: String,
    /// The response parameters (paths, query strings, and headers) for the response.
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,
    /// The HTTP status code for the response.
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,

}



}

pub mod cfn_method {

#[derive(serde::Serialize, Default)]
pub struct CfnMethod {
    /// The ID of the RestApi resource in which API Gateway creates the method.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// The backend system that the method calls when it receives a request.
    #[serde(rename = "Integration")]
    pub integration: Option<Integration>,
    /// The responses that can be sent to the client who calls the method.
    #[serde(rename = "MethodResponses")]
    pub method_responses: Option<Vec<MethodResponse>>,
    /// The backend system that the method calls when it receives a request.
    #[serde(rename = "HttpMethod")]
    pub http_method: String,
    /// The method's authorization type.
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: Option<String>,
    /// A list of authorization scopes configured on the method.
    #[serde(rename = "AuthorizationScopes")]
    pub authorization_scopes: Option<Vec<String>>,
    /// Indicates whether the method requires clients to submit a valid API key.
    #[serde(rename = "ApiKeyRequired")]
    pub api_key_required: Option<bool>,
    /// The identifier of the authorizer to use on this method.
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: Option<String>,
    /// A friendly operation name for the method.
    #[serde(rename = "OperationName")]
    pub operation_name: Option<String>,
    /// The resources that are used for the request's content type. Specify request models as key-value pairs (string-to-string mapping), with a content type as the key and a Model resource name as the value.
    #[serde(rename = "RequestModels")]
    pub request_models: Option<()>,
    /// The request parameters that API Gateway accepts. Specify request parameters as key-value pairs (string-to-Boolean mapping), with a source as the key and a Boolean as the value.
    #[serde(rename = "RequestParameters")]
    pub request_parameters: Option<()>,
    /// The ID of the associated request validator.
    #[serde(rename = "RequestValidatorId")]
    pub request_validator_id: Option<String>,
    /// The ID of an API Gateway resource.
    #[serde(rename = "ResourceId")]
    pub resource_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Integration {
    #[serde(rename = "CacheNamespace")]
    pub cache_namespace: Option<String>,
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<String>,
    #[serde(rename = "CacheKeyParameters")]
    pub cache_key_parameters: Option<Vec<String>>,
    #[serde(rename = "ConnectionId")]
    pub connection_id: Option<String>,
    #[serde(rename = "Credentials")]
    pub credentials: Option<String>,
    #[serde(rename = "RequestParameters")]
    pub request_parameters: Option<()>,
    #[serde(rename = "RequestTemplates")]
    pub request_templates: Option<()>,
    #[serde(rename = "Uri")]
    pub uri: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "PassthroughBehavior")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "IntegrationHttpMethod")]
    pub integration_http_method: Option<String>,
    #[serde(rename = "ContentHandling")]
    pub content_handling: Option<String>,
    #[serde(rename = "TimeoutInMillis")]
    pub timeout_in_millis: Option<usize>,
    #[serde(rename = "IntegrationResponses")]
    pub integration_responses: Option<Vec<IntegrationResponse>>,

}

#[derive(serde::Serialize, Default)]
pub struct MethodResponse {
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "ResponseModels")]
    pub response_models: Option<()>,
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegrationResponse {
    #[serde(rename = "ResponseTemplates")]
    pub response_templates: Option<()>,
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,
    #[serde(rename = "SelectionPattern")]
    pub selection_pattern: Option<String>,
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "ContentHandling")]
    pub content_handling: Option<String>,

}


}

pub mod cfn_model {

#[derive(serde::Serialize, Default)]
pub struct CfnModel {
    /// A name for the model. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the model name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// A description that identifies this model.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The schema to use to transform data to one or more output formats. Specify null ({}) if you don't want to specify a schema.
    #[serde(rename = "Schema")]
    pub schema: Option<()>,
    /// The content type for the model.
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    /// The ID of a REST API with which to associate this model.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,

}



}

pub mod cfn_request_validator {

#[derive(serde::Serialize, Default)]
pub struct CfnRequestValidator {
    /// Indicates whether to validate request parameters.
    #[serde(rename = "ValidateRequestParameters")]
    pub validate_request_parameters: Option<bool>,
    /// The identifier of the targeted API entity.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// Name of the request validator.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Indicates whether to validate the request body according to the configured schema for the targeted API and method.
    #[serde(rename = "ValidateRequestBody")]
    pub validate_request_body: Option<bool>,

}



}

pub mod cfn_resource {

#[derive(serde::Serialize, Default)]
pub struct CfnResource {
    /// The last path segment for this resource.
    #[serde(rename = "PathPart")]
    pub path_part: String,
    /// The ID of the RestApi resource in which you want to create this resource..
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// The parent resource's identifier.
    #[serde(rename = "ParentId")]
    pub parent_id: String,

}



}

pub mod cfn_rest_api {

#[derive(serde::Serialize, Default)]
pub struct CfnRestApi {
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableExecuteApiEndpoint")]
    pub disable_execute_api_endpoint: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "BinaryMediaTypes")]
    pub binary_media_types: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Body")]
    pub body: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiKeySourceType")]
    pub api_key_source_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "MinimumCompressionSize")]
    pub minimum_compression_size: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "BodyS3Location")]
    pub body_s3_location: Option<S3Location>,
    /// No documentation provided by AWS
    #[serde(rename = "FailOnWarnings")]
    pub fail_on_warnings: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "CloneFrom")]
    pub clone_from: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EndpointConfiguration {
    #[serde(rename = "VpcEndpointIds")]
    pub vpc_endpoint_ids: Option<Vec<String>>,
    #[serde(rename = "Types")]
    pub types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

}


}

pub mod cfn_stage {

#[derive(serde::Serialize, Default)]
pub struct CfnStage {
    /// A description of the stage.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The version ID of the API documentation snapshot.
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: Option<String>,
    /// Indicates whether cache clustering is enabled for the stage.
    #[serde(rename = "CacheClusterEnabled")]
    pub cache_cluster_enabled: Option<bool>,
    /// An array of arbitrary tags (key-value pairs) to associate with the stage.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies settings for logging access in this stage.
    #[serde(rename = "AccessLogSetting")]
    pub access_log_setting: Option<AccessLogSetting>,
    /// Specifies whether active X-Ray tracing is enabled for this stage.
    #[serde(rename = "TracingEnabled")]
    pub tracing_enabled: Option<bool>,
    /// The ID of the deployment that the stage is associated with. This parameter is required to create a stage.
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,
    /// The stage's cache cluster size.
    #[serde(rename = "CacheClusterSize")]
    pub cache_cluster_size: Option<String>,
    /// Settings for all methods in the stage.
    #[serde(rename = "MethodSettings")]
    pub method_settings: Option<Vec<MethodSetting>>,
    /// The name of the stage, which API Gateway uses as the first path segment in the invoked Uniform Resource Identifier (URI).
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,
    /// The ID of the client certificate that API Gateway uses to call your integration endpoints in the stage.
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,
    /// The ID of the RestApi resource that you're deploying with this stage.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,
    /// A map (string-to-string map) that defines the stage variables, where the variable name is the key and the variable value is the value.
    #[serde(rename = "Variables")]
    pub variables: Option<()>,
    /// Specifies settings for the canary deployment in this stage.
    #[serde(rename = "CanarySetting")]
    pub canary_setting: Option<CanarySetting>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct CanarySetting {
    #[serde(rename = "UseStageCache")]
    pub use_stage_cache: Option<bool>,
    #[serde(rename = "PercentTraffic")]
    pub percent_traffic: Option<f64>,
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,
    #[serde(rename = "StageVariableOverrides")]
    pub stage_variable_overrides: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessLogSetting {
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,
    #[serde(rename = "Format")]
    pub format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MethodSetting {
    #[serde(rename = "ResourcePath")]
    pub resource_path: Option<String>,
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<usize>,
    #[serde(rename = "CachingEnabled")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: Option<bool>,
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    #[serde(rename = "CacheTtlInSeconds")]
    pub cache_ttl_in_seconds: Option<usize>,
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "HttpMethod")]
    pub http_method: Option<String>,
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "CacheDataEncrypted")]
    pub cache_data_encrypted: Option<bool>,

}


}

pub mod cfn_usage_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnUsagePlan {
    /// Configures the number of requests that users can make within a given interval.
    #[serde(rename = "Quota")]
    pub quota: Option<QuotaSettings>,
    /// Configures the overall request rate (average requests per second) and burst capacity.
    #[serde(rename = "Throttle")]
    pub throttle: Option<ThrottleSettings>,
    /// A name for the usage plan.
    #[serde(rename = "UsagePlanName")]
    pub usage_plan_name: Option<String>,
    /// An array of arbitrary tags (key-value pairs) to associate with the usage plan.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The API stages to associate with this usage plan.
    #[serde(rename = "ApiStages")]
    pub api_stages: Option<Vec<ApiStage>>,
    /// A description of the usage plan.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ThrottleSettings {
    #[serde(rename = "RateLimit")]
    pub rate_limit: Option<f64>,
    #[serde(rename = "BurstLimit")]
    pub burst_limit: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ApiStage {
    #[serde(rename = "ApiId")]
    pub api_id: Option<String>,
    #[serde(rename = "Stage")]
    pub stage: Option<String>,
    #[serde(rename = "Throttle")]
    pub throttle: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct QuotaSettings {
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "Offset")]
    pub offset: Option<usize>,
    #[serde(rename = "Period")]
    pub period: Option<String>,

}


}

pub mod cfn_usage_plan_key {

#[derive(serde::Serialize, Default)]
pub struct CfnUsagePlanKey {
    /// The ID of the usage plan key.
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// The type of usage plan key. Currently, the only valid key type is API_KEY.
    #[serde(rename = "KeyType")]
    pub key_type: String,
    /// The ID of the usage plan.
    #[serde(rename = "UsagePlanId")]
    pub usage_plan_id: String,

}



}

pub mod cfn_vpc_link {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcLink {
    /// A name for the VPC link.
    #[serde(rename = "Name")]
    pub name: String,
    /// An array of arbitrary tags (key-value pairs) to associate with the stage.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ARN of network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same AWS account of the API owner.
    #[serde(rename = "TargetArns")]
    pub target_arns: Vec<String>,
    /// A description of the VPC link.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
