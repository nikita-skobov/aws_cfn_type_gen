/// Creates a CloudWatch RUM app monitor, which you can use to collect telemetry data from your application       and send it to CloudWatch RUM. The data includes performance and reliability information such as       page load time, client-side errors,       and user behavior.
///
/// After you create an app monitor, sign in to the CloudWatch RUM console to get       the JavaScript code snippet to add to your web application. For more information, see       How do I find a code snippet         that I've already generated?
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAppMonitor {
    ///
    /// A structure that contains much of the configuration data for the app monitor. If you are using       Amazon Cognito for authorization, you must include this structure in your request, and it       must include the ID of the       Amazon Cognito identity pool to use for authorization. If you don't       include AppMonitorConfiguration, you must set up your own       authorization method. For more information, see       Authorize your application         to send data to AWS.
    ///
    /// If you omit this argument, the sample rate used for CloudWatch RUM is set to 10% of the user sessions.
    ///
    /// Required: No
    ///
    /// Type: AppMonitorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppMonitorConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_monitor_configuration: Option<AppMonitorConfiguration>,

    ///
    /// Specifies whether this app monitor allows the web client to define and send custom events. If you omit       this parameter, custom events are DISABLED.
    ///
    /// Required: No
    ///
    /// Type: CustomEvents
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_events: Option<CustomEvents>,

    ///
    /// Data collected by CloudWatch RUM is kept by RUM for 30 days and then deleted. This parameter specifies whether       CloudWatch RUM       sends a copy of this telemetry data to Amazon CloudWatch Logs       in your account. This enables you to keep the telemetry data for more than 30 days, but it does incur       Amazon CloudWatch Logs charges.
    ///
    /// If you omit this parameter, the default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CwLogEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cw_log_enabled: Option<bool>,

    ///
    /// The top-level internet domain name for which your application has administrative authority. This parameter is required.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: cfn_resources::StrVal,

    ///
    /// A name for the app monitor. This parameter is required.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Assigns one or more tags (key-value pairs) to the app monitor.
    ///
    /// Tags can help you organize and categorize your resources. You can also use them to scope user       permissions by granting a user       permission to access or change only resources with certain tag values.
    ///
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    ///
    /// You can associate as many as 50 tags with an app monitor.
    ///
    /// For more information, see Tagging AWS resources.
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
    pub att_id: CfnAppMonitorid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAppMonitorid;
impl CfnAppMonitorid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnAppMonitor {
    fn type_string(&self) -> &'static str {
        "AWS::RUM::AppMonitor"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.app_monitor_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.custom_events
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This structure contains much of the configuration data for the app monitor.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AppMonitorConfiguration {
    ///
    /// If you set this to true, the CloudWatch RUM web client sets two cookies, a session       cookie and a user cookie. The cookies allow the CloudWatch RUM web client to collect data relating to       the number of users an application has and the behavior of the application across a       sequence of events. Cookies are stored in the top-level domain of the current page.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cookies: Option<bool>,

    ///
    /// If you set this to true, CloudWatch RUM sends client-side traces to       X-Ray for each sampled session. You can then see traces and       segments from these user sessions       in the RUM dashboard and the CloudWatch ServiceLens console. For more information,       see What is AWS X-Ray?
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableXRay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_xray: Option<bool>,

    ///
    /// A list of URLs in your website or application to exclude from RUM data collection.
    ///
    /// You can't include both ExcludedPages and IncludedPages in the same app monitor.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_pages: Option<Vec<String>>,

    ///
    /// A list of pages in your application that are to be displayed with a "favorite" icon       in the CloudWatch RUM console.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FavoritePages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_pages: Option<Vec<String>>,

    ///
    /// The ARN of the guest IAM role that is attached to the Amazon Cognito identity pool       that is used to authorize the sending of data to CloudWatch RUM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GuestRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the Amazon Cognito identity pool       that is used to authorize the sending of data to CloudWatch RUM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<cfn_resources::StrVal>,

    ///
    /// If this app monitor is to collect data from only certain pages in your application, this structure lists those pages.
    ///
    /// You can't include both ExcludedPages and IncludedPages in the same app monitor.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_pages: Option<Vec<String>>,

    ///
    /// An array of structures that each define a destination that this app monitor will send extended metrics to.
    ///
    /// Required: No
    ///
    /// Type: List of MetricDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_destinations: Option<Vec<MetricDestination>>,

    ///
    /// Specifies the portion of user sessions to use for CloudWatch RUM data collection. Choosing a higher portion gives you       more data but also incurs more costs.
    ///
    /// The range for this value is 0 to 1 inclusive. Setting this to 1 means that 100% of user sessions are sampled, and setting       it to 0.1 means that 10% of user sessions are sampled.
    ///
    /// If you omit this parameter, the default of 0.1 is used, and 10% of sessions will be sampled.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionSampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_sample_rate: Option<f64>,

    ///
    /// An array that lists the types of telemetry data that this app monitor is to collect.
    ///
    /// errors indicates that RUM collects data about unhandled JavaScript errors raised         by your application.performance indicates that RUM collects performance data about how your application         and its resources are loaded and rendered. This includes Core Web Vitals.http indicates that RUM collects data about HTTP errors thrown by your application.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Telemetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetries: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for AppMonitorConfiguration {
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

/// This structure specifies whether this app monitor allows the web client to define and send custom events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomEvents {
    ///
    /// Set this to ENABLED to allow the web client to send custom events for this app monitor.
    ///
    /// Valid values are ENABLED and DISABLED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CustomEvents {
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

/// Specifies one custom metric or extended metric that you want the CloudWatch RUM app monitor to       send to a destination. Valid       destinations include CloudWatch and Evidently.
///
/// By default, RUM app monitors send some metrics to CloudWatch. These default metrics       are listed in CloudWatch metrics that you can collect.
///
/// In addition to these default metrics, you can choose to send extended metrics or custom metrics or both.
///
/// For information about syntax rules for specifying custom metrics and extended metrics,           see           MetridDefinitionRequest         in the CloudWatch RUM API Reference.
///
/// The maximum number of metric definitions that one destination can contain is 2000.
///
/// Extended metrics sent to CloudWatch and RUM custom metrics are charged as CloudWatch custom metrics. Each combination of additional dimension name and dimension       value counts as a custom metric.
///
/// If some metric definitions that you specify are not valid,      then the operation will not modify any metric definitions even if other metric definitions specified are valid.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricDefinition {
    ///
    /// This field is a map of field paths to dimension names. It defines the dimensions to associate with this       metric in CloudWatch. The value of this field is used only if the metric destination is CloudWatch.       If the metric destination is Evidently, the value of DimensionKeys is ignored.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_keys: Option<std::collections::HashMap<String, String>>,

    ///
    /// The pattern that defines the metric. RUM checks events that happen in a user's session       against the pattern, and events that match the pattern are sent to the metric destination.
    ///
    /// If the metrics destination       is CloudWatch and the event       also matches a value in DimensionKeys, then the metric is published with the specified dimensions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<cfn_resources::StrVal>,

    ///
    /// The name of the metric that is defined in this structure.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// If you are creating a custom metric instead of an extended metrics, use this parameter to define       the metric namespace for that custom metric. Do not specify this parameter if you are creating an extended metric.
    ///
    /// You can't use any string that starts with AWS/ for your namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,

    ///
    /// Use this field only if you are sending this metric to CloudWatch. It defines       the CloudWatch metric unit that this metric is measured in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnitLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<cfn_resources::StrVal>,

    ///
    /// The field within the event object that the metric value is sourced from.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MetricDefinition {
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

/// Creates or updates a destination to receive extended metrics from CloudWatch RUM. You can send       extended metrics to CloudWatch or to a CloudWatch Evidently experiment.
///
/// For more information about extended metrics, see             Extended metrics that you can send to CloudWatch and CloudWatch Evidently.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricDestination {
    ///
    /// Defines the destination to send the metrics to. Valid values are CloudWatch and       Evidently. If       you specify Evidently, you must also specify the ARN of the       CloudWatchEvidently experiment that is to       be the destination and an IAM role that has permission to write to the experiment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: cfn_resources::StrVal,

    ///
    /// Use this parameter only if Destination is Evidently. This parameter specifies       the ARN of the Evidently experiment that will receive the extended metrics.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<cfn_resources::StrVal>,

    ///
    /// This parameter is required if Destination is Evidently. If Destination is     CloudWatch, do not use this parameter.
    ///
    /// This parameter specifies     the ARN of an IAM role that RUM will assume to write to the Evidently     experiment that you are sending metrics to. This role must have permission to write to that experiment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// An array of structures which define the metrics that you want to send.
    ///
    /// Required: No
    ///
    /// Type: List of MetricDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,
}

impl cfn_resources::CfnResource for MetricDestination {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
