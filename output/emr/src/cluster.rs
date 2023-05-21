/// The AWS::EMR::Cluster resource specifies an Amazon EMR cluster. This cluster is a collection of Amazon EC2 instances that run open source big data frameworks and applications to process and analyze vast amounts of data. For more information, see the Amazon EMR Management Guide.
///
/// Amazon EMR now supports launching task instance groups and task instance     fleets as part of the AWS::EMR::Cluster resource. This can be done by using     the JobFlowInstancesConfig property type's TaskInstanceGroups and       TaskInstanceFleets subproperties. Using these subproperties reduces delays     in provisioning task nodes compared to specifying task nodes with the       AWS::EMR::InstanceGroupConfig and       AWS::EMR::InstanceFleetConfig resources. Please refer to the examples at     the bottom of this page to learn how to use these subproperties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {
    ///
    /// A JSON string for selecting additional features.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<serde_json::Value>,

    ///
    /// The applications to install on this cluster, for example, Spark, Flink, Oozie, Zeppelin, and so on.
    ///
    /// Required: No
    ///
    /// Type: List of Application
    ///
    /// Update requires: Replacement
    #[serde(rename = "Applications")]
    pub applications: Option<Vec<Application>>,

    ///
    /// An IAM role for automatic scaling policies. The default role is       EMR_AutoScaling_DefaultRole. The IAM role provides permissions that the     automatic scaling feature requires to launch and terminate EC2 instances in an instance     group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingRole")]
    pub auto_scaling_role: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AutoTerminationPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoTerminationPolicy")]
    pub auto_termination_policy: Option<AutoTerminationPolicy>,

    ///
    /// A list of bootstrap actions to run before Hadoop starts on the cluster nodes.
    ///
    /// Required: No
    ///
    /// Type: List of BootstrapActionConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "BootstrapActions")]
    pub bootstrap_actions: Option<Vec<BootstrapActionConfig>>,

    ///
    /// Applies only to Amazon EMR releases 4.x and later. The list of Configurations     supplied to the EMR cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,

    ///
    /// Available only in Amazon EMR version 5.7.0 and later. The ID of a custom Amazon     EBS-backed Linux AMI if the cluster uses a custom AMI.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,

    ///
    /// The size, in GiB, of the Amazon EBS root device volume of the Linux AMI that is     used for each EC2 instance. Available in Amazon EMR version 4.x and later.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsRootVolumeSize")]
    pub ebs_root_volume_size: Option<i64>,

    ///
    /// A specification of the number and type of Amazon EC2 instances.
    ///
    /// Required: Yes
    ///
    /// Type: JobFlowInstancesConfig
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Instances")]
    pub instances: JobFlowInstancesConfig,

    ///
    /// Also called instance profile and EC2 role. An IAM role for an EMR cluster. The EC2     instances of the cluster assume this role. The default role is       EMR_EC2_DefaultRole. In order to use the default role, you must have     already created it using the CLI or console.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobFlowRole")]
    pub job_flow_role: String,

    ///
    /// Attributes for Kerberos configuration when Kerberos authentication is enabled using a     security configuration. For more information see Use Kerberos Authentication     in the Amazon EMR Management Guide.
    ///
    /// Required: No
    ///
    /// Type: KerberosAttributes
    ///
    /// Update requires: Replacement
    #[serde(rename = "KerberosAttributes")]
    pub kerberos_attributes: Option<KerberosAttributes>,

    ///
    /// The AWS KMS key used for encrypting log files. This attribute is only     available with EMR version 5.30.0 and later, excluding EMR 6.0.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogEncryptionKmsKeyId")]
    pub log_encryption_kms_key_id: Option<String>,

    ///
    /// The path to the Amazon S3 location where logs for this cluster are     stored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogUri")]
    pub log_uri: Option<String>,

    ///
    /// Creates or updates a managed scaling policy for an Amazon EMR cluster. The     managed scaling policy defines the limits for resources, such as EC2 instances that can be     added or terminated from a cluster. The policy only applies to the core and task nodes. The     master node cannot be scaled after initial configuration.
    ///
    /// Required: No
    ///
    /// Type: ManagedScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedScalingPolicy")]
    pub managed_scaling_policy: Option<ManagedScalingPolicy>,

    ///
    /// The name of the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OSReleaseLabel")]
    pub osrelease_label: Option<String>,

    ///
    /// The Amazon EMR release label, which determines the version of open-source     application packages installed on the cluster. Release labels are in the form       emr-x.x.x, where x.x.x is an Amazon EMR release version such as       emr-5.14.0. For more information about Amazon EMR release versions     and included application versions and features, see https://docs.aws.amazon.com/emr/latest/ReleaseGuide/. The release label applies only to Amazon EMR     releases version 4.0 and later. Earlier versions use AmiVersion.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReleaseLabel")]
    pub release_label: Option<String>,

    ///
    /// The way that individual Amazon EC2 instances terminate when an automatic     scale-in activity occurs or an instance group is resized.       TERMINATE_AT_INSTANCE_HOUR indicates that Amazon EMR terminates     nodes at the instance-hour boundary, regardless of when the request to terminate the     instance was submitted. This option is only available with Amazon EMR 5.1.0 and     later and is the default for clusters created using that version.       TERMINATE_AT_TASK_COMPLETION indicates that Amazon EMR adds nodes     to a deny list and drains tasks from nodes before terminating the Amazon EC2     instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to     HDFS corruption. TERMINATE_AT_TASK_COMPLETION is available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: TERMINATE_AT_INSTANCE_HOUR | TERMINATE_AT_TASK_COMPLETION
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScaleDownBehavior")]
    pub scale_down_behavior: Option<ClusterScaleDownBehaviorEnum>,

    ///
    /// The name of the security configuration applied to the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,

    ///
    /// The IAM role that Amazon EMR assumes in order to access AWS     resources on your behalf.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRole")]
    pub service_role: String,

    ///
    /// Specifies the number of steps that can be executed concurrently. The default value is       1. The maximum value is 256.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepConcurrencyLevel")]
    pub step_concurrency_level: Option<i64>,

    ///
    /// A list of steps to run.
    ///
    /// Required: No
    ///
    /// Type: List of StepConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Steps")]
    pub steps: Option<Vec<StepConfig>>,

    ///
    /// A list of tags associated with a cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Indicates whether the cluster is visible to all IAM users of the AWS account associated with the cluster. If this value is set to true, all IAM users of that AWS account can view and manage the cluster if they have the proper policy permissions set.      If this value is false, only the IAM user that created the cluster can view and manage it. This value can be changed using the SetVisibleToAllUsers action.
    ///
    /// NoteWhen you create clusters directly through the EMR console or API, this value is set to true by default. However, for AWS::EMR::Cluster resources in CloudFormation, the default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibleToAllUsers")]
    pub visible_to_all_users: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterScaleDownBehaviorEnum {
    /// TERMINATE_AT_INSTANCE_HOUR
    #[serde(rename = "TERMINATE_AT_INSTANCE_HOUR")]
    Terminateatinstancehour,

    /// TERMINATE_AT_TASK_COMPLETION
    #[serde(rename = "TERMINATE_AT_TASK_COMPLETION")]
    Terminateattaskcompletion,
}

impl Default for ClusterScaleDownBehaviorEnum {
    fn default() -> Self {
        ClusterScaleDownBehaviorEnum::Terminateatinstancehour
    }
}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::Cluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.auto_scaling_role {
            if the_val.len() > 10280 as _ {
                return Err(format!(
                    "Max validation failed on field 'auto_scaling_role'. {} is greater than 10280",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.auto_scaling_role {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'auto_scaling_role'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.auto_termination_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_ami_id'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'custom_ami_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.instances.validate()?;

        let the_val = &self.job_flow_role;

        if the_val.len() > 10280 as _ {
            return Err(format!(
                "Max validation failed on field 'job_flow_role'. {} is greater than 10280",
                the_val.len()
            ));
        }

        let the_val = &self.job_flow_role;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'job_flow_role'. {} is less than 0",
                the_val.len()
            ));
        }

        self.kerberos_attributes
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.managed_scaling_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.security_configuration {
            if the_val.len() > 10280 as _ {
                return Err(format!("Max validation failed on field 'security_configuration'. {} is greater than 10280", the_val.len()));
            }
        }

        if let Some(the_val) = &self.security_configuration {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'security_configuration'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Application is a property of AWS::EMR::Cluster. The Application property type defines the open-source big data applications for EMR to install and configure when a cluster is created.
///
/// With Amazon EMR release version 4.0 and later, the only accepted parameter is the application Name. To pass arguments to these applications, you use configuration classifications specified using JSON objects in a Configuration property. For more information, see Configuring Applications.
///
/// With earlier Amazon EMR releases, the application is any AWS or third-party software that you can add to the cluster. You can specify the version of the application and arguments to pass to it. Amazon EMR accepts and forwards the argument list to the corresponding installation script as a bootstrap action argument.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Application {
    ///
    /// This option is for advanced users only. This is meta information about clusters and applications that are used for testing and troubleshooting.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<std::collections::HashMap<String, String>>,

    ///
    /// Arguments for Amazon EMR to pass to the application.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

    ///
    /// The name of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The version of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl cfn_resources::CfnResource for Application {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// AutoScalingPolicy is a subproperty of InstanceGroupConfig. AutoScalingPolicy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. For more information, see Using Automatic Scaling in Amazon EMR in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoScalingPolicy {
    ///
    /// The upper and lower EC2 instance limits for an automatic scaling policy. Automatic     scaling activity will not cause an instance group to grow above or below these     limits.
    ///
    /// Required: Yes
    ///
    /// Type: ScalingConstraints
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constraints")]
    pub constraints: ScalingConstraints,

    ///
    /// The scale-in and scale-out rules that comprise the automatic scaling policy.
    ///
    /// Required: Yes
    ///
    /// Type: List of ScalingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<ScalingRule>,
}

impl cfn_resources::CfnResource for AutoScalingPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.constraints.validate()?;

        Ok(())
    }
}

/// The AutoTerminationPolicy property type specifies Property description not available. for an AWS::EMR::Cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoTerminationPolicy {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Long
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleTimeout")]
    pub idle_timeout: Option<i64>,
}

impl cfn_resources::CfnResource for AutoTerminationPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// BootstrapActionConfig is a property of AWS::EMR::Cluster that can be used to run bootstrap actions on EMR clusters. You can use a bootstrap action to install software and configure EC2 instances for all cluster nodes before EMR installs and configures open-source big data applications on cluster instances. For more information, see Create Bootstrap Actions to Install Additional Software in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BootstrapActionConfig {
    ///
    /// The name of the bootstrap action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The script run by the bootstrap action.
    ///
    /// Required: Yes
    ///
    /// Type: ScriptBootstrapActionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScriptBootstrapAction")]
    pub script_bootstrap_action: ScriptBootstrapActionConfig,
}

impl cfn_resources::CfnResource for BootstrapActionConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 0",
                the_val.len()
            ));
        }

        self.script_bootstrap_action.validate()?;

        Ok(())
    }
}

/// CloudWatchAlarmDefinition is a subproperty of the ScalingTrigger property, which determines when to trigger an automatic scaling activity. Scaling activity begins when you satisfy the defined alarm conditions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchAlarmDefinition {
    ///
    /// Determines how the metric specified by MetricName is compared to the value     specified by Threshold.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GREATER_THAN | GREATER_THAN_OR_EQUAL | LESS_THAN | LESS_THAN_OR_EQUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: CloudWatchAlarmDefinitionComparisonOperatorEnum,

    ///
    /// A CloudWatch metric dimension.
    ///
    /// Required: No
    ///
    /// Type: List of MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,

    ///
    /// The number of periods, in five-minute increments, during which the alarm condition must     exist before the alarm triggers automatic scaling activity. The default value is       1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: Option<i64>,

    ///
    /// The name of the CloudWatch metric that is watched to determine an alarm     condition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,

    ///
    /// The namespace for the CloudWatch metric. The default is       AWS/ElasticMapReduce.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

    ///
    /// The period, in seconds, over which the statistic is applied. EMR CloudWatch metrics are     emitted every five minutes (300 seconds), so if an EMR CloudWatch metric is specified,     specify 300.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: i64,

    ///
    /// The statistic to apply to the metric associated with the alarm. The default is       AVERAGE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | MAXIMUM | MINIMUM | SAMPLE_COUNT | SUM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: Option<CloudWatchAlarmDefinitionStatisticEnum>,

    ///
    /// The value against which the specified statistic is compared.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: f64,

    ///
    /// The unit of measure associated with the CloudWatch metric being watched. The value     specified for Unit must correspond to the units specified in the CloudWatch     metric.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BITS | BITS_PER_SECOND | BYTES | BYTES_PER_SECOND | COUNT | COUNT_PER_SECOND | GIGA_BITS | GIGA_BITS_PER_SECOND | GIGA_BYTES | GIGA_BYTES_PER_SECOND | KILO_BITS | KILO_BITS_PER_SECOND | KILO_BYTES | KILO_BYTES_PER_SECOND | MEGA_BITS | MEGA_BITS_PER_SECOND | MEGA_BYTES | MEGA_BYTES_PER_SECOND | MICRO_SECONDS | MILLI_SECONDS | NONE | PERCENT | SECONDS | TERA_BITS | TERA_BITS_PER_SECOND | TERA_BYTES | TERA_BYTES_PER_SECOND
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<CloudWatchAlarmDefinitionUnitEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CloudWatchAlarmDefinitionComparisonOperatorEnum {
    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    Greaterthanorequal,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    Lessthanorequal,
}

impl Default for CloudWatchAlarmDefinitionComparisonOperatorEnum {
    fn default() -> Self {
        CloudWatchAlarmDefinitionComparisonOperatorEnum::Greaterthan
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CloudWatchAlarmDefinitionStatisticEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// MAXIMUM
    #[serde(rename = "MAXIMUM")]
    Maximum,

    /// MINIMUM
    #[serde(rename = "MINIMUM")]
    Minimum,

    /// SAMPLE_COUNT
    #[serde(rename = "SAMPLE_COUNT")]
    Samplecount,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,
}

impl Default for CloudWatchAlarmDefinitionStatisticEnum {
    fn default() -> Self {
        CloudWatchAlarmDefinitionStatisticEnum::Average
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CloudWatchAlarmDefinitionUnitEnum {
    /// BITS
    #[serde(rename = "BITS")]
    Bits,

    /// BITS_PER_SECOND
    #[serde(rename = "BITS_PER_SECOND")]
    Bitspersecond,

    /// BYTES
    #[serde(rename = "BYTES")]
    Bytes,

    /// BYTES_PER_SECOND
    #[serde(rename = "BYTES_PER_SECOND")]
    Bytespersecond,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// COUNT_PER_SECOND
    #[serde(rename = "COUNT_PER_SECOND")]
    Countpersecond,

    /// GIGA_BITS
    #[serde(rename = "GIGA_BITS")]
    Gigabits,

    /// GIGA_BITS_PER_SECOND
    #[serde(rename = "GIGA_BITS_PER_SECOND")]
    Gigabitspersecond,

    /// GIGA_BYTES
    #[serde(rename = "GIGA_BYTES")]
    Gigabytes,

    /// GIGA_BYTES_PER_SECOND
    #[serde(rename = "GIGA_BYTES_PER_SECOND")]
    Gigabytespersecond,

    /// KILO_BITS
    #[serde(rename = "KILO_BITS")]
    Kilobits,

    /// KILO_BITS_PER_SECOND
    #[serde(rename = "KILO_BITS_PER_SECOND")]
    Kilobitspersecond,

    /// KILO_BYTES
    #[serde(rename = "KILO_BYTES")]
    Kilobytes,

    /// KILO_BYTES_PER_SECOND
    #[serde(rename = "KILO_BYTES_PER_SECOND")]
    Kilobytespersecond,

    /// MEGA_BITS
    #[serde(rename = "MEGA_BITS")]
    Megabits,

    /// MEGA_BITS_PER_SECOND
    #[serde(rename = "MEGA_BITS_PER_SECOND")]
    Megabitspersecond,

    /// MEGA_BYTES
    #[serde(rename = "MEGA_BYTES")]
    Megabytes,

    /// MEGA_BYTES_PER_SECOND
    #[serde(rename = "MEGA_BYTES_PER_SECOND")]
    Megabytespersecond,

    /// MICRO_SECONDS
    #[serde(rename = "MICRO_SECONDS")]
    Microseconds,

    /// MILLI_SECONDS
    #[serde(rename = "MILLI_SECONDS")]
    Milliseconds,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// PERCENT
    #[serde(rename = "PERCENT")]
    Percent,

    /// SECONDS
    #[serde(rename = "SECONDS")]
    Seconds,

    /// TERA_BITS
    #[serde(rename = "TERA_BITS")]
    Terabits,

    /// TERA_BITS_PER_SECOND
    #[serde(rename = "TERA_BITS_PER_SECOND")]
    Terabitspersecond,

    /// TERA_BYTES
    #[serde(rename = "TERA_BYTES")]
    Terabytes,

    /// TERA_BYTES_PER_SECOND
    #[serde(rename = "TERA_BYTES_PER_SECOND")]
    Terabytespersecond,
}

impl Default for CloudWatchAlarmDefinitionUnitEnum {
    fn default() -> Self {
        CloudWatchAlarmDefinitionUnitEnum::Bits
    }
}

impl cfn_resources::CfnResource for CloudWatchAlarmDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The EC2 unit limits for a managed scaling policy. The managed scaling activity of a     cluster can not be above or below these limits. The limit only applies to the core and task     nodes. The master node cannot be scaled after initial configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComputeLimits {
    ///
    /// The upper boundary of EC2 units. It is measured through vCPU cores or instances for     instance groups and measured through units for instance fleets. Managed scaling activities     are not allowed beyond this boundary. The limit only applies to the core and task nodes.     The master node cannot be scaled after initial configuration.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumCapacityUnits")]
    pub maximum_capacity_units: i64,

    ///
    /// The upper boundary of EC2 units for core node type in a cluster. It is measured through     vCPU cores or instances for instance groups and measured through units for instance fleets.     The core units are not allowed to scale beyond this boundary. The parameter is used to     split capacity allocation between core and task nodes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumCoreCapacityUnits")]
    pub maximum_core_capacity_units: Option<i64>,

    ///
    /// The upper boundary of On-Demand EC2 units. It is measured through vCPU cores or     instances for instance groups and measured through units for instance fleets. The On-Demand     units are not allowed to scale beyond this boundary. The parameter is used to split     capacity allocation between On-Demand and Spot Instances.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumOnDemandCapacityUnits")]
    pub maximum_on_demand_capacity_units: Option<i64>,

    ///
    /// The lower boundary of EC2 units. It is measured through vCPU cores or instances for     instance groups and measured through units for instance fleets. Managed scaling activities     are not allowed beyond this boundary. The limit only applies to the core and task nodes.     The master node cannot be scaled after initial configuration.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumCapacityUnits")]
    pub minimum_capacity_units: i64,

    ///
    /// The unit type used for specifying a managed scaling policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: InstanceFleetUnits | Instances | VCPU
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnitType")]
    pub unit_type: ComputeLimitsUnitTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComputeLimitsUnitTypeEnum {
    /// InstanceFleetUnits
    #[serde(rename = "InstanceFleetUnits")]
    Instancefleetunits,

    /// Instances
    #[serde(rename = "Instances")]
    Instances,

    /// VCPU
    #[serde(rename = "VCPU")]
    Vcpu,
}

impl Default for ComputeLimitsUnitTypeEnum {
    fn default() -> Self {
        ComputeLimitsUnitTypeEnum::Instancefleetunits
    }
}

impl cfn_resources::CfnResource for ComputeLimits {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Configuration is a subproperty of InstanceFleetConfig or InstanceGroupConfig. Configuration specifies optional configurations for customizing open-source big data applications and environment parameters. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see Configuring Applications in the Amazon EMR Release Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Configuration {
    ///
    /// The classification within a configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classification")]
    pub classification: Option<String>,

    ///
    /// A list of additional configurations to apply within a configuration object.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<std::collections::HashMap<String, String>>,

    ///
    /// A list of additional configurations to apply within a configuration object.
    ///
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
}

impl cfn_resources::CfnResource for Configuration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// EbsBlockDeviceConfig is a subproperty of the EbsConfiguration property type. EbsBlockDeviceConfig defines the number and type of EBS volumes to associate with all EC2 instances in an EMR cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbsBlockDeviceConfig {
    ///
    /// EBS volume specifications such as volume type, IOPS, size (GiB) and throughput (MiB/s)     that are requested for the EBS volume attached to an EC2 instance in the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: VolumeSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,

    ///
    /// Number of EBS volumes with a specific volume configuration that are associated with     every instance in the instance group
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<i64>,
}

impl cfn_resources::CfnResource for EbsBlockDeviceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.volume_specification.validate()?;

        Ok(())
    }
}

/// EbsConfiguration is a subproperty of InstanceFleetConfig or InstanceGroupConfig. EbsConfiguration determines the EBS volumes to attach to EMR cluster instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbsConfiguration {
    ///
    /// An array of Amazon EBS volume specifications attached to a cluster     instance.
    ///
    /// Required: No
    ///
    /// Type: List of EbsBlockDeviceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,

    ///
    /// Indicates whether an Amazon EBS volume is EBS-optimized.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
}

impl cfn_resources::CfnResource for EbsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The HadoopJarStepConfig property type specifies a job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for the cluster to execute as a step on the master node, and then waits for the job to finish or fail before executing subsequent steps.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HadoopJarStepConfig {
    ///
    /// A list of command line arguments passed to the JAR file's main function when     executed.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

    ///
    /// A path to a JAR file run during the step.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Jar")]
    pub jar: String,

    ///
    /// The name of the main class in the specified Java file. If not specified, the JAR file     should specify a Main-Class in its manifest file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MainClass")]
    pub main_class: Option<String>,

    ///
    /// A list of Java properties that are set when the step runs. You can use these properties     to pass key-value pairs to your main function.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepProperties")]
    pub step_properties: Option<Vec<KeyValue>>,
}

impl cfn_resources::CfnResource for HadoopJarStepConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.jar;

        if the_val.len() > 10280 as _ {
            return Err(format!(
                "Max validation failed on field 'jar'. {} is greater than 10280",
                the_val.len()
            ));
        }

        let the_val = &self.jar;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'jar'. {} is less than 0",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.main_class {
            if the_val.len() > 10280 as _ {
                return Err(format!(
                    "Max validation failed on field 'main_class'. {} is greater than 10280",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.main_class {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'main_class'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Use InstanceFleetConfig to define instance fleets for an EMR cluster. A cluster can not use both instance fleets and instance groups. For more information, see Configure Instance Fleets in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceFleetConfig {
    ///
    /// The instance type configurations that define the EC2 instances in the instance     fleet.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceTypeConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceTypeConfigs")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,

    ///
    /// The launch specification for the instance fleet.
    ///
    /// Required: No
    ///
    /// Type: InstanceFleetProvisioningSpecifications
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchSpecifications")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,

    ///
    /// The friendly name of the instance fleet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision. When the instance fleet launches, Amazon EMR tries to provision On-Demand instances as specified by InstanceTypeConfig. Each instance configuration has a specified WeightedCapacity. When an On-Demand instance is provisioned, the WeightedCapacity units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a WeightedCapacity of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.
    ///
    /// NoteIf not specified or set to 0, only Spot instances are provisioned for the instance fleet using TargetSpotCapacity. At least one of TargetSpotCapacity and TargetOnDemandCapacity should be greater than 0. For a master instance fleet, only one of TargetSpotCapacity and TargetOnDemandCapacity can be specified, and its value must be 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetOnDemandCapacity")]
    pub target_on_demand_capacity: Option<i64>,

    ///
    /// The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision. When the instance fleet launches, Amazon EMR tries to provision Spot instances as specified by InstanceTypeConfig. Each instance configuration has a specified WeightedCapacity. When a Spot instance is provisioned, the WeightedCapacity units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a WeightedCapacity of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.
    ///
    /// NoteIf not specified or set to 0, only On-Demand instances are provisioned for the instance fleet. At least one of TargetSpotCapacity and TargetOnDemandCapacity should be greater than 0. For a master instance fleet, only one of TargetSpotCapacity and TargetOnDemandCapacity can be specified, and its value must be 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetSpotCapacity")]
    pub target_spot_capacity: Option<i64>,
}

impl cfn_resources::CfnResource for InstanceFleetConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.launch_specifications
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.target_on_demand_capacity {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_on_demand_capacity'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.target_spot_capacity {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_spot_capacity'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// InstanceFleetProvisioningSpecification is a subproperty of InstanceFleetConfig. InstanceFleetProvisioningSpecification defines the launch specification for Spot instances in an instance fleet, which determines the defined duration and provisioning timeout behavior for Spot instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceFleetProvisioningSpecifications {
    ///
    /// The launch specification for On-Demand Instances in the instance fleet, which     determines the allocation strategy.
    ///
    /// NoteThe instance fleet configuration is available only in Amazon EMR versions       4.8.0 and later, excluding 5.0.x versions. On-Demand Instances allocation strategy is       available in Amazon EMR version 5.12.1 and later.
    ///
    /// Required: No
    ///
    /// Type: OnDemandProvisioningSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnDemandSpecification")]
    pub on_demand_specification: Option<OnDemandProvisioningSpecification>,

    ///
    /// The launch specification for Spot instances in the fleet, which determines the defined     duration, provisioning timeout behavior, and allocation strategy.
    ///
    /// Required: No
    ///
    /// Type: SpotProvisioningSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotSpecification")]
    pub spot_specification: Option<SpotProvisioningSpecification>,
}

impl cfn_resources::CfnResource for InstanceFleetProvisioningSpecifications {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.on_demand_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.spot_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use InstanceGroupConfig to define instance groups for an EMR cluster. A cluster can not use both instance groups and instance fleets. For more information, see Create a Cluster with Instance Fleets or Uniform Instance Groups in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceGroupConfig {
    ///
    /// AutoScalingPolicy is a subproperty of the InstanceGroupConfig property type that specifies the constraints and rules of an automatic scaling policy in Amazon EMR. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. Only core and task instance groups can use automatic scaling policies. For more information, see Using Automatic Scaling in Amazon EMR.
    ///
    /// Required: No
    ///
    /// Type: AutoScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,

    ///
    /// If specified, indicates that the instance group uses Spot Instances. This is the maximum price you are willing to pay for Spot Instances. Specify OnDemandPrice to set the amount equal to the On-Demand price, or specify an amount in USD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,

    ///
    /// NoteAmazon EMR releases 4.x or later.
    ///
    /// The list of configurations supplied for an EMR cluster instance group. You can specify a     separate configuration for each instance group (master, core, and task).
    ///
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,

    ///
    /// The custom AMI ID to use for the provisioned instance group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,

    ///
    /// EBS configurations that will be attached to each EC2 instance in the instance     group.
    ///
    /// Required: No
    ///
    /// Type: EbsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,

    ///
    /// Target number of instances for the instance group.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

    ///
    /// The EC2 instance type for all instances in the instance group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

    ///
    /// Market type of the EC2 instances used to create a cluster node.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: Replacement
    #[serde(rename = "Market")]
    pub market: Option<InstanceGroupConfigMarketEnum>,

    ///
    /// Friendly name given to the instance group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceGroupConfigMarketEnum {
    /// ON_DEMAND
    #[serde(rename = "ON_DEMAND")]
    Ondemand,

    /// SPOT
    #[serde(rename = "SPOT")]
    Spot,
}

impl Default for InstanceGroupConfigMarketEnum {
    fn default() -> Self {
        InstanceGroupConfigMarketEnum::Ondemand
    }
}

impl cfn_resources::CfnResource for InstanceGroupConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_scaling_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.bid_price {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'bid_price'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.bid_price {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'bid_price'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_ami_id'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'custom_ami_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.ebs_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.instance_type;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_type'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.instance_type;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'instance_type'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// InstanceTypeConfig is a sub-property of InstanceFleetConfig. InstanceTypeConfig determines the EC2 instances that Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceTypeConfig {
    ///
    /// The bid price for each EC2 Spot Instance type as defined by InstanceType.     Expressed in USD. If neither BidPrice nor       BidPriceAsPercentageOfOnDemandPrice is provided,       BidPriceAsPercentageOfOnDemandPrice defaults to 100%.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,

    ///
    /// The bid price, as a percentage of On-Demand price, for each EC2 Spot Instance as defined     by InstanceType. Expressed as a number (for example, 20 specifies 20%). If     neither BidPrice nor BidPriceAsPercentageOfOnDemandPrice is     provided, BidPriceAsPercentageOfOnDemandPrice defaults to 100%.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,

    ///
    /// A configuration classification that applies when provisioning cluster instances, which     can include configurations for applications and software that run on the cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,

    ///
    /// The custom AMI ID to use for the instance type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,

    ///
    /// The configuration of Amazon Elastic Block Store (Amazon EBS) attached to each     instance as defined by InstanceType.
    ///
    /// Required: No
    ///
    /// Type: EbsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,

    ///
    /// An EC2 instance type, such as m3.xlarge.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

    ///
    /// The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in InstanceFleetConfig. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: Replacement
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<i64>,
}

impl cfn_resources::CfnResource for InstanceTypeConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.bid_price {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'bid_price'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.bid_price {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'bid_price'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_ami_id'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'custom_ami_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.ebs_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.instance_type;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_type'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.instance_type;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'instance_type'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.weighted_capacity {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'weighted_capacity'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// JobFlowInstancesConfig is a property of the AWS::EMR::Cluster resource. JobFlowInstancesConfig defines the instance groups or instance fleets that comprise the cluster. JobFlowInstancesConfig must contain either InstanceFleetConfig or InstanceGroupConfig. They cannot be used together.
///
/// You can now define task instance groups or task instance fleets using the       TaskInstanceGroups and TaskInstanceFleets subproperties. Using     these subproperties reduces delays in provisioning task nodes compared to specifying task     nodes with the InstanceFleetConfig and InstanceGroupConfig     resources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JobFlowInstancesConfig {
    ///
    /// A list of additional Amazon EC2 security group IDs for the master node.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    pub additional_master_security_groups: Option<Vec<String>>,

    ///
    /// A list of additional Amazon EC2 security group IDs for the core and task     nodes.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    pub additional_slave_security_groups: Option<Vec<String>>,

    ///
    /// Describes the EC2 instances and instance configurations for the core instance fleet when using clusters with the instance fleet configuration.
    ///
    /// Required: No
    ///
    /// Type: InstanceFleetConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreInstanceFleet")]
    pub core_instance_fleet: Option<InstanceFleetConfig>,

    ///
    /// Describes the EC2 instances and instance configurations for core instance groups when using clusters with the uniform instance group configuration.
    ///
    /// Required: No
    ///
    /// Type: InstanceGroupConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreInstanceGroup")]
    pub core_instance_group: Option<InstanceGroupConfig>,

    ///
    /// The name of the EC2 key pair that can be used to connect to the master node using SSH as     the user called "hadoop."
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2KeyName")]
    pub ec2_key_name: Option<String>,

    ///
    /// Applies to clusters that use the uniform instance group configuration. To launch the     cluster in Amazon Virtual Private Cloud (Amazon VPC), set this parameter to the     identifier of the Amazon VPC subnet where you want the cluster to launch. If you do     not specify this value and your account supports EC2-Classic, the cluster launches in     EC2-Classic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2SubnetId")]
    pub ec2_subnet_id: Option<String>,

    ///
    /// Applies to clusters that use the instance fleet configuration. When multiple EC2 subnet     IDs are specified, Amazon EMR evaluates them and launches instances in the optimal     subnet.
    ///
    /// NoteThe instance fleet configuration is available only in Amazon EMR versions       4.8.0 and later, excluding 5.0.x versions.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2SubnetIds")]
    pub ec2_subnet_ids: Option<Vec<String>>,

    ///
    /// The identifier of the Amazon EC2 security group for the master node. If you     specify EmrManagedMasterSecurityGroup, you must also specify       EmrManagedSlaveSecurityGroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    pub emr_managed_master_security_group: Option<String>,

    ///
    /// The identifier of the Amazon EC2 security group for the core and task nodes. If     you specify EmrManagedSlaveSecurityGroup, you must also specify       EmrManagedMasterSecurityGroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    pub emr_managed_slave_security_group: Option<String>,

    ///
    /// Applies only to Amazon EMR release versions earlier than 4.0. The Hadoop version     for the cluster. Valid inputs are "0.18" (no longer maintained), "0.20" (no longer     maintained), "0.20.205" (no longer maintained), "1.0.3", "2.2.0", or "2.4.0". If you do not     set this value, the default of 0.18 is used, unless the AmiVersion parameter     is set in the RunJobFlow call, in which case the default version of Hadoop for that AMI     version is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "HadoopVersion")]
    pub hadoop_version: Option<String>,

    ///
    /// Specifies whether the cluster should remain available after completing all steps.     Defaults to true. For more information about configuring cluster termination,     see Control Cluster Termination in the EMR Management     Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,

    ///
    /// Describes the EC2 instances and instance configurations for the master instance fleet when using clusters with the instance fleet configuration.
    ///
    /// Required: No
    ///
    /// Type: InstanceFleetConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterInstanceFleet")]
    pub master_instance_fleet: Option<InstanceFleetConfig>,

    ///
    /// Describes the EC2 instances and instance configurations for the master instance group when using clusters with the uniform instance group configuration.
    ///
    /// Required: No
    ///
    /// Type: InstanceGroupConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterInstanceGroup")]
    pub master_instance_group: Option<InstanceGroupConfig>,

    ///
    /// The Availability Zone in which the cluster runs.
    ///
    /// Required: No
    ///
    /// Type: PlacementType
    ///
    /// Update requires: Replacement
    #[serde(rename = "Placement")]
    pub placement: Option<PlacementType>,

    ///
    /// The identifier of the Amazon EC2 security group for the Amazon EMR     service to access clusters in VPC private subnets.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceAccessSecurityGroup")]
    pub service_access_security_group: Option<String>,

    ///
    /// Describes the EC2 instances and instance configurations for the task instance fleets when using clusters with the instance fleet configuration. These task instance fleets are added to the cluster as part of the cluster launch. Each task instance fleet must have a unique name specified so that CloudFormation can differentiate between the task instance fleets.
    ///
    /// NoteYou can currently specify only one task instance fleet for a cluster. After creating the cluster, you can only modify the mutable properties of InstanceFleetConfig, which are TargetOnDemandCapacity and TargetSpotCapacity. Modifying any other property results in cluster replacement.
    ///
    /// ImportantTo allow a maximum of 30 Amazon EC2 instance types per fleet, include TaskInstanceFleets when you create your cluster.     If you create your cluster without TaskInstanceFleets, Amazon EMR uses its default allocation strategy, which allows for a maximum of five Amazon EC2 instance types.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceFleetConfig
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "TaskInstanceFleets")]
    pub task_instance_fleets: Option<Vec<InstanceFleetConfig>>,

    ///
    /// Describes the EC2 instances and instance configurations for task instance groups when using clusters with the uniform instance group configuration. These task instance groups are added to the cluster as part of the cluster launch. Each task instance group must have a unique name specified so that CloudFormation can differentiate between the task instance groups.
    ///
    /// NoteAfter creating the cluster, you can only modify the mutable properties of InstanceGroupConfig, which are AutoScalingPolicy and InstanceCount. Modifying any other property results in cluster replacement.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceGroupConfig
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "TaskInstanceGroups")]
    pub task_instance_groups: Option<Vec<InstanceGroupConfig>>,

    ///
    /// Specifies whether to lock the cluster to prevent the Amazon EC2 instances from     being terminated by API call, user intervention, or in the event of a job-flow     error.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminationProtected")]
    pub termination_protected: Option<bool>,
}

impl cfn_resources::CfnResource for JobFlowInstancesConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.core_instance_fleet
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.core_instance_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.ec2_key_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'ec2_key_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.ec2_key_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'ec2_key_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.ec2_subnet_id {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'ec2_subnet_id'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.ec2_subnet_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'ec2_subnet_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.emr_managed_master_security_group {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'emr_managed_master_security_group'. {} is greater than 256", the_val.len()));
            }
        }

        if let Some(the_val) = &self.emr_managed_master_security_group {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'emr_managed_master_security_group'. {} is less than 0", the_val.len()));
            }
        }

        if let Some(the_val) = &self.emr_managed_slave_security_group {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'emr_managed_slave_security_group'. {} is greater than 256", the_val.len()));
            }
        }

        if let Some(the_val) = &self.emr_managed_slave_security_group {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'emr_managed_slave_security_group'. {} is less than 0", the_val.len()));
            }
        }

        if let Some(the_val) = &self.hadoop_version {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'hadoop_version'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.hadoop_version {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'hadoop_version'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.master_instance_fleet
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.master_instance_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.placement
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.service_access_security_group {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'service_access_security_group'. {} is greater than 256", the_val.len()));
            }
        }

        if let Some(the_val) = &self.service_access_security_group {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'service_access_security_group'. {} is less than 0", the_val.len()));
            }
        }

        Ok(())
    }
}

/// KerberosAttributes is a property of the AWS::EMR::Cluster resource. KerberosAttributes define the cluster-specific Kerberos configuration when Kerberos authentication is enabled using a security configuration. The cluster-specific configuration must be compatible with the security configuration. For more information see Use Kerberos Authentication in the EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KerberosAttributes {
    ///
    /// The Active Directory password for ADDomainJoinUser.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ADDomainJoinPassword")]
    pub addomain_join_password: Option<String>,

    ///
    /// Required only when establishing a cross-realm trust with an Active Directory domain. A     user with sufficient privileges to join resources to the domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ADDomainJoinUser")]
    pub addomain_join_user: Option<String>,

    ///
    /// Required only when establishing a cross-realm trust with a KDC in a different realm. The     cross-realm principal password, which must be identical across realms.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossRealmTrustPrincipalPassword")]
    pub cross_realm_trust_principal_password: Option<String>,

    ///
    /// The password used within the cluster for the kadmin service on the cluster-dedicated     KDC, which maintains Kerberos principals, password policies, and keytabs for the     cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KdcAdminPassword")]
    pub kdc_admin_password: String,

    ///
    /// The name of the Kerberos realm to which all nodes in a cluster belong. For example,       EC2.INTERNAL.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Realm")]
    pub realm: String,
}

impl cfn_resources::CfnResource for KerberosAttributes {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.addomain_join_password {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'addomain_join_password'. {} is greater than 256", the_val.len()));
            }
        }

        if let Some(the_val) = &self.addomain_join_password {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'addomain_join_password'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.addomain_join_user {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'addomain_join_user'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.addomain_join_user {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'addomain_join_user'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.cross_realm_trust_principal_password {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'cross_realm_trust_principal_password'. {} is greater than 256", the_val.len()));
            }
        }

        if let Some(the_val) = &self.cross_realm_trust_principal_password {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'cross_realm_trust_principal_password'. {} is less than 0", the_val.len()));
            }
        }

        let the_val = &self.kdc_admin_password;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'kdc_admin_password'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.kdc_admin_password;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'kdc_admin_password'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.realm;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'realm'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.realm;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'realm'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// KeyValue is a subproperty of the HadoopJarStepConfig property type. KeyValue is used to pass parameters to a step.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeyValue {
    ///
    /// The unique identifier of a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

    ///
    /// The value part of the identified key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl cfn_resources::CfnResource for KeyValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.key {
            if the_val.len() > 10280 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 10280",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.key {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.value {
            if the_val.len() > 10280 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 10280",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.value {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Managed scaling policy for an Amazon EMR cluster. The policy specifies the     limits for resources that can be added or terminated from a cluster. The policy only     applies to the core and task nodes. The master node cannot be scaled after initial     configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ManagedScalingPolicy {
    ///
    /// The EC2 unit limits for a managed scaling policy. The managed scaling activity of a     cluster is not allowed to go above or below these limits. The limit only applies to the     core and task nodes. The master node cannot be scaled after initial configuration.
    ///
    /// Required: No
    ///
    /// Type: ComputeLimits
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeLimits")]
    pub compute_limits: Option<ComputeLimits>,
}

impl cfn_resources::CfnResource for ManagedScalingPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.compute_limits
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MetricDimension is a subproperty of the CloudWatchAlarmDefinition property type. MetricDimension specifies a CloudWatch dimension, which is specified with a Key Value pair. The key is known as a Name in CloudWatch. By default, Amazon EMR uses one dimension whose Key is JobFlowID and Value is a variable representing the cluster ID, which is ${emr.clusterId}. This enables the automatic scaling rule for EMR to bootstrap when the cluster ID becomes available during cluster creation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDimension {
    ///
    /// The dimension name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The dimension value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for MetricDimension {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The launch specification for On-Demand Instances in the instance fleet, which     determines the allocation strategy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnDemandProvisioningSpecification {
    ///
    /// Specifies the strategy to use in launching On-Demand instance fleets. Currently, the     only option is lowest-price (the default), which launches the lowest price     first.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: lowest-price
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: OnDemandProvisioningSpecificationAllocationStrategyEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OnDemandProvisioningSpecificationAllocationStrategyEnum {
    /// lowest-price
    #[serde(rename = "lowest-price")]
    Lowestprice,
}

impl Default for OnDemandProvisioningSpecificationAllocationStrategyEnum {
    fn default() -> Self {
        OnDemandProvisioningSpecificationAllocationStrategyEnum::Lowestprice
    }
}

impl cfn_resources::CfnResource for OnDemandProvisioningSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// PlacementType is a property of the AWS::EMR::Cluster resource. PlacementType determines the Amazon EC2 Availability Zone configuration of the cluster (job flow).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PlacementType {
    ///
    /// The Amazon EC2 Availability Zone for the cluster. AvailabilityZone     is used for uniform instance groups, while AvailabilityZones (plural) is used     for instance fleets.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
}

impl cfn_resources::CfnResource for PlacementType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.availability_zone;

        if the_val.len() > 10280 as _ {
            return Err(format!(
                "Max validation failed on field 'availability_zone'. {} is greater than 10280",
                the_val.len()
            ));
        }

        let the_val = &self.availability_zone;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'availability_zone'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// ScalingAction is a subproperty of the ScalingRule property type. ScalingAction determines the type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingAction {
    ///
    /// Not available for instance groups. Instance groups use the market type specified for the     group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Market")]
    pub market: Option<ScalingActionMarketEnum>,

    ///
    /// The type of adjustment the automatic scaling activity makes when triggered, and the     periodicity of the adjustment.
    ///
    /// Required: Yes
    ///
    /// Type: SimpleScalingPolicyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingActionMarketEnum {
    /// ON_DEMAND
    #[serde(rename = "ON_DEMAND")]
    Ondemand,

    /// SPOT
    #[serde(rename = "SPOT")]
    Spot,
}

impl Default for ScalingActionMarketEnum {
    fn default() -> Self {
        ScalingActionMarketEnum::Ondemand
    }
}

impl cfn_resources::CfnResource for ScalingAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.simple_scaling_policy_configuration.validate()?;

        Ok(())
    }
}

/// ScalingConstraints is a subproperty of the AutoScalingPolicy property type. ScalingConstraints defines the upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or shrink below these limits.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingConstraints {
    ///
    /// The upper boundary of EC2 instances in an instance group beyond which scaling activities     are not allowed to grow. Scale-out activities will not add instances beyond this     boundary.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,

    ///
    /// The lower boundary of EC2 instances in an instance group below which scaling activities     are not allowed to shrink. Scale-in activities will not terminate instances below this     boundary.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
}

impl cfn_resources::CfnResource for ScalingConstraints {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ScalingRule is a subproperty of the AutoScalingPolicy property type. ScalingRule defines the scale-in or scale-out rules for scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingRule {
    ///
    /// The conditions that trigger an automatic scaling activity.
    ///
    /// Required: Yes
    ///
    /// Type: ScalingAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: ScalingAction,

    ///
    /// A friendly, more verbose description of the automatic scaling rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name used to identify an automatic scaling rule. Rule names must be unique within a     scaling policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The CloudWatch alarm definition that determines when automatic scaling activity is     triggered.
    ///
    /// Required: Yes
    ///
    /// Type: ScalingTrigger
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trigger")]
    pub trigger: ScalingTrigger,
}

impl cfn_resources::CfnResource for ScalingRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

        self.trigger.validate()?;

        Ok(())
    }
}

/// ScalingTrigger is a subproperty of the ScalingRule property type. ScalingTrigger determines the conditions that trigger an automatic scaling activity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingTrigger {
    ///
    /// The definition of a CloudWatch metric alarm. When the defined alarm conditions are met     along with other trigger parameters, scaling activity begins.
    ///
    /// Required: Yes
    ///
    /// Type: CloudWatchAlarmDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
}

impl cfn_resources::CfnResource for ScalingTrigger {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_alarm_definition.validate()?;

        Ok(())
    }
}

/// ScriptBootstrapActionConfig is a subproperty of the BootstrapActionConfig property type. ScriptBootstrapActionConfig specifies the arguments and location of the bootstrap script for EMR to run on all cluster nodes before it installs open-source big data applications on them.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScriptBootstrapActionConfig {
    ///
    /// A list of command line arguments to pass to the bootstrap action script.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

    ///
    /// Location in Amazon S3 of the script to run during a bootstrap action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: String,
}

impl cfn_resources::CfnResource for ScriptBootstrapActionConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.path;

        if the_val.len() > 10280 as _ {
            return Err(format!(
                "Max validation failed on field 'path'. {} is greater than 10280",
                the_val.len()
            ));
        }

        let the_val = &self.path;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'path'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// SimpleScalingPolicyConfiguration is a subproperty of the ScalingAction property type. SimpleScalingPolicyConfiguration determines how an automatic scaling action adds or removes instances, the cooldown period, and the number of EC2 instances that are added each time the CloudWatch metric alarm condition is satisfied.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SimpleScalingPolicyConfiguration {
    ///
    /// The way in which EC2 instances are added (if ScalingAdjustment is a     positive number) or terminated (if ScalingAdjustment is a negative number)     each time the scaling activity is triggered. CHANGE_IN_CAPACITY is the     default. CHANGE_IN_CAPACITY indicates that the EC2 instance count increments     or decrements by ScalingAdjustment, which should be expressed as an integer.       PERCENT_CHANGE_IN_CAPACITY indicates the instance count increments or     decrements by the percentage specified by ScalingAdjustment, which should be     expressed as an integer. For example, 20 indicates an increase in 20% increments of cluster     capacity. EXACT_CAPACITY indicates the scaling activity results in an instance     group with the number of EC2 instances specified by ScalingAdjustment, which     should be expressed as a positive integer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CHANGE_IN_CAPACITY | EXACT_CAPACITY | PERCENT_CHANGE_IN_CAPACITY
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<SimpleScalingPolicyConfigurationAdjustmentTypeEnum>,

    ///
    /// The amount of time, in seconds, after a scaling activity completes before any further     trigger-related scaling activities can start. The default value is 0.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoolDown")]
    pub cool_down: Option<i64>,

    ///
    /// The amount by which to scale in or scale out, based on the specified       AdjustmentType. A positive value adds to the instance group's EC2 instance     count while a negative number removes instances. If AdjustmentType is set to       EXACT_CAPACITY, the number should only be a positive integer. If       AdjustmentType is set to PERCENT_CHANGE_IN_CAPACITY, the value     should express the percentage as an integer. For example, -20 indicates a decrease in 20%     increments of cluster capacity.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SimpleScalingPolicyConfigurationAdjustmentTypeEnum {
    /// CHANGE_IN_CAPACITY
    #[serde(rename = "CHANGE_IN_CAPACITY")]
    Changeincapacity,

    /// EXACT_CAPACITY
    #[serde(rename = "EXACT_CAPACITY")]
    Exactcapacity,

    /// PERCENT_CHANGE_IN_CAPACITY
    #[serde(rename = "PERCENT_CHANGE_IN_CAPACITY")]
    Percentchangeincapacity,
}

impl Default for SimpleScalingPolicyConfigurationAdjustmentTypeEnum {
    fn default() -> Self {
        SimpleScalingPolicyConfigurationAdjustmentTypeEnum::Changeincapacity
    }
}

impl cfn_resources::CfnResource for SimpleScalingPolicyConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// SpotProvisioningSpecification is a subproperty of the InstanceFleetProvisioningSpecifications property type. SpotProvisioningSpecification determines the launch specification for Spot instances in the instance fleet, which includes the defined duration and provisioning timeout behavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpotProvisioningSpecification {
    ///
    /// Specifies the strategy to use in launching Spot Instance fleets. Currently, the only     option is capacity-optimized (the default), which launches instances from Spot Instance     pools with optimal capacity for the number of instances that are launching.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-optimized
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<SpotProvisioningSpecificationAllocationStrategyEnum>,

    ///
    /// The defined duration for Spot Instances (also known as Spot blocks) in minutes. When     specified, the Spot Instance does not terminate before the defined duration expires, and     defined duration pricing for Spot Instances applies. Valid values are 60, 120, 180, 240,     300, or 360. The duration period starts as soon as a Spot Instance receives its instance     ID. At the end of the duration, Amazon EC2 marks the Spot Instance for termination     and provides a Spot Instance termination notice, which gives the instance a two-minute     warning before it terminates.
    ///
    /// NoteSpot Instances with a defined duration (also known as Spot blocks) are no longer       available to new customers from July 1, 2021. For customers who have previously used the       feature, we will continue to support Spot Instances with a defined duration until       December 31, 2022.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockDurationMinutes")]
    pub block_duration_minutes: Option<i64>,

    ///
    /// The action to take when TargetSpotCapacity has not been fulfilled when the       TimeoutDurationMinutes has expired; that is, when all Spot Instances could     not be provisioned within the Spot provisioning timeout. Valid values are       TERMINATE_CLUSTER and SWITCH_TO_ON_DEMAND. SWITCH_TO_ON_DEMAND     specifies that if no Spot Instances are available, On-Demand Instances should be     provisioned to fulfill any remaining Spot capacity.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SWITCH_TO_ON_DEMAND | TERMINATE_CLUSTER
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: SpotProvisioningSpecificationTimeoutActionEnum,

    ///
    /// The Spot provisioning timeout period in minutes. If Spot Instances are not provisioned     within this time period, the TimeOutAction is taken. Minimum value is 5 and     maximum value is 1440. The timeout applies only during initial provisioning, when the     cluster is first created.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutDurationMinutes")]
    pub timeout_duration_minutes: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SpotProvisioningSpecificationAllocationStrategyEnum {
    /// capacity-optimized
    #[serde(rename = "capacity-optimized")]
    Capacityoptimized,
}

impl Default for SpotProvisioningSpecificationAllocationStrategyEnum {
    fn default() -> Self {
        SpotProvisioningSpecificationAllocationStrategyEnum::Capacityoptimized
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SpotProvisioningSpecificationTimeoutActionEnum {
    /// SWITCH_TO_ON_DEMAND
    #[serde(rename = "SWITCH_TO_ON_DEMAND")]
    Switchtoondemand,

    /// TERMINATE_CLUSTER
    #[serde(rename = "TERMINATE_CLUSTER")]
    Terminatecluster,
}

impl Default for SpotProvisioningSpecificationTimeoutActionEnum {
    fn default() -> Self {
        SpotProvisioningSpecificationTimeoutActionEnum::Switchtoondemand
    }
}

impl cfn_resources::CfnResource for SpotProvisioningSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.block_duration_minutes {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'block_duration_minutes'. {} is less than 0",
                    the_val
                ));
            }
        }

        let the_val = &self.timeout_duration_minutes;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'timeout_duration_minutes'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// StepConfig is a property of the AWS::EMR::Cluster resource. The StepConfig property type specifies a cluster (job flow) step, which runs only on the master node. Steps are used to submit data processing jobs to the cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepConfig {
    ///
    /// The action to take when the cluster step fails. Possible values are CANCEL_AND_WAIT and CONTINUE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: Option<String>,

    ///
    /// The JAR file used for the step.
    ///
    /// Required: Yes
    ///
    /// Type: HadoopJarStepConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,

    ///
    /// The name of the step.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,
}

impl cfn_resources::CfnResource for StepConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.hadoop_jar_step.validate()?;

        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 0",
                the_val.len()
            ));
        }

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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// VolumeSpecification is a subproperty of the EbsBlockDeviceConfig property type. VolumeSecification determines the volume type, IOPS, and size (GiB) for EBS volumes attached to EC2 instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VolumeSpecification {
    ///
    /// The number of I/O operations per second (IOPS) that the volume supports.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,

    ///
    /// The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume     type is EBS-optimized, the minimum value is 10.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: i64,

    ///
    /// The volume type. Volume types supported are gp3, gp2, io1, st1, sc1, and     standard.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: String,
}

impl cfn_resources::CfnResource for VolumeSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
