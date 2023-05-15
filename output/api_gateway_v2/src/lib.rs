
pub mod cfn_api {

#[derive(serde::Serialize, Default)]
pub struct CfnApi {
    /// No documentation provided by AWS
    #[serde(rename = "FailOnWarnings")]
    pub fail_on_warnings: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BodyS3Location")]
    pub body_s3_location: Option<BodyS3Location>,
    /// No documentation provided by AWS
    #[serde(rename = "Target")]
    pub target: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Body")]
    pub body: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteKey")]
    pub route_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiKeySelectionExpression")]
    pub api_key_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableSchemaValidation")]
    pub disable_schema_validation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableExecuteApiEndpoint")]
    pub disable_execute_api_endpoint: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<String>,
    /// This resource type use map for Tags, suggest to use List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteSelectionExpression")]
    pub route_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CredentialsArn")]
    pub credentials_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CorsConfiguration")]
    pub cors_configuration: Option<Cors>,

}


#[derive(serde::Serialize, Default)]
pub struct Cors {
    #[serde(rename = "AllowOrigins")]
    pub allow_origins: Option<Vec<String>>,
    #[serde(rename = "AllowCredentials")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "AllowHeaders")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge")]
    pub max_age: Option<usize>,
    #[serde(rename = "AllowMethods")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct BodyS3Location {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Etag")]
    pub etag: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


}

pub mod cfn_api_gateway_managed_overrides {

#[derive(serde::Serialize, Default)]
pub struct CfnApiGatewayManagedOverrides {
    /// No documentation provided by AWS
    #[serde(rename = "Integration")]
    pub integration: Option<IntegrationOverrides>,
    /// No documentation provided by AWS
    #[serde(rename = "Stage")]
    pub stage: Option<StageOverrides>,
    /// No documentation provided by AWS
    #[serde(rename = "Route")]
    pub route: Option<RouteOverrides>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct RouteSettings {
    #[serde(rename = "DetailedMetricsEnabled")]
    pub detailed_metrics_enabled: Option<bool>,
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<usize>,
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessLogSettings {
    #[serde(rename = "Format")]
    pub format: Option<String>,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RouteOverrides {
    #[serde(rename = "Target")]
    pub target: Option<String>,
    #[serde(rename = "AuthorizationScopes")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "OperationName")]
    pub operation_name: Option<String>,
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StageOverrides {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "StageVariables")]
    pub stage_variables: Option<()>,
    #[serde(rename = "RouteSettings")]
    pub route_settings: Option<()>,
    #[serde(rename = "DefaultRouteSettings")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "AccessLogSettings")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "AutoDeploy")]
    pub auto_deploy: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegrationOverrides {
    #[serde(rename = "TimeoutInMillis")]
    pub timeout_in_millis: Option<usize>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "PayloadFormatVersion")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "IntegrationMethod")]
    pub integration_method: Option<String>,

}


}

pub mod cfn_api_mapping {

#[derive(serde::Serialize, Default)]
pub struct CfnApiMapping {
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiMappingKey")]
    pub api_mapping_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Stage")]
    pub stage: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,

}



}

pub mod cfn_authorizer {

#[derive(serde::Serialize, Default)]
pub struct CfnAuthorizer {
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerCredentialsArn")]
    pub authorizer_credentials_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IdentitySource")]
    pub identity_source: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerType")]
    pub authorizer_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "JwtConfiguration")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "IdentityValidationExpression")]
    pub identity_validation_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerUri")]
    pub authorizer_uri: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerPayloadFormatVersion")]
    pub authorizer_payload_format_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableSimpleResponses")]
    pub enable_simple_responses: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct JWTConfiguration {
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,
    #[serde(rename = "Audience")]
    pub audience: Option<Vec<String>>,

}


}

pub mod cfn_deployment {

#[derive(serde::Serialize, Default)]
pub struct CfnDeployment {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,

}



}

pub mod cfn_domain_name {

#[derive(serde::Serialize, Default)]
pub struct CfnDomainName {
    /// List of DomainNameConfiguration
    #[serde(rename = "DomainNameConfigurations")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "MutualTlsAuthentication")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,

}


#[derive(serde::Serialize, Default)]
pub struct DomainNameConfiguration {
    #[serde(rename = "CertificateName")]
    pub certificate_name: Option<String>,
    #[serde(rename = "SecurityPolicy")]
    pub security_policy: Option<String>,
    #[serde(rename = "OwnershipVerificationCertificateArn")]
    pub ownership_verification_certificate_arn: Option<String>,
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MutualTlsAuthentication {
    #[serde(rename = "TruststoreVersion")]
    pub truststore_version: Option<String>,
    #[serde(rename = "TruststoreUri")]
    pub truststore_uri: Option<String>,

}


}

pub mod cfn_integration {

#[derive(serde::Serialize, Default)]
pub struct CfnIntegration {
    /// No documentation provided by AWS
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "IntegrationMethod")]
    pub integration_method: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IntegrationUri")]
    pub integration_uri: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionId")]
    pub connection_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PayloadFormatVersion")]
    pub payload_format_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestTemplates")]
    pub request_templates: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeoutInMillis")]
    pub timeout_in_millis: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "CredentialsArn")]
    pub credentials_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestParameters")]
    pub request_parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateSelectionExpression")]
    pub template_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PassthroughBehavior")]
    pub passthrough_behavior: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IntegrationType")]
    pub integration_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ContentHandlingStrategy")]
    pub content_handling_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IntegrationSubtype")]
    pub integration_subtype: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TlsConfig")]
    pub tls_config: Option<TlsConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct TlsConfig {
    #[serde(rename = "ServerNameToVerify")]
    pub server_name_to_verify: Option<String>,

}


}

pub mod cfn_integration_response {

#[derive(serde::Serialize, Default)]
pub struct CfnIntegrationResponse {
    /// The collection of response templates for the integration response as a string-to-string map of key-value pairs
    #[serde(rename = "ResponseTemplates")]
    pub response_templates: Option<()>,
    /// The integration response key
    #[serde(rename = "IntegrationResponseKey")]
    pub integration_response_key: String,
    /// A key-value map specifying response parameters that are passed to the method response from the backend
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,
    /// The template selection expression for the integration response. Supported only for WebSocket APIs
    #[serde(rename = "TemplateSelectionExpression")]
    pub template_selection_expression: Option<String>,
    /// Specifies how to handle response payload content type conversions
    #[serde(rename = "ContentHandlingStrategy")]
    pub content_handling_strategy: Option<String>,
    /// The integration ID
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// The API identifier
    #[serde(rename = "ApiId")]
    pub api_id: String,

}



}

pub mod cfn_model {

#[derive(serde::Serialize, Default)]
pub struct CfnModel {
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Schema")]
    pub schema: (),

}



}

pub mod cfn_route {

#[derive(serde::Serialize, Default)]
pub struct CfnRoute {
    /// No documentation provided by AWS
    #[serde(rename = "RequestModels")]
    pub request_models: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteResponseSelectionExpression")]
    pub route_response_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestParameters")]
    pub request_parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelSelectionExpression")]
    pub model_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OperationName")]
    pub operation_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Target")]
    pub target: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizationScopes")]
    pub authorization_scopes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteKey")]
    pub route_key: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiKeyRequired")]
    pub api_key_required: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct ParameterConstraints {
    #[serde(rename = "Required")]
    pub required: Option<bool>,

}


}

pub mod cfn_route_response {

#[derive(serde::Serialize, Default)]
pub struct CfnRouteResponse {
    /// No documentation provided by AWS
    #[serde(rename = "ModelSelectionExpression")]
    pub model_selection_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteResponseKey")]
    pub route_response_key: String,
    /// No documentation provided by AWS
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ResponseModels")]
    pub response_models: Option<()>,

}



}

pub mod cfn_stage {

#[derive(serde::Serialize, Default)]
pub struct CfnStage {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessPolicyId")]
    pub access_policy_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteSettings")]
    pub route_settings: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRouteSettings")]
    pub default_route_settings: Option<RouteSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoDeploy")]
    pub auto_deploy: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessLogSettings")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "StageVariables")]
    pub stage_variables: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct AccessLogSettings {
    #[serde(rename = "Format")]
    pub format: Option<String>,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RouteSettings {
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<usize>,
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "DetailedMetricsEnabled")]
    pub detailed_metrics_enabled: Option<bool>,
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,

}


}

pub mod cfn_vpc_link {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcLink {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// This resource type use map for Tags, suggest to use List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}



}
