/// Creates a layer. For more information, see How to     Create a Layer.
///
/// Required Permissions: To use this action, an IAM user must      have a Manage permissions    level for the stack, or an attached policy that explicitly grants permissions. For more    information on user permissions, see Managing User     Permissions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLayer {
    ///
    /// One or more user-defined key-value pairs to be added to the stack attributes.
    ///
    /// To create a cluster layer, set the EcsClusterArn attribute to the cluster's ARN.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

    ///
    /// Whether to automatically assign an Elastic IP     address to the layer's instances. For more information, see How to Edit     a Layer.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: bool,

    ///
    /// For stacks that are running in a VPC, whether to automatically assign a public IP address to    the layer's instances. For more information, see How to Edit     a Layer.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAssignPublicIps")]
    pub auto_assign_public_ips: bool,

    ///
    /// The ARN of an IAM profile to be used for the layer's EC2 instances. For more information      about IAM ARNs, see Using Identifiers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<cfn_resources::StrVal>,

    ///
    /// A JSON-formatted string containing custom stack configuration and deployment attributes   to be installed on the layer's instances. For more information, see         Using Custom JSON. This feature is supported as of version 1.7.42 of the AWS CLI.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<serde_json::Value>,

    ///
    /// A LayerCustomRecipes object that specifies the layer custom recipes.
    ///
    /// Required: No
    ///
    /// Type: Recipes
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<Recipes>,

    ///
    /// An array containing the layer custom security group IDs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,

    ///
    /// Whether to disable auto healing for the layer.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoHealing")]
    pub enable_auto_healing: bool,

    ///
    /// Whether to install operating system and package updates when the instance boots. The default    value is true. To control when updates are installed, set this value to     false. You must then update your instances manually by using     CreateDeployment to run the update_dependencies stack command or    by manually running yum (Amazon Linux) or apt-get (Ubuntu) on the    instances.
    ///
    /// NoteTo ensure that your     instances have the latest security updates, we strongly recommend using the default value of true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,

    ///
    /// A LifeCycleEventConfiguration object that you can use to configure the Shutdown event to    specify an execution timeout and enable or disable Elastic Load Balancer connection    draining.
    ///
    /// Required: No
    ///
    /// Type: LifecycleEventConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,

    ///
    /// The load-based scaling configuration for the AWS OpsWorks layer.
    ///
    /// Required: No
    ///
    /// Type: LoadBasedAutoScaling
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBasedAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_based_auto_scaling: Option<LoadBasedAutoScaling>,

    ///
    /// The layer name, which is used by the console. Layer names can be a maximum of 32 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// An array of Package objects that describes the layer packages.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,

    ///
    /// For custom layers only, use this parameter to specify the layer's short name, which is used internally by      AWS OpsWorks Stacks and by Chef recipes. The short name is also used as the name for the directory where your      app files are installed. It can have a maximum of 32 characters, which are limited to the alphanumeric      characters, '-', '_', and '.'.
    ///
    /// Built-in layer short names are defined by AWS OpsWorks Stacks. For more information, see the      Layer Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Shortname")]
    pub shortname: cfn_resources::StrVal,

    ///
    /// The layer stack ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackId")]
    pub stack_id: cfn_resources::StrVal,

    ///
    /// Specifies one or more sets of tags (key–value pairs) to associate with this AWS OpsWorks layer.      Use tags to manage your resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The layer type. A stack cannot have more than one built-in layer of the same type. It can have any number of custom layers.      Built-in layers are not available in Chef 12 stacks.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: aws-flow-ruby | custom | db-master | ecs-cluster | java-app | lb | memcached | monitoring-master | nodejs-app | php-app | rails-app | web
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: LayerTypeEnum,

    ///
    /// Whether to use Amazon EBS-optimized instances.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,

    ///
    /// A VolumeConfigurations object that describes the layer's Amazon EBS volumes.
    ///
    /// Required: No
    ///
    /// Type: List of VolumeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LayerTypeEnum {
    /// aws-flow-ruby
    #[serde(rename = "aws-flow-ruby")]
    Awsflowruby,

    /// custom
    #[serde(rename = "custom")]
    Custom,

    /// db-master
    #[serde(rename = "db-master")]
    Dbmaster,

    /// ecs-cluster
    #[serde(rename = "ecs-cluster")]
    Ecscluster,

    /// java-app
    #[serde(rename = "java-app")]
    Javaapp,

    /// lb
    #[serde(rename = "lb")]
    Lb,

    /// memcached
    #[serde(rename = "memcached")]
    Memcached,

    /// monitoring-master
    #[serde(rename = "monitoring-master")]
    Monitoringmaster,

    /// nodejs-app
    #[serde(rename = "nodejs-app")]
    Nodejsapp,

    /// php-app
    #[serde(rename = "php-app")]
    Phpapp,

    /// rails-app
    #[serde(rename = "rails-app")]
    Railsapp,

    /// web
    #[serde(rename = "web")]
    Web,
}

impl Default for LayerTypeEnum {
    fn default() -> Self {
        LayerTypeEnum::Awsflowruby
    }
}

impl cfn_resources::CfnResource for CfnLayer {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::Layer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_recipes
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.lifecycle_event_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.load_based_auto_scaling
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a load-based auto scaling upscaling or downscaling threshold configuration, which specifies when AWS OpsWorks Stacks starts or      stops load-based instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoScalingThresholds {
    ///
    /// The CPU utilization threshold, as a percent of the available CPU. A value of -1 disables the threshold.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_threshold: Option<f64>,

    ///
    /// The amount of time (in minutes) after a scaling event occurs that AWS OpsWorks Stacks should ignore metrics      and suppress additional scaling events. For example, AWS OpsWorks Stacks adds new instances following    an upscaling event but the instances won't start reducing the load until they have been booted    and configured. There is no point in raising additional scaling events during that operation,    which typically takes several minutes. IgnoreMetricsTime allows you to direct      AWS OpsWorks Stacks to suppress scaling events long enough to get the new instances online.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreMetricsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_metrics_time: Option<i64>,

    ///
    /// The number of instances to add or remove when the load exceeds a threshold.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,

    ///
    /// The load threshold. A value of -1 disables the threshold. For more information about how load is computed,      see Load (computing).
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_threshold: Option<f64>,

    ///
    /// The memory utilization threshold, as a percent of the available memory. A value of -1 disables the threshold.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_threshold: Option<f64>,

    ///
    /// The amount of time, in minutes, that the load must exceed a threshold before more instances are added or removed.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdsWaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds_wait_time: Option<i64>,
}

impl cfn_resources::CfnResource for AutoScalingThresholds {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ignore_metrics_time {
            if *the_val > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'ignore_metrics_time'. {} is greater than 100",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.ignore_metrics_time {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'ignore_metrics_time'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.thresholds_wait_time {
            if *the_val > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'thresholds_wait_time'. {} is greater than 100",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.thresholds_wait_time {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'thresholds_wait_time'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the lifecycle event configuration
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LifecycleEventConfiguration {
    ///
    /// The Shutdown event configuration.
    ///
    /// Required: No
    ///
    /// Type: ShutdownEventConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShutdownEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_event_configuration: Option<ShutdownEventConfiguration>,
}

impl cfn_resources::CfnResource for LifecycleEventConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.shutdown_event_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a layer's load-based auto scaling configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoadBasedAutoScaling {
    ///
    /// An AutoScalingThresholds object that describes the downscaling configuration,      which defines how and when AWS OpsWorks Stacks reduces the number of instances.
    ///
    /// Required: No
    ///
    /// Type: AutoScalingThresholds
    ///
    /// Update requires: No interruption
    #[serde(rename = "DownScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_scaling: Option<AutoScalingThresholds>,

    ///
    /// Whether load-based auto scaling is enabled for the layer.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    ///
    /// An AutoScalingThresholds object that describes the upscaling configuration,      which defines how and when AWS OpsWorks Stacks increases the number of instances.
    ///
    /// Required: No
    ///
    /// Type: AutoScalingThresholds
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_scaling: Option<AutoScalingThresholds>,
}

impl cfn_resources::CfnResource for LoadBasedAutoScaling {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.down_scaling
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.up_scaling
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// AWS OpsWorks Stacks supports five    lifecycle events:     setup, configuration, deploy, undeploy, and shutdown. For      each layer, AWS OpsWorks Stacks runs a set of standard recipes for each event. In addition, you can provide      custom recipes for any or all layers and events. AWS OpsWorks Stacks runs custom event recipes after the    standard recipes. LayerCustomRecipes specifies the custom recipes for a    particular layer to be run in response to each of the five events.
///
/// To specify a recipe, use the cookbook's directory name in the repository followed by two colons and the recipe name,      which is the recipe's file name without the .rb extension. For example: phpapp2::dbsetup specifies the dbsetup.rb recipe in      the repository's phpapp2 folder.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Recipes {
    ///
    /// An array of custom recipe names to be run following a configure event.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure: Option<Vec<String>>,

    ///
    /// An array of custom recipe names to be run following a deploy event.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Deploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Vec<String>>,

    ///
    /// An array of custom recipe names to be run following a setup event.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Setup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<Vec<String>>,

    ///
    /// An array of custom recipe names to be run following a shutdown event.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Shutdown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown: Option<Vec<String>>,

    ///
    /// An array of custom recipe names to be run following a undeploy event.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Undeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undeploy: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for Recipes {
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

/// The Shutdown event configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShutdownEventConfiguration {
    ///
    /// Whether to enable Elastic Load Balancing connection draining. For more information, see Connection Draining
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DelayUntilElbConnectionsDrained")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_until_elb_connections_drained: Option<bool>,

    ///
    /// The time, in seconds, that AWS OpsWorks Stacks waits after triggering a Shutdown event before      shutting down an instance.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<i64>,
}

impl cfn_resources::CfnResource for ShutdownEventConfiguration {
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

/// Describes an Amazon EBS volume configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VolumeConfiguration {
    ///
    /// Specifies whether an Amazon EBS volume is encrypted. For more information,       see Amazon EBS Encryption.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    ///
    /// The number of I/O operations per second (IOPS) to provision for the volume. For PIOPS volumes, the IOPS per disk.
    ///
    /// If you specify io1 for the volume type, you must specify this property.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    ///
    /// The volume mount point. For example "/dev/sdh".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<cfn_resources::StrVal>,

    ///
    /// The number of disks in the volume.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfDisks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_disks: Option<i64>,

    ///
    /// The volume RAID level.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RaidLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_level: Option<i64>,

    ///
    /// The volume size.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    ///
    /// The volume type. For more information, see      Amazon EBS Volume Types.
    ///
    /// standard - Magnetic. Magnetic volumes must have a minimum size of 1 GiB and a maximum size of 1024 GiB.                        io1 - Provisioned IOPS (SSD). PIOPS volumes must have a minimum size of 4 GiB and a maximum size of 16384 GiB.                        gp2 - General Purpose (SSD). General purpose volumes must have a minimum size of 1 GiB and a maximum size        of 16384 GiB.                        st1 - Throughput Optimized hard disk drive (HDD). Throughput optimized HDD volumes must have a          minimum size of 500 GiB and a maximum size of 16384 GiB.                        sc1 - Cold HDD. Cold HDD volumes must have a minimum size of 500 GiB and a maximum size of 16384 GiB.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VolumeConfiguration {
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
