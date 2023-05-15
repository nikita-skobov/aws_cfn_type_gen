
pub mod cfn_api_cache {

#[derive(serde::Serialize, Default)]
pub struct CfnApiCache {
    /// No documentation provided by AWS
    #[serde(rename = "TransitEncryptionEnabled")]
    pub transit_encryption_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiCachingBehavior")]
    pub api_caching_behavior: String,
    /// No documentation provided by AWS
    #[serde(rename = "Ttl")]
    pub ttl: f64,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "AtRestEncryptionEnabled")]
    pub at_rest_encryption_enabled: Option<bool>,

}



}

pub mod cfn_api_key {

#[derive(serde::Serialize, Default)]
pub struct CfnApiKey {
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Expires")]
    pub expires: Option<f64>,

}



}

pub mod cfn_data_source {

#[derive(serde::Serialize, Default)]
pub struct CfnDataSource {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "LambdaConfig")]
    pub lambda_config: Option<LambdaConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "DynamoDBConfig")]
    pub dynamo_dbconfig: Option<DynamoDBConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "EventBridgeConfig")]
    pub event_bridge_config: Option<EventBridgeConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HttpConfig")]
    pub http_config: Option<HttpConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "RelationalDatabaseConfig")]
    pub relational_database_config: Option<RelationalDatabaseConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "OpenSearchServiceConfig")]
    pub open_search_service_config: Option<OpenSearchServiceConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticsearchConfig")]
    pub elasticsearch_config: Option<ElasticsearchConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct RdsHttpEndpointConfig {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    #[serde(rename = "Schema")]
    pub schema: Option<String>,
    #[serde(rename = "DbClusterIdentifier")]
    pub db_cluster_identifier: String,
    #[serde(rename = "AwsSecretStoreArn")]
    pub aws_secret_store_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct HttpConfig {
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    #[serde(rename = "AuthorizationConfig")]
    pub authorization_config: Option<AuthorizationConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct EventBridgeConfig {
    #[serde(rename = "EventBusArn")]
    pub event_bus_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct AuthorizationConfig {
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: String,
    #[serde(rename = "AwsIamConfig")]
    pub aws_iam_config: Option<AwsIamConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConfig {
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeltaSyncConfig {
    #[serde(rename = "BaseTableTTL")]
    pub base_table_ttl: String,
    #[serde(rename = "DeltaSyncTableTTL")]
    pub delta_sync_table_ttl: String,
    #[serde(rename = "DeltaSyncTableName")]
    pub delta_sync_table_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct OpenSearchServiceConfig {
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}

#[derive(serde::Serialize, Default)]
pub struct RelationalDatabaseConfig {
    #[serde(rename = "RelationalDatabaseSourceType")]
    pub relational_database_source_type: String,
    #[serde(rename = "RdsHttpEndpointConfig")]
    pub rds_http_endpoint_config: Option<RdsHttpEndpointConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchConfig {
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBConfig {
    #[serde(rename = "Versioned")]
    pub versioned: Option<bool>,
    #[serde(rename = "DeltaSyncConfig")]
    pub delta_sync_config: Option<DeltaSyncConfig>,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    #[serde(rename = "UseCallerCredentials")]
    pub use_caller_credentials: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsIamConfig {
    #[serde(rename = "SigningRegion")]
    pub signing_region: Option<String>,
    #[serde(rename = "SigningServiceName")]
    pub signing_service_name: Option<String>,

}


}

pub mod cfn_domain_name {

#[derive(serde::Serialize, Default)]
pub struct CfnDomainName {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}



}

pub mod cfn_domain_name_api_association {

#[derive(serde::Serialize, Default)]
pub struct CfnDomainNameApiAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,

}



}

pub mod cfn_function_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnFunctionConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "ResponseMappingTemplate")]
    pub response_mapping_template: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionVersion")]
    pub function_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Runtime")]
    pub runtime: Option<AppSyncRuntime>,
    /// No documentation provided by AWS
    #[serde(rename = "Code")]
    pub code: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestMappingTemplate")]
    pub request_mapping_template: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CodeS3Location")]
    pub code_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestMappingTemplateS3Location")]
    pub request_mapping_template_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncConfig")]
    pub sync_config: Option<SyncConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DataSourceName")]
    pub data_source_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ResponseMappingTemplateS3Location")]
    pub response_mapping_template_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxBatchSize")]
    pub max_batch_size: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct SyncConfig {
    #[serde(rename = "ConflictHandler")]
    pub conflict_handler: Option<String>,
    #[serde(rename = "LambdaConflictHandlerConfig")]
    pub lambda_conflict_handler_config: Option<LambdaConflictHandlerConfig>,
    #[serde(rename = "ConflictDetection")]
    pub conflict_detection: String,

}

#[derive(serde::Serialize, Default)]
pub struct AppSyncRuntime {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RuntimeVersion")]
    pub runtime_version: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConflictHandlerConfig {
    #[serde(rename = "LambdaConflictHandlerArn")]
    pub lambda_conflict_handler_arn: Option<String>,

}


}

pub mod cfn_graph_qlapi {

#[derive(serde::Serialize, Default)]
pub struct CfnGraphQLApi {
    /// No documentation provided by AWS
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolConfig")]
    pub user_pool_config: Option<UserPoolConfig>,
    /// List of AdditionalAuthenticationProvider
    #[serde(rename = "AdditionalAuthenticationProviders")]
    pub additional_authentication_providers: Option<Vec<AdditionalAuthenticationProvider>>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "LambdaAuthorizerConfig")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "OpenIDConnectConfig")]
    pub open_idconnect_config: Option<OpenIDConnectConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MergedApiExecutionRoleArn")]
    pub merged_api_execution_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "XrayEnabled")]
    pub xray_enabled: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "LogConfig")]
    pub log_config: Option<LogConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiType")]
    pub api_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OwnerContact")]
    pub owner_contact: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct UserPoolConfig {
    #[serde(rename = "AwsRegion")]
    pub aws_region: Option<String>,
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "AppIdClientRegex")]
    pub app_id_client_regex: Option<String>,
    #[serde(rename = "DefaultAction")]
    pub default_action: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalAuthenticationProvider {
    #[serde(rename = "OpenIDConnectConfig")]
    pub open_idconnect_config: Option<OpenIDConnectConfig>,
    #[serde(rename = "LambdaAuthorizerConfig")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "UserPoolConfig")]
    pub user_pool_config: Option<CognitoUserPoolConfig>,
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogConfig {
    #[serde(rename = "CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "ExcludeVerboseContent")]
    pub exclude_verbose_content: Option<bool>,
    #[serde(rename = "FieldLogLevel")]
    pub field_log_level: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OpenIDConnectConfig {
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,
    #[serde(rename = "AuthTTL")]
    pub auth_ttl: Option<f64>,
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,
    #[serde(rename = "IatTTL")]
    pub iat_ttl: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct CognitoUserPoolConfig {
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    pub aws_region: Option<String>,
    #[serde(rename = "AppIdClientRegex")]
    pub app_id_client_regex: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaAuthorizerConfig {
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: Option<f64>,
    #[serde(rename = "AuthorizerUri")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "IdentityValidationExpression")]
    pub identity_validation_expression: Option<String>,

}


}

pub mod cfn_graph_qlschema {

#[derive(serde::Serialize, Default)]
pub struct CfnGraphQLSchema {
    /// No documentation provided by AWS
    #[serde(rename = "DefinitionS3Location")]
    pub definition_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Option<String>,

}



}

pub mod cfn_resolver {

#[derive(serde::Serialize, Default)]
pub struct CfnResolver {
    /// No documentation provided by AWS
    #[serde(rename = "ResponseMappingTemplate")]
    pub response_mapping_template: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Kind")]
    pub kind: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestMappingTemplateS3Location")]
    pub request_mapping_template_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncConfig")]
    pub sync_config: Option<SyncConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Runtime")]
    pub runtime: Option<AppSyncRuntime>,
    /// No documentation provided by AWS
    #[serde(rename = "ResponseMappingTemplateS3Location")]
    pub response_mapping_template_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestMappingTemplate")]
    pub request_mapping_template: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PipelineConfig")]
    pub pipeline_config: Option<PipelineConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "DataSourceName")]
    pub data_source_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TypeName")]
    pub type_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "CachingConfig")]
    pub caching_config: Option<CachingConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxBatchSize")]
    pub max_batch_size: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Code")]
    pub code: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CodeS3Location")]
    pub code_s3_location: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FieldName")]
    pub field_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct PipelineConfig {
    #[serde(rename = "Functions")]
    pub functions: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct SyncConfig {
    #[serde(rename = "LambdaConflictHandlerConfig")]
    pub lambda_conflict_handler_config: Option<LambdaConflictHandlerConfig>,
    #[serde(rename = "ConflictHandler")]
    pub conflict_handler: Option<String>,
    #[serde(rename = "ConflictDetection")]
    pub conflict_detection: String,

}

#[derive(serde::Serialize, Default)]
pub struct AppSyncRuntime {
    #[serde(rename = "RuntimeVersion")]
    pub runtime_version: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConflictHandlerConfig {
    #[serde(rename = "LambdaConflictHandlerArn")]
    pub lambda_conflict_handler_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CachingConfig {
    #[serde(rename = "CachingKeys")]
    pub caching_keys: Option<Vec<String>>,
    #[serde(rename = "Ttl")]
    pub ttl: f64,

}


}
