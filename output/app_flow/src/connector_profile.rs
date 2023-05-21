/// The AWS::AppFlow::ConnectorProfile resource is an Amazon AppFlow resource    type that specifies the configuration profile for an instance of a connector. This includes    the provided name, credentials ARN, connection-mode, and so on. The fields that are common to    all types of connector profiles are explicitly specified under the Properties    field. The rest of the connector-specific properties are specified under     Properties/ConnectorProfileConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorProfile {
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
    pub connection_mode: ConnectorProfileConnectionModeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,

    ///
    /// Defines the connector-specific configuration and credentials.
    ///
    /// Required: No
    ///
    /// Type: ConnectorProfileConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_config: Option<ConnectorProfileConfig>,

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
    pub connector_type: ConnectorProfileConnectorTypeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmsarn: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorProfileConnectionModeEnum {
    /// Private
    #[serde(rename = "Private")]
    Private,

    /// Public
    #[serde(rename = "Public")]
    Public,
}

impl Default for ConnectorProfileConnectionModeEnum {
    fn default() -> Self {
        ConnectorProfileConnectionModeEnum::Private
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorProfileConnectorTypeEnum {
    /// Amplitude
    #[serde(rename = "Amplitude")]
    Amplitude,

    /// CustomConnector
    #[serde(rename = "CustomConnector")]
    Customconnector,

    /// CustomerProfiles
    #[serde(rename = "CustomerProfiles")]
    Customerprofiles,

    /// Datadog
    #[serde(rename = "Datadog")]
    Datadog,

    /// Dynatrace
    #[serde(rename = "Dynatrace")]
    Dynatrace,

    /// EventBridge
    #[serde(rename = "EventBridge")]
    Eventbridge,

    /// Googleanalytics
    #[serde(rename = "Googleanalytics")]
    Googleanalytics,

    /// Honeycode
    #[serde(rename = "Honeycode")]
    Honeycode,

    /// Infornexus
    #[serde(rename = "Infornexus")]
    Infornexus,

    /// LookoutMetrics
    #[serde(rename = "LookoutMetrics")]
    Lookoutmetrics,

    /// Marketo
    #[serde(rename = "Marketo")]
    Marketo,

    /// Pardot
    #[serde(rename = "Pardot")]
    Pardot,

    /// Redshift
    #[serde(rename = "Redshift")]
    Redshift,

    /// S3
    #[serde(rename = "S3")]
    S3,

    /// Salesforce
    #[serde(rename = "Salesforce")]
    Salesforce,

    /// SAPOData
    #[serde(rename = "SAPOData")]
    Sapodata,

    /// Servicenow
    #[serde(rename = "Servicenow")]
    Servicenow,

    /// Singular
    #[serde(rename = "Singular")]
    Singular,

    /// Slack
    #[serde(rename = "Slack")]
    Slack,

    /// Snowflake
    #[serde(rename = "Snowflake")]
    Snowflake,

    /// Trendmicro
    #[serde(rename = "Trendmicro")]
    Trendmicro,

    /// Upsolver
    #[serde(rename = "Upsolver")]
    Upsolver,

    /// Veeva
    #[serde(rename = "Veeva")]
    Veeva,

    /// Zendesk
    #[serde(rename = "Zendesk")]
    Zendesk,
}

impl Default for ConnectorProfileConnectorTypeEnum {
    fn default() -> Self {
        ConnectorProfileConnectorTypeEnum::Amplitude
    }
}

impl cfn_resources::CfnResource for CfnConnectorProfile {
    fn type_string(&self) -> &'static str {
        "AWS::AppFlow::ConnectorProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.connector_label {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'connector_label'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        self.connector_profile_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.connector_profile_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'connector_profile_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.kmsarn {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'kmsarn'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.kmsarn {
            if the_val.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'kmsarn'. {} is less than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connector-specific credentials required when using Amplitude.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for AmplitudeConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.secret_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'secret_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The API key credentials required for API key authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_secret_key: Option<String>,
}

impl cfn_resources::CfnResource for ApiKeyCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.api_secret_key {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'api_secret_key'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The basic auth credentials required for basic authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BasicAuthCredentials {
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
}

impl cfn_resources::CfnResource for BasicAuthCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'password'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectorOAuthRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

impl cfn_resources::CfnResource for ConnectorOAuthRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.auth_code {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'auth_code'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.redirect_uri {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'redirect_uri'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Defines the connector-specific configuration and credentials for the connector profile.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_properties: Option<ConnectorProfileProperties>,
}

impl cfn_resources::CfnResource for ConnectorProfileConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connector_profile_credentials
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.connector_profile_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific credentials required by a connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectorProfileCredentials {
    ///
    /// The connector-specific credentials required when using Amplitude.
    ///
    /// Required: No
    ///
    /// Type: AmplitudeConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<AmplitudeConnectorProfileCredentials>,

    ///
    /// The connector-specific profile credentials that are required when using the custom    connector.
    ///
    /// Required: No
    ///
    /// Type: CustomConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Datadog.
    ///
    /// Required: No
    ///
    /// Type: DatadogConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Dynatrace.
    ///
    /// Required: No
    ///
    /// Type: DynatraceConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<DynatraceConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Google Analytics.
    ///
    /// Required: No
    ///
    /// Type: GoogleAnalyticsConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<GoogleAnalyticsConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Infor Nexus.
    ///
    /// Required: No
    ///
    /// Type: InforNexusConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Marketo.
    ///
    /// Required: No
    ///
    /// Type: MarketoConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoConnectorProfileCredentials>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Amazon Redshift.
    ///
    /// Required: No
    ///
    /// Type: RedshiftConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftConnectorProfileCredentials>,

    ///
    /// The connector-specific profile credentials required when using SAPOData.
    ///
    /// Required: No
    ///
    /// Type: SAPODataConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sapodata: Option<SAPODataConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Salesforce.
    ///
    /// Required: No
    ///
    /// Type: SalesforceConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using ServiceNow.
    ///
    /// Required: No
    ///
    /// Type: ServiceNowConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Singular.
    ///
    /// Required: No
    ///
    /// Type: SingularConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<SingularConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Slack.
    ///
    /// Required: No
    ///
    /// Type: SlackConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Snowflake.
    ///
    /// Required: No
    ///
    /// Type: SnowflakeConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Trend Micro.
    ///
    /// Required: No
    ///
    /// Type: TrendmicroConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<TrendmicroConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Veeva.
    ///
    /// Required: No
    ///
    /// Type: VeevaConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaConnectorProfileCredentials>,

    ///
    /// The connector-specific credentials required when using Zendesk.
    ///
    /// Required: No
    ///
    /// Type: ZendeskConnectorProfileCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskConnectorProfileCredentials>,
}

impl cfn_resources::CfnResource for ConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.amplitude
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.custom_connector
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.datadog.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.dynatrace
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.google_analytics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.infor_nexus
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.marketo.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.pardot.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.redshift
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sapodata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.salesforce
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.service_now
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.singular
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.slack.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.snowflake
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.trendmicro
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.veeva.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.zendesk.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific profile properties required by each connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Datadog.
    ///
    /// Required: No
    ///
    /// Type: DatadogConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Dynatrace.
    ///
    /// Required: No
    ///
    /// Type: DynatraceConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Marketo.
    ///
    /// Required: No
    ///
    /// Type: MarketoConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoConnectorProfileProperties>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Amazon Redshift.
    ///
    /// Required: No
    ///
    /// Type: RedshiftConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftConnectorProfileProperties>,

    ///
    /// The connector-specific profile properties required when using SAPOData.
    ///
    /// Required: No
    ///
    /// Type: SAPODataConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sapodata: Option<SAPODataConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Salesforce.
    ///
    /// Required: No
    ///
    /// Type: SalesforceConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by serviceNow.
    ///
    /// Required: No
    ///
    /// Type: ServiceNowConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Slack.
    ///
    /// Required: No
    ///
    /// Type: SlackConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Snowflake.
    ///
    /// Required: No
    ///
    /// Type: SnowflakeConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Veeva.
    ///
    /// Required: No
    ///
    /// Type: VeevaConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaConnectorProfileProperties>,

    ///
    /// The connector-specific properties required by Zendesk.
    ///
    /// Required: No
    ///
    /// Type: ZendeskConnectorProfileProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskConnectorProfileProperties>,
}

impl cfn_resources::CfnResource for ConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_connector
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.datadog.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.dynatrace
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.infor_nexus
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.marketo.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.pardot.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.redshift
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sapodata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.salesforce
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.service_now
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.slack.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.snowflake
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.veeva.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.zendesk.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The custom credentials required for custom authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl cfn_resources::CfnResource for CustomAuthCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.custom_authentication_type;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'custom_authentication_type'. {} is greater than 256", the_val.len()));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials that are required when using the custom    connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKeyCredentials>,

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
    pub authentication_type: CustomConnectorProfileCredentialsAuthenticationTypeEnum,

    ///
    /// The basic credentials that are required for the authentication of the user.
    ///
    /// Required: No
    ///
    /// Type: BasicAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Basic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<BasicAuthCredentials>,

    ///
    /// If the connector uses the custom authentication mechanism, this holds the required    credentials.
    ///
    /// Required: No
    ///
    /// Type: CustomAuthCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Custom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomAuthCredentials>,

    ///
    /// The OAuth 2.0 credentials required for the authentication of the user.
    ///
    /// Required: No
    ///
    /// Type: OAuth2Credentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Oauth2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<OAuth2Credentials>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomConnectorProfileCredentialsAuthenticationTypeEnum {
    /// APIKEY
    #[serde(rename = "APIKEY")]
    Apikey,

    /// BASIC
    #[serde(rename = "BASIC")]
    Basic,

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// OAUTH2
    #[serde(rename = "OAUTH2")]
    Oauth2,
}

impl Default for CustomConnectorProfileCredentialsAuthenticationTypeEnum {
    fn default() -> Self {
        CustomConnectorProfileCredentialsAuthenticationTypeEnum::Apikey
    }
}

impl cfn_resources::CfnResource for CustomConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.api_key.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.basic.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.custom.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.oauth2.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The profile properties required by the custom connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_properties: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CustomConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.oauth2_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific credentials required by Datadog.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatadogConnectorProfileCredentials {
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
}

impl cfn_resources::CfnResource for DatadogConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.application_key;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'application_key'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required by Datadog.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for DatadogConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required by Dynatrace.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for DynatraceConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_token;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_token'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required by Dynatrace.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for DynatraceConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required by Google Analytics.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GoogleAnalyticsConnectorProfileCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl cfn_resources::CfnResource for GoogleAnalyticsConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_id'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.client_secret;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_secret'. {} is greater than 512",
                the_val.len()
            ));
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.refresh_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'refresh_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required by Infor Nexus.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InforNexusConnectorProfileCredentials {
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
}

impl cfn_resources::CfnResource for InforNexusConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.access_key_id;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'access_key_id'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.datakey;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'datakey'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.secret_access_key;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'secret_access_key'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.user_id;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'user_id'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required by Infor Nexus.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for InforNexusConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required by Marketo.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MarketoConnectorProfileCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
}

impl cfn_resources::CfnResource for MarketoConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_id'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.client_secret;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_secret'. {} is greater than 512",
                the_val.len()
            ));
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific profile properties required when using Marketo.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for MarketoConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The OAuth 2.0 credentials required for OAuth 2.0 authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OAuth2Credentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_request: Option<ConnectorOAuthRequest>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl cfn_resources::CfnResource for OAuth2Credentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_id {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_id'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_secret {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_secret'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        self.oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.refresh_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'refresh_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The OAuth 2.0 properties required for OAuth 2.0 authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_grant_type: Option<OAuth2PropertiesOAuth2GrantTypeEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,

    ///
    /// Associates your token URL with a map of properties that you define. Use this parameter to    provide any additional details that the connector requires to authenticate your    request.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenUrlCustomProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_custom_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OAuth2PropertiesOAuth2GrantTypeEnum {
    /// AUTHORIZATION_CODE
    #[serde(rename = "AUTHORIZATION_CODE")]
    Authorizationcode,

    /// CLIENT_CREDENTIALS
    #[serde(rename = "CLIENT_CREDENTIALS")]
    Clientcredentials,

    /// JWT_BEARER
    #[serde(rename = "JWT_BEARER")]
    Jwtbearer,
}

impl Default for OAuth2PropertiesOAuth2GrantTypeEnum {
    fn default() -> Self {
        OAuth2PropertiesOAuth2GrantTypeEnum::Authorizationcode
    }
}

impl cfn_resources::CfnResource for OAuth2Properties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.token_url {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'token_url'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The OAuth credentials required for OAuth type authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OAuthCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl cfn_resources::CfnResource for OAuthCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_id {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_id'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_secret {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_secret'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.refresh_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'refresh_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The OAuth properties required for OAuth type authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
}

impl cfn_resources::CfnResource for OAuthProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.auth_code_url {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'auth_code_url'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.token_url {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'token_url'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The PardotConnectorProfileCredentials property type specifies Property description not available. for an AWS::AppFlow::ConnectorProfile.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PardotConnectorProfileCredentials {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_arn: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl cfn_resources::CfnResource for PardotConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The PardotConnectorProfileProperties property type specifies Property description not available. for an AWS::AppFlow::ConnectorProfile.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PardotConnectorProfileProperties {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessUnitId")]
    pub business_unit_id: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsSandboxEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sandbox_environment: Option<bool>,
}

impl cfn_resources::CfnResource for PardotConnectorProfileProperties {
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

/// The connector-specific profile credentials required when using Amazon Redshift.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl cfn_resources::CfnResource for RedshiftConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.password {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'password'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.username {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'username'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connector-specific profile properties when using Amazon Redshift.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataApiRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_api_role_arn: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_url: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsRedshiftServerless")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_redshift_serverless: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

impl cfn_resources::CfnResource for RedshiftConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.bucket_name;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'bucket_name'. {} is less than 3",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.bucket_prefix {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket_prefix'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.database_url {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'database_url'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using SAPOData.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_credentials: Option<OAuthCredentials>,
}

impl cfn_resources::CfnResource for SAPODataConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.basic_auth_credentials
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.oauth_credentials
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific profile properties required when using SAPOData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SAPODataConnectorProfileProperties {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_host_url: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_service_path: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_language: Option<String>,

    ///
    /// The SAPOData OAuth properties required for OAuth type authentication.
    ///
    /// Required: No
    ///
    /// Type: OAuthProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_properties: Option<OAuthProperties>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_service_name: Option<String>,
}

impl cfn_resources::CfnResource for SAPODataConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.application_host_url {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'application_host_url'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.application_service_path {
            if the_val.len() > 512 as _ {
                return Err(format!("Max validation failed on field 'application_service_path'. {} is greater than 512", the_val.len()));
            }
        }

        if let Some(the_val) = &self.client_number {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_number'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_number {
            if the_val.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'client_number'. {} is less than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.logon_language {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'logon_language'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        self.oauth_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.port_number {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port_number'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port_number {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'port_number'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.private_link_service_name {
            if the_val.len() > 512 as _ {
                return Err(format!("Max validation failed on field 'private_link_service_name'. {} is greater than 512", the_val.len()));
            }
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Salesforce.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceConnectorProfileCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_arn: Option<String>,

    ///
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JwtToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuth2GrantType")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl cfn_resources::CfnResource for SalesforceConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_credentials_arn {
            if the_val.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'client_credentials_arn'. {} is greater than 2048", the_val.len()));
            }
        }

        if let Some(the_val) = &self.client_credentials_arn {
            if the_val.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'client_credentials_arn'. {} is less than 20",
                    the_val.len()
                ));
            }
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.refresh_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'refresh_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connector-specific profile properties required when using Salesforce.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sandbox_environment: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "usePrivateLinkForMetadataAndAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_private_link_for_metadata_and_authorization: Option<bool>,
}

impl cfn_resources::CfnResource for SalesforceConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.instance_url {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_url'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using ServiceNow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNowConnectorProfileCredentials {
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

impl cfn_resources::CfnResource for ServiceNowConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'password'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required when using ServiceNow.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for ServiceNowConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Singular.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for SingularConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Slack.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SlackConnectorProfileCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

    ///
    /// Used by select connectors for which the OAuth workflow is supported, such as Salesforce,    Google Analytics, Marketo, Zendesk, and Slack.
    ///
    /// Required: No
    ///
    /// Type: ConnectorOAuthRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOAuthRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
}

impl cfn_resources::CfnResource for SlackConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_id'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.client_secret;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_secret'. {} is greater than 512",
                the_val.len()
            ));
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific profile properties required when using Slack.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for SlackConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Snowflake.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for SnowflakeConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'password'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required when using Snowflake.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnowflakeConnectorProfileProperties {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_service_name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

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
}

impl cfn_resources::CfnResource for SnowflakeConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.account_name {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'account_name'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.bucket_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.bucket_name;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'bucket_name'. {} is less than 3",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.bucket_prefix {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket_prefix'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.private_link_service_name {
            if the_val.len() > 512 as _ {
                return Err(format!("Max validation failed on field 'private_link_service_name'. {} is greater than 512", the_val.len()));
            }
        }

        if let Some(the_val) = &self.region {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'region'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.stage;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'stage'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.warehouse;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'warehouse'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Trend Micro.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for TrendmicroConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_secret_key;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'api_secret_key'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Veeva.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for VeevaConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'password'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 512",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile properties required when using Veeva.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for VeevaConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The connector-specific profile credentials required when using Zendesk.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_oauth_request: Option<ConnectorOAuthRequest>,
}

impl cfn_resources::CfnResource for ZendeskConnectorProfileCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_token {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_token'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_id'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.client_secret;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'client_secret'. {} is greater than 512",
                the_val.len()
            ));
        }

        self.connector_oauth_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The connector-specific profile properties required when using Zendesk.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for ZendeskConnectorProfileProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_url;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_url'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}
