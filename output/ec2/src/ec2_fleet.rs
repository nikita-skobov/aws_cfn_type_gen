

/// Specifies the configuration information to launch a fleet--or group--of instances. An     EC2 Fleet can launch multiple instance types across multiple Availability Zones, using the     On-Demand Instance, Reserved Instance, and Spot Instance purchasing models together. Using     EC2 Fleet, you can define separate On-Demand and Spot capacity targets, specify the     instance types that work best for your applications, and specify how Amazon EC2 should     distribute your fleet capacity within each purchasing model. For more information, see       Launching an       EC2 Fleet in the Amazon EC2 User Guide for Linux     Instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEC2Fleet {


    /// 
    /// Indicates whether running instances should be terminated when the EC2 Fleet expires.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "TerminateInstancesWithExpiration")]
    pub terminate_instances_with_expiration: Option<bool>,


    /// 
    /// Indicates whether EC2 Fleet should replace unhealthy Spot Instances. Supported only for     fleets of type maintain. For more information, see EC2 Fleet       health checks in the Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplaceUnhealthyInstances")]
    pub replace_unhealthy_instances: Option<bool>,


    /// 
    /// Indicates whether running instances should be terminated if the total target capacity of     the EC2 Fleet is decreased below the current size of the EC2 Fleet.
    /// 
    /// Supported only for fleets of type maintain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: no-termination | termination
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcessCapacityTerminationPolicy")]
    pub excess_capacity_termination_policy: Option<String>,


    /// 
    /// The start date and time of the request, in UTC format (for example,       YYYY-MM-DDTHH:MM:SSZ).     The default is to start fulfilling the request immediately.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidFrom")]
    pub valid_from: Option<String>,


    /// 
    /// Reserved.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Context")]
    pub context: Option<String>,


    /// 
    /// The configuration for the EC2 Fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FleetLaunchTemplateConfigRequest
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchTemplateConfigs")]
    pub launch_template_configs: Vec<FleetLaunchTemplateConfigRequest>,


    /// 
    /// Describes the configuration of Spot Instances in an EC2 Fleet.
    /// 
    /// Required: No
    ///
    /// Type: SpotOptionsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpotOptions")]
    pub spot_options: Option<SpotOptionsRequest>,


    /// 
    /// The fleet type. The default value is maintain.
    /// 
    /// maintain - The EC2 Fleet places an asynchronous request for your desired        capacity, and continues to maintain your desired Spot capacity by replenishing        interrupted Spot Instances.                        request - The EC2 Fleet places an asynchronous one-time request for your        desired capacity, but does submit Spot requests in alternative capacity pools if Spot        capacity is unavailable, and does not maintain Spot capacity if Spot Instances are        interrupted.                        instant - The EC2 Fleet places a synchronous one-time request for your        desired capacity, and returns errors for any instances that could not be        launched.
    /// 
    /// For more information, see EC2 Fleet       request types in the Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: instant | maintain | request
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The number of units to request.
    /// 
    /// Required: Yes
    ///
    /// Type: TargetCapacitySpecificationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetCapacitySpecification")]
    pub target_capacity_specification: TargetCapacitySpecificationRequest,


    /// 
    /// The end date and time of the request, in UTC format (for example,       YYYY-MM-DDTHH:MM:SSZ).     At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidUntil")]
    pub valid_until: Option<String>,


    /// 
    /// Describes the configuration of On-Demand Instances in an EC2 Fleet.
    /// 
    /// Required: No
    ///
    /// Type: OnDemandOptionsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "OnDemandOptions")]
    pub on_demand_options: Option<OnDemandOptionsRequest>,


    /// 
    /// The key-value pair for tagging the EC2 Fleet request on creation. For more information, see      Tagging your resources.
    /// 
    /// If the fleet type is instant, specify a resource type of fleet      to tag the fleet or instance to tag the instances at launch.
    /// 
    /// If the fleet type is maintain or request, specify a resource     type of fleet to tag the fleet. You cannot specify a resource type of       instance. To tag instances at launch, specify the tags in a launch template.
    /// 
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,

}

impl cfn_resources::CfnResource for CfnEC2Fleet {
    fn type_string() -> &'static str {
        "AWS::EC2::EC2Fleet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the tags to apply to a resource when the resource is being created for an EC2     Fleet.
///
/// TagSpecification is a property of the       AWS::EC2::EC2Fleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagSpecification {


    /// 
    /// The type of resource to tag. ResourceType must be     fleet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-reservation | capacity-reservation-fleet | carrier-gateway | client-vpn-endpoint | coip-pool | customer-gateway | dedicated-host | dhcp-options | egress-only-internet-gateway | elastic-gpu | elastic-ip | export-image-task | export-instance-task | fleet | fpga-image | host-reservation | image | import-image-task | import-snapshot-task | instance | instance-event-window | internet-gateway | ipam | ipam-pool | ipam-resource-discovery | ipam-resource-discovery-association | ipam-scope | ipv4pool-ec2 | ipv6pool-ec2 | key-pair | launch-template | local-gateway | local-gateway-route-table | local-gateway-route-table-virtual-interface-group-association | local-gateway-route-table-vpc-association | local-gateway-virtual-interface | local-gateway-virtual-interface-group | natgateway | network-acl | network-insights-access-scope | network-insights-access-scope-analysis | network-insights-analysis | network-insights-path | network-interface | placement-group | prefix-list | replace-root-volume-task | reserved-instances | route-table | security-group | security-group-rule | snapshot | spot-fleet-request | spot-instances-request | subnet | subnet-cidr-reservation | traffic-mirror-filter | traffic-mirror-filter-rule | traffic-mirror-session | traffic-mirror-target | transit-gateway | transit-gateway-attachment | transit-gateway-connect-peer | transit-gateway-multicast-domain | transit-gateway-policy-table | transit-gateway-route-table | transit-gateway-route-table-announcement | verified-access-endpoint | verified-access-group | verified-access-instance | verified-access-policy | verified-access-trust-provider | volume | vpc | vpc-block-public-access-exclusion | vpc-endpoint | vpc-endpoint-connection | vpc-endpoint-connection-device-type | vpc-endpoint-service | vpc-endpoint-service-permission | vpc-flow-log | vpc-peering-connection | vpn-connection | vpn-connection-device-type | vpn-gateway
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,


    /// 
    /// The tags to apply to the resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// The minimum and maximum amount of memory per vCPU, in GiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemoryGiBPerVCpuRequest {


    /// 
    /// The minimum amount of memory per vCPU, in GiB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<f64>,


    /// 
    /// The maximum amount of memory per vCPU, in GiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}


/// Specifies the allocation strategy of On-Demand Instances in an EC2 Fleet.
///
/// OnDemandOptionsRequest is a property of the AWS::EC2::EC2Fleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnDemandOptionsRequest {


    /// 
    /// The maximum amount per hour for On-Demand Instances that you're willing to pay.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxTotalPrice")]
    pub max_total_price: Option<String>,


    /// 
    /// Indicates that the fleet launches all On-Demand Instances into a single Availability Zone.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleAvailabilityZone")]
    pub single_availability_zone: Option<bool>,


    /// 
    /// The minimum target capacity for On-Demand Instances in the fleet. If the minimum target capacity is     not reached, the fleet launches no instances.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// At least one of the following must be specified: SingleAvailabilityZone |     SingleInstanceType
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinTargetCapacity")]
    pub min_target_capacity: Option<i64>,


    /// 
    /// Indicates that the fleet uses a single instance type to launch all On-Demand Instances in the     fleet.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleInstanceType")]
    pub single_instance_type: Option<bool>,


    /// 
    /// The strategy that determines the order of the launch template overrides to use in     fulfilling On-Demand capacity.
    /// 
    /// lowest-price - EC2 Fleet uses price to determine the order, launching the lowest     price first.
    /// 
    /// prioritized - EC2 Fleet uses the priority that you assigned to each launch     template override, launching the highest priority first.
    /// 
    /// Default: lowest-price
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: lowest-price | prioritized
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,


    /// 
    /// The strategy for using unused Capacity Reservations for fulfilling On-Demand     capacity.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// Required: No
    ///
    /// Type: CapacityReservationOptionsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "CapacityReservationOptions")]
    pub capacity_reservation_options: Option<CapacityReservationOptionsRequest>,

}


/// The attributes for the instance types. When you specify instance attributes, Amazon EC2 will     identify instance types with these attributes.
///
/// When you specify multiple attributes, you get instance types that satisfy all of the     specified attributes. If you specify multiple values for an attribute, you get instance     types that satisfy any of the specified values.
///
/// To limit the list of instance types from which Amazon EC2 can identify matching instance types,      you can use one of the following parameters, but not both in the same request:
///
/// For more information, see Attribute-based instance type selection for EC2 Fleet, Attribute-based instance type selection for Spot Fleet, and Spot        placement score in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceRequirementsRequest {


    /// 
    /// The minimum and maximum baseline bandwidth to Amazon EBS, in Mbps. For more information, see       Amazon       EBS–optimized instances in the Amazon EC2 User Guide.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: BaselineEbsBandwidthMbpsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,


    /// 
    /// Indicates whether bare metal instance types must be included, excluded, or required.
    /// 
    /// To include bare metal instance types, specify included.               To require only bare metal instance types, specify required.               To exclude bare metal instance types, specify excluded.
    /// 
    /// Default: excluded
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: Replacement
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<String>,


    /// 
    /// The price protection threshold for On-Demand Instances. This is the maximum you’ll pay for an On-Demand Instance,     expressed as a percentage above the least expensive current generation M, C, or R instance type with your specified     attributes. When Amazon EC2 selects instance types with your attributes, it excludes instance     types priced above your threshold.
    /// 
    /// The parameter accepts an integer, which Amazon EC2 interprets as a percentage.
    /// 
    /// To turn off price protection, specify a high value, such as 999999.
    /// 
    /// This parameter is not supported for GetSpotPlacementScores and GetInstanceTypesFromInstanceRequirements.
    /// 
    /// NoteIf you set TargetCapacityUnitType to vcpu or     memory-mib, the price protection threshold is applied based on the     per-vCPU or per-memory price instead of the per-instance price.
    /// 
    /// Default: 20
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i64>,


    /// 
    /// Indicates whether burstable performance T instance types are included, excluded, or required. For more information, see    Burstable performance instances.
    /// 
    /// To include burstable performance instance types, specify included.               To require only burstable performance instance types, specify required.               To exclude burstable performance instance types, specify excluded.
    /// 
    /// Default: excluded
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: Replacement
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<String>,


    /// 
    /// Indicates whether instance types must have accelerators by specific manufacturers.
    /// 
    /// For instance types with NVIDIA devices, specify nvidia.               For instance types with AMD devices, specify amd.               For instance types with AWS devices, specify amazon-web-services.               For instance types with Xilinx devices, specify xilinx.
    /// 
    /// Default: Any manufacturer
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of memory, in MiB.
    /// 
    /// Required: No
    ///
    /// Type: MemoryMiBRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiBRequest>,


    /// 
    /// The accelerators that must be on the instance type.
    /// 
    /// For instance types with NVIDIA A100 GPUs, specify a100.               For instance types with NVIDIA V100 GPUs, specify v100.               For instance types with NVIDIA K80 GPUs, specify k80.               For instance types with NVIDIA T4 GPUs, specify t4.               For instance types with NVIDIA M60 GPUs, specify m60.               For instance types with AMD Radeon Pro V520 GPUs, specify radeon-pro-v520.               For instance types with Xilinx VU9P FPGAs, specify vu9p.               For instance types with AWS Inferentia chips, specify inferentia.               For instance types with NVIDIA GRID K520 GPUs, specify k520.
    /// 
    /// Default: Any accelerator
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of memory per vCPU, in GiB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: MemoryGiBPerVCpuRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpuRequest>,


    /// 
    /// The minimum and maximum number of network interfaces.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: NetworkInterfaceCountRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,


    /// 
    /// The price protection threshold for Spot Instance. This is the maximum you’ll pay for an Spot Instance,     expressed as a percentage above the least expensive current generation M, C, or R instance type with your specified     attributes. When Amazon EC2 selects instance types with your attributes, it excludes instance     types priced above your threshold.
    /// 
    /// The parameter accepts an integer, which Amazon EC2 interprets as a percentage.
    /// 
    /// To turn off price protection, specify a high value, such as 999999.
    /// 
    /// This parameter is not supported for GetSpotPlacementScores and GetInstanceTypesFromInstanceRequirements.
    /// 
    /// NoteIf you set TargetCapacityUnitType to vcpu or     memory-mib, the price protection threshold is applied based on the     per-vCPU or per-memory price instead of the per-instance price.
    /// 
    /// Default: 100
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<i64>,


    /// 
    /// The accelerator types that must be on the instance type.
    /// 
    /// To include instance types with GPU hardware, specify gpu.               To include instance types with FPGA hardware, specify fpga.               To include instance types with inference hardware, specify inference.
    /// 
    /// Default: Any accelerator type
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,


    /// 
    /// Indicates whether current or previous generation instance types are included. The    current generation instance types are recommended for use. Current generation instance types are    typically the latest two to three generations in each instance family. For more    information, see Instance types in the    Amazon EC2 User Guide.
    /// 
    /// For current generation instance types, specify current.
    /// 
    /// For previous generation instance types, specify previous.
    /// 
    /// Default: Current and previous generation instance types
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of total accelerator memory, in MiB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: AcceleratorTotalMemoryMiBRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,


    /// 
    /// The minimum and maximum number of vCPUs.
    /// 
    /// Required: No
    ///
    /// Type: VCpuCountRangeRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCountRangeRequest>,


    /// 
    /// The instance types to apply your specified attributes against. All other instance types      are ignored, even if they match your specified attributes.
    /// 
    /// You can use strings with one or more wild cards, represented by     an asterisk (*), to allow an instance type, size, or generation. The     following are examples: m5.8xlarge, c5*.*, m5a.*,     r*, *3*.
    /// 
    /// For example, if you specify c5*,Amazon EC2 will allow the entire C5 instance     family, which includes all C5a and C5n instance types. If you specify     m5a.*, Amazon EC2 will allow all the M5a instance types, but not the M5n     instance types.
    /// 
    /// NoteIf you specify AllowedInstanceTypes, you can't specify ExcludedInstanceTypes.
    /// 
    /// Default: All instance types
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 400
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of total local storage, in GB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: TotalLocalStorageGBRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGBRequest>,


    /// 
    /// Indicates whether instance types with instance store volumes are included, excluded, or required. For more information,    Amazon     EC2 instance store in the Amazon EC2 User Guide.
    /// 
    /// To include instance types with instance store volumes, specify          included.               To require only instance types with instance store volumes, specify          required.               To exclude instance types with instance store volumes, specify          excluded.
    /// 
    /// Default: included
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<String>,


    /// 
    /// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips) on     an instance.
    /// 
    /// To exclude accelerator-enabled instance types, set Max to 0.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: AcceleratorCountRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCountRequest>,


    /// 
    /// The instance types to exclude.
    /// 
    /// You can use strings with one or more wild cards, represented by     an asterisk (*), to exclude an instance family, type, size, or generation. The     following are examples: m5.8xlarge, c5*.*, m5a.*,       r*, *3*.
    /// 
    /// For example, if you specify c5*,Amazon EC2 will exclude the entire C5 instance    family, which includes all C5a and C5n instance types. If you specify    m5a.*, Amazon EC2 will exclude all the M5a instance types, but not the M5n    instance types.
    /// 
    /// NoteIf you specify ExcludedInstanceTypes, you can't specify AllowedInstanceTypes.
    /// 
    /// Default: No excluded instance types
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 400
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps).
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: NetworkBandwidthGbpsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,


    /// 
    /// The type of local storage that is required.
    /// 
    /// For instance types with hard disk drive (HDD) storage, specify hdd.               For instance types with solid state drive (SSD) storage, specify        ssd.
    /// 
    /// Default: hdd and ssd
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,


    /// 
    /// Indicates whether instance types must support hibernation for On-Demand Instances.
    /// 
    /// This parameter is not supported for GetSpotPlacementScores.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,


    /// 
    /// The CPU manufacturers to include.
    /// 
    /// For instance types with Intel CPUs, specify intel.               For instance types with AMD CPUs, specify amd.               For instance types with AWS CPUs, specify amazon-web-services.
    /// 
    /// NoteDon't confuse the CPU manufacturer with the CPU architecture. Instances will      be launched with a compatible CPU architecture based on the Amazon Machine Image (AMI) that you      specify in your launch template.
    /// 
    /// Default: Any manufacturer
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,

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


/// The minimum and maximum amount of total accelerator memory, in MiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AcceleratorTotalMemoryMiBRequest {


    /// 
    /// The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}


/// The Spot Instance replacement strategy to use when Amazon EC2 emits a rebalance     notification signal that your Spot Instance is at an elevated risk of being interrupted.     For more information, see Capacity rebalancing in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityRebalance {


    /// 
    /// The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot     Instance after launching a new replacement Spot Instance.
    /// 
    /// Required when ReplacementStrategy is set to launch-before-terminate.
    /// 
    /// Not valid when ReplacementStrategy is set to launch.
    /// 
    /// Valid values: Minimum value of 120 seconds. Maximum value of 7200 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TerminationDelay")]
    pub termination_delay: Option<i64>,


    /// 
    /// The replacement strategy to use. Only available for fleets of type     maintain.
    /// 
    /// launch - EC2 Fleet launches a replacement Spot Instance when a rebalance     notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not     terminate the instances that receive a rebalance notification. You can terminate the old     instances, or you can leave them running. You are charged for all instances while they are     running.
    /// 
    /// launch-before-terminate - EC2 Fleet launches a replacement Spot Instance     when a rebalance notification is emitted for an existing Spot Instance in the fleet, and     then, after a delay that you specify (in TerminationDelay), terminates the     instances that received a rebalance notification.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: launch | launch-before-terminate
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplacementStrategy")]
    pub replacement_strategy: Option<String>,

}


/// Specifies a launch template and overrides for an EC2 Fleet.
///
/// FleetLaunchTemplateConfigRequest is a property of the AWS::EC2::EC2Fleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FleetLaunchTemplateConfigRequest {


    /// 
    /// Any parameters that you specify override the same parameters in the launch     template.
    /// 
    /// For fleets of type request and maintain, a maximum of 300     items is allowed across all launch templates.
    /// 
    /// Required: No
    ///
    /// Type: List of FleetLaunchTemplateOverridesRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<FleetLaunchTemplateOverridesRequest>>,


    /// 
    /// The launch template to use. You must specify either the launch template ID or launch     template name in the request.
    /// 
    /// Required: No
    ///
    /// Type: FleetLaunchTemplateSpecificationRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: Option<FleetLaunchTemplateSpecificationRequest>,

}


/// The minimum and maximum number of network interfaces.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInterfaceCountRequest {


    /// 
    /// The maximum number of network interfaces. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum number of network interfaces. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}


/// Describes the placement of an instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Placement {


    /// 
    /// The tenancy of the instance. An instance with a       tenancy of dedicated runs on single-tenant hardware.
    /// 
    /// This parameter is not supported for CreateFleet. The         host tenancy is not supported for ImportInstance or       for T3 instances that are configured for the unlimited CPU credit       option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dedicated | default | host
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,


    /// 
    /// The ID of the Dedicated Host on which the instance resides.
    /// 
    /// This parameter is not supported for CreateFleet or ImportInstance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,


    /// 
    /// The ARN of the host resource group in which to launch the instances.
    /// 
    /// If you specify this parameter, either omit the Tenancy parameter or set it to host.
    /// 
    /// This parameter is not supported for CreateFleet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,


    /// 
    /// The number of the partition that the instance is in. Valid only if the placement group       strategy is set to partition.
    /// 
    /// This parameter is not supported for CreateFleet.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<i64>,


    /// 
    /// The affinity setting for the instance on the Dedicated Host.
    /// 
    /// This parameter is not supported for CreateFleet or ImportInstance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,


    /// 
    /// Reserved for future use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpreadDomain")]
    pub spread_domain: Option<String>,


    /// 
    /// The name of the placement group that the instance is in. If you specify         GroupName, you can't specify GroupId.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,


    /// 
    /// The Availability Zone of the instance.
    /// 
    /// If not specified, an Availability Zone will be automatically chosen for you based on       the load balancing criteria for the Region.
    /// 
    /// This parameter is not supported for CreateFleet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

}


/// Describes the strategy for using unused Capacity Reservations for fulfilling On-Demand     capacity.
///
/// For more information about Capacity Reservations, see On-Demand Capacity       Reservations in the Amazon EC2 User Guide. For examples of using     Capacity Reservations in an EC2 Fleet, see EC2 Fleet example       configurations in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityReservationOptionsRequest {


    /// 
    /// Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity.
    /// 
    /// If you specify use-capacity-reservations-first, the fleet uses unused     Capacity Reservations to fulfill On-Demand capacity up to the target On-Demand capacity. If     multiple instance pools have unused Capacity Reservations, the On-Demand allocation     strategy (lowest-price or prioritized) is applied. If the number     of unused Capacity Reservations is less than the On-Demand target capacity, the remaining     On-Demand target capacity is launched according to the On-Demand allocation strategy       (lowest-price or prioritized).
    /// 
    /// If you do not specify a value, the fleet fulfils the On-Demand capacity according to the     chosen On-Demand allocation strategy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: use-capacity-reservations-first
    ///
    /// Update requires: Replacement
    #[serde(rename = "UsageStrategy")]
    pub usage_strategy: Option<String>,

}


/// The minimum and maximum amount of memory, in MiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemoryMiBRequest {


    /// 
    /// The minimum amount of memory, in MiB. To specify no minimum limit, specify     0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,


    /// 
    /// The maximum amount of memory, in MiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,

}


/// Specifies overrides for a launch template for an EC2 Fleet.
///
/// FleetLaunchTemplateOverridesRequest is a property of the FleetLaunchTemplateConfigRequest property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FleetLaunchTemplateOverridesRequest {


    /// 
    /// The location where the instance launched, if applicable.
    /// 
    /// Required: No
    ///
    /// Type: Placement
    ///
    /// Update requires: Replacement
    #[serde(rename = "Placement")]
    pub placement: Option<Placement>,


    /// 
    /// The number of units provided by the specified instance type.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<f64>,


    /// 
    /// The priority for the launch template override. The highest priority is launched     first.
    /// 
    /// If the On-Demand AllocationStrategy is set to prioritized,     EC2 Fleet uses priority to determine which launch template override to use first in fulfilling     On-Demand capacity.
    /// 
    /// If the Spot AllocationStrategy is set to       capacity-optimized-prioritized, EC2 Fleet uses priority on a best-effort basis     to determine which launch template override to use in fulfilling Spot capacity, but     optimizes for capacity first.
    /// 
    /// Valid values are whole numbers starting at 0. The lower the number, the     higher the priority. If no number is set, the launch template override has the lowest     priority. You can set the same priority for different launch template overrides.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Priority")]
    pub priority: Option<f64>,


    /// 
    /// The Availability Zone in which to launch the instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The attributes for the instance types. When you specify instance attributes, Amazon EC2 will     identify instance types with those attributes.
    /// 
    /// NoteIf you specify InstanceRequirements, you can't specify       InstanceType.
    /// 
    /// Required: No
    ///
    /// Type: InstanceRequirementsRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,


    /// 
    /// The IDs of the subnets in which to launch the instances. Separate multiple subnet IDs using commas (for example, subnet-1234abcdeexample1, subnet-0987cdef6example2). A request of type instant can have only one subnet ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The instance type.
    /// 
    /// mac1.metal is not supported as a launch template override.
    /// 
    /// NoteIf you specify InstanceType, you can't specify        InstanceRequirements.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: a1.2xlarge | a1.4xlarge | a1.large | a1.medium | a1.metal | a1.xlarge | c1.medium | c1.xlarge | c3.2xlarge | c3.4xlarge | c3.8xlarge | c3.large | c3.xlarge | c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.metal | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c5ad.12xlarge | c5ad.16xlarge | c5ad.24xlarge | c5ad.2xlarge | c5ad.4xlarge | c5ad.8xlarge | c5ad.large | c5ad.xlarge | c5d.12xlarge | c5d.18xlarge | c5d.24xlarge | c5d.2xlarge | c5d.4xlarge | c5d.9xlarge | c5d.large | c5d.metal | c5d.xlarge | c5n.18xlarge | c5n.2xlarge | c5n.4xlarge | c5n.9xlarge | c5n.large | c5n.metal | c5n.xlarge | c6a.12xlarge | c6a.16xlarge | c6a.24xlarge | c6a.2xlarge | c6a.32xlarge | c6a.48xlarge | c6a.4xlarge | c6a.8xlarge | c6a.large | c6a.metal | c6a.xlarge | c6g.12xlarge | c6g.16xlarge | c6g.2xlarge | c6g.4xlarge | c6g.8xlarge | c6g.large | c6g.medium | c6g.metal | c6g.xlarge | c6gd.12xlarge | c6gd.16xlarge | c6gd.2xlarge | c6gd.4xlarge | c6gd.8xlarge | c6gd.large | c6gd.medium | c6gd.metal | c6gd.xlarge | c6gn.12xlarge | c6gn.16xlarge | c6gn.2xlarge | c6gn.4xlarge | c6gn.8xlarge | c6gn.large | c6gn.medium | c6gn.xlarge | c6i.12xlarge | c6i.16xlarge | c6i.24xlarge | c6i.2xlarge | c6i.32xlarge | c6i.4xlarge | c6i.8xlarge | c6i.large | c6i.metal | c6i.xlarge | c6id.12xlarge | c6id.16xlarge | c6id.24xlarge | c6id.2xlarge | c6id.32xlarge | c6id.4xlarge | c6id.8xlarge | c6id.large | c6id.metal | c6id.xlarge | c6in.12xlarge | c6in.16xlarge | c6in.24xlarge | c6in.2xlarge | c6in.32xlarge | c6in.4xlarge | c6in.8xlarge | c6in.large | c6in.metal | c6in.xlarge | c7g.12xlarge | c7g.16xlarge | c7g.2xlarge | c7g.4xlarge | c7g.8xlarge | c7g.large | c7g.medium | c7g.xlarge | cc1.4xlarge | cc2.8xlarge | cg1.4xlarge | cr1.8xlarge | d2.2xlarge | d2.4xlarge | d2.8xlarge | d2.xlarge | d3.2xlarge | d3.4xlarge | d3.8xlarge | d3.xlarge | d3en.12xlarge | d3en.2xlarge | d3en.4xlarge | d3en.6xlarge | d3en.8xlarge | d3en.xlarge | dl1.24xlarge | f1.16xlarge | f1.2xlarge | f1.4xlarge | g2.2xlarge | g2.8xlarge | g3.16xlarge | g3.4xlarge | g3.8xlarge | g3s.xlarge | g4ad.16xlarge | g4ad.2xlarge | g4ad.4xlarge | g4ad.8xlarge | g4ad.xlarge | g4dn.12xlarge | g4dn.16xlarge | g4dn.2xlarge | g4dn.4xlarge | g4dn.8xlarge | g4dn.metal | g4dn.xlarge | g5.12xlarge | g5.16xlarge | g5.24xlarge | g5.2xlarge | g5.48xlarge | g5.4xlarge | g5.8xlarge | g5.xlarge | g5g.16xlarge | g5g.2xlarge | g5g.4xlarge | g5g.8xlarge | g5g.metal | g5g.xlarge | h1.16xlarge | h1.2xlarge | h1.4xlarge | h1.8xlarge | hi1.4xlarge | hpc6a.48xlarge | hpc6id.32xlarge | hs1.8xlarge | i2.2xlarge | i2.4xlarge | i2.8xlarge | i2.xlarge | i3.16xlarge | i3.2xlarge | i3.4xlarge | i3.8xlarge | i3.large | i3.metal | i3.xlarge | i3en.12xlarge | i3en.24xlarge | i3en.2xlarge | i3en.3xlarge | i3en.6xlarge | i3en.large | i3en.metal | i3en.xlarge | i4g.16xlarge | i4g.2xlarge | i4g.4xlarge | i4g.8xlarge | i4g.large | i4g.xlarge | i4i.16xlarge | i4i.2xlarge | i4i.32xlarge | i4i.4xlarge | i4i.8xlarge | i4i.large | i4i.metal | i4i.xlarge | im4gn.16xlarge | im4gn.2xlarge | im4gn.4xlarge | im4gn.8xlarge | im4gn.large | im4gn.xlarge | inf1.24xlarge | inf1.2xlarge | inf1.6xlarge | inf1.xlarge | inf2.24xlarge | inf2.48xlarge | inf2.8xlarge | inf2.xlarge | is4gen.2xlarge | is4gen.4xlarge | is4gen.8xlarge | is4gen.large | is4gen.medium | is4gen.xlarge | m1.large | m1.medium | m1.small | m1.xlarge | m2.2xlarge | m2.4xlarge | m2.xlarge | m3.2xlarge | m3.large | m3.medium | m3.xlarge | m4.10xlarge | m4.16xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.metal | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | m5ad.12xlarge | m5ad.16xlarge | m5ad.24xlarge | m5ad.2xlarge | m5ad.4xlarge | m5ad.8xlarge | m5ad.large | m5ad.xlarge | m5d.12xlarge | m5d.16xlarge | m5d.24xlarge | m5d.2xlarge | m5d.4xlarge | m5d.8xlarge | m5d.large | m5d.metal | m5d.xlarge | m5dn.12xlarge | m5dn.16xlarge | m5dn.24xlarge | m5dn.2xlarge | m5dn.4xlarge | m5dn.8xlarge | m5dn.large | m5dn.metal | m5dn.xlarge | m5n.12xlarge | m5n.16xlarge | m5n.24xlarge | m5n.2xlarge | m5n.4xlarge | m5n.8xlarge | m5n.large | m5n.metal | m5n.xlarge | m5zn.12xlarge | m5zn.2xlarge | m5zn.3xlarge | m5zn.6xlarge | m5zn.large | m5zn.metal | m5zn.xlarge | m6a.12xlarge | m6a.16xlarge | m6a.24xlarge | m6a.2xlarge | m6a.32xlarge | m6a.48xlarge | m6a.4xlarge | m6a.8xlarge | m6a.large | m6a.metal | m6a.xlarge | m6g.12xlarge | m6g.16xlarge | m6g.2xlarge | m6g.4xlarge | m6g.8xlarge | m6g.large | m6g.medium | m6g.metal | m6g.xlarge | m6gd.12xlarge | m6gd.16xlarge | m6gd.2xlarge | m6gd.4xlarge | m6gd.8xlarge | m6gd.large | m6gd.medium | m6gd.metal | m6gd.xlarge | m6i.12xlarge | m6i.16xlarge | m6i.24xlarge | m6i.2xlarge | m6i.32xlarge | m6i.4xlarge | m6i.8xlarge | m6i.large | m6i.metal | m6i.xlarge | m6id.12xlarge | m6id.16xlarge | m6id.24xlarge | m6id.2xlarge | m6id.32xlarge | m6id.4xlarge | m6id.8xlarge | m6id.large | m6id.metal | m6id.xlarge | m6idn.12xlarge | m6idn.16xlarge | m6idn.24xlarge | m6idn.2xlarge | m6idn.32xlarge | m6idn.4xlarge | m6idn.8xlarge | m6idn.large | m6idn.metal | m6idn.xlarge | m6in.12xlarge | m6in.16xlarge | m6in.24xlarge | m6in.2xlarge | m6in.32xlarge | m6in.4xlarge | m6in.8xlarge | m6in.large | m6in.metal | m6in.xlarge | mac1.metal | mac2.metal | p2.16xlarge | p2.8xlarge | p2.xlarge | p3.16xlarge | p3.2xlarge | p3.8xlarge | p3dn.24xlarge | p4d.24xlarge | p4de.24xlarge | r3.2xlarge | r3.4xlarge | r3.8xlarge | r3.large | r3.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.metal | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r5ad.12xlarge | r5ad.16xlarge | r5ad.24xlarge | r5ad.2xlarge | r5ad.4xlarge | r5ad.8xlarge | r5ad.large | r5ad.xlarge | r5b.12xlarge | r5b.16xlarge | r5b.24xlarge | r5b.2xlarge | r5b.4xlarge | r5b.8xlarge | r5b.large | r5b.metal | r5b.xlarge | r5d.12xlarge | r5d.16xlarge | r5d.24xlarge | r5d.2xlarge | r5d.4xlarge | r5d.8xlarge | r5d.large | r5d.metal | r5d.xlarge | r5dn.12xlarge | r5dn.16xlarge | r5dn.24xlarge | r5dn.2xlarge | r5dn.4xlarge | r5dn.8xlarge | r5dn.large | r5dn.metal | r5dn.xlarge | r5n.12xlarge | r5n.16xlarge | r5n.24xlarge | r5n.2xlarge | r5n.4xlarge | r5n.8xlarge | r5n.large | r5n.metal | r5n.xlarge | r6a.12xlarge | r6a.16xlarge | r6a.24xlarge | r6a.2xlarge | r6a.32xlarge | r6a.48xlarge | r6a.4xlarge | r6a.8xlarge | r6a.large | r6a.metal | r6a.xlarge | r6g.12xlarge | r6g.16xlarge | r6g.2xlarge | r6g.4xlarge | r6g.8xlarge | r6g.large | r6g.medium | r6g.metal | r6g.xlarge | r6gd.12xlarge | r6gd.16xlarge | r6gd.2xlarge | r6gd.4xlarge | r6gd.8xlarge | r6gd.large | r6gd.medium | r6gd.metal | r6gd.xlarge | r6i.12xlarge | r6i.16xlarge | r6i.24xlarge | r6i.2xlarge | r6i.32xlarge | r6i.4xlarge | r6i.8xlarge | r6i.large | r6i.metal | r6i.xlarge | r6id.12xlarge | r6id.16xlarge | r6id.24xlarge | r6id.2xlarge | r6id.32xlarge | r6id.4xlarge | r6id.8xlarge | r6id.large | r6id.metal | r6id.xlarge | r6idn.12xlarge | r6idn.16xlarge | r6idn.24xlarge | r6idn.2xlarge | r6idn.32xlarge | r6idn.4xlarge | r6idn.8xlarge | r6idn.large | r6idn.metal | r6idn.xlarge | r6in.12xlarge | r6in.16xlarge | r6in.24xlarge | r6in.2xlarge | r6in.32xlarge | r6in.4xlarge | r6in.8xlarge | r6in.large | r6in.metal | r6in.xlarge | t1.micro | t2.2xlarge | t2.large | t2.medium | t2.micro | t2.nano | t2.small | t2.xlarge | t3.2xlarge | t3.large | t3.medium | t3.micro | t3.nano | t3.small | t3.xlarge | t3a.2xlarge | t3a.large | t3a.medium | t3a.micro | t3a.nano | t3a.small | t3a.xlarge | t4g.2xlarge | t4g.large | t4g.medium | t4g.micro | t4g.nano | t4g.small | t4g.xlarge | trn1.2xlarge | trn1.32xlarge | trn1n.32xlarge | u-12tb1.112xlarge | u-12tb1.metal | u-18tb1.112xlarge | u-18tb1.metal | u-24tb1.112xlarge | u-24tb1.metal | u-3tb1.56xlarge | u-6tb1.112xlarge | u-6tb1.56xlarge | u-6tb1.metal | u-9tb1.112xlarge | u-9tb1.metal | vt1.24xlarge | vt1.3xlarge | vt1.6xlarge | x1.16xlarge | x1.32xlarge | x1e.16xlarge | x1e.2xlarge | x1e.32xlarge | x1e.4xlarge | x1e.8xlarge | x1e.xlarge | x2gd.12xlarge | x2gd.16xlarge | x2gd.2xlarge | x2gd.4xlarge | x2gd.8xlarge | x2gd.large | x2gd.medium | x2gd.metal | x2gd.xlarge | x2idn.16xlarge | x2idn.24xlarge | x2idn.32xlarge | x2idn.metal | x2iedn.16xlarge | x2iedn.24xlarge | x2iedn.2xlarge | x2iedn.32xlarge | x2iedn.4xlarge | x2iedn.8xlarge | x2iedn.metal | x2iedn.xlarge | x2iezn.12xlarge | x2iezn.2xlarge | x2iezn.4xlarge | x2iezn.6xlarge | x2iezn.8xlarge | x2iezn.metal | z1d.12xlarge | z1d.2xlarge | z1d.3xlarge | z1d.6xlarge | z1d.large | z1d.metal | z1d.xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.
    /// 
    /// ImportantIf you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxPrice")]
    pub max_price: Option<String>,

}


/// Specifies the configuration of Spot Instances for an EC2 Fleet.
///
/// SpotOptionsRequest is a property of the       AWS::EC2::EC2Fleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpotOptionsRequest {


    /// 
    /// Indicates how to allocate the target Spot Instance capacity across the Spot Instance     pools specified by the EC2 Fleet.
    /// 
    /// If the allocation strategy is lowestPrice, EC2 Fleet launches instances     from the Spot Instance pools with the lowest price. This is the default allocation     strategy.
    /// 
    /// If the allocation strategy is diversified, EC2 Fleet launches instances     from all the Spot Instance pools that you specify.
    /// 
    /// If the allocation strategy is capacityOptimized, EC2 Fleet launches     instances from Spot Instance pools that are optimally chosen based on the available Spot     Instance capacity.
    /// 
    /// Allowed Values: lowestPrice | diversified     | capacityOptimized | capacityOptimizedPrioritized
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,


    /// 
    /// The maximum amount per hour for Spot Instances that you're willing to pay. We do not recommend     using this parameter because it can lead to increased interruptions. If you do not specify     this parameter, you will pay the current Spot price.
    /// 
    /// ImportantIf you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxTotalPrice")]
    pub max_total_price: Option<String>,


    /// 
    /// The strategies for managing your Spot Instances that are at an elevated risk of being     interrupted.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceStrategies
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaintenanceStrategies")]
    pub maintenance_strategies: Option<MaintenanceStrategies>,


    /// 
    /// The minimum target capacity for Spot Instances in the fleet. If the minimum target capacity is     not reached, the fleet launches no instances.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// At least one of the following must be specified: SingleAvailabilityZone |     SingleInstanceType
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinTargetCapacity")]
    pub min_target_capacity: Option<i64>,


    /// 
    /// Indicates that the fleet launches all Spot Instances into a single Availability Zone.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleAvailabilityZone")]
    pub single_availability_zone: Option<bool>,


    /// 
    /// The behavior when a Spot Instance is interrupted.
    /// 
    /// Default: terminate
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: hibernate | stop | terminate
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceInterruptionBehavior")]
    pub instance_interruption_behavior: Option<String>,


    /// 
    /// Indicates that the fleet uses a single instance type to launch all Spot Instances in the     fleet.
    /// 
    /// Supported only for fleets of type instant.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleInstanceType")]
    pub single_instance_type: Option<bool>,


    /// 
    /// The number of Spot pools across which to allocate your target Spot capacity. Supported     only when Spot AllocationStrategy is set to lowest-price. EC2 Fleet     selects the cheapest Spot pools and evenly allocates your target Spot capacity across the     number of Spot pools that you specify.
    /// 
    /// Note that EC2 Fleet attempts to draw Spot Instances from the number of pools that you specify on a     best effort basis. If a pool runs out of Spot capacity before fulfilling your target     capacity, EC2 Fleet will continue to fulfill your request by drawing from the next cheapest     pool. To ensure that your target capacity is met, you might receive Spot Instances from more than     the number of pools that you specified. Similarly, if most of the pools have no Spot     capacity, you might receive your full target capacity from fewer than the number of pools     that you specified.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstancePoolsToUseCount")]
    pub instance_pools_to_use_count: Option<i64>,

}


/// The strategies for managing your Spot Instances that are at an elevated risk of being     interrupted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceStrategies {


    /// 
    /// The strategy to use when Amazon EC2 emits a signal that your Spot Instance is at an     elevated risk of being interrupted.
    /// 
    /// Required: No
    ///
    /// Type: CapacityRebalance
    ///
    /// Update requires: Replacement
    #[serde(rename = "CapacityRebalance")]
    pub capacity_rebalance: Option<CapacityRebalance>,

}


/// The minimum and maximum number of vCPUs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VCpuCountRangeRequest {


    /// 
    /// The maximum number of vCPUs. To specify no maximum limit, omit this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum number of vCPUs. To specify no minimum limit, specify 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}


/// Specifies the number of units to request for an EC2 Fleet. You can choose to set the     target capacity in terms of instances or a performance characteristic that is important to     your application workload, such as vCPUs, memory, or I/O. If the request type is       maintain, you can specify a target capacity of 0 and add     capacity later.
///
/// TargetCapacitySpecificationRequest is a property of the AWS::EC2::EC2Fleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetCapacitySpecificationRequest {


    /// 
    /// The number of units to request, filled using     DefaultTargetCapacityType.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalTargetCapacity")]
    pub total_target_capacity: i64,


    /// 
    /// The number of On-Demand units to request.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnDemandTargetCapacity")]
    pub on_demand_target_capacity: Option<i64>,


    /// 
    /// The default TotalTargetCapacity, which is either Spot or     On-Demand.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: on-demand | spot
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTargetCapacityType")]
    pub default_target_capacity_type: Option<String>,


    /// 
    /// The number of Spot units to request.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotTargetCapacity")]
    pub spot_target_capacity: Option<i64>,


    /// 
    /// The unit for the target capacity. TargetCapacityUnitType can only be specified when InstanceRequirements is specified.
    /// 
    /// Default: units (translates to number of instances)
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: memory-mib | units | vcpu
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetCapacityUnitType")]
    pub target_capacity_unit_type: Option<String>,

}


/// The minimum and maximum baseline bandwidth to Amazon EBS, in Mbps. For more information, see       Amazon       EBS–optimized instances in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BaselineEbsBandwidthMbpsRequest {


    /// 
    /// The maximum baseline bandwidth, in Mbps. To specify no maximum limit, omit     this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum baseline bandwidth, in Mbps. To specify no minimum limit, omit     this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}


/// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips)     on an instance. To exclude accelerator-enabled instance types, set Max to       0.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AcceleratorCountRequest {


    /// 
    /// The minimum number of accelerators. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<i64>,


    /// 
    /// The maximum number of accelerators. To specify no maximum limit, omit this     parameter. To exclude accelerator-enabled instance types, set Max to     0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<i64>,

}


/// The minimum and maximum amount of total local storage, in GB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TotalLocalStorageGBRequest {


    /// 
    /// The maximum amount of total local storage, in GB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum amount of total local storage, in GB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}


/// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkBandwidthGbpsRequest {


    /// 
    /// The maximum amount of network bandwidth, in Gbps. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum amount of network bandwidth, in Gbps. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}


/// Specifies the launch template to be used by the EC2 Fleet for configuring Amazon EC2 instances.
///
/// You must specify the following:
///
/// FleetLaunchTemplateSpecificationRequest is a property of the FleetLaunchTemplateConfigRequest property type.
///
/// For information about creating a launch template, see      AWS::EC2::LaunchTemplate and      Create a launch template     in the Amazon EC2 User Guide.
///
/// For examples of launch templates, see Examples.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FleetLaunchTemplateSpecificationRequest {


    /// 
    /// The ID of the launch template.
    /// 
    /// You must specify the LaunchTemplateId or the LaunchTemplateName, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,


    /// 
    /// The launch template version number, $Latest, or $Default. You must specify a value, otherwise the request fails.
    /// 
    /// If the value is $Latest, Amazon EC2 uses the latest version of the launch template.
    /// 
    /// If the value is $Default, Amazon EC2 uses the default version of the launch template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: String,


    /// 
    /// The name of the launch template.
    /// 
    /// You must specify the LaunchTemplateName or the LaunchTemplateId, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9\(\)\.\-/_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,

}
