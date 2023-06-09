/// The AWS::ApplicationInsights::Application resource adds an application that is created from a resource group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApplication {
    ///
    /// If set to true, the application components will be configured with the monitoring configuration recommended by Application Insights.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoConfigurationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_configuration_enabled: Option<bool>,

    ///
    /// Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CWEMonitorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwemonitor_enabled: Option<bool>,

    ///
    /// The monitoring settings of the components.
    ///
    /// Required: No
    ///
    /// Type: List of ComponentMonitoringSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentMonitoringSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_monitoring_settings: Option<Vec<ComponentMonitoringSetting>>,

    ///
    /// Describes a custom component by grouping similar standalone instances to monitor.
    ///
    /// Required: No
    ///
    /// Type: List of CustomComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_components: Option<Vec<CustomComponent>>,

    ///
    /// Application Insights can create applications based on a resource group or on an account.     To create an account-based application using all of the resources in the account, set this     parameter to ACCOUNT_BASED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACCOUNT_BASED
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_type: Option<ApplicationGroupingTypeEnum>,

    ///
    /// The log pattern sets.
    ///
    /// Required: No
    ///
    /// Type: List of LogPatternSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern_sets: Option<Vec<LogPatternSet>>,

    ///
    /// Indicates whether Application Insights will create OpsItems for any problem that is       detected by Application Insights for an application.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,

    ///
    /// The SNS topic provided to Application Insights that is associated with the created       OpsItems to receive SNS notifications for opsItem updates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 300
    ///
    /// Pattern: ^arn:aws(-\w+)*:[\w\d-]+:([\w\d-]*)?:[\w\d_-]*([:/].+)*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_snstopic_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the resource group used for the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9\.\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: cfn_resources::StrVal,

    ///
    /// An array of Tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_application_arn: CfnApplicationapplicationarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ApplicationGroupingTypeEnum {
    /// ACCOUNT_BASED
    #[serde(rename = "ACCOUNT_BASED")]
    Accountbased,
}

impl Default for ApplicationGroupingTypeEnum {
    fn default() -> Self {
        ApplicationGroupingTypeEnum::Accountbased
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApplicationapplicationarn;
impl CfnApplicationapplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationARN"#
    }
}

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::ApplicationInsights::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ops_item_snstopic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 300 as _ {
                    return Err(format!("Max validation failed on field 'ops_item_snstopic_arn'. {} is greater than 300", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.ops_item_snstopic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!("Min validation failed on field 'ops_item_snstopic_arn'. {} is less than 20", s.len()));
                }
            }
        }

        let the_val = &self.resource_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_group_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.resource_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_group_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application Alarm property type defines a CloudWatch alarm to be monitored for the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Alarm {
    ///
    /// The name of the CloudWatch alarm to be monitored for the component.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmName")]
    pub alarm_name: cfn_resources::StrVal,

    ///
    /// Indicates the degree of outage when the alarm goes off.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Alarm {
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

/// The AWS::ApplicationInsights::Application AlarmMetric property type defines a metric to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AlarmMetric {
    ///
    /// The name of the metric to be monitored for the component. For metrics supported by Application Insights, see Logs and metrics supported by Amazon CloudWatch Application Insights.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmMetricName")]
    pub alarm_metric_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AlarmMetric {
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

/// The AWS::ApplicationInsights::Application ComponentConfiguration property type defines the configuration settings of the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComponentConfiguration {
    ///
    /// The configuration settings.
    ///
    /// Required: No
    ///
    /// Type: ConfigurationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_details: Option<ConfigurationDetails>,

    ///
    /// Sub-component configurations of the component.
    ///
    /// Required: No
    ///
    /// Type: List of SubComponentTypeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubComponentTypeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_component_type_configurations: Option<Vec<SubComponentTypeConfiguration>>,
}

impl cfn_resources::CfnResource for ComponentConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.configuration_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application ComponentMonitoringSetting property type defines the monitoring setting of the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComponentMonitoringSetting {
    ///
    /// The ARN of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_arn: Option<cfn_resources::StrVal>,

    ///
    /// Component monitoring can be configured in one of the following three modes:
    ///
    /// DEFAULT: The component will be configured with the recommended default monitoring settings of the selected Tier.CUSTOM: The component will be configured with the customized monitoring settings           that are specified in CustomComponentConfiguration. If used,             CustomComponentConfiguration must be provided.DEFAULT_WITH_OVERWRITE: The component will be configured with the recommended           default monitoring settings of the selected Tier, and merged with           customized overwrite settings that are specified in             DefaultOverwriteComponentConfiguration. If used,             DefaultOverwriteComponentConfiguration must be provided.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentConfigurationMode")]
    pub component_configuration_mode: cfn_resources::StrVal,

    ///
    /// The name of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<cfn_resources::StrVal>,

    ///
    /// Customized monitoring settings. Required if CUSTOM mode is configured in ComponentConfigurationMode.
    ///
    /// Required: No
    ///
    /// Type: ComponentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomComponentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_component_configuration: Option<ComponentConfiguration>,

    ///
    /// Customized overwrite monitoring settings. Required if CUSTOM mode is configured in ComponentConfigurationMode.
    ///
    /// Required: No
    ///
    /// Type: ComponentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultOverwriteComponentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_overwrite_component_configuration: Option<ComponentConfiguration>,

    ///
    /// The tier of the application component. Supported tiers include         DOT_NET_CORE, DOT_NET_WORKER, DOT_NET_WEB,         SQL_SERVER, SQL_SERVER_ALWAYSON_AVAILABILITY_GROUP,         SQL_SERVER_FAILOVER_CLUSTER_INSTANCE, MYSQL,         POSTGRESQL, JAVA_JMX, ORACLE,         SAP_HANA_MULTI_NODE, SAP_HANA_SINGLE_NODE,         SAP_HANA_HIGH_AVAILABILITY, SHAREPOINT.         ACTIVE_DIRECTORY, and DEFAULT.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    pub tier: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ComponentMonitoringSetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_component_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_overwrite_component_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application ConfigurationDetails property type specifies the configuration settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigurationDetails {
    ///
    /// A list of metrics to monitor for the component. All component types can use AlarmMetrics.
    ///
    /// Required: No
    ///
    /// Type: List of AlarmMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_metrics: Option<Vec<AlarmMetric>>,

    ///
    /// A list of alarms to monitor for the component. All component types can use         Alarm.
    ///
    /// Required: No
    ///
    /// Type: List of Alarm
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,

    ///
    /// The HA cluster Prometheus Exporter settings.
    ///
    /// Required: No
    ///
    /// Type: HAClusterPrometheusExporter
    ///
    /// Update requires: No interruption
    #[serde(rename = "HAClusterPrometheusExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hacluster_prometheus_exporter: Option<HAClusterPrometheusExporter>,

    ///
    /// The HANA DB Prometheus Exporter settings.
    ///
    /// Required: No
    ///
    /// Type: HANAPrometheusExporter
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANAPrometheusExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hanaprometheus_exporter: Option<HANAPrometheusExporter>,

    ///
    /// A list of Java metrics to monitor for the component.
    ///
    /// Required: No
    ///
    /// Type: JMXPrometheusExporter
    ///
    /// Update requires: No interruption
    #[serde(rename = "JMXPrometheusExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmxprometheus_exporter: Option<JMXPrometheusExporter>,

    ///
    /// A list of logs to monitor for the component. Only Amazon EC2 instances can use         Logs.
    ///
    /// Required: No
    ///
    /// Type: List of Log
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<Log>>,

    ///
    /// A list of Windows Events to monitor for the component. Only Amazon EC2 instances       running on Windows can use WindowsEvents.
    ///
    /// Required: No
    ///
    /// Type: List of WindowsEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowsEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_events: Option<Vec<WindowsEvent>>,
}

impl cfn_resources::CfnResource for ConfigurationDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hacluster_prometheus_exporter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hanaprometheus_exporter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.jmxprometheus_exporter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application CustomComponent property type describes a custom component by grouping similar standalone instances to monitor.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomComponent {
    ///
    /// The name of the component.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\d\w\-_\.+]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentName")]
    pub component_name: cfn_resources::StrVal,

    ///
    /// The list of resource ARNs that belong to the component.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceList")]
    pub resource_list: Vec<String>,
}

impl cfn_resources::CfnResource for CustomComponent {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.component_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'component_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.component_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'component_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application HAClusterPrometheusExporter       property type defines the HA cluster Prometheus Exporter settings. For more information,       see the component configuration in the CloudWatch Application Insights       documentation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HAClusterPrometheusExporter {
    ///
    /// The target port to which Prometheus sends metrics. If not specified, the default port 9668 is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrometheusPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_port: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HAClusterPrometheusExporter {
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

/// The AWS::ApplicationInsights::Application HANAPrometheusExporter property       type defines the HANA DB Prometheus Exporter settings. For more information, see the         component configuration in the CloudWatch Application Insights       documentation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HANAPrometheusExporter {
    ///
    /// Designates whether you agree to install the HANA DB client.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgreeToInstallHANADBClient")]
    pub agree_to_install_hanadbclient: bool,

    ///
    /// The HANA database port by which the exporter will query HANA metrics.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANAPort")]
    pub hanaport: cfn_resources::StrVal,

    ///
    /// The three-character SAP system ID (SID) of the SAP HANA system.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANASID")]
    pub hanasid: cfn_resources::StrVal,

    ///
    /// The AWS Secrets Manager secret that stores HANA monitoring user credentials. The HANA Prometheus exporter uses these credentials to connect to the database and query HANA metrics.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANASecretName")]
    pub hanasecret_name: cfn_resources::StrVal,

    ///
    /// The target port to which Prometheus sends metrics. If not specified, the default port 9668 is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrometheusPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_port: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HANAPrometheusExporter {
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

/// The AWS::ApplicationInsights::Application JMXPrometheusExporter property type       defines the JMXPrometheus Exporter configuration. For more information, see the      component configuration in the CloudWatch Application Insights    documentation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JMXPrometheusExporter {
    ///
    /// The host and port to connect to through remote JMX. Only one of jmxURL and hostPort can be     specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<cfn_resources::StrVal>,

    ///
    /// The complete JMX URL to connect to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JMXURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmxurl: Option<cfn_resources::StrVal>,

    ///
    /// The target port to send Prometheus metrics to. If not specified, the default port 9404 is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrometheusPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_port: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JMXPrometheusExporter {
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

/// The AWS::ApplicationInsights::Application Log property type specifies a log to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Log {
    ///
    /// The type of encoding of the logs to be monitored. The specified encoding should be included in the list of CloudWatch agent supported encodings. If not provided, CloudWatch Application Insights uses the default encoding type for the log type:
    ///
    /// APPLICATION/DEFAULT: utf-8 encodingSQL_SERVER: utf-16 encodingIIS: ascii encoding
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<cfn_resources::StrVal>,

    ///
    /// The CloudWatch log group name to be associated with the monitored log.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The path of the logs to be monitored. The log path must be an absolute Windows or Linux system file path. For more information, see CloudWatch Agent Configuration File: Logs Section.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<cfn_resources::StrVal>,

    ///
    /// The log type decides the log patterns against which Application Insights analyzes the       log. The log type is selected from the following: SQL_SERVER,         MYSQL, MYSQL_SLOW_QUERY, POSTGRESQL,         ORACLE_ALERT, ORACLE_LISTENER, IIS,         APPLICATION, WINDOWS_EVENTS,         WINDOWS_EVENTS_ACTIVE_DIRECTORY, WINDOWS_EVENTS_DNS ,         WINDOWS_EVENTS_IIS , WINDOWS_EVENTS_SHAREPOINT,         SQL_SERVER_ALWAYSON_AVAILABILITY_GROUP,         SQL_SERVER_FAILOVER_CLUSTER_INSTANCE, STEP_FUNCTION,         API_GATEWAY_ACCESS, API_GATEWAY_EXECUTION,         SAP_HANA_LOGS, SAP_HANA_TRACE,         SAP_HANA_HIGH_AVAILABILITY, and DEFAULT.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogType")]
    pub log_type: cfn_resources::StrVal,

    ///
    /// The log pattern set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Log {
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

/// The AWS::ApplicationInsights::Application LogPattern property type       specifies an object that defines the log patterns that belong to a         LogPatternSet.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogPattern {
    ///
    /// A regular expression that defines the log pattern. A log pattern can contain up to 50       characters, and it cannot be empty.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\S\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: cfn_resources::StrVal,

    ///
    /// The name of the log pattern. A log pattern name can contain up to 50 characters, and       it cannot be empty. The characters can be Unicode letters, digits, or one of the       following symbols: period, dash, underscore.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [a-zA-Z0-9\.\-_]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternName")]
    pub pattern_name: cfn_resources::StrVal,

    ///
    /// The rank of the log pattern.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rank")]
    pub rank: i64,
}

impl cfn_resources::CfnResource for LogPattern {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.pattern;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'pattern'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.pattern;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'pattern'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.pattern_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'pattern_name'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.pattern_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'pattern_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application LogPatternSet property type specifies the log pattern set.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogPatternSet {
    ///
    /// A list of objects that define the log patterns that belong to LogPatternSet.
    ///
    /// Required: Yes
    ///
    /// Type: List of LogPattern
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPatterns")]
    pub log_patterns: Vec<LogPattern>,

    ///
    /// The name of the log pattern. A log pattern name can contain up to 30 characters, and       it cannot be empty. The characters can be Unicode letters, digits, or one of the       following symbols: period, dash, underscore.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: [a-zA-Z0-9\.\-_]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LogPatternSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.pattern_set_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'pattern_set_name'. {} is greater than 30",
                    s.len()
                ));
            }
        }

        let the_val = &self.pattern_set_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'pattern_set_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The AWS::ApplicationInsights::Application SubComponentConfigurationDetails property type specifies the configuration settings of the sub-components.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubComponentConfigurationDetails {
    ///
    /// A list of metrics to monitor for the component. All component types can use AlarmMetrics.
    ///
    /// Required: No
    ///
    /// Type: List of AlarmMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_metrics: Option<Vec<AlarmMetric>>,

    ///
    /// A list of logs to monitor for the component. Only Amazon EC2 instances can use         Logs.
    ///
    /// Required: No
    ///
    /// Type: List of Log
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<Log>>,

    ///
    /// A list of Windows Events to monitor for the component. Only Amazon EC2 instances running       on Windows can use WindowsEvents.
    ///
    /// Required: No
    ///
    /// Type: List of WindowsEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowsEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_events: Option<Vec<WindowsEvent>>,
}

impl cfn_resources::CfnResource for SubComponentConfigurationDetails {
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

/// The AWS::ApplicationInsights::Application SubComponentTypeConfiguration property type specifies the sub-component configurations for a component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubComponentTypeConfiguration {
    ///
    /// The configuration settings of the sub-components.
    ///
    /// Required: Yes
    ///
    /// Type: SubComponentConfigurationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubComponentConfigurationDetails")]
    pub sub_component_configuration_details: SubComponentConfigurationDetails,

    ///
    /// The sub-component type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubComponentType")]
    pub sub_component_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SubComponentTypeConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sub_component_configuration_details.validate()?;

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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

/// The AWS::ApplicationInsights::Application WindowsEvent property type specifies a Windows Event to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WindowsEvent {
    ///
    /// The levels of event to log. You must specify each level to log. Possible values include INFORMATION, WARNING, ERROR, CRITICAL, and VERBOSE. This field is required for each type of Windows Event to log.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventLevels")]
    pub event_levels: Vec<String>,

    ///
    /// The type of Windows Events to log, equivalent to the Windows Event log channel name. For       example, System, Security, CustomEventName, and so on. This field is required for each       type of Windows event to log.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventName")]
    pub event_name: cfn_resources::StrVal,

    ///
    /// The CloudWatch log group name to be associated with the monitored log.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: cfn_resources::StrVal,

    ///
    /// The log pattern set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for WindowsEvent {
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
