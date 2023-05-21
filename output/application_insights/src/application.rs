

/// The AWS::ApplicationInsights::Application resource adds an application that is created from a resource group.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub ops_item_snstopic_arn: Option<String>,


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
    pub resource_group_name: String,


    /// 
    /// An array of Tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
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


impl cfn_resources::CfnResource for CfnApplication {
    fn type_string() -> &'static str {
        "AWS::ApplicationInsights::Application"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AWS::ApplicationInsights::Application Alarm property type defines a CloudWatch alarm to be monitored for the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub alarm_name: String,


    /// 
    /// Indicates the degree of outage when the alarm goes off.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    pub severity: Option<String>,

}




/// The AWS::ApplicationInsights::Application AlarmMetric property type defines a metric to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub alarm_metric_name: String,

}




/// The AWS::ApplicationInsights::Application ComponentConfiguration property type defines the configuration settings of the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub sub_component_type_configurations: Option<Vec<SubComponentTypeConfiguration>>,

}




/// The AWS::ApplicationInsights::Application ComponentMonitoringSetting property type defines the monitoring setting of the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub component_arn: Option<String>,


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
    pub component_configuration_mode: String,


    /// 
    /// The name of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentName")]
    pub component_name: Option<String>,


    /// 
    /// Customized monitoring settings. Required if CUSTOM mode is configured in ComponentConfigurationMode.
    /// 
    /// Required: No
    ///
    /// Type: ComponentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomComponentConfiguration")]
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
    pub tier: String,

}




/// The AWS::ApplicationInsights::Application ConfigurationDetails property type specifies the configuration settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub windows_events: Option<Vec<WindowsEvent>>,

}




/// The AWS::ApplicationInsights::Application CustomComponent property type describes a custom component by grouping similar standalone instances to monitor.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub component_name: String,


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




/// The AWS::ApplicationInsights::Application HAClusterPrometheusExporter       property type defines the HA cluster Prometheus Exporter settings. For more information,       see the component configuration in the CloudWatch Application Insights       documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub prometheus_port: Option<String>,

}




/// The AWS::ApplicationInsights::Application HANAPrometheusExporter property       type defines the HANA DB Prometheus Exporter settings. For more information, see the         component configuration in the CloudWatch Application Insights       documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub hanaport: String,


    /// 
    /// The three-character SAP system ID (SID) of the SAP HANA system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANASID")]
    pub hanasid: String,


    /// 
    /// The AWS Secrets Manager secret that stores HANA monitoring user credentials. The HANA Prometheus exporter uses these credentials to connect to the database and query HANA metrics.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HANASecretName")]
    pub hanasecret_name: String,


    /// 
    /// The target port to which Prometheus sends metrics. If not specified, the default port 9668 is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrometheusPort")]
    pub prometheus_port: Option<String>,

}




/// The AWS::ApplicationInsights::Application JMXPrometheusExporter property type       defines the JMXPrometheus Exporter configuration. For more information, see the      component configuration in the CloudWatch Application Insights    documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub host_port: Option<String>,


    /// 
    /// The complete JMX URL to connect to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JMXURL")]
    pub jmxurl: Option<String>,


    /// 
    /// The target port to send Prometheus metrics to. If not specified, the default port 9404 is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrometheusPort")]
    pub prometheus_port: Option<String>,

}




/// The AWS::ApplicationInsights::Application Log property type specifies a log to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub encoding: Option<String>,


    /// 
    /// The CloudWatch log group name to be associated with the monitored log.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,


    /// 
    /// The path of the logs to be monitored. The log path must be an absolute Windows or Linux system file path. For more information, see CloudWatch Agent Configuration File: Logs Section.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPath")]
    pub log_path: Option<String>,


    /// 
    /// The log type decides the log patterns against which Application Insights analyzes the       log. The log type is selected from the following: SQL_SERVER,         MYSQL, MYSQL_SLOW_QUERY, POSTGRESQL,         ORACLE_ALERT, ORACLE_LISTENER, IIS,         APPLICATION, WINDOWS_EVENTS,         WINDOWS_EVENTS_ACTIVE_DIRECTORY, WINDOWS_EVENTS_DNS ,         WINDOWS_EVENTS_IIS , WINDOWS_EVENTS_SHAREPOINT,         SQL_SERVER_ALWAYSON_AVAILABILITY_GROUP,         SQL_SERVER_FAILOVER_CLUSTER_INSTANCE, STEP_FUNCTION,         API_GATEWAY_ACCESS, API_GATEWAY_EXECUTION,         SAP_HANA_LOGS, SAP_HANA_TRACE,         SAP_HANA_HIGH_AVAILABILITY, and DEFAULT.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogType")]
    pub log_type: String,


    /// 
    /// The log pattern set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternSet")]
    pub pattern_set: Option<String>,

}




/// The AWS::ApplicationInsights::Application LogPattern property type       specifies an object that defines the log patterns that belong to a         LogPatternSet.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub pattern: String,


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
    pub pattern_name: String,


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




/// The AWS::ApplicationInsights::Application LogPatternSet property type specifies the log pattern set.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub pattern_set_name: String,

}




/// The AWS::ApplicationInsights::Application SubComponentConfigurationDetails property type specifies the configuration settings of the sub-components.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub windows_events: Option<Vec<WindowsEvent>>,

}




/// The AWS::ApplicationInsights::Application SubComponentTypeConfiguration property type specifies the sub-component configurations for a component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub sub_component_type: String,

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




/// The AWS::ApplicationInsights::Application WindowsEvent property type specifies a Windows Event to monitor for the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub event_name: String,


    /// 
    /// The CloudWatch log group name to be associated with the monitored log.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,


    /// 
    /// The log pattern set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternSet")]
    pub pattern_set: Option<String>,

}


