
pub mod cfn_connector {

#[derive(serde::Serialize, Default)]
pub struct CfnConnector {
    /// A description about the connector that's being registered.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The provisioning type of the connector. Currently the only supported value is LAMBDA.
    #[serde(rename = "ConnectorProvisioningType")]
    pub connector_provisioning_type: String,
    /// The name of the connector. The name is unique for each ConnectorRegistration in your AWS account.
    #[serde(rename = "ConnectorLabel")]
    pub connector_label: Option<String>,
    /// Contains information about the configuration of the connector being registered.
    #[serde(rename = "ConnectorProvisioningConfig")]
    pub connector_provisioning_config: ConnectorProvisioningConfig,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectorProvisioningConfig {
    #[serde(rename = "Lambda")]
    pub lambda: Option<LambdaConnectorProvisioningConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConnectorProvisioningConfig {
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: String,

}


}

pub mod cfn_connector_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnConnectorProfile {
    /// Mode in which data transfer should be enabled. Private connection mode is currently enabled for Salesforce, Snowflake, Trendmicro and Singular
    #[serde(rename = "ConnectionMode")]
    pub connection_mode: String,
    /// List of Saas providers that need connector profile to be created
    #[serde(rename = "ConnectorType")]
    pub connector_type: ConnectorType,
    /// Connector specific configurations needed to create connector profile
    #[serde(rename = "ConnectorProfileConfig")]
    pub connector_profile_config: Option<ConnectorProfileConfig>,
    /// The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for CUSTOMCONNECTOR connector type/.
    #[serde(rename = "ConnectorLabel")]
    pub connector_label: Option<String>,
    /// The maximum number of items to retrieve in a single batch.
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: String,
    /// The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.
    #[serde(rename = "KMSArn")]
    pub kmsarn: Option<String>,

}

pub type OAuth2GrantType = String;pub type Key = String;
#[derive(serde::Serialize, Default)]
pub struct RedshiftConnectorProfileCredentials {
    #[serde(rename = "Username")]
    pub username: Option<Username>,
    #[serde(rename = "Password")]
    pub password: Option<Password>,

}
pub type ClientId = String;pub type DataApiRoleArn = String;pub type ClusterIdentifier = String;pub type DatabaseName = String;
#[derive(serde::Serialize, Default)]
pub struct ConnectorOAuthRequest {
    #[serde(rename = "AuthCode")]
    pub auth_code: Option<String>,
    #[serde(rename = "RedirectUri")]
    pub redirect_uri: Option<String>,

}
pub type PortNumber = usize;
#[derive(serde::Serialize, Default)]
pub struct ZendeskConnectorProfileCredentials {
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "ClientId")]
    pub client_id: ClientId,
    #[serde(rename = "ClientSecret")]
    pub client_secret: ClientSecret,
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,

}
pub type ApplicationHostUrl = String;
#[derive(serde::Serialize, Default)]
pub struct SingularConnectorProfileCredentials {
    #[serde(rename = "ApiKey")]
    pub api_key: ApiKey,

}

#[derive(serde::Serialize, Default)]
pub struct ProfileProperties {

}

#[derive(serde::Serialize, Default)]
pub struct ConnectorProfileProperties {
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaConnectorProfileProperties>,
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorProfileProperties>,
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotConnectorProfileProperties>,
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskConnectorProfileProperties>,
    #[serde(rename = "Slack")]
    pub slack: Option<SlackConnectorProfileProperties>,
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceConnectorProfileProperties>,
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftConnectorProfileProperties>,
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowConnectorProfileProperties>,
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusConnectorProfileProperties>,
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogConnectorProfileProperties>,
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoConnectorProfileProperties>,
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataConnectorProfileProperties>,
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceConnectorProfileProperties>,
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeConnectorProfileProperties>,

}
pub type AccessToken = String;pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct ConnectorProfileCredentials {
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceConnectorProfileCredentials>,
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeConnectorProfileCredentials>,
    #[serde(rename = "Slack")]
    pub slack: Option<SlackConnectorProfileCredentials>,
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataConnectorProfileCredentials>,
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotConnectorProfileCredentials>,
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoConnectorProfileCredentials>,
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaConnectorProfileCredentials>,
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorProfileCredentials>,
    #[serde(rename = "Singular")]
    pub singular: Option<SingularConnectorProfileCredentials>,
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowConnectorProfileCredentials>,
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroConnectorProfileCredentials>,
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskConnectorProfileCredentials>,
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogConnectorProfileCredentials>,
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusConnectorProfileCredentials>,
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceConnectorProfileCredentials>,
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsConnectorProfileCredentials>,
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftConnectorProfileCredentials>,
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeConnectorProfileCredentials>,

}
pub type WorkgroupName = String;pub type AuthenticationType = String;pub type ConnectorType = String;
#[derive(serde::Serialize, Default)]
pub struct AmplitudeConnectorProfileCredentials {
    #[serde(rename = "ApiKey")]
    pub api_key: ApiKey,
    #[serde(rename = "SecretKey")]
    pub secret_key: SecretKey,

}
pub type InstanceUrl = String;pub type ClientSecret = String;pub type PrivateLinkServiceName = String;pub type AccessKeyId = String;
#[derive(serde::Serialize, Default)]
pub struct SnowflakeConnectorProfileCredentials {
    #[serde(rename = "Password")]
    pub password: Password,
    #[serde(rename = "Username")]
    pub username: Username,

}

#[derive(serde::Serialize, Default)]
pub struct PardotConnectorProfileCredentials {
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,
    #[serde(rename = "ClientCredentialsArn")]
    pub client_credentials_arn: Option<ClientCredentialsArn>,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<RefreshToken>,

}
pub type Username = String;
#[derive(serde::Serialize, Default)]
pub struct BasicAuthCredentials {
    #[serde(rename = "Username")]
    pub username: Username,
    #[serde(rename = "Password")]
    pub password: Password,

}
pub type Stage = String;pub type BusinessUnitId = String;pub type LogonLanguage = String;
#[derive(serde::Serialize, Default)]
pub struct DatadogConnectorProfileCredentials {
    #[serde(rename = "ApiKey")]
    pub api_key: ApiKey,
    #[serde(rename = "ApplicationKey")]
    pub application_key: ApplicationKey,

}

#[derive(serde::Serialize, Default)]
pub struct MarketoConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}
pub type ApiKey = String;pub type Warehouse = String;
#[derive(serde::Serialize, Default)]
pub struct RedshiftConnectorProfileProperties {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<DatabaseName>,
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "DataApiRoleArn")]
    pub data_api_role_arn: Option<DataApiRoleArn>,
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: Option<WorkgroupName>,
    #[serde(rename = "DatabaseUrl")]
    pub database_url: Option<DatabaseUrl>,
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,
    #[serde(rename = "IsRedshiftServerless")]
    pub is_redshift_serverless: Option<bool>,
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: Option<ClusterIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomAuthCredentials {
    #[serde(rename = "CustomAuthenticationType")]
    pub custom_authentication_type: CustomAuthenticationType,
    #[serde(rename = "CredentialsMap")]
    pub credentials_map: Option<CredentialsMap>,

}
pub type DatabaseUrl = String;pub type AccountName = String;pub type Region = String;pub type Password = String;pub type AuthCode = String;
#[derive(serde::Serialize, Default)]
pub struct SAPODataConnectorProfileCredentials {
    #[serde(rename = "OAuthCredentials")]
    pub oauth_credentials: Option<()>,
    #[serde(rename = "BasicAuthCredentials")]
    pub basic_auth_credentials: Option<BasicAuthCredentials>,

}

#[derive(serde::Serialize, Default)]
pub struct SlackConnectorProfileCredentials {
    #[serde(rename = "ClientSecret")]
    pub client_secret: ClientSecret,
    #[serde(rename = "ClientId")]
    pub client_id: ClientId,
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,

}

#[derive(serde::Serialize, Default)]
pub struct GoogleAnalyticsConnectorProfileCredentials {
    #[serde(rename = "ClientId")]
    pub client_id: ClientId,
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,
    #[serde(rename = "ClientSecret")]
    pub client_secret: ClientSecret,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<RefreshToken>,
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

}
pub type ApiToken = String;
#[derive(serde::Serialize, Default)]
pub struct OAuthProperties {
    #[serde(rename = "OAuthScopes")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "AuthCodeUrl")]
    pub auth_code_url: Option<String>,
    #[serde(rename = "TokenUrl")]
    pub token_url: Option<String>,

}
pub type ApplicationKey = String;
#[derive(serde::Serialize, Default)]
pub struct ConnectorProfileConfig {
    #[serde(rename = "ConnectorProfileProperties")]
    pub connector_profile_properties: Option<ConnectorProfileProperties>,
    #[serde(rename = "ConnectorProfileCredentials")]
    pub connector_profile_credentials: Option<ConnectorProfileCredentials>,

}

#[derive(serde::Serialize, Default)]
pub struct TokenUrlCustomProperties {

}
pub type SecretKey = String;pub type ApplicationServicePath = String;
#[derive(serde::Serialize, Default)]
pub struct ApiKeyCredentials {
    #[serde(rename = "ApiKey")]
    pub api_key: ApiKey,
    #[serde(rename = "ApiSecretKey")]
    pub api_secret_key: Option<ApiSecretKey>,

}

#[derive(serde::Serialize, Default)]
pub struct InforNexusConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}

#[derive(serde::Serialize, Default)]
pub struct CustomConnectorProfileProperties {
    #[serde(rename = "OAuth2Properties")]
    pub oauth2_properties: Option<OAuth2Properties>,
    #[serde(rename = "ProfileProperties")]
    pub profile_properties: Option<ProfileProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct OAuth2Properties {
    #[serde(rename = "TokenUrlCustomProperties")]
    pub token_url_custom_properties: Option<TokenUrlCustomProperties>,
    #[serde(rename = "TokenUrl")]
    pub token_url: Option<String>,
    #[serde(rename = "OAuth2GrantType")]
    pub oauth2_grant_type: Option<OAuth2GrantType>,

}

#[derive(serde::Serialize, Default)]
pub struct TrendmicroConnectorProfileCredentials {
    #[serde(rename = "ApiSecretKey")]
    pub api_secret_key: ApiSecretKey,

}
pub type ClientCredentialsArn = String;
#[derive(serde::Serialize, Default)]
pub struct ServiceNowConnectorProfileCredentials {
    #[serde(rename = "Username")]
    pub username: Username,
    #[serde(rename = "Password")]
    pub password: Password,

}

#[derive(serde::Serialize, Default)]
pub struct SnowflakeConnectorProfileProperties {
    #[serde(rename = "AccountName")]
    pub account_name: Option<AccountName>,
    #[serde(rename = "Stage")]
    pub stage: Stage,
    #[serde(rename = "Warehouse")]
    pub warehouse: Warehouse,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "PrivateLinkServiceName")]
    pub private_link_service_name: Option<PrivateLinkServiceName>,
    #[serde(rename = "Region")]
    pub region: Option<Region>,
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,

}

#[derive(serde::Serialize, Default)]
pub struct VeevaConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}
pub type ClientNumber = String;
#[derive(serde::Serialize, Default)]
pub struct DynatraceConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}

#[derive(serde::Serialize, Default)]
pub struct SAPODataConnectorProfileProperties {
    #[serde(rename = "PortNumber")]
    pub port_number: Option<PortNumber>,
    #[serde(rename = "ClientNumber")]
    pub client_number: Option<ClientNumber>,
    #[serde(rename = "LogonLanguage")]
    pub logon_language: Option<LogonLanguage>,
    #[serde(rename = "OAuthProperties")]
    pub oauth_properties: Option<OAuthProperties>,
    #[serde(rename = "ApplicationHostUrl")]
    pub application_host_url: Option<ApplicationHostUrl>,
    #[serde(rename = "ApplicationServicePath")]
    pub application_service_path: Option<ApplicationServicePath>,
    #[serde(rename = "PrivateLinkServiceName")]
    pub private_link_service_name: Option<PrivateLinkServiceName>,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: Option<InstanceUrl>,
    #[serde(rename = "isSandboxEnvironment")]
    pub is_sandbox_environment: Option<bool>,
    #[serde(rename = "usePrivateLinkForMetadataAndAuthorization")]
    pub use_private_link_for_metadata_and_authorization: Option<bool>,

}
pub type CustomAuthenticationType = String;
#[derive(serde::Serialize, Default)]
pub struct MarketoConnectorProfileCredentials {
    #[serde(rename = "ClientId")]
    pub client_id: ClientId,
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "ClientSecret")]
    pub client_secret: ClientSecret,
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,

}

#[derive(serde::Serialize, Default)]
pub struct DynatraceConnectorProfileCredentials {
    #[serde(rename = "ApiToken")]
    pub api_token: ApiToken,

}
pub type ApiSecretKey = String;pub type RefreshToken = String;
#[derive(serde::Serialize, Default)]
pub struct PardotConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: Option<InstanceUrl>,
    #[serde(rename = "IsSandboxEnvironment")]
    pub is_sandbox_environment: Option<bool>,
    #[serde(rename = "BusinessUnitId")]
    pub business_unit_id: BusinessUnitId,

}

#[derive(serde::Serialize, Default)]
pub struct InforNexusConnectorProfileCredentials {
    #[serde(rename = "Datakey")]
    pub datakey: Key,
    #[serde(rename = "SecretAccessKey")]
    pub secret_access_key: Key,
    #[serde(rename = "UserId")]
    pub user_id: Username,
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: AccessKeyId,

}

#[derive(serde::Serialize, Default)]
pub struct SlackConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}
pub type BucketPrefix = String;
#[derive(serde::Serialize, Default)]
pub struct SalesforceConnectorProfileCredentials {
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,
    #[serde(rename = "ClientCredentialsArn")]
    pub client_credentials_arn: Option<ClientCredentialsArn>,
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<RefreshToken>,

}

#[derive(serde::Serialize, Default)]
pub struct ZendeskConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}

#[derive(serde::Serialize, Default)]
pub struct OAuth2Credentials {
    #[serde(rename = "AccessToken")]
    pub access_token: Option<AccessToken>,
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<ClientSecret>,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<RefreshToken>,
    #[serde(rename = "OAuthRequest")]
    pub oauth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "ClientId")]
    pub client_id: Option<ClientId>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomConnectorProfileCredentials {
    #[serde(rename = "ApiKey")]
    pub api_key: Option<ApiKeyCredentials>,
    #[serde(rename = "Oauth2")]
    pub oauth2: Option<OAuth2Credentials>,
    #[serde(rename = "Basic")]
    pub basic: Option<BasicAuthCredentials>,
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: AuthenticationType,
    #[serde(rename = "Custom")]
    pub custom: Option<CustomAuthCredentials>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceNowConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}

#[derive(serde::Serialize, Default)]
pub struct CredentialsMap {

}

#[derive(serde::Serialize, Default)]
pub struct VeevaConnectorProfileCredentials {
    #[serde(rename = "Username")]
    pub username: Username,
    #[serde(rename = "Password")]
    pub password: Password,

}
pub type BucketName = String;
#[derive(serde::Serialize, Default)]
pub struct DatadogConnectorProfileProperties {
    #[serde(rename = "InstanceUrl")]
    pub instance_url: InstanceUrl,

}


}

pub mod cfn_flow {

#[derive(serde::Serialize, Default)]
pub struct CfnFlow {
    /// Description of the flow.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Tags.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Name of the flow.
    #[serde(rename = "FlowName")]
    pub flow_name: String,
    /// List of tasks for the flow.
    #[serde(rename = "Tasks")]
    pub tasks: Vec<Task>,
    /// Configurations of metadata catalog of the flow.
    #[serde(rename = "MetadataCatalogConfig")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,
    /// Trigger settings of the flow.
    #[serde(rename = "TriggerConfig")]
    pub trigger_config: TriggerConfig,
    /// Configurations of Source connector of the flow.
    #[serde(rename = "SourceFlowConfig")]
    pub source_flow_config: SourceFlowConfig,
    /// The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.
    #[serde(rename = "KMSArn")]
    pub kmsarn: Option<String>,
    /// List of Destination connectors of the flow.
    #[serde(rename = "DestinationFlowConfigList")]
    pub destination_flow_config_list: Vec<DestinationFlowConfig>,

}

pub type GoogleAnalyticsConnectorOperator = String;pub type SlackConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct PathPrefixHierarchy {

}

#[derive(serde::Serialize, Default)]
pub struct DynatraceSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type IncludeAllVersions = bool;
#[derive(serde::Serialize, Default)]
pub struct ScheduledTriggerProperties {
    #[serde(rename = "ScheduleStartTime")]
    pub schedule_start_time: Option<f64>,
    #[serde(rename = "DataPullMode")]
    pub data_pull_mode: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<f64>,
    #[serde(rename = "ScheduleEndTime")]
    pub schedule_end_time: Option<f64>,
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename = "FlowErrorDeactivationThreshold")]
    pub flow_error_deactivation_threshold: Option<usize>,
    #[serde(rename = "FirstExecutionFrom")]
    pub first_execution_from: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct S3InputFormatConfig {
    #[serde(rename = "S3InputFileType")]
    pub s3_input_file_type: Option<String>,

}
pub type SAPODataConnectorOperator = String;pub type Status = String;pub type EnableDynamicFieldUpdate = bool;
#[derive(serde::Serialize, Default)]
pub struct ConnectorOperator {
    #[serde(rename = "Singular")]
    pub singular: Option<SingularConnectorOperator>,
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogConnectorOperator>,
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoConnectorOperator>,
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeConnectorOperator>,
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaConnectorOperator>,
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskConnectorOperator>,
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotConnectorOperator>,
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceConnectorOperator>,
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataConnectorOperator>,
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsConnectorOperator>,
    #[serde(rename = "Slack")]
    pub slack: Option<SlackConnectorOperator>,
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroConnectorOperator>,
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<Operator>,
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowConnectorOperator>,
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceConnectorOperator>,
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusConnectorOperator>,
    #[serde(rename = "S3")]
    pub s3: Option<S3ConnectorOperator>,

}
pub type AggregationType = String;pub type S3ConnectorOperator = String;pub type InforNexusConnectorOperator = String;pub type DataTransferApi = String;
#[derive(serde::Serialize, Default)]
pub struct ZendeskDestinationProperties {
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<WriteOperationType>,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationConnectorProperties {
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoDestinationProperties>,
    #[serde(rename = "S3")]
    pub s3: Option<S3DestinationProperties>,
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeDestinationProperties>,
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskDestinationProperties>,
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceDestinationProperties>,
    #[serde(rename = "Upsolver")]
    pub upsolver: Option<UpsolverDestinationProperties>,
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorDestinationProperties>,
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftDestinationProperties>,
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataDestinationProperties>,
    #[serde(rename = "LookoutMetrics")]
    pub lookout_metrics: Option<LookoutMetricsDestinationProperties>,
    #[serde(rename = "EventBridge")]
    pub event_bridge: Option<EventBridgeDestinationProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct S3SourceProperties {
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: BucketPrefix,
    #[serde(rename = "S3InputFormatConfig")]
    pub s3_input_format_config: Option<S3InputFormatConfig>,

}
pub type DocumentType = String;
#[derive(serde::Serialize, Default)]
pub struct SuccessResponseHandlingConfig {
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<BucketName>,

}
pub type WriteOperationType = String;pub type TargetFileSize = usize;pub type PathPrefix = String;pub type ApiVersion = String;
#[derive(serde::Serialize, Default)]
pub struct MarketoSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}

#[derive(serde::Serialize, Default)]
pub struct SAPODataSourceProperties {
    #[serde(rename = "ObjectPath")]
    pub object_path: Object,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceSourceProperties {
    #[serde(rename = "EnableDynamicFieldUpdate")]
    pub enable_dynamic_field_update: Option<EnableDynamicFieldUpdate>,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<DataTransferApi>,
    #[serde(rename = "IncludeDeletedRecords")]
    pub include_deleted_records: Option<IncludeDeletedRecords>,

}
pub type DatadogConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct GoogleAnalyticsSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type TaskType = String;pub type TrendmicroConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct AmplitudeSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type EntityName = String;pub type ConnectorType = String;
#[derive(serde::Serialize, Default)]
pub struct Task {
    #[serde(rename = "ConnectorOperator")]
    pub connector_operator: Option<ConnectorOperator>,
    #[serde(rename = "TaskProperties")]
    pub task_properties: Option<Vec<TaskPropertiesObject>>,
    #[serde(rename = "SourceFields")]
    pub source_fields: Vec<String>,
    #[serde(rename = "DestinationField")]
    pub destination_field: Option<String>,
    #[serde(rename = "TaskType")]
    pub task_type: TaskType,

}
pub type Object = String;pub type PrefixType = String;pub type ConnectorProfileName = String;
#[derive(serde::Serialize, Default)]
pub struct DatadogSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type IncludeSourceFiles = bool;pub type ZendeskConnectorOperator = String;pub type DynatraceConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct DestinationFlowConfig {
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<ConnectorProfileName>,
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<ApiVersion>,
    #[serde(rename = "DestinationConnectorProperties")]
    pub destination_connector_properties: DestinationConnectorProperties,
    #[serde(rename = "ConnectorType")]
    pub connector_type: ConnectorType,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceDestinationProperties {
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<DataTransferApi>,
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<WriteOperationType>,

}

#[derive(serde::Serialize, Default)]
pub struct SnowflakeDestinationProperties {
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: BucketName,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}
pub type Operator = String;pub type BucketPrefix = String;pub type PardotConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct EventBridgeDestinationProperties {
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomConnectorSourceProperties {
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(rename = "EntityName")]
    pub entity_name: EntityName,

}
pub type PrefixFormat = String;pub type VeevaConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct SourceConnectorProperties {
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskSourceProperties>,
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorSourceProperties>,
    #[serde(rename = "Singular")]
    pub singular: Option<SingularSourceProperties>,
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusSourceProperties>,
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowSourceProperties>,
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroSourceProperties>,
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsSourceProperties>,
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceSourceProperties>,
    #[serde(rename = "Slack")]
    pub slack: Option<SlackSourceProperties>,
    #[serde(rename = "S3")]
    pub s3: Option<S3SourceProperties>,
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotSourceProperties>,
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogSourceProperties>,
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoSourceProperties>,
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceSourceProperties>,
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaSourceProperties>,
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataSourceProperties>,
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeSourceProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type TriggerType = String;
#[derive(serde::Serialize, Default)]
pub struct TrendmicroSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type FileType = String;
#[derive(serde::Serialize, Default)]
pub struct SlackSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}

#[derive(serde::Serialize, Default)]
pub struct TriggerConfig {
    #[serde(rename = "TriggerProperties")]
    pub trigger_properties: Option<ScheduledTriggerProperties>,
    #[serde(rename = "ActivateFlowOnCreate")]
    pub activate_flow_on_create: Option<bool>,
    #[serde(rename = "TriggerType")]
    pub trigger_type: TriggerType,

}

#[derive(serde::Serialize, Default)]
pub struct ErrorHandlingConfig {
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "FailOnFirstError")]
    pub fail_on_first_error: Option<bool>,
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<BucketName>,

}

#[derive(serde::Serialize, Default)]
pub struct InforNexusSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type Name = String;pub type PreserveSourceDataTyping = bool;
#[derive(serde::Serialize, Default)]
pub struct AggregationConfig {
    #[serde(rename = "TargetFileSize")]
    pub target_file_size: Option<TargetFileSize>,
    #[serde(rename = "AggregationType")]
    pub aggregation_type: Option<AggregationType>,

}

#[derive(serde::Serialize, Default)]
pub struct GlueDataCatalog {
    #[serde(rename = "TablePrefix")]
    pub table_prefix: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct CustomConnectorDestinationProperties {
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(rename = "EntityName")]
    pub entity_name: EntityName,
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<WriteOperationType>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceNowSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type DatetimeTypeFieldName = String;pub type BucketName = String;
#[derive(serde::Serialize, Default)]
pub struct UpsolverS3OutputFormatConfig {
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: PrefixConfig,
    #[serde(rename = "FileType")]
    pub file_type: Option<FileType>,
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,

}
pub type OperatorPropertiesKeys = String;
#[derive(serde::Serialize, Default)]
pub struct UpsolverDestinationProperties {
    #[serde(rename = "BucketName")]
    pub bucket_name: UpsolverBucketName,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: UpsolverS3OutputFormatConfig,

}

#[derive(serde::Serialize, Default)]
pub struct PardotSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}

#[derive(serde::Serialize, Default)]
pub struct IncrementalPullConfig {
    #[serde(rename = "DatetimeTypeFieldName")]
    pub datetime_type_field_name: Option<DatetimeTypeFieldName>,

}

#[derive(serde::Serialize, Default)]
pub struct ZendeskSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}

#[derive(serde::Serialize, Default)]
pub struct SingularSourceProperties {
    #[serde(rename = "Object")]
    pub object: Object,

}
pub type IncludeDeletedRecords = bool;
#[derive(serde::Serialize, Default)]
pub struct VeevaSourceProperties {
    #[serde(rename = "IncludeSourceFiles")]
    pub include_source_files: Option<IncludeSourceFiles>,
    #[serde(rename = "DocumentType")]
    pub document_type: Option<DocumentType>,
    #[serde(rename = "IncludeRenditions")]
    pub include_renditions: Option<IncludeRenditions>,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "IncludeAllVersions")]
    pub include_all_versions: Option<IncludeAllVersions>,

}

#[derive(serde::Serialize, Default)]
pub struct RedshiftDestinationProperties {
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: BucketName,
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct MarketoDestinationProperties {
    #[serde(rename = "Object")]
    pub object: Object,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct MetadataCatalogConfig {
    #[serde(rename = "GlueDataCatalog")]
    pub glue_data_catalog: Option<GlueDataCatalog>,

}

#[derive(serde::Serialize, Default)]
pub struct LookoutMetricsDestinationProperties {
    #[serde(rename = "Object")]
    pub object: Option<Object>,

}
pub type MarketoConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct SourceFlowConfig {
    #[serde(rename = "ConnectorType")]
    pub connector_type: ConnectorType,
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<ConnectorProfileName>,
    #[serde(rename = "SourceConnectorProperties")]
    pub source_connector_properties: SourceConnectorProperties,
    #[serde(rename = "IncrementalPullConfig")]
    pub incremental_pull_config: Option<IncrementalPullConfig>,
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<ApiVersion>,

}
pub type IncludeRenditions = bool;
#[derive(serde::Serialize, Default)]
pub struct S3DestinationProperties {
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<BucketPrefix>,
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: Option<S3OutputFormatConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct SAPODataDestinationProperties {
    #[serde(rename = "SuccessResponseHandlingConfig")]
    pub success_response_handling_config: Option<SuccessResponseHandlingConfig>,
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<WriteOperationType>,
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "ObjectPath")]
    pub object_path: Object,

}

#[derive(serde::Serialize, Default)]
pub struct TaskPropertiesObject {
    #[serde(rename = "Key")]
    pub key: OperatorPropertiesKeys,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type SalesforceConnectorOperator = String;pub type ServiceNowConnectorOperator = String;pub type SingularConnectorOperator = String;
#[derive(serde::Serialize, Default)]
pub struct S3OutputFormatConfig {
    #[serde(rename = "PreserveSourceDataTyping")]
    pub preserve_source_data_typing: Option<PreserveSourceDataTyping>,
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: Option<PrefixConfig>,
    #[serde(rename = "FileType")]
    pub file_type: Option<FileType>,
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomProperties {

}
pub type UpsolverBucketName = String;
#[derive(serde::Serialize, Default)]
pub struct PrefixConfig {
    #[serde(rename = "PrefixType")]
    pub prefix_type: Option<PrefixType>,
    #[serde(rename = "PathPrefixHierarchy")]
    pub path_prefix_hierarchy: Option<PathPrefixHierarchy>,
    #[serde(rename = "PrefixFormat")]
    pub prefix_format: Option<PrefixFormat>,

}
pub type AmplitudeConnectorOperator = String;

}
