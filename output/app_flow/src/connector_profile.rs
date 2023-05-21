

/// The AWS::AppFlow::ConnectorProfile resource is an Amazon AppFlow resource    type that specifies the configuration profile for an instance of a connector. This includes    the provided name, credentials ARN, connection-mode, and so on. The fields that are common to    all types of connector profiles are explicitly specified under the Properties    field. The rest of the connector-specific properties are specified under     Properties/ConnectorProfileConfig.
#[derive(Default, serde::Serialize)]
pub struct CfnConnectorProfile {


    /// 
    /// The name of the connector profile. The name is unique for each     ConnectorProfile in the AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w/!@#+=.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: String,


    /// 
    /// Defines the connector-specific configuration and credentials.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorProfileConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileConfig")]
    pub connector_profile_config: Option<ConnectorProfileConfig>,


    /// 
    /// The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for    encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS    key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws:kms:.*:[0-9]+:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSArn")]
    pub kmsarn: Option<String>,


    /// 
    /// The type of connector, such as Salesforce, Amplitude, and so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Amplitude | CustomConnector | CustomerProfiles | Datadog | Dynatrace | EventBridge | Googleanalytics | Honeycode | Infornexus | LookoutMetrics | Marketo | Pardot | Redshift | S3 | Salesforce | SAPOData | Servicenow | Singular | Slack | Snowflake | Trendmicro | Upsolver | Veeva | Zendesk
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorType")]
    pub connector_type: String,


    /// 
    /// The label for the connector profile being created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9][\w!@#.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorLabel")]
    pub connector_label: Option<String>,


    /// 
    /// Indicates the connection mode and if it is public or private.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Private | Public
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionMode")]
    pub connection_mode: String,

}


/// The connector-specific profile properties required when using Zendesk.
#[derive(Default, serde::Serialize)]
pub struct ZendeskConnectorProfileProperties {


    /// 
    /// The location of the Zendesk resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile credentials required when using Zendesk.
#[derive(Default, serde::Serialize)]
pub struct ZendeskConnectorProfileCredentials {


    /// 
    /// The credentials used to access protected Zendesk resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization server.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,


    /// 
    /// The identifier for the desired client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

}


/// The OAuth properties required for OAuth type authentication.
#[derive(Default, serde::Serialize)]
pub struct OAuthProperties {


    /// 
    /// The authorization code url required to redirect to SAP Login Page to fetch authorization    code for OAuth type authentication.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthCodeUrl")]
    pub auth_code_url: Option<String>,


    /// 
    /// The OAuth scopes required for OAuth type authentication.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthScopes")]
    pub oauth_scopes: Option<Vec<String>>,


    /// 
    /// The token url required to fetch access/refresh tokens using authorization code and also    to refresh expired access token using refresh token.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenUrl")]
    pub token_url: Option<String>,

}


/// The connector-specific credentials required when using Amplitude.
#[derive(Default, serde::Serialize)]
pub struct AmplitudeConnectorProfileCredentials {


    /// 
    /// A unique alphanumeric identifier used to authenticate a user, developer, or calling    program to your API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKey")]
    pub api_key: String,


    /// 
    /// The Secret Access Key portion of the credentials.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretKey")]
    pub secret_key: String,

}


/// The connector-specific profile properties required when using SAPOData.
#[derive(Default, serde::Serialize)]
pub struct SAPODataConnectorProfileProperties {


    /// 
    /// The logon language of SAPOData instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2
    ///
    /// Pattern: ^[a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogonLanguage")]
    pub logon_language: Option<String>,


    /// 
    /// The port number of the SAPOData instance.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortNumber")]
    pub port_number: Option<i64>,


    /// 
    /// The SAPOData Private Link service name to be used for private data transfers.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^$|com.amazonaws.vpce.[\w/!:@#.\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateLinkServiceName")]
    pub private_link_service_name: Option<String>,


    /// 
    /// The client number for the client creating the connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 3
    ///
    /// Pattern: ^\d{3}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientNumber")]
    pub client_number: Option<String>,


    /// 
    /// The SAPOData OAuth properties required for OAuth type authentication.
    /// 
    /// Required: No
    ///
    /// Type: OAuthProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthProperties")]
    pub oauth_properties: Option<OAuthProperties>,


    /// 
    /// The application path to catalog service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationServicePath")]
    pub application_service_path: Option<String>,


    /// 
    /// The location of the SAPOData resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationHostUrl")]
    pub application_host_url: Option<String>,

}


/// The connector-specific profile properties required when using Marketo.
#[derive(Default, serde::Serialize)]
pub struct MarketoConnectorProfileProperties {


    /// 
    /// The location of the Marketo resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific credentials required by Datadog.
#[derive(Default, serde::Serialize)]
pub struct DatadogConnectorProfileCredentials {


    /// 
    /// Application keys, in conjunction with your API key, give you full access to Datadog’s    programmatic API. Application keys are associated with the user account that created them. The    application key is used to log all requests made to the API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationKey")]
    pub application_key: String,


    /// 
    /// A unique alphanumeric identifier used to authenticate a user, developer, or calling    program to your API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKey")]
    pub api_key: String,

}


/// The connector-specific profile credentials required when using Salesforce.
#[derive(Default, serde::Serialize)]
pub struct SalesforceConnectorProfileCredentials {


    /// 
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,


    /// 
    /// The credentials used to access protected Salesforce resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuth2GrantType")]
    pub oauth2_grant_type: Option<String>,


    /// 
    /// The credentials used to acquire new access tokens.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JwtToken")]
    pub jwt_token: Option<String>,


    /// 
    /// The secret manager ARN, which contains the client ID and client secret of the connected    app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws:secretsmanager:.*:[0-9]+:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCredentialsArn")]
    pub client_credentials_arn: Option<String>,

}


/// The connector-specific profile credentials required by Infor Nexus.
#[derive(Default, serde::Serialize)]
pub struct InforNexusConnectorProfileCredentials {


    /// 
    /// The identifier for the user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserId")]
    pub user_id: String,


    /// 
    /// The Access Key portion of the credentials.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: String,


    /// 
    /// The secret key used to sign requests.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretAccessKey")]
    pub secret_access_key: String,


    /// 
    /// The encryption keys used to encrypt data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datakey")]
    pub datakey: String,

}


/// The OAuth credentials required for OAuth type authentication.
#[derive(Default, serde::Serialize)]
pub struct OAuthCredentials {


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,


    /// 
    /// The identifier for the desired client.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,


    /// 
    /// The access token used to access protected SAPOData resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The refresh token used to refresh expired access token.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

}


/// The basic auth credentials required for basic authentication.
#[derive(Default, serde::Serialize)]
pub struct BasicAuthCredentials {


    /// 
    /// The username to use to connect to a resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,


    /// 
    /// The password to use to connect to a resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,

}


/// The connector-specific profile properties required by each connector.
#[derive(Default, serde::Serialize)]
pub struct ConnectorProfileProperties {


    /// 
    /// The properties required by the custom connector.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorProfileProperties>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: SnowflakeConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Dynatrace.
    /// 
    /// Required: No
    ///
    /// Type: DynatraceConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Infor Nexus.
    /// 
    /// Required: No
    ///
    /// Type: InforNexusConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Slack.
    /// 
    /// Required: No
    ///
    /// Type: SlackConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<SlackConnectorProfileProperties>,


    /// 
    /// The connector-specific profile properties required when using SAPOData.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Datadog.
    /// 
    /// Required: No
    ///
    /// Type: DatadogConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Veeva.
    /// 
    /// Required: No
    ///
    /// Type: VeevaConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by Amazon Redshift.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftConnectorProfileProperties>,


    /// 
    /// The connector-specific properties required by serviceNow.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowConnectorProfileProperties>,

}


/// The API key credentials required for API key authentication.
#[derive(Default, serde::Serialize)]
pub struct ApiKeyCredentials {


    /// 
    /// The API key required for API key authentication.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKey")]
    pub api_key: String,


    /// 
    /// The API secret key required for API key authentication.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiSecretKey")]
    pub api_secret_key: Option<String>,

}


/// The custom credentials required for custom authentication.
#[derive(Default, serde::Serialize)]
pub struct CustomAuthCredentials {


    /// 
    /// A map that holds custom authentication credentials.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialsMap")]
    pub credentials_map: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The custom authentication type that the connector uses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomAuthenticationType")]
    pub custom_authentication_type: String,

}


/// The connector-specific profile properties required when using ServiceNow.
#[derive(Default, serde::Serialize)]
pub struct ServiceNowConnectorProfileProperties {


    /// 
    /// The location of the ServiceNow resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile credentials required when using Singular.
#[derive(Default, serde::Serialize)]
pub struct SingularConnectorProfileCredentials {


    /// 
    /// A unique alphanumeric identifier used to authenticate a user, developer, or calling    program to your API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKey")]
    pub api_key: String,

}


/// The connector-specific credentials required by a connector.
#[derive(Default, serde::Serialize)]
pub struct ConnectorProfileCredentials {


    /// 
    /// The connector-specific credentials required when using Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Trend Micro.
    /// 
    /// Required: No
    ///
    /// Type: TrendmicroConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Slack.
    /// 
    /// Required: No
    ///
    /// Type: SlackConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<SlackConnectorProfileCredentials>,


    /// 
    /// The connector-specific profile credentials required when using SAPOData.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Infor Nexus.
    /// 
    /// Required: No
    ///
    /// Type: InforNexusConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Amazon Redshift.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Veeva.
    /// 
    /// Required: No
    ///
    /// Type: VeevaConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Amplitude.
    /// 
    /// Required: No
    ///
    /// Type: AmplitudeConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Singular.
    /// 
    /// Required: No
    ///
    /// Type: SingularConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    pub singular: Option<SingularConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Datadog.
    /// 
    /// Required: No
    ///
    /// Type: DatadogConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogConnectorProfileCredentials>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Google Analytics.
    /// 
    /// Required: No
    ///
    /// Type: GoogleAnalyticsConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: SnowflakeConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoConnectorProfileCredentials>,


    /// 
    /// The connector-specific profile credentials that are required when using the custom    connector.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using Dynatrace.
    /// 
    /// Required: No
    ///
    /// Type: DynatraceConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceConnectorProfileCredentials>,


    /// 
    /// The connector-specific credentials required when using ServiceNow.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowConnectorProfileCredentials>,

}


/// The OAuth 2.0 credentials required for OAuth 2.0 authentication.
#[derive(Default, serde::Serialize)]
pub struct OAuth2Credentials {


    /// 
    /// The refresh token used to refresh an expired access token.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization    server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthRequest")]
    pub oauth_request: Option<ConnectorOAuthRequest>,


    /// 
    /// The access token used to access the connector on your behalf.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The identifier for the desired client.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,

}


/// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
#[derive(Default, serde::Serialize)]
pub struct ConnectorOAuthRequest {


    /// 
    /// The URL to which the authentication server redirects the browser after authorization has    been granted.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectUri")]
    pub redirect_uri: Option<String>,


    /// 
    /// The code provided by the connector when it has been authenticated via the connected app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthCode")]
    pub auth_code: Option<String>,

}


/// The connector-specific profile credentials required when using Snowflake.
#[derive(Default, serde::Serialize)]
pub struct SnowflakeConnectorProfileCredentials {


    /// 
    /// The password that corresponds to the user name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// The name of the user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,

}


/// The connector-specific profile credentials required when using Veeva.
#[derive(Default, serde::Serialize)]
pub struct VeevaConnectorProfileCredentials {


    /// 
    /// The password that corresponds to the user name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// The name of the user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,

}


/// The connector-specific profile properties required when using Slack.
#[derive(Default, serde::Serialize)]
pub struct SlackConnectorProfileProperties {


    /// 
    /// The location of the Slack resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile credentials required when using ServiceNow.
#[derive(Default, serde::Serialize)]
pub struct ServiceNowConnectorProfileCredentials {


    /// 
    /// The name of the user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,


    /// 
    /// The password that corresponds to the user name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,

}


/// The connector-specific profile properties required by Datadog.
#[derive(Default, serde::Serialize)]
pub struct DatadogConnectorProfileProperties {


    /// 
    /// The location of the Datadog resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile properties required when using Salesforce.
#[derive(Default, serde::Serialize)]
pub struct SalesforceConnectorProfileProperties {


    /// 
    /// The location of the Salesforce resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: Option<String>,


    /// 
    /// Indicates whether the connector profile applies to a sandbox or production environment.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "isSandboxEnvironment")]
    pub is_sandbox_environment: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "usePrivateLinkForMetadataAndAuthorization")]
    pub use_private_link_for_metadata_and_authorization: Option<bool>,

}


/// The PardotConnectorProfileCredentials property type specifies Property description not available. for an AWS::AppFlow::ConnectorProfile.
#[derive(Default, serde::Serialize)]
pub struct PardotConnectorProfileCredentials {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCredentialsArn")]
    pub client_credentials_arn: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

}


/// The connector-specific profile credentials required when using Amazon Redshift.
#[derive(Default, serde::Serialize)]
pub struct RedshiftConnectorProfileCredentials {


    /// 
    /// The password that corresponds to the user name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: Option<String>,


    /// 
    /// The name of the user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: Option<String>,

}


/// The connector-specific profile credentials required by Dynatrace.
#[derive(Default, serde::Serialize)]
pub struct DynatraceConnectorProfileCredentials {


    /// 
    /// The API tokens used by Dynatrace API to authenticate various API calls.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiToken")]
    pub api_token: String,

}


/// The connector-specific profile properties required when using Veeva.
#[derive(Default, serde::Serialize)]
pub struct VeevaConnectorProfileProperties {


    /// 
    /// The location of the Veeva resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile credentials required by Google Analytics.
#[derive(Default, serde::Serialize)]
pub struct GoogleAnalyticsConnectorProfileCredentials {


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization server.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,


    /// 
    /// The identifier for the desired client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,


    /// 
    /// The credentials used to access protected Google Analytics resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The credentials used to acquire new access tokens. This is required only for OAuth2    access tokens, and is not required for OAuth1 access tokens.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,

}


/// The connector-specific profile credentials that are required when using the custom    connector.
#[derive(Default, serde::Serialize)]
pub struct CustomConnectorProfileCredentials {


    /// 
    /// The API keys required for the authentication of the user.
    /// 
    /// Required: No
    ///
    /// Type: ApiKeyCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKey")]
    pub api_key: Option<ApiKeyCredentials>,


    /// 
    /// The OAuth 2.0 credentials required for the authentication of the user.
    /// 
    /// Required: No
    ///
    /// Type: OAuth2Credentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Oauth2")]
    pub oauth2: Option<OAuth2Credentials>,


    /// 
    /// The authentication type that the custom connector uses for authenticating while creating a    connector profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: APIKEY | BASIC | CUSTOM | OAUTH2
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,


    /// 
    /// If the connector uses the custom authentication mechanism, this holds the required    credentials.
    /// 
    /// Required: No
    ///
    /// Type: CustomAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Custom")]
    pub custom: Option<CustomAuthCredentials>,


    /// 
    /// The basic credentials that are required for the authentication of the user.
    /// 
    /// Required: No
    ///
    /// Type: BasicAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Basic")]
    pub basic: Option<BasicAuthCredentials>,

}


/// The profile properties required by the custom connector.
#[derive(Default, serde::Serialize)]
pub struct CustomConnectorProfileProperties {


    /// 
    /// The OAuth 2.0 properties required for OAuth 2.0 authentication.
    /// 
    /// Required: No
    ///
    /// Type: OAuth2Properties
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuth2Properties")]
    pub oauth2_properties: Option<OAuth2Properties>,


    /// 
    /// A map of properties that are required to create a profile for the custom connector.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProfileProperties")]
    pub profile_properties: Option<std::collections::HashMap<String, String>>,

}


/// The connector-specific profile properties required by Infor Nexus.
#[derive(Default, serde::Serialize)]
pub struct InforNexusConnectorProfileProperties {


    /// 
    /// The location of the Infor Nexus resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The connector-specific profile properties required by Dynatrace.
#[derive(Default, serde::Serialize)]
pub struct DynatraceConnectorProfileProperties {


    /// 
    /// The location of the Dynatrace resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: String,

}


/// The PardotConnectorProfileProperties property type specifies Property description not available. for an AWS::AppFlow::ConnectorProfile.
#[derive(Default, serde::Serialize)]
pub struct PardotConnectorProfileProperties {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    pub instance_url: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsSandboxEnvironment")]
    pub is_sandbox_environment: Option<bool>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessUnitId")]
    pub business_unit_id: String,

}


/// The connector-specific profile credentials required when using SAPOData.
#[derive(Default, serde::Serialize)]
pub struct SAPODataConnectorProfileCredentials {


    /// 
    /// The SAPOData basic authentication credentials.
    /// 
    /// Required: No
    ///
    /// Type: BasicAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthCredentials")]
    pub basic_auth_credentials: Option<BasicAuthCredentials>,


    /// 
    /// The SAPOData OAuth type authentication credentials.
    /// 
    /// Required: No
    ///
    /// Type: OAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthCredentials")]
    pub oauth_credentials: Option<OAuthCredentials>,

}


/// The connector-specific profile credentials required when using Slack.
#[derive(Default, serde::Serialize)]
pub struct SlackConnectorProfileCredentials {


    /// 
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,


    /// 
    /// The credentials used to access protected Slack resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The identifier for the client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization server.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,

}


/// The OAuth 2.0 properties required for OAuth 2.0 authentication.
#[derive(Default, serde::Serialize)]
pub struct OAuth2Properties {


    /// 
    /// The OAuth 2.0 grant type used by connector for OAuth 2.0 authentication.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHORIZATION_CODE | CLIENT_CREDENTIALS | JWT_BEARER
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuth2GrantType")]
    pub oauth2_grant_type: Option<String>,


    /// 
    /// Associates your token URL with a map of properties that you define. Use this parameter to    provide any additional details that the connector requires to authenticate your    request.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenUrlCustomProperties")]
    pub token_url_custom_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The token URL required for OAuth 2.0 authentication.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenUrl")]
    pub token_url: Option<String>,

}


/// The connector-specific profile credentials required by Marketo.
#[derive(Default, serde::Serialize)]
pub struct MarketoConnectorProfileCredentials {


    /// 
    /// The identifier for the desired client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// The credentials used to access protected Marketo resources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,


    /// 
    /// The client secret used by the OAuth client to authenticate to the authorization server.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,


    /// 
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

}


/// Defines the connector-specific configuration and credentials for the connector profile.
#[derive(Default, serde::Serialize)]
pub struct ConnectorProfileConfig {


    /// 
    /// The connector-specific credentials required by each connector.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileCredentials")]
    pub connector_profile_credentials: Option<ConnectorProfileCredentials>,


    /// 
    /// The connector-specific properties of the profile configuration.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileProperties")]
    pub connector_profile_properties: Option<ConnectorProfileProperties>,

}


/// The connector-specific profile credentials required when using Trend Micro.
#[derive(Default, serde::Serialize)]
pub struct TrendmicroConnectorProfileCredentials {


    /// 
    /// The Secret Access Key portion of the credentials.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiSecretKey")]
    pub api_secret_key: String,

}


/// The connector-specific profile properties when using Amazon Redshift.
#[derive(Default, serde::Serialize)]
pub struct RedshiftConnectorProfileProperties {


    /// 
    /// A name for the associated Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsRedshiftServerless")]
    pub is_redshift_serverless: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataApiRoleArn")]
    pub data_api_role_arn: Option<String>,


    /// 
    /// The object key for the destination bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// The JDBC URL of the Amazon Redshift cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseUrl")]
    pub database_url: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of IAM role that grants Amazon Redshift    read-only access to Amazon S3. For more information, and for the polices that you    attach to this role, see Allow Amazon Redshift to access your Amazon AppFlow data in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:aws:iam:.*:[0-9]+:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: Option<String>,

}


/// The connector-specific profile properties required when using Snowflake.
#[derive(Default, serde::Serialize)]
pub struct SnowflakeConnectorProfileProperties {


    /// 
    /// The name of the Amazon S3 bucket associated with Snowflake.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The AWS Region of the Snowflake account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: Option<String>,


    /// 
    /// The name of the account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,


    /// 
    /// The name of the Amazon S3 stage that was created while setting up an Amazon S3 stage in the Snowflake account. This is written in the following format: <    Database>< Schema><Stage Name>.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    pub stage: String,


    /// 
    /// The bucket path that refers to the Amazon S3 bucket associated with Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The name of the Snowflake warehouse.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\w/!@#+=.-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Warehouse")]
    pub warehouse: String,


    /// 
    /// The Snowflake Private Link service name to be used for private data transfers.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^$|com.amazonaws.vpce.[\w/!:@#.\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateLinkServiceName")]
    pub private_link_service_name: Option<String>,

}
