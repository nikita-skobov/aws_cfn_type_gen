/// Use the AWS::NetworkFirewall::LoggingConfiguration to define the destinations and logging options for an AWS::NetworkFirewall::Firewall.
///
/// You must change the logging configuration by changing one LogDestinationConfig setting at a time in your LogDestinationConfigs.
///
/// You can make only one of the following changes to your AWS::NetworkFirewall::LoggingConfiguration resource:
///
/// You can't change the LogDestinationType or LogType in a       LogDestinationConfig. To change these settings, delete the existing       LogDestinationConfig object and create a new one, in two separate modifications.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoggingConfiguration {
    ///
    /// The Amazon Resource Name (ARN) of the AWS::NetworkFirewall::Firewall that the logging configuration is associated with.       You can't change the firewall specification after you create the logging configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirewallArn")]
    pub firewall_arn: cfn_resources::StrVal,

    ///
    /// The name of the firewall that the logging configuration is associated with.       You can't change the firewall specification after you create the logging configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirewallName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<cfn_resources::StrVal>,

    ///
    /// Defines how AWS Network Firewall performs logging for a AWS::NetworkFirewall::Firewall.
    ///
    /// Required: Yes
    ///
    /// Type: LoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Box<LoggingConfiguration>,
}

impl cfn_resources::CfnResource for CfnLoggingConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkFirewall::LoggingConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.logging_configuration.validate()?;

        Ok(())
    }
}

/// Defines where AWS Network Firewall sends logs for the firewall for one log type. This is used     in AWS::NetworkFirewall::LoggingConfiguration. You can send each type of log to an Amazon S3 bucket, a CloudWatch log group, or a Kinesis Data Firehose delivery stream.
///
/// Network Firewall generates logs for stateful rule groups. You can save alert and flow log      types. The stateful rules engine records flow logs for all network traffic that it receives.      It records alert logs for traffic that matches stateful rules that have the rule      action set to DROP or ALERT.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogDestinationConfig {
    ///
    /// The named location for the logs, provided in a key:value mapping that is specific to the     chosen destination type.
    ///
    /// For an Amazon S3 bucket, provide the name of the bucket, with key bucketName,        and optionally provide a prefix, with key prefix. The following example        specifies an Amazon S3 bucket named        DOC-EXAMPLE-BUCKET and the prefix alerts:                  "LogDestination": { "bucketName": "DOC-EXAMPLE-BUCKET", "prefix": "alerts"          }                       For a CloudWatch log group, provide the name of the CloudWatch log group, with key          logGroup. The following example specifies a log group named          alert-log-group:                  "LogDestination": { "logGroup": "alert-log-group" }                       For a Kinesis Data Firehose delivery stream, provide the name of the delivery stream, with key          deliveryStream. The following example specifies a delivery stream        named alert-delivery-stream:                  "LogDestination": { "deliveryStream": "alert-delivery-stream"        }
    ///
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDestination")]
    pub log_destination: std::collections::HashMap<String, String>,

    ///
    /// The type of storage destination to send these logs to. You can send logs to an Amazon S3 bucket,     a CloudWatch log group, or a Kinesis Data Firehose delivery stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CloudWatchLogs | KinesisDataFirehose | S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDestinationType")]
    pub log_destination_type: LogDestinationConfigLogDestinationTypeEnum,

    ///
    /// The type of log to send. Alert logs report traffic that matches a stateful rule with an action setting that sends an alert log message. Flow logs are     standard network traffic flow logs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALERT | FLOW
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogType")]
    pub log_type: LogDestinationConfigLogTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LogDestinationConfigLogDestinationTypeEnum {
    /// CloudWatchLogs
    #[serde(rename = "CloudWatchLogs")]
    Cloudwatchlogs,

    /// KinesisDataFirehose
    #[serde(rename = "KinesisDataFirehose")]
    Kinesisdatafirehose,

    /// S3
    #[serde(rename = "S3")]
    S3,
}

impl Default for LogDestinationConfigLogDestinationTypeEnum {
    fn default() -> Self {
        LogDestinationConfigLogDestinationTypeEnum::Cloudwatchlogs
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LogDestinationConfigLogTypeEnum {
    /// ALERT
    #[serde(rename = "ALERT")]
    Alert,

    /// FLOW
    #[serde(rename = "FLOW")]
    Flow,
}

impl Default for LogDestinationConfigLogTypeEnum {
    fn default() -> Self {
        LogDestinationConfigLogTypeEnum::Alert
    }
}

impl cfn_resources::CfnResource for LogDestinationConfig {
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

/// Defines how AWS Network Firewall performs logging for a AWS::NetworkFirewall::Firewall.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfiguration {
    ///
    /// Defines the logging destinations for the logs for a firewall. Network Firewall generates     logs for stateful rule groups.
    ///
    /// Required: Yes
    ///
    /// Type: List of LogDestinationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<LogDestinationConfig>,
}

impl cfn_resources::CfnResource for LoggingConfiguration {
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
