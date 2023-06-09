/// Use InstanceGroupConfig to define instance groups for an EMR cluster. A cluster can not use both instance groups and instance fleets. For more information, see Create a Cluster with Instance Fleets or Uniform Instance Groups in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnInstanceGroupConfig {
    ///
    /// AutoScalingPolicy is a subproperty of InstanceGroupConfig. AutoScalingPolicy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. For more information, see Using Automatic Scaling in Amazon EMR in the Amazon EMR Management Guide.
    ///
    /// Required: No
    ///
    /// Type: AutoScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<cfn_resources::StrVal>,

    ///
    /// EbsConfiguration determines the EBS volumes to attach to EMR cluster instances.
    ///
    /// Required: No
    ///
    /// Type: EbsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// The role of the instance group in the cluster.
    ///
    /// Allowed Values: TASK
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceRole")]
    pub instance_role: InstanceGroupConfigInstanceRoleEnum,

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
    pub instance_type: cfn_resources::StrVal,

    ///
    /// The ID of an Amazon EMR cluster that you want to associate this instance group with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceGroupConfigInstanceRoleEnum {
    /// TASK
    #[serde(rename = "TASK")]
    Task,
}

impl Default for InstanceGroupConfigInstanceRoleEnum {
    fn default() -> Self {
        InstanceGroupConfigInstanceRoleEnum::Task
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for CfnInstanceGroupConfig {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::InstanceGroupConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_scaling_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.bid_price {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bid_price'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bid_price {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bid_price'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'custom_ami_id'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_ami_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_ami_id'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.ebs_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_type'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_type'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// AutoScalingPolicy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. For more information, see Using Automatic Scaling in Amazon EMR in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constraints.validate()?;

        Ok(())
    }
}

/// CloudWatchAlarmDefinition is a subproperty of the ScalingTrigger property, which determines when to trigger an automatic scaling activity. Scaling activity begins when you satisfy the defined alarm conditions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub metric_name: cfn_resources::StrVal,

    ///
    /// The namespace for the CloudWatch metric. The default is       AWS/ElasticMapReduce.
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<CloudWatchAlarmDefinitionUnitEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Configurations is a property of the AWS::EMR::Cluster resource that specifies the configuration of applications on an Amazon EMR cluster.
///
/// Configurations are optional. You can use them to have EMR customize applications and software bundled with Amazon EMR when a cluster is created. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see Configuring Applications.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Configuration {
    ///
    /// The classification within a configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<cfn_resources::StrVal>,

    ///
    /// Within a configuration classification, a set of properties that represent the settings that you want to change in the configuration file. Duplicates not allowed.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<std::collections::HashMap<String, String>>,

    ///
    /// A list of additional configurations to apply within a configuration object.
    ///
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
}

impl cfn_resources::CfnResource for Configuration {
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

/// Configuration of requested EBS block device associated with the instance group with     count of volumes that are associated to every instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_per_instance: Option<i64>,
}

impl cfn_resources::CfnResource for EbsBlockDeviceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.volume_specification.validate()?;

        Ok(())
    }
}

/// The Amazon EBS configuration of a cluster instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
}

impl cfn_resources::CfnResource for EbsConfiguration {
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

/// MetricDimension is a subproperty of the CloudWatchAlarmDefinition property type. MetricDimension specifies a CloudWatch dimension, which is specified with a Key Value pair. The key is known as a Name in CloudWatch. By default, Amazon EMR uses one dimension whose Key is JobFlowID and Value is a variable representing the cluster ID, which is ${emr.clusterId}. This enables the automatic scaling rule for EMR to bootstrap when the cluster ID becomes available during cluster creation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub key: cfn_resources::StrVal,

    ///
    /// The dimension value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MetricDimension {
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

/// ScalingAction is a subproperty of the ScalingRule property type. ScalingAction determines the type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.simple_scaling_policy_configuration.validate()?;

        Ok(())
    }
}

/// ScalingConstraints is a subproperty of the AutoScalingPolicy property type. ScalingConstraints defines the upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or shrink below these limits.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ScalingRule is a subproperty of the AutoScalingPolicy property type. ScalingRule defines the scale-in or scale-out rules for scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name used to identify an automatic scaling rule. Rule names must be unique within a     scaling policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

        self.trigger.validate()?;

        Ok(())
    }
}

/// ScalingTrigger is a subproperty of the ScalingRule property type. ScalingTrigger determines the conditions that trigger an automatic scaling activity.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_alarm_definition.validate()?;

        Ok(())
    }
}

/// SimpleScalingPolicyConfiguration is a subproperty of the ScalingAction property type. SimpleScalingPolicyConfiguration determines how an automatic scaling action adds or removes instances, the cooldown period, and the number of EC2 instances that are added each time the CloudWatch metric alarm condition is satisfied.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// VolumeSpecification is a subproperty of the EbsBlockDeviceConfig property type. VolumeSecification determines the volume type, IOPS, and size (GiB) for EBS volumes attached to EC2 instances.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub volume_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for VolumeSpecification {
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
