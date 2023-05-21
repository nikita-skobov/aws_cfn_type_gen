

/// Use InstanceFleetConfig to define instance fleets for an EMR cluster. A cluster can not use both instance fleets and instance groups. For more information, see Configure Instance Fleets in the Amazon EMR Management Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnInstanceFleetConfig {


    /// 
    /// The node type that the instance fleet hosts.
    /// 
    /// Allowed Values: TASK
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceFleetType")]
    pub instance_fleet_type: String,


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
    /// InstanceTypeConfigs determine the EC2 instances that Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities.
    /// 
    /// NoteThe instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceTypeConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceTypeConfigs")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,


    /// 
    /// The unique identifier of the EMR cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,


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

}


/// VolumeSpecification is a subproperty of the EbsBlockDeviceConfig property type. VolumeSecification determines the volume type, IOPS, and size (GiB) for EBS volumes attached to EC2 instances.
#[derive(Default, serde::Serialize)]
pub struct VolumeSpecification {


    /// 
    /// The volume type. Volume types supported are gp3, gp2, io1, st1, sc1, and     standard.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeType")]
    pub volume_type: String,


    /// 
    /// The number of I/O operations per second (IOPS) that the volume supports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume     type is EBS-optimized, the minimum value is 10.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: i64,

}


/// EbsBlockDeviceConfig is a subproperty of the EbsConfiguration property type. EbsBlockDeviceConfig defines the number and type of EBS volumes to associate with all EC2 instances in an EMR cluster.
#[derive(Default, serde::Serialize)]
pub struct EbsBlockDeviceConfig {


    /// 
    /// EBS volume specifications such as volume type, IOPS, size (GiB) and throughput (MiB/s)     that are requested for the EBS volume attached to an EC2 instance in the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: VolumeSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,


    /// 
    /// Number of EBS volumes with a specific volume configuration that are associated with     every instance in the instance group
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<i64>,

}


/// The launch specification for On-Demand Instances in the instance fleet, which     determines the allocation strategy.
#[derive(Default, serde::Serialize)]
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
    pub allocation_strategy: String,

}


/// InstanceTypeConfig is a sub-property of InstanceFleetConfig. InstanceTypeConfig determines the EC2 instances that Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities.
#[derive(Default, serde::Serialize)]
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


/// SpotProvisioningSpecification is a subproperty of the InstanceFleetProvisioningSpecifications property type. SpotProvisioningSpecification determines the launch specification for Spot instances in the instance fleet, which includes the defined duration and provisioning timeout behavior.
#[derive(Default, serde::Serialize)]
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
    pub allocation_strategy: Option<String>,


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
    pub timeout_action: String,

}


/// InstanceType config is a subproperty of InstanceFleetConfig. An instance type configuration specifies each instance type in an instance fleet. The configuration determines the EC2 instances Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities.
#[derive(Default, serde::Serialize)]
pub struct InstanceTypeConfig {


    /// 
    /// NoteAmazon EMR releases 4.x or later.
    /// 
    /// An optional configuration specification to be used when provisioning cluster instances,     which can include configurations for applications and software bundled with Amazon EMR. A configuration consists of a classification, properties, and optional     nested configurations. A classification refers to an application-specific configuration     file. Properties are the settings you want to change in that file. For more information,     see Configuring Applications.
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

}


/// Configuration specifies optional configurations for customizing open-source big data applications and environment parameters. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see Configuring Applications in the Amazon EMR Release Guide.
#[derive(Default, serde::Serialize)]
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
    pub classification: Option<String>,


    /// 
    /// Within a configuration classification, a set of properties that represent the settings that you want to change in the configuration file. Duplicates not allowed.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProperties")]
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
    pub configurations: Option<Vec<Configuration>>,

}


/// EbsConfiguration determines the EBS volumes to attach to EMR cluster instances.
#[derive(Default, serde::Serialize)]
pub struct EbsConfiguration {


    /// 
    /// Indicates whether an Amazon EBS volume is EBS-optimized.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,


    /// 
    /// An array of Amazon EBS volume specifications attached to a cluster     instance.
    /// 
    /// Required: No
    ///
    /// Type: List of EbsBlockDeviceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,

}
