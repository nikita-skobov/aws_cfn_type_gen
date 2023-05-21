

/// Specifies the properties for creating a launch template.
///
/// The minimum required properties for specifying a launch template are as follows:
///
/// A launch template can contain some or all of the configuration information to launch an     instance. When you launch an instance using a launch template, instance properties that are     not specified in the launch template use default values, except the ImageId     property, which has no default value. If you do not specify an AMI ID for the launch     template ImageId property, you must specify an AMI ID for the instance       ImageId property.
///
/// For more information, see Launch an instance from a       launch template in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLaunchTemplate {


    /// 
    /// The information for the launch template.
    /// 
    /// Required: Yes
    ///
    /// Type: LaunchTemplateData
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateData")]
    pub launch_template_data: LaunchTemplateData,


    /// 
    /// A name for the launch template.
    /// 
    /// Required: No
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


    /// 
    /// The tags to apply to the launch template on creation. To tag the launch template, the       resource type must be launch-template.
    /// 
    /// NoteTo specify the tags for the resources that are created when an instance is         launched, you must use the TagSpecifications parameter in the launch           template data structure.
    /// 
    /// Required: No
    ///
    /// Type: List of LaunchTemplateTagSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<LaunchTemplateTagSpecification>>,


    /// 
    /// A description for the first version of the launch template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,

}



impl cfn_resources::CfnResource for CfnLaunchTemplate {
    fn type_string() -> &'static str {
        "AWS::EC2::LaunchTemplate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips)     on an instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AcceleratorCount {


    /// 
    /// The maximum number of accelerators. To specify no maximum limit, omit this     parameter. To exclude accelerator-enabled instance types, set Max to     0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum number of accelerators. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}




/// The minimum and maximum amount of total accelerator memory, in MiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AcceleratorTotalMemoryMiB {


    /// 
    /// The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}




/// The minimum and maximum baseline bandwidth to Amazon EBS, in Mbps. For more information, see       Amazon       EBS–optimized instances in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BaselineEbsBandwidthMbps {


    /// 
    /// The maximum baseline bandwidth, in Mbps. To specify no maximum limit, omit     this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum baseline bandwidth, in Mbps. To specify no minimum limit, omit     this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}




/// Information about a block device mapping for an Amazon EC2 launch template.
///
/// BlockDeviceMapping is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlockDeviceMapping {


    /// 
    /// The device name (for example, /dev/sdh or xvdh).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,


    /// 
    /// Parameters used to automatically set up EBS volumes when the instance is       launched.
    /// 
    /// Required: No
    ///
    /// Type: Ebs
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ebs")]
    pub ebs: Option<Ebs>,


    /// 
    /// To omit the device from the block device mapping, specify an empty string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,


    /// 
    /// The virtual device name (ephemeralN). Instance store volumes are numbered starting       from 0. An instance type with 2 available instance store volumes can specify mappings       for ephemeral0 and ephemeral1. The number of available instance store volumes depends on       the instance type. After you connect to the instance, you must mount the volume.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,

}




/// Specifies an instance's Capacity Reservation targeting option. You can specify only one     option at a time.
///
/// CapacityReservationSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityReservationSpecification {


    /// 
    /// Indicates the instance's Capacity Reservation preferences. Possible preferences       include:
    /// 
    /// open - The instance can run in any open Capacity           Reservation that has matching attributes (instance type, platform, Availability           Zone).                        none - The instance avoids running in a Capacity Reservation even           if one is available. The instance runs in On-Demand capacity.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: none | open
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityReservationPreference")]
    pub capacity_reservation_preference: Option<CapacityReservationSpecificationCapacityReservationPreferenceEnum>,


    /// 
    /// Information about the target Capacity Reservation or Capacity Reservation       group.
    /// 
    /// Required: No
    ///
    /// Type: CapacityReservationTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityReservationTarget")]
    pub capacity_reservation_target: Option<CapacityReservationTarget>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CapacityReservationSpecificationCapacityReservationPreferenceEnum {

    /// none
    #[serde(rename = "none")]
    None,

    /// open
    #[serde(rename = "open")]
    Open,

}

impl Default for CapacityReservationSpecificationCapacityReservationPreferenceEnum {
    fn default() -> Self {
        CapacityReservationSpecificationCapacityReservationPreferenceEnum::None
    }
}



/// Specifies a target Capacity Reservation.
///
/// CapacityReservationTarget is a property of the Amazon EC2 LaunchTemplate LaunchTemplateData property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityReservationTarget {


    /// 
    /// The ID of the Capacity Reservation in which to run the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityReservationId")]
    pub capacity_reservation_id: Option<String>,


    /// 
    /// The ARN of the Capacity Reservation resource group in which to run the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityReservationResourceGroupArn")]
    pub capacity_reservation_resource_group_arn: Option<String>,

}




/// Specifies the CPU options for an instance. For more information, see Optimize       CPU options in the Amazon Elastic Compute Cloud User     Guide.
///
/// CpuOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CpuOptions {


    /// 
    /// Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported       with M6a, R6a, and C6a instance types only. For more information, see       AMD SEV-SNP.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmdSevSnp")]
    pub amd_sev_snp: Option<CpuOptionsAmdSevSnpEnum>,


    /// 
    /// The number of CPU cores for the instance.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoreCount")]
    pub core_count: Option<i64>,


    /// 
    /// The number of threads per CPU core. To disable multithreading for the instance,       specify a value of 1. Otherwise, specify the default value of         2.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThreadsPerCore")]
    pub threads_per_core: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CpuOptionsAmdSevSnpEnum {

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,

}

impl Default for CpuOptionsAmdSevSnpEnum {
    fn default() -> Self {
        CpuOptionsAmdSevSnpEnum::Disabled
    }
}



/// Specifies the credit option for CPU usage of a T2, T3, or T3a instance.
///
/// CreditSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreditSpecification {


    /// 
    /// The credit option for CPU usage of a T instance.
    /// 
    /// Valid values: standard | unlimited
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuCredits")]
    pub cpu_credits: Option<String>,

}




/// Parameters for a block device for an EBS volume in an Amazon EC2 launch template.
///
/// Ebs is a property of AWS::EC2::LaunchTemplate BlockDeviceMapping.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ebs {


    /// 
    /// Indicates whether the EBS volume is deleted on instance termination.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// Indicates whether the EBS volume is encrypted. Encrypted volumes can only be attached       to instances that support Amazon EBS encryption. If you are creating a volume from a       snapshot, you can't specify an encryption value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// The number of I/O operations per second (IOPS). For gp3,       io1, and io2 volumes, this represents the number of IOPS that       are provisioned for the volume. For gp2 volumes, this represents the       baseline performance of the volume and the rate at which the volume accumulates I/O       credits for bursting.
    /// 
    /// The following are the supported values for each volume type:
    /// 
    /// gp3: 3,000-16,000 IOPS                        io1: 100-64,000 IOPS                        io2: 100-64,000 IOPS
    /// 
    /// For io1 and io2 volumes, we guarantee       64,000 IOPS only for Instances built on the         Nitro System. Other instance families guarantee performance up to       32,000 IOPS.
    /// 
    /// This parameter is supported for io1, io2, and gp3 volumes only. This parameter       is not supported for gp2, st1, sc1, or standard volumes.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// The ARN of the symmetric AWS Key Management Service (AWS KMS) CMK used for encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The ID of the snapshot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// 
    /// The throughput to provision for a gp3 volume, with a maximum of 1,000       MiB/s.
    /// 
    /// Valid Range: Minimum value of 125. Maximum value of 1000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throughput")]
    pub throughput: Option<i64>,


    /// 
    /// The size of the volume, in GiBs. You must specify either a snapshot ID or a volume       size. The following are the supported volumes sizes for each volume type:
    /// 
    /// gp2 and gp3: 1-16,384                        io1 and io2: 4-16,384                        st1 and sc1: 125-16,384                        standard: 1-1,024
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// 
    /// The volume type. For more information, see Amazon EBS volume types in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | io2 | sc1 | st1 | standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<EbsVolumeTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EbsVolumeTypeEnum {

    /// gp2
    #[serde(rename = "gp2")]
    Gp2,

    /// gp3
    #[serde(rename = "gp3")]
    Gp3,

    /// io1
    #[serde(rename = "io1")]
    Io1,

    /// io2
    #[serde(rename = "io2")]
    Io2,

    /// sc1
    #[serde(rename = "sc1")]
    Sc1,

    /// st1
    #[serde(rename = "st1")]
    St1,

    /// standard
    #[serde(rename = "standard")]
    Standard,

}

impl Default for EbsVolumeTypeEnum {
    fn default() -> Self {
        EbsVolumeTypeEnum::Gp2
    }
}



/// Specifies a specification for an Elastic GPU for an Amazon EC2 launch template.
///
/// ElasticGpuSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticGpuSpecification {


    /// 
    /// The type of Elastic Graphics accelerator. For more information about the values to specify for       Type, see Elastic Graphics Basics, specifically the Elastic Graphics accelerator column, in the       Amazon Elastic Compute Cloud User Guide for Windows Instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}




/// Indicates whether the instance is enabled for AWS Nitro       Enclaves.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EnclaveOptions {


    /// 
    /// If this parameter is set to true, the instance is enabled for AWS Nitro Enclaves; otherwise, it is not enabled for AWS Nitro       Enclaves.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}




/// Specifies whether your instance is configured for hibernation. This parameter is valid     only if the instance meets the hibernation       prerequisites. For more information, see Hibernate Your Instance in the       Amazon EC2 User Guide.
///
/// HibernationOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HibernationOptions {


    /// 
    /// If you set this parameter to true, the instance is enabled for       hibernation.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configured")]
    pub configured: Option<bool>,

}




/// Specifies an IAM instance profile, which is a container for an IAM role for your     instance. You can use an IAM role to distribute your AWS credentials to     your instances.
///
/// If you are creating the launch template for use with an Amazon EC2 Auto Scaling group,     you can specify either the name or the ARN of the instance profile, but not both.
///
/// IamInstanceProfile is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IamInstanceProfile {


    /// 
    /// The Amazon Resource Name (ARN) of the instance profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// The name of the instance profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}




/// Specifies the market (purchasing) option for an instance.
///
/// InstanceMarketOptions is a property of the AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceMarketOptions {


    /// 
    /// The market type.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: spot
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarketType")]
    pub market_type: Option<InstanceMarketOptionsMarketTypeEnum>,


    /// 
    /// The options for Spot Instances.
    /// 
    /// Required: No
    ///
    /// Type: SpotOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotOptions")]
    pub spot_options: Option<SpotOptions>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceMarketOptionsMarketTypeEnum {

    /// spot
    #[serde(rename = "spot")]
    Spot,

}

impl Default for InstanceMarketOptionsMarketTypeEnum {
    fn default() -> Self {
        InstanceMarketOptionsMarketTypeEnum::Spot
    }
}



/// The attributes for the instance types. When you specify instance attributes, Amazon EC2 will     identify instance types with these attributes.
///
/// When you specify multiple attributes, you get instance types that satisfy all of the     specified attributes. If you specify multiple values for an attribute, you get instance     types that satisfy any of the specified values.
///
/// To limit the list of instance types from which Amazon EC2 can identify matching instance types,      you can use one of the following parameters, but not both in the same request:
///
/// For more information, see Attribute-based instance type selection for EC2 Fleet, Attribute-based instance type selection for Spot Fleet, and Spot        placement score in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceRequirements {


    /// 
    /// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips) on     an instance.
    /// 
    /// To exclude accelerator-enabled instance types, set Max to 0.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: AcceleratorCount
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCount>,


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
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,


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
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of total accelerator memory, in MiB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: AcceleratorTotalMemoryMiB
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiB>,


    /// 
    /// The accelerator types that must be on the instance type.
    /// 
    /// For instance types with GPU accelerators, specify gpu.               For instance types with FPGA accelerators, specify fpga.               For instance types with inference accelerators, specify inference.
    /// 
    /// Default: Any accelerator type
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,


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
    /// Update requires: No interruption
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,


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
    /// Update requires: No interruption
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<InstanceRequirementsBareMetalEnum>,


    /// 
    /// The minimum and maximum baseline bandwidth to Amazon EBS, in Mbps. For more information, see       Amazon       EBS–optimized instances in the Amazon EC2 User Guide.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: BaselineEbsBandwidthMbps
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbps>,


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
    /// Update requires: No interruption
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<InstanceRequirementsBurstablePerformanceEnum>,


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
    /// Update requires: No interruption
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,


    /// 
    /// The instance types to exclude.
    /// 
    /// You can use strings with one or more wild cards, represented by    an asterisk (*), to exclude an instance type, size, or generation. The    following are examples: m5.8xlarge, c5*.*, m5a.*,    r*, *3*.
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
    /// Update requires: No interruption
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,


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
    /// Update requires: No interruption
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,


    /// 
    /// Indicates whether instance types with instance store volumes are included, excluded, or required. For more information,       Amazon       EC2 instance store in the Amazon EC2 User Guide.
    /// 
    /// To include instance types with instance store volumes, specify        included.               To require only instance types with instance store volumes, specify          required.               To exclude instance types with instance store volumes, specify        excluded.
    /// 
    /// Default: included
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<InstanceRequirementsLocalStorageEnum>,


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
    /// Update requires: No interruption
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,


    /// 
    /// The minimum and maximum amount of memory per vCPU, in GiB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: MemoryGiBPerVCpu
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpu>,


    /// 
    /// The minimum and maximum amount of memory, in MiB.
    /// 
    /// Required: No
    ///
    /// Type: MemoryMiB
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiB>,


    /// 
    /// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps).
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: NetworkBandwidthGbps
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbps>,


    /// 
    /// The minimum and maximum number of network interfaces.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: NetworkInterfaceCount
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCount>,


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
    /// Update requires: No interruption
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i64>,


    /// 
    /// Indicates whether instance types must support hibernation for On-Demand     Instances.
    /// 
    /// This parameter is not supported for GetSpotPlacementScores.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,


    /// 
    /// The price protection threshold for Spot Instances. This is the maximum you’ll pay for a Spot Instance,     expressed as a percentage above the least expensive current generation M, C, or R instance type with your specified     attributes. When Amazon EC2 selects instance types with your attributes, it excludes instance     types priced above your threshold.
    /// 
    /// The parameter accepts an integer, which Amazon EC2 interprets as a percentage.
    /// 
    /// To turn off price protection, specify a high value, such as 999999.
    /// 
    /// This parameter is not supported for GetSpotPlacementScores and GetInstanceTypesFromInstanceRequirements.
    /// 
    /// NoteIf you set TargetCapacityUnitType to vcpu or        memory-mib, the price protection threshold is applied based on the       per-vCPU or per-memory price instead of the per-instance price.
    /// 
    /// Default: 100
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<i64>,


    /// 
    /// The minimum and maximum amount of total local storage, in GB.
    /// 
    /// Default: No minimum or maximum limits
    /// 
    /// Required: No
    ///
    /// Type: TotalLocalStorageGB
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGB>,


    /// 
    /// The minimum and maximum number of vCPUs.
    /// 
    /// Required: No
    ///
    /// Type: VCpuCount
    ///
    /// Update requires: No interruption
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCount>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceRequirementsBareMetalEnum {

    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,

}

impl Default for InstanceRequirementsBareMetalEnum {
    fn default() -> Self {
        InstanceRequirementsBareMetalEnum::Excluded
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceRequirementsBurstablePerformanceEnum {

    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,

}

impl Default for InstanceRequirementsBurstablePerformanceEnum {
    fn default() -> Self {
        InstanceRequirementsBurstablePerformanceEnum::Excluded
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceRequirementsLocalStorageEnum {

    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,

}

impl Default for InstanceRequirementsLocalStorageEnum {
    fn default() -> Self {
        InstanceRequirementsLocalStorageEnum::Excluded
    }
}



/// Specifies an IPv4 prefix for a network interface.
///
/// Ipv4PrefixSpecification is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ipv4PrefixSpecification {


    /// 
    /// The IPv4 prefix. For information, see       Assigning prefixes to Amazon EC2 network interfaces in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv4Prefix")]
    pub ipv4_prefix: Option<String>,

}




/// Specifies an IPv6 address in an Amazon EC2 launch template.
///
/// Ipv6Add is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ipv6Add {


    /// 
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You       can't use this option if you're specifying a number of IPv6 addresses.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: Option<String>,

}




/// Specifies an IPv6 prefix for a network interface.
///
/// Ipv6PrefixSpecification is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ipv6PrefixSpecification {


    /// 
    /// The IPv6 prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Prefix")]
    pub ipv6_prefix: Option<String>,

}




/// The information to include in the launch template.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateData {


    /// 
    /// The block device mapping.
    /// 
    /// Required: No
    ///
    /// Type: List of BlockDeviceMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,


    /// 
    /// The Capacity Reservation targeting option. If you do not specify this parameter, the       instance's Capacity Reservation preference defaults to open, which enables       it to run in any open Capacity Reservation that has matching attributes (instance type,       platform, Availability Zone).
    /// 
    /// Required: No
    ///
    /// Type: CapacityReservationSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityReservationSpecification")]
    pub capacity_reservation_specification: Option<CapacityReservationSpecification>,


    /// 
    /// The CPU options for the instance. For more information, see Optimizing CPU Options in the Amazon Elastic Compute Cloud User         Guide.
    /// 
    /// Required: No
    ///
    /// Type: CpuOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuOptions")]
    pub cpu_options: Option<CpuOptions>,


    /// 
    /// The credit option for CPU usage of the instance. Valid only for T instances.
    /// 
    /// Required: No
    ///
    /// Type: CreditSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreditSpecification")]
    pub credit_specification: Option<CreditSpecification>,


    /// 
    /// Indicates whether to enable the instance for stop protection. For more information,       see Stop         protection in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableApiStop")]
    pub disable_api_stop: Option<bool>,


    /// 
    /// If you set this parameter to true, you can't terminate the instance using       the Amazon EC2 console, CLI, or API; otherwise, you can. To change this attribute after       launch, use ModifyInstanceAttribute. Alternatively, if you set         InstanceInitiatedShutdownBehavior to terminate, you can       terminate the instance by running the shutdown command from the instance.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableApiTermination")]
    pub disable_api_termination: Option<bool>,


    /// 
    /// Indicates whether the instance is optimized for Amazon EBS I/O. This optimization       provides dedicated throughput to Amazon EBS and an optimized configuration stack to       provide optimal Amazon EBS I/O performance. This optimization isn't available with all       instance types. Additional usage charges apply when using an EBS-optimized       instance.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,


    /// 
    /// An elastic GPU to associate with the instance.
    /// 
    /// Required: No
    ///
    /// Type: List of ElasticGpuSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticGpuSpecifications")]
    pub elastic_gpu_specifications: Option<Vec<ElasticGpuSpecification>>,


    /// 
    /// The elastic inference accelerator for the instance.
    /// 
    /// Required: No
    ///
    /// Type: List of LaunchTemplateElasticInferenceAccelerator
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticInferenceAccelerators")]
    pub elastic_inference_accelerators: Option<Vec<LaunchTemplateElasticInferenceAccelerator>>,


    /// 
    /// Indicates whether the instance is enabled for AWS Nitro Enclaves. For more       information, see What is AWS Nitro Enclaves?       in the         AWS Nitro Enclaves User Guide.
    /// 
    /// You can't enable AWS Nitro Enclaves and hibernation on the same instance.
    /// 
    /// Required: No
    ///
    /// Type: EnclaveOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnclaveOptions")]
    pub enclave_options: Option<EnclaveOptions>,


    /// 
    /// Indicates whether an instance is enabled for hibernation. This parameter is valid only       if the instance meets the hibernation         prerequisites. For more information, see Hibernate your instance in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: HibernationOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "HibernationOptions")]
    pub hibernation_options: Option<HibernationOptions>,


    /// 
    /// The name or Amazon Resource Name (ARN) of an IAM instance profile.
    /// 
    /// Required: No
    ///
    /// Type: IamInstanceProfile
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<IamInstanceProfile>,


    /// 
    /// The ID of the AMI. Alternatively, you can specify a Systems Manager parameter, which     will resolve to an AMI ID on launch.
    /// 
    /// Valid formats:
    /// 
    /// ami-17characters00000                          resolve:ssm:parameter-name                          resolve:ssm:parameter-name:version-number                          resolve:ssm:parameter-name:label
    /// 
    /// For more information, see Use a Systems Manager parameter to find an AMI in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,


    /// 
    /// Indicates whether an instance stops or terminates when you initiate shutdown from the       instance (using the operating system command for system shutdown).
    /// 
    /// Default: stop
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: stop | terminate
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    pub instance_initiated_shutdown_behavior: Option<LaunchTemplateDataInstanceInitiatedShutdownBehaviorEnum>,


    /// 
    /// The market (purchasing) option for the instances.
    /// 
    /// Required: No
    ///
    /// Type: InstanceMarketOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMarketOptions")]
    pub instance_market_options: Option<InstanceMarketOptions>,


    /// 
    /// The attributes for the instance types. When you specify instance attributes, Amazon EC2 will     identify instance types with these attributes.
    /// 
    /// If you specify InstanceRequirements, you can't specify       InstanceType.
    /// 
    /// Required: No
    ///
    /// Type: InstanceRequirements
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirements>,


    /// 
    /// The instance type. For more information, see Instance types in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// If you specify InstanceType, you can't specify         InstanceRequirements.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: a1.2xlarge | a1.4xlarge | a1.large | a1.medium | a1.metal | a1.xlarge | c1.medium | c1.xlarge | c3.2xlarge | c3.4xlarge | c3.8xlarge | c3.large | c3.xlarge | c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.metal | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c5ad.12xlarge | c5ad.16xlarge | c5ad.24xlarge | c5ad.2xlarge | c5ad.4xlarge | c5ad.8xlarge | c5ad.large | c5ad.xlarge | c5d.12xlarge | c5d.18xlarge | c5d.24xlarge | c5d.2xlarge | c5d.4xlarge | c5d.9xlarge | c5d.large | c5d.metal | c5d.xlarge | c5n.18xlarge | c5n.2xlarge | c5n.4xlarge | c5n.9xlarge | c5n.large | c5n.metal | c5n.xlarge | c6a.12xlarge | c6a.16xlarge | c6a.24xlarge | c6a.2xlarge | c6a.32xlarge | c6a.48xlarge | c6a.4xlarge | c6a.8xlarge | c6a.large | c6a.metal | c6a.xlarge | c6g.12xlarge | c6g.16xlarge | c6g.2xlarge | c6g.4xlarge | c6g.8xlarge | c6g.large | c6g.medium | c6g.metal | c6g.xlarge | c6gd.12xlarge | c6gd.16xlarge | c6gd.2xlarge | c6gd.4xlarge | c6gd.8xlarge | c6gd.large | c6gd.medium | c6gd.metal | c6gd.xlarge | c6gn.12xlarge | c6gn.16xlarge | c6gn.2xlarge | c6gn.4xlarge | c6gn.8xlarge | c6gn.large | c6gn.medium | c6gn.xlarge | c6i.12xlarge | c6i.16xlarge | c6i.24xlarge | c6i.2xlarge | c6i.32xlarge | c6i.4xlarge | c6i.8xlarge | c6i.large | c6i.metal | c6i.xlarge | c6id.12xlarge | c6id.16xlarge | c6id.24xlarge | c6id.2xlarge | c6id.32xlarge | c6id.4xlarge | c6id.8xlarge | c6id.large | c6id.metal | c6id.xlarge | c6in.12xlarge | c6in.16xlarge | c6in.24xlarge | c6in.2xlarge | c6in.32xlarge | c6in.4xlarge | c6in.8xlarge | c6in.large | c6in.metal | c6in.xlarge | c7g.12xlarge | c7g.16xlarge | c7g.2xlarge | c7g.4xlarge | c7g.8xlarge | c7g.large | c7g.medium | c7g.xlarge | cc1.4xlarge | cc2.8xlarge | cg1.4xlarge | cr1.8xlarge | d2.2xlarge | d2.4xlarge | d2.8xlarge | d2.xlarge | d3.2xlarge | d3.4xlarge | d3.8xlarge | d3.xlarge | d3en.12xlarge | d3en.2xlarge | d3en.4xlarge | d3en.6xlarge | d3en.8xlarge | d3en.xlarge | dl1.24xlarge | f1.16xlarge | f1.2xlarge | f1.4xlarge | g2.2xlarge | g2.8xlarge | g3.16xlarge | g3.4xlarge | g3.8xlarge | g3s.xlarge | g4ad.16xlarge | g4ad.2xlarge | g4ad.4xlarge | g4ad.8xlarge | g4ad.xlarge | g4dn.12xlarge | g4dn.16xlarge | g4dn.2xlarge | g4dn.4xlarge | g4dn.8xlarge | g4dn.metal | g4dn.xlarge | g5.12xlarge | g5.16xlarge | g5.24xlarge | g5.2xlarge | g5.48xlarge | g5.4xlarge | g5.8xlarge | g5.xlarge | g5g.16xlarge | g5g.2xlarge | g5g.4xlarge | g5g.8xlarge | g5g.metal | g5g.xlarge | h1.16xlarge | h1.2xlarge | h1.4xlarge | h1.8xlarge | hi1.4xlarge | hpc6a.48xlarge | hpc6id.32xlarge | hs1.8xlarge | i2.2xlarge | i2.4xlarge | i2.8xlarge | i2.xlarge | i3.16xlarge | i3.2xlarge | i3.4xlarge | i3.8xlarge | i3.large | i3.metal | i3.xlarge | i3en.12xlarge | i3en.24xlarge | i3en.2xlarge | i3en.3xlarge | i3en.6xlarge | i3en.large | i3en.metal | i3en.xlarge | i4g.16xlarge | i4g.2xlarge | i4g.4xlarge | i4g.8xlarge | i4g.large | i4g.xlarge | i4i.16xlarge | i4i.2xlarge | i4i.32xlarge | i4i.4xlarge | i4i.8xlarge | i4i.large | i4i.metal | i4i.xlarge | im4gn.16xlarge | im4gn.2xlarge | im4gn.4xlarge | im4gn.8xlarge | im4gn.large | im4gn.xlarge | inf1.24xlarge | inf1.2xlarge | inf1.6xlarge | inf1.xlarge | inf2.24xlarge | inf2.48xlarge | inf2.8xlarge | inf2.xlarge | is4gen.2xlarge | is4gen.4xlarge | is4gen.8xlarge | is4gen.large | is4gen.medium | is4gen.xlarge | m1.large | m1.medium | m1.small | m1.xlarge | m2.2xlarge | m2.4xlarge | m2.xlarge | m3.2xlarge | m3.large | m3.medium | m3.xlarge | m4.10xlarge | m4.16xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.metal | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | m5ad.12xlarge | m5ad.16xlarge | m5ad.24xlarge | m5ad.2xlarge | m5ad.4xlarge | m5ad.8xlarge | m5ad.large | m5ad.xlarge | m5d.12xlarge | m5d.16xlarge | m5d.24xlarge | m5d.2xlarge | m5d.4xlarge | m5d.8xlarge | m5d.large | m5d.metal | m5d.xlarge | m5dn.12xlarge | m5dn.16xlarge | m5dn.24xlarge | m5dn.2xlarge | m5dn.4xlarge | m5dn.8xlarge | m5dn.large | m5dn.metal | m5dn.xlarge | m5n.12xlarge | m5n.16xlarge | m5n.24xlarge | m5n.2xlarge | m5n.4xlarge | m5n.8xlarge | m5n.large | m5n.metal | m5n.xlarge | m5zn.12xlarge | m5zn.2xlarge | m5zn.3xlarge | m5zn.6xlarge | m5zn.large | m5zn.metal | m5zn.xlarge | m6a.12xlarge | m6a.16xlarge | m6a.24xlarge | m6a.2xlarge | m6a.32xlarge | m6a.48xlarge | m6a.4xlarge | m6a.8xlarge | m6a.large | m6a.metal | m6a.xlarge | m6g.12xlarge | m6g.16xlarge | m6g.2xlarge | m6g.4xlarge | m6g.8xlarge | m6g.large | m6g.medium | m6g.metal | m6g.xlarge | m6gd.12xlarge | m6gd.16xlarge | m6gd.2xlarge | m6gd.4xlarge | m6gd.8xlarge | m6gd.large | m6gd.medium | m6gd.metal | m6gd.xlarge | m6i.12xlarge | m6i.16xlarge | m6i.24xlarge | m6i.2xlarge | m6i.32xlarge | m6i.4xlarge | m6i.8xlarge | m6i.large | m6i.metal | m6i.xlarge | m6id.12xlarge | m6id.16xlarge | m6id.24xlarge | m6id.2xlarge | m6id.32xlarge | m6id.4xlarge | m6id.8xlarge | m6id.large | m6id.metal | m6id.xlarge | m6idn.12xlarge | m6idn.16xlarge | m6idn.24xlarge | m6idn.2xlarge | m6idn.32xlarge | m6idn.4xlarge | m6idn.8xlarge | m6idn.large | m6idn.metal | m6idn.xlarge | m6in.12xlarge | m6in.16xlarge | m6in.24xlarge | m6in.2xlarge | m6in.32xlarge | m6in.4xlarge | m6in.8xlarge | m6in.large | m6in.metal | m6in.xlarge | mac1.metal | mac2.metal | p2.16xlarge | p2.8xlarge | p2.xlarge | p3.16xlarge | p3.2xlarge | p3.8xlarge | p3dn.24xlarge | p4d.24xlarge | p4de.24xlarge | r3.2xlarge | r3.4xlarge | r3.8xlarge | r3.large | r3.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.metal | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r5ad.12xlarge | r5ad.16xlarge | r5ad.24xlarge | r5ad.2xlarge | r5ad.4xlarge | r5ad.8xlarge | r5ad.large | r5ad.xlarge | r5b.12xlarge | r5b.16xlarge | r5b.24xlarge | r5b.2xlarge | r5b.4xlarge | r5b.8xlarge | r5b.large | r5b.metal | r5b.xlarge | r5d.12xlarge | r5d.16xlarge | r5d.24xlarge | r5d.2xlarge | r5d.4xlarge | r5d.8xlarge | r5d.large | r5d.metal | r5d.xlarge | r5dn.12xlarge | r5dn.16xlarge | r5dn.24xlarge | r5dn.2xlarge | r5dn.4xlarge | r5dn.8xlarge | r5dn.large | r5dn.metal | r5dn.xlarge | r5n.12xlarge | r5n.16xlarge | r5n.24xlarge | r5n.2xlarge | r5n.4xlarge | r5n.8xlarge | r5n.large | r5n.metal | r5n.xlarge | r6a.12xlarge | r6a.16xlarge | r6a.24xlarge | r6a.2xlarge | r6a.32xlarge | r6a.48xlarge | r6a.4xlarge | r6a.8xlarge | r6a.large | r6a.metal | r6a.xlarge | r6g.12xlarge | r6g.16xlarge | r6g.2xlarge | r6g.4xlarge | r6g.8xlarge | r6g.large | r6g.medium | r6g.metal | r6g.xlarge | r6gd.12xlarge | r6gd.16xlarge | r6gd.2xlarge | r6gd.4xlarge | r6gd.8xlarge | r6gd.large | r6gd.medium | r6gd.metal | r6gd.xlarge | r6i.12xlarge | r6i.16xlarge | r6i.24xlarge | r6i.2xlarge | r6i.32xlarge | r6i.4xlarge | r6i.8xlarge | r6i.large | r6i.metal | r6i.xlarge | r6id.12xlarge | r6id.16xlarge | r6id.24xlarge | r6id.2xlarge | r6id.32xlarge | r6id.4xlarge | r6id.8xlarge | r6id.large | r6id.metal | r6id.xlarge | r6idn.12xlarge | r6idn.16xlarge | r6idn.24xlarge | r6idn.2xlarge | r6idn.32xlarge | r6idn.4xlarge | r6idn.8xlarge | r6idn.large | r6idn.metal | r6idn.xlarge | r6in.12xlarge | r6in.16xlarge | r6in.24xlarge | r6in.2xlarge | r6in.32xlarge | r6in.4xlarge | r6in.8xlarge | r6in.large | r6in.metal | r6in.xlarge | t1.micro | t2.2xlarge | t2.large | t2.medium | t2.micro | t2.nano | t2.small | t2.xlarge | t3.2xlarge | t3.large | t3.medium | t3.micro | t3.nano | t3.small | t3.xlarge | t3a.2xlarge | t3a.large | t3a.medium | t3a.micro | t3a.nano | t3a.small | t3a.xlarge | t4g.2xlarge | t4g.large | t4g.medium | t4g.micro | t4g.nano | t4g.small | t4g.xlarge | trn1.2xlarge | trn1.32xlarge | trn1n.32xlarge | u-12tb1.112xlarge | u-12tb1.metal | u-18tb1.112xlarge | u-18tb1.metal | u-24tb1.112xlarge | u-24tb1.metal | u-3tb1.56xlarge | u-6tb1.112xlarge | u-6tb1.56xlarge | u-6tb1.metal | u-9tb1.112xlarge | u-9tb1.metal | vt1.24xlarge | vt1.3xlarge | vt1.6xlarge | x1.16xlarge | x1.32xlarge | x1e.16xlarge | x1e.2xlarge | x1e.32xlarge | x1e.4xlarge | x1e.8xlarge | x1e.xlarge | x2gd.12xlarge | x2gd.16xlarge | x2gd.2xlarge | x2gd.4xlarge | x2gd.8xlarge | x2gd.large | x2gd.medium | x2gd.metal | x2gd.xlarge | x2idn.16xlarge | x2idn.24xlarge | x2idn.32xlarge | x2idn.metal | x2iedn.16xlarge | x2iedn.24xlarge | x2iedn.2xlarge | x2iedn.32xlarge | x2iedn.4xlarge | x2iedn.8xlarge | x2iedn.metal | x2iedn.xlarge | x2iezn.12xlarge | x2iezn.2xlarge | x2iezn.4xlarge | x2iezn.6xlarge | x2iezn.8xlarge | x2iezn.metal | z1d.12xlarge | z1d.2xlarge | z1d.3xlarge | z1d.6xlarge | z1d.large | z1d.metal | z1d.xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<LaunchTemplateDataInstanceTypeEnum>,


    /// 
    /// The ID of the kernel.
    /// 
    /// We recommend that you use PV-GRUB instead of kernels and RAM disks. For more     information, see User Provided Kernels in     the Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,


    /// 
    /// The name of the key pair. You can create a key pair using CreateKeyPair or         ImportKeyPair.
    /// 
    /// ImportantIf you do not specify a key pair, you can't connect to the instance unless you         choose an AMI that is configured to allow users another way to log in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,


    /// 
    /// The license configurations.
    /// 
    /// Required: No
    ///
    /// Type: List of LicenseSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseSpecifications")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,


    /// 
    /// The maintenance options of your instance.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceOptions")]
    pub maintenance_options: Option<MaintenanceOptions>,


    /// 
    /// The metadata options for the instance. For more information, see Instance metadata and user data in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: MetadataOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataOptions")]
    pub metadata_options: Option<MetadataOptions>,


    /// 
    /// The monitoring for the instance.
    /// 
    /// Required: No
    ///
    /// Type: Monitoring
    ///
    /// Update requires: No interruption
    #[serde(rename = "Monitoring")]
    pub monitoring: Option<Monitoring>,


    /// 
    /// One or more network interfaces. If you specify a network interface, you must specify       any security groups and subnets as part of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: List of NetworkInterface
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,


    /// 
    /// The placement for the instance.
    /// 
    /// Required: No
    ///
    /// Type: Placement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Placement")]
    pub placement: Option<Placement>,


    /// 
    /// The hostname type for EC2 instances launched into this subnet and how DNS A and AAAA record queries should be handled. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PrivateDnsNameOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateDnsNameOptions")]
    pub private_dns_name_options: Option<PrivateDnsNameOptions>,


    /// 
    /// The ID of the RAM disk.
    /// 
    /// ImportantWe recommend that you use PV-GRUB instead of kernels and RAM disks. For more         information, see User provided           kernels in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RamDiskId")]
    pub ram_disk_id: Option<String>,


    /// 
    /// The IDs of the security groups. You can specify the IDs of existing security groups and     references to resources created by the stack template.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// One or more security group names. For a nondefault VPC, you must use security group       IDs instead. You cannot specify both a security group ID and security name in the same       request.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,


    /// 
    /// The tags to apply to the resources that are created during instance launch.
    /// 
    /// You can specify tags for the following resources only:
    /// 
    /// Instances               Volumes               Elastic graphics               Spot Instance requests               Network interfaces
    /// 
    /// To tag a resource after it has been created, see CreateTags.
    /// 
    /// NoteTo tag the launch template itself, you must use the TagSpecification parameter.
    /// 
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,


    /// 
    /// The user data to make available to the instance. You must provide base64-encoded text.       User data is limited to 16 KB. For more information, see Run commands on your Linux instance at         launch (Linux) or Work with instance         user data (Windows) in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// If you are creating the launch template for use with AWS Batch, the user       data must be provided in the MIME multi-part archive format. For more information, see Amazon EC2 user data in launch templates in the         AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LaunchTemplateDataInstanceInitiatedShutdownBehaviorEnum {

    /// stop
    #[serde(rename = "stop")]
    Stop,

    /// terminate
    #[serde(rename = "terminate")]
    Terminate,

}

impl Default for LaunchTemplateDataInstanceInitiatedShutdownBehaviorEnum {
    fn default() -> Self {
        LaunchTemplateDataInstanceInitiatedShutdownBehaviorEnum::Stop
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LaunchTemplateDataInstanceTypeEnum {

    /// a1.2xlarge
    #[serde(rename = "a1.2xlarge")]
    A12xlarge,

    /// a1.4xlarge
    #[serde(rename = "a1.4xlarge")]
    A14xlarge,

    /// a1.large
    #[serde(rename = "a1.large")]
    A1large,

    /// a1.medium
    #[serde(rename = "a1.medium")]
    A1medium,

    /// a1.metal
    #[serde(rename = "a1.metal")]
    A1metal,

    /// a1.xlarge
    #[serde(rename = "a1.xlarge")]
    A1xlarge,

    /// c1.medium
    #[serde(rename = "c1.medium")]
    C1medium,

    /// c1.xlarge
    #[serde(rename = "c1.xlarge")]
    C1xlarge,

    /// c3.2xlarge
    #[serde(rename = "c3.2xlarge")]
    C32xlarge,

    /// c3.4xlarge
    #[serde(rename = "c3.4xlarge")]
    C34xlarge,

    /// c3.8xlarge
    #[serde(rename = "c3.8xlarge")]
    C38xlarge,

    /// c3.large
    #[serde(rename = "c3.large")]
    C3large,

    /// c3.xlarge
    #[serde(rename = "c3.xlarge")]
    C3xlarge,

    /// c4.2xlarge
    #[serde(rename = "c4.2xlarge")]
    C42xlarge,

    /// c4.4xlarge
    #[serde(rename = "c4.4xlarge")]
    C44xlarge,

    /// c4.8xlarge
    #[serde(rename = "c4.8xlarge")]
    C48xlarge,

    /// c4.large
    #[serde(rename = "c4.large")]
    C4large,

    /// c4.xlarge
    #[serde(rename = "c4.xlarge")]
    C4xlarge,

    /// c5.12xlarge
    #[serde(rename = "c5.12xlarge")]
    C512xlarge,

    /// c5.18xlarge
    #[serde(rename = "c5.18xlarge")]
    C518xlarge,

    /// c5.24xlarge
    #[serde(rename = "c5.24xlarge")]
    C524xlarge,

    /// c5.2xlarge
    #[serde(rename = "c5.2xlarge")]
    C52xlarge,

    /// c5.4xlarge
    #[serde(rename = "c5.4xlarge")]
    C54xlarge,

    /// c5.9xlarge
    #[serde(rename = "c5.9xlarge")]
    C59xlarge,

    /// c5.large
    #[serde(rename = "c5.large")]
    C5large,

    /// c5.metal
    #[serde(rename = "c5.metal")]
    C5metal,

    /// c5.xlarge
    #[serde(rename = "c5.xlarge")]
    C5xlarge,

    /// c5a.12xlarge
    #[serde(rename = "c5a.12xlarge")]
    C5a12xlarge,

    /// c5a.16xlarge
    #[serde(rename = "c5a.16xlarge")]
    C5a16xlarge,

    /// c5a.24xlarge
    #[serde(rename = "c5a.24xlarge")]
    C5a24xlarge,

    /// c5a.2xlarge
    #[serde(rename = "c5a.2xlarge")]
    C5a2xlarge,

    /// c5a.4xlarge
    #[serde(rename = "c5a.4xlarge")]
    C5a4xlarge,

    /// c5a.8xlarge
    #[serde(rename = "c5a.8xlarge")]
    C5a8xlarge,

    /// c5a.large
    #[serde(rename = "c5a.large")]
    C5alarge,

    /// c5a.xlarge
    #[serde(rename = "c5a.xlarge")]
    C5axlarge,

    /// c5ad.12xlarge
    #[serde(rename = "c5ad.12xlarge")]
    C5ad12xlarge,

    /// c5ad.16xlarge
    #[serde(rename = "c5ad.16xlarge")]
    C5ad16xlarge,

    /// c5ad.24xlarge
    #[serde(rename = "c5ad.24xlarge")]
    C5ad24xlarge,

    /// c5ad.2xlarge
    #[serde(rename = "c5ad.2xlarge")]
    C5ad2xlarge,

    /// c5ad.4xlarge
    #[serde(rename = "c5ad.4xlarge")]
    C5ad4xlarge,

    /// c5ad.8xlarge
    #[serde(rename = "c5ad.8xlarge")]
    C5ad8xlarge,

    /// c5ad.large
    #[serde(rename = "c5ad.large")]
    C5adlarge,

    /// c5ad.xlarge
    #[serde(rename = "c5ad.xlarge")]
    C5adxlarge,

    /// c5d.12xlarge
    #[serde(rename = "c5d.12xlarge")]
    C5d12xlarge,

    /// c5d.18xlarge
    #[serde(rename = "c5d.18xlarge")]
    C5d18xlarge,

    /// c5d.24xlarge
    #[serde(rename = "c5d.24xlarge")]
    C5d24xlarge,

    /// c5d.2xlarge
    #[serde(rename = "c5d.2xlarge")]
    C5d2xlarge,

    /// c5d.4xlarge
    #[serde(rename = "c5d.4xlarge")]
    C5d4xlarge,

    /// c5d.9xlarge
    #[serde(rename = "c5d.9xlarge")]
    C5d9xlarge,

    /// c5d.large
    #[serde(rename = "c5d.large")]
    C5dlarge,

    /// c5d.metal
    #[serde(rename = "c5d.metal")]
    C5dmetal,

    /// c5d.xlarge
    #[serde(rename = "c5d.xlarge")]
    C5dxlarge,

    /// c5n.18xlarge
    #[serde(rename = "c5n.18xlarge")]
    C5n18xlarge,

    /// c5n.2xlarge
    #[serde(rename = "c5n.2xlarge")]
    C5n2xlarge,

    /// c5n.4xlarge
    #[serde(rename = "c5n.4xlarge")]
    C5n4xlarge,

    /// c5n.9xlarge
    #[serde(rename = "c5n.9xlarge")]
    C5n9xlarge,

    /// c5n.large
    #[serde(rename = "c5n.large")]
    C5nlarge,

    /// c5n.metal
    #[serde(rename = "c5n.metal")]
    C5nmetal,

    /// c5n.xlarge
    #[serde(rename = "c5n.xlarge")]
    C5nxlarge,

    /// c6a.12xlarge
    #[serde(rename = "c6a.12xlarge")]
    C6a12xlarge,

    /// c6a.16xlarge
    #[serde(rename = "c6a.16xlarge")]
    C6a16xlarge,

    /// c6a.24xlarge
    #[serde(rename = "c6a.24xlarge")]
    C6a24xlarge,

    /// c6a.2xlarge
    #[serde(rename = "c6a.2xlarge")]
    C6a2xlarge,

    /// c6a.32xlarge
    #[serde(rename = "c6a.32xlarge")]
    C6a32xlarge,

    /// c6a.48xlarge
    #[serde(rename = "c6a.48xlarge")]
    C6a48xlarge,

    /// c6a.4xlarge
    #[serde(rename = "c6a.4xlarge")]
    C6a4xlarge,

    /// c6a.8xlarge
    #[serde(rename = "c6a.8xlarge")]
    C6a8xlarge,

    /// c6a.large
    #[serde(rename = "c6a.large")]
    C6alarge,

    /// c6a.metal
    #[serde(rename = "c6a.metal")]
    C6ametal,

    /// c6a.xlarge
    #[serde(rename = "c6a.xlarge")]
    C6axlarge,

    /// c6g.12xlarge
    #[serde(rename = "c6g.12xlarge")]
    C6g12xlarge,

    /// c6g.16xlarge
    #[serde(rename = "c6g.16xlarge")]
    C6g16xlarge,

    /// c6g.2xlarge
    #[serde(rename = "c6g.2xlarge")]
    C6g2xlarge,

    /// c6g.4xlarge
    #[serde(rename = "c6g.4xlarge")]
    C6g4xlarge,

    /// c6g.8xlarge
    #[serde(rename = "c6g.8xlarge")]
    C6g8xlarge,

    /// c6g.large
    #[serde(rename = "c6g.large")]
    C6glarge,

    /// c6g.medium
    #[serde(rename = "c6g.medium")]
    C6gmedium,

    /// c6g.metal
    #[serde(rename = "c6g.metal")]
    C6gmetal,

    /// c6g.xlarge
    #[serde(rename = "c6g.xlarge")]
    C6gxlarge,

    /// c6gd.12xlarge
    #[serde(rename = "c6gd.12xlarge")]
    C6gd12xlarge,

    /// c6gd.16xlarge
    #[serde(rename = "c6gd.16xlarge")]
    C6gd16xlarge,

    /// c6gd.2xlarge
    #[serde(rename = "c6gd.2xlarge")]
    C6gd2xlarge,

    /// c6gd.4xlarge
    #[serde(rename = "c6gd.4xlarge")]
    C6gd4xlarge,

    /// c6gd.8xlarge
    #[serde(rename = "c6gd.8xlarge")]
    C6gd8xlarge,

    /// c6gd.large
    #[serde(rename = "c6gd.large")]
    C6gdlarge,

    /// c6gd.medium
    #[serde(rename = "c6gd.medium")]
    C6gdmedium,

    /// c6gd.metal
    #[serde(rename = "c6gd.metal")]
    C6gdmetal,

    /// c6gd.xlarge
    #[serde(rename = "c6gd.xlarge")]
    C6gdxlarge,

    /// c6gn.12xlarge
    #[serde(rename = "c6gn.12xlarge")]
    C6gn12xlarge,

    /// c6gn.16xlarge
    #[serde(rename = "c6gn.16xlarge")]
    C6gn16xlarge,

    /// c6gn.2xlarge
    #[serde(rename = "c6gn.2xlarge")]
    C6gn2xlarge,

    /// c6gn.4xlarge
    #[serde(rename = "c6gn.4xlarge")]
    C6gn4xlarge,

    /// c6gn.8xlarge
    #[serde(rename = "c6gn.8xlarge")]
    C6gn8xlarge,

    /// c6gn.large
    #[serde(rename = "c6gn.large")]
    C6gnlarge,

    /// c6gn.medium
    #[serde(rename = "c6gn.medium")]
    C6gnmedium,

    /// c6gn.xlarge
    #[serde(rename = "c6gn.xlarge")]
    C6gnxlarge,

    /// c6i.12xlarge
    #[serde(rename = "c6i.12xlarge")]
    C6i12xlarge,

    /// c6i.16xlarge
    #[serde(rename = "c6i.16xlarge")]
    C6i16xlarge,

    /// c6i.24xlarge
    #[serde(rename = "c6i.24xlarge")]
    C6i24xlarge,

    /// c6i.2xlarge
    #[serde(rename = "c6i.2xlarge")]
    C6i2xlarge,

    /// c6i.32xlarge
    #[serde(rename = "c6i.32xlarge")]
    C6i32xlarge,

    /// c6i.4xlarge
    #[serde(rename = "c6i.4xlarge")]
    C6i4xlarge,

    /// c6i.8xlarge
    #[serde(rename = "c6i.8xlarge")]
    C6i8xlarge,

    /// c6i.large
    #[serde(rename = "c6i.large")]
    C6ilarge,

    /// c6i.metal
    #[serde(rename = "c6i.metal")]
    C6imetal,

    /// c6i.xlarge
    #[serde(rename = "c6i.xlarge")]
    C6ixlarge,

    /// c6id.12xlarge
    #[serde(rename = "c6id.12xlarge")]
    C6id12xlarge,

    /// c6id.16xlarge
    #[serde(rename = "c6id.16xlarge")]
    C6id16xlarge,

    /// c6id.24xlarge
    #[serde(rename = "c6id.24xlarge")]
    C6id24xlarge,

    /// c6id.2xlarge
    #[serde(rename = "c6id.2xlarge")]
    C6id2xlarge,

    /// c6id.32xlarge
    #[serde(rename = "c6id.32xlarge")]
    C6id32xlarge,

    /// c6id.4xlarge
    #[serde(rename = "c6id.4xlarge")]
    C6id4xlarge,

    /// c6id.8xlarge
    #[serde(rename = "c6id.8xlarge")]
    C6id8xlarge,

    /// c6id.large
    #[serde(rename = "c6id.large")]
    C6idlarge,

    /// c6id.metal
    #[serde(rename = "c6id.metal")]
    C6idmetal,

    /// c6id.xlarge
    #[serde(rename = "c6id.xlarge")]
    C6idxlarge,

    /// c6in.12xlarge
    #[serde(rename = "c6in.12xlarge")]
    C6in12xlarge,

    /// c6in.16xlarge
    #[serde(rename = "c6in.16xlarge")]
    C6in16xlarge,

    /// c6in.24xlarge
    #[serde(rename = "c6in.24xlarge")]
    C6in24xlarge,

    /// c6in.2xlarge
    #[serde(rename = "c6in.2xlarge")]
    C6in2xlarge,

    /// c6in.32xlarge
    #[serde(rename = "c6in.32xlarge")]
    C6in32xlarge,

    /// c6in.4xlarge
    #[serde(rename = "c6in.4xlarge")]
    C6in4xlarge,

    /// c6in.8xlarge
    #[serde(rename = "c6in.8xlarge")]
    C6in8xlarge,

    /// c6in.large
    #[serde(rename = "c6in.large")]
    C6inlarge,

    /// c6in.metal
    #[serde(rename = "c6in.metal")]
    C6inmetal,

    /// c6in.xlarge
    #[serde(rename = "c6in.xlarge")]
    C6inxlarge,

    /// c7g.12xlarge
    #[serde(rename = "c7g.12xlarge")]
    C7g12xlarge,

    /// c7g.16xlarge
    #[serde(rename = "c7g.16xlarge")]
    C7g16xlarge,

    /// c7g.2xlarge
    #[serde(rename = "c7g.2xlarge")]
    C7g2xlarge,

    /// c7g.4xlarge
    #[serde(rename = "c7g.4xlarge")]
    C7g4xlarge,

    /// c7g.8xlarge
    #[serde(rename = "c7g.8xlarge")]
    C7g8xlarge,

    /// c7g.large
    #[serde(rename = "c7g.large")]
    C7glarge,

    /// c7g.medium
    #[serde(rename = "c7g.medium")]
    C7gmedium,

    /// c7g.xlarge
    #[serde(rename = "c7g.xlarge")]
    C7gxlarge,

    /// cc1.4xlarge
    #[serde(rename = "cc1.4xlarge")]
    Cc14xlarge,

    /// cc2.8xlarge
    #[serde(rename = "cc2.8xlarge")]
    Cc28xlarge,

    /// cg1.4xlarge
    #[serde(rename = "cg1.4xlarge")]
    Cg14xlarge,

    /// cr1.8xlarge
    #[serde(rename = "cr1.8xlarge")]
    Cr18xlarge,

    /// d2.2xlarge
    #[serde(rename = "d2.2xlarge")]
    D22xlarge,

    /// d2.4xlarge
    #[serde(rename = "d2.4xlarge")]
    D24xlarge,

    /// d2.8xlarge
    #[serde(rename = "d2.8xlarge")]
    D28xlarge,

    /// d2.xlarge
    #[serde(rename = "d2.xlarge")]
    D2xlarge,

    /// d3.2xlarge
    #[serde(rename = "d3.2xlarge")]
    D32xlarge,

    /// d3.4xlarge
    #[serde(rename = "d3.4xlarge")]
    D34xlarge,

    /// d3.8xlarge
    #[serde(rename = "d3.8xlarge")]
    D38xlarge,

    /// d3.xlarge
    #[serde(rename = "d3.xlarge")]
    D3xlarge,

    /// d3en.12xlarge
    #[serde(rename = "d3en.12xlarge")]
    D3en12xlarge,

    /// d3en.2xlarge
    #[serde(rename = "d3en.2xlarge")]
    D3en2xlarge,

    /// d3en.4xlarge
    #[serde(rename = "d3en.4xlarge")]
    D3en4xlarge,

    /// d3en.6xlarge
    #[serde(rename = "d3en.6xlarge")]
    D3en6xlarge,

    /// d3en.8xlarge
    #[serde(rename = "d3en.8xlarge")]
    D3en8xlarge,

    /// d3en.xlarge
    #[serde(rename = "d3en.xlarge")]
    D3enxlarge,

    /// dl1.24xlarge
    #[serde(rename = "dl1.24xlarge")]
    Dl124xlarge,

    /// f1.16xlarge
    #[serde(rename = "f1.16xlarge")]
    F116xlarge,

    /// f1.2xlarge
    #[serde(rename = "f1.2xlarge")]
    F12xlarge,

    /// f1.4xlarge
    #[serde(rename = "f1.4xlarge")]
    F14xlarge,

    /// g2.2xlarge
    #[serde(rename = "g2.2xlarge")]
    G22xlarge,

    /// g2.8xlarge
    #[serde(rename = "g2.8xlarge")]
    G28xlarge,

    /// g3.16xlarge
    #[serde(rename = "g3.16xlarge")]
    G316xlarge,

    /// g3.4xlarge
    #[serde(rename = "g3.4xlarge")]
    G34xlarge,

    /// g3.8xlarge
    #[serde(rename = "g3.8xlarge")]
    G38xlarge,

    /// g3s.xlarge
    #[serde(rename = "g3s.xlarge")]
    G3sxlarge,

    /// g4ad.16xlarge
    #[serde(rename = "g4ad.16xlarge")]
    G4ad16xlarge,

    /// g4ad.2xlarge
    #[serde(rename = "g4ad.2xlarge")]
    G4ad2xlarge,

    /// g4ad.4xlarge
    #[serde(rename = "g4ad.4xlarge")]
    G4ad4xlarge,

    /// g4ad.8xlarge
    #[serde(rename = "g4ad.8xlarge")]
    G4ad8xlarge,

    /// g4ad.xlarge
    #[serde(rename = "g4ad.xlarge")]
    G4adxlarge,

    /// g4dn.12xlarge
    #[serde(rename = "g4dn.12xlarge")]
    G4dn12xlarge,

    /// g4dn.16xlarge
    #[serde(rename = "g4dn.16xlarge")]
    G4dn16xlarge,

    /// g4dn.2xlarge
    #[serde(rename = "g4dn.2xlarge")]
    G4dn2xlarge,

    /// g4dn.4xlarge
    #[serde(rename = "g4dn.4xlarge")]
    G4dn4xlarge,

    /// g4dn.8xlarge
    #[serde(rename = "g4dn.8xlarge")]
    G4dn8xlarge,

    /// g4dn.metal
    #[serde(rename = "g4dn.metal")]
    G4dnmetal,

    /// g4dn.xlarge
    #[serde(rename = "g4dn.xlarge")]
    G4dnxlarge,

    /// g5.12xlarge
    #[serde(rename = "g5.12xlarge")]
    G512xlarge,

    /// g5.16xlarge
    #[serde(rename = "g5.16xlarge")]
    G516xlarge,

    /// g5.24xlarge
    #[serde(rename = "g5.24xlarge")]
    G524xlarge,

    /// g5.2xlarge
    #[serde(rename = "g5.2xlarge")]
    G52xlarge,

    /// g5.48xlarge
    #[serde(rename = "g5.48xlarge")]
    G548xlarge,

    /// g5.4xlarge
    #[serde(rename = "g5.4xlarge")]
    G54xlarge,

    /// g5.8xlarge
    #[serde(rename = "g5.8xlarge")]
    G58xlarge,

    /// g5.xlarge
    #[serde(rename = "g5.xlarge")]
    G5xlarge,

    /// g5g.16xlarge
    #[serde(rename = "g5g.16xlarge")]
    G5g16xlarge,

    /// g5g.2xlarge
    #[serde(rename = "g5g.2xlarge")]
    G5g2xlarge,

    /// g5g.4xlarge
    #[serde(rename = "g5g.4xlarge")]
    G5g4xlarge,

    /// g5g.8xlarge
    #[serde(rename = "g5g.8xlarge")]
    G5g8xlarge,

    /// g5g.metal
    #[serde(rename = "g5g.metal")]
    G5gmetal,

    /// g5g.xlarge
    #[serde(rename = "g5g.xlarge")]
    G5gxlarge,

    /// h1.16xlarge
    #[serde(rename = "h1.16xlarge")]
    H116xlarge,

    /// h1.2xlarge
    #[serde(rename = "h1.2xlarge")]
    H12xlarge,

    /// h1.4xlarge
    #[serde(rename = "h1.4xlarge")]
    H14xlarge,

    /// h1.8xlarge
    #[serde(rename = "h1.8xlarge")]
    H18xlarge,

    /// hi1.4xlarge
    #[serde(rename = "hi1.4xlarge")]
    Hi14xlarge,

    /// hpc6a.48xlarge
    #[serde(rename = "hpc6a.48xlarge")]
    Hpc6a48xlarge,

    /// hpc6id.32xlarge
    #[serde(rename = "hpc6id.32xlarge")]
    Hpc6id32xlarge,

    /// hs1.8xlarge
    #[serde(rename = "hs1.8xlarge")]
    Hs18xlarge,

    /// i2.2xlarge
    #[serde(rename = "i2.2xlarge")]
    I22xlarge,

    /// i2.4xlarge
    #[serde(rename = "i2.4xlarge")]
    I24xlarge,

    /// i2.8xlarge
    #[serde(rename = "i2.8xlarge")]
    I28xlarge,

    /// i2.xlarge
    #[serde(rename = "i2.xlarge")]
    I2xlarge,

    /// i3.16xlarge
    #[serde(rename = "i3.16xlarge")]
    I316xlarge,

    /// i3.2xlarge
    #[serde(rename = "i3.2xlarge")]
    I32xlarge,

    /// i3.4xlarge
    #[serde(rename = "i3.4xlarge")]
    I34xlarge,

    /// i3.8xlarge
    #[serde(rename = "i3.8xlarge")]
    I38xlarge,

    /// i3.large
    #[serde(rename = "i3.large")]
    I3large,

    /// i3.metal
    #[serde(rename = "i3.metal")]
    I3metal,

    /// i3.xlarge
    #[serde(rename = "i3.xlarge")]
    I3xlarge,

    /// i3en.12xlarge
    #[serde(rename = "i3en.12xlarge")]
    I3en12xlarge,

    /// i3en.24xlarge
    #[serde(rename = "i3en.24xlarge")]
    I3en24xlarge,

    /// i3en.2xlarge
    #[serde(rename = "i3en.2xlarge")]
    I3en2xlarge,

    /// i3en.3xlarge
    #[serde(rename = "i3en.3xlarge")]
    I3en3xlarge,

    /// i3en.6xlarge
    #[serde(rename = "i3en.6xlarge")]
    I3en6xlarge,

    /// i3en.large
    #[serde(rename = "i3en.large")]
    I3enlarge,

    /// i3en.metal
    #[serde(rename = "i3en.metal")]
    I3enmetal,

    /// i3en.xlarge
    #[serde(rename = "i3en.xlarge")]
    I3enxlarge,

    /// i4g.16xlarge
    #[serde(rename = "i4g.16xlarge")]
    I4g16xlarge,

    /// i4g.2xlarge
    #[serde(rename = "i4g.2xlarge")]
    I4g2xlarge,

    /// i4g.4xlarge
    #[serde(rename = "i4g.4xlarge")]
    I4g4xlarge,

    /// i4g.8xlarge
    #[serde(rename = "i4g.8xlarge")]
    I4g8xlarge,

    /// i4g.large
    #[serde(rename = "i4g.large")]
    I4glarge,

    /// i4g.xlarge
    #[serde(rename = "i4g.xlarge")]
    I4gxlarge,

    /// i4i.16xlarge
    #[serde(rename = "i4i.16xlarge")]
    I4i16xlarge,

    /// i4i.2xlarge
    #[serde(rename = "i4i.2xlarge")]
    I4i2xlarge,

    /// i4i.32xlarge
    #[serde(rename = "i4i.32xlarge")]
    I4i32xlarge,

    /// i4i.4xlarge
    #[serde(rename = "i4i.4xlarge")]
    I4i4xlarge,

    /// i4i.8xlarge
    #[serde(rename = "i4i.8xlarge")]
    I4i8xlarge,

    /// i4i.large
    #[serde(rename = "i4i.large")]
    I4ilarge,

    /// i4i.metal
    #[serde(rename = "i4i.metal")]
    I4imetal,

    /// i4i.xlarge
    #[serde(rename = "i4i.xlarge")]
    I4ixlarge,

    /// im4gn.16xlarge
    #[serde(rename = "im4gn.16xlarge")]
    Im4gn16xlarge,

    /// im4gn.2xlarge
    #[serde(rename = "im4gn.2xlarge")]
    Im4gn2xlarge,

    /// im4gn.4xlarge
    #[serde(rename = "im4gn.4xlarge")]
    Im4gn4xlarge,

    /// im4gn.8xlarge
    #[serde(rename = "im4gn.8xlarge")]
    Im4gn8xlarge,

    /// im4gn.large
    #[serde(rename = "im4gn.large")]
    Im4gnlarge,

    /// im4gn.xlarge
    #[serde(rename = "im4gn.xlarge")]
    Im4gnxlarge,

    /// inf1.24xlarge
    #[serde(rename = "inf1.24xlarge")]
    Inf124xlarge,

    /// inf1.2xlarge
    #[serde(rename = "inf1.2xlarge")]
    Inf12xlarge,

    /// inf1.6xlarge
    #[serde(rename = "inf1.6xlarge")]
    Inf16xlarge,

    /// inf1.xlarge
    #[serde(rename = "inf1.xlarge")]
    Inf1xlarge,

    /// inf2.24xlarge
    #[serde(rename = "inf2.24xlarge")]
    Inf224xlarge,

    /// inf2.48xlarge
    #[serde(rename = "inf2.48xlarge")]
    Inf248xlarge,

    /// inf2.8xlarge
    #[serde(rename = "inf2.8xlarge")]
    Inf28xlarge,

    /// inf2.xlarge
    #[serde(rename = "inf2.xlarge")]
    Inf2xlarge,

    /// is4gen.2xlarge
    #[serde(rename = "is4gen.2xlarge")]
    Is4gen2xlarge,

    /// is4gen.4xlarge
    #[serde(rename = "is4gen.4xlarge")]
    Is4gen4xlarge,

    /// is4gen.8xlarge
    #[serde(rename = "is4gen.8xlarge")]
    Is4gen8xlarge,

    /// is4gen.large
    #[serde(rename = "is4gen.large")]
    Is4genlarge,

    /// is4gen.medium
    #[serde(rename = "is4gen.medium")]
    Is4genmedium,

    /// is4gen.xlarge
    #[serde(rename = "is4gen.xlarge")]
    Is4genxlarge,

    /// m1.large
    #[serde(rename = "m1.large")]
    M1large,

    /// m1.medium
    #[serde(rename = "m1.medium")]
    M1medium,

    /// m1.small
    #[serde(rename = "m1.small")]
    M1small,

    /// m1.xlarge
    #[serde(rename = "m1.xlarge")]
    M1xlarge,

    /// m2.2xlarge
    #[serde(rename = "m2.2xlarge")]
    M22xlarge,

    /// m2.4xlarge
    #[serde(rename = "m2.4xlarge")]
    M24xlarge,

    /// m2.xlarge
    #[serde(rename = "m2.xlarge")]
    M2xlarge,

    /// m3.2xlarge
    #[serde(rename = "m3.2xlarge")]
    M32xlarge,

    /// m3.large
    #[serde(rename = "m3.large")]
    M3large,

    /// m3.medium
    #[serde(rename = "m3.medium")]
    M3medium,

    /// m3.xlarge
    #[serde(rename = "m3.xlarge")]
    M3xlarge,

    /// m4.10xlarge
    #[serde(rename = "m4.10xlarge")]
    M410xlarge,

    /// m4.16xlarge
    #[serde(rename = "m4.16xlarge")]
    M416xlarge,

    /// m4.2xlarge
    #[serde(rename = "m4.2xlarge")]
    M42xlarge,

    /// m4.4xlarge
    #[serde(rename = "m4.4xlarge")]
    M44xlarge,

    /// m4.large
    #[serde(rename = "m4.large")]
    M4large,

    /// m4.xlarge
    #[serde(rename = "m4.xlarge")]
    M4xlarge,

    /// m5.12xlarge
    #[serde(rename = "m5.12xlarge")]
    M512xlarge,

    /// m5.16xlarge
    #[serde(rename = "m5.16xlarge")]
    M516xlarge,

    /// m5.24xlarge
    #[serde(rename = "m5.24xlarge")]
    M524xlarge,

    /// m5.2xlarge
    #[serde(rename = "m5.2xlarge")]
    M52xlarge,

    /// m5.4xlarge
    #[serde(rename = "m5.4xlarge")]
    M54xlarge,

    /// m5.8xlarge
    #[serde(rename = "m5.8xlarge")]
    M58xlarge,

    /// m5.large
    #[serde(rename = "m5.large")]
    M5large,

    /// m5.metal
    #[serde(rename = "m5.metal")]
    M5metal,

    /// m5.xlarge
    #[serde(rename = "m5.xlarge")]
    M5xlarge,

    /// m5a.12xlarge
    #[serde(rename = "m5a.12xlarge")]
    M5a12xlarge,

    /// m5a.16xlarge
    #[serde(rename = "m5a.16xlarge")]
    M5a16xlarge,

    /// m5a.24xlarge
    #[serde(rename = "m5a.24xlarge")]
    M5a24xlarge,

    /// m5a.2xlarge
    #[serde(rename = "m5a.2xlarge")]
    M5a2xlarge,

    /// m5a.4xlarge
    #[serde(rename = "m5a.4xlarge")]
    M5a4xlarge,

    /// m5a.8xlarge
    #[serde(rename = "m5a.8xlarge")]
    M5a8xlarge,

    /// m5a.large
    #[serde(rename = "m5a.large")]
    M5alarge,

    /// m5a.xlarge
    #[serde(rename = "m5a.xlarge")]
    M5axlarge,

    /// m5ad.12xlarge
    #[serde(rename = "m5ad.12xlarge")]
    M5ad12xlarge,

    /// m5ad.16xlarge
    #[serde(rename = "m5ad.16xlarge")]
    M5ad16xlarge,

    /// m5ad.24xlarge
    #[serde(rename = "m5ad.24xlarge")]
    M5ad24xlarge,

    /// m5ad.2xlarge
    #[serde(rename = "m5ad.2xlarge")]
    M5ad2xlarge,

    /// m5ad.4xlarge
    #[serde(rename = "m5ad.4xlarge")]
    M5ad4xlarge,

    /// m5ad.8xlarge
    #[serde(rename = "m5ad.8xlarge")]
    M5ad8xlarge,

    /// m5ad.large
    #[serde(rename = "m5ad.large")]
    M5adlarge,

    /// m5ad.xlarge
    #[serde(rename = "m5ad.xlarge")]
    M5adxlarge,

    /// m5d.12xlarge
    #[serde(rename = "m5d.12xlarge")]
    M5d12xlarge,

    /// m5d.16xlarge
    #[serde(rename = "m5d.16xlarge")]
    M5d16xlarge,

    /// m5d.24xlarge
    #[serde(rename = "m5d.24xlarge")]
    M5d24xlarge,

    /// m5d.2xlarge
    #[serde(rename = "m5d.2xlarge")]
    M5d2xlarge,

    /// m5d.4xlarge
    #[serde(rename = "m5d.4xlarge")]
    M5d4xlarge,

    /// m5d.8xlarge
    #[serde(rename = "m5d.8xlarge")]
    M5d8xlarge,

    /// m5d.large
    #[serde(rename = "m5d.large")]
    M5dlarge,

    /// m5d.metal
    #[serde(rename = "m5d.metal")]
    M5dmetal,

    /// m5d.xlarge
    #[serde(rename = "m5d.xlarge")]
    M5dxlarge,

    /// m5dn.12xlarge
    #[serde(rename = "m5dn.12xlarge")]
    M5dn12xlarge,

    /// m5dn.16xlarge
    #[serde(rename = "m5dn.16xlarge")]
    M5dn16xlarge,

    /// m5dn.24xlarge
    #[serde(rename = "m5dn.24xlarge")]
    M5dn24xlarge,

    /// m5dn.2xlarge
    #[serde(rename = "m5dn.2xlarge")]
    M5dn2xlarge,

    /// m5dn.4xlarge
    #[serde(rename = "m5dn.4xlarge")]
    M5dn4xlarge,

    /// m5dn.8xlarge
    #[serde(rename = "m5dn.8xlarge")]
    M5dn8xlarge,

    /// m5dn.large
    #[serde(rename = "m5dn.large")]
    M5dnlarge,

    /// m5dn.metal
    #[serde(rename = "m5dn.metal")]
    M5dnmetal,

    /// m5dn.xlarge
    #[serde(rename = "m5dn.xlarge")]
    M5dnxlarge,

    /// m5n.12xlarge
    #[serde(rename = "m5n.12xlarge")]
    M5n12xlarge,

    /// m5n.16xlarge
    #[serde(rename = "m5n.16xlarge")]
    M5n16xlarge,

    /// m5n.24xlarge
    #[serde(rename = "m5n.24xlarge")]
    M5n24xlarge,

    /// m5n.2xlarge
    #[serde(rename = "m5n.2xlarge")]
    M5n2xlarge,

    /// m5n.4xlarge
    #[serde(rename = "m5n.4xlarge")]
    M5n4xlarge,

    /// m5n.8xlarge
    #[serde(rename = "m5n.8xlarge")]
    M5n8xlarge,

    /// m5n.large
    #[serde(rename = "m5n.large")]
    M5nlarge,

    /// m5n.metal
    #[serde(rename = "m5n.metal")]
    M5nmetal,

    /// m5n.xlarge
    #[serde(rename = "m5n.xlarge")]
    M5nxlarge,

    /// m5zn.12xlarge
    #[serde(rename = "m5zn.12xlarge")]
    M5zn12xlarge,

    /// m5zn.2xlarge
    #[serde(rename = "m5zn.2xlarge")]
    M5zn2xlarge,

    /// m5zn.3xlarge
    #[serde(rename = "m5zn.3xlarge")]
    M5zn3xlarge,

    /// m5zn.6xlarge
    #[serde(rename = "m5zn.6xlarge")]
    M5zn6xlarge,

    /// m5zn.large
    #[serde(rename = "m5zn.large")]
    M5znlarge,

    /// m5zn.metal
    #[serde(rename = "m5zn.metal")]
    M5znmetal,

    /// m5zn.xlarge
    #[serde(rename = "m5zn.xlarge")]
    M5znxlarge,

    /// m6a.12xlarge
    #[serde(rename = "m6a.12xlarge")]
    M6a12xlarge,

    /// m6a.16xlarge
    #[serde(rename = "m6a.16xlarge")]
    M6a16xlarge,

    /// m6a.24xlarge
    #[serde(rename = "m6a.24xlarge")]
    M6a24xlarge,

    /// m6a.2xlarge
    #[serde(rename = "m6a.2xlarge")]
    M6a2xlarge,

    /// m6a.32xlarge
    #[serde(rename = "m6a.32xlarge")]
    M6a32xlarge,

    /// m6a.48xlarge
    #[serde(rename = "m6a.48xlarge")]
    M6a48xlarge,

    /// m6a.4xlarge
    #[serde(rename = "m6a.4xlarge")]
    M6a4xlarge,

    /// m6a.8xlarge
    #[serde(rename = "m6a.8xlarge")]
    M6a8xlarge,

    /// m6a.large
    #[serde(rename = "m6a.large")]
    M6alarge,

    /// m6a.metal
    #[serde(rename = "m6a.metal")]
    M6ametal,

    /// m6a.xlarge
    #[serde(rename = "m6a.xlarge")]
    M6axlarge,

    /// m6g.12xlarge
    #[serde(rename = "m6g.12xlarge")]
    M6g12xlarge,

    /// m6g.16xlarge
    #[serde(rename = "m6g.16xlarge")]
    M6g16xlarge,

    /// m6g.2xlarge
    #[serde(rename = "m6g.2xlarge")]
    M6g2xlarge,

    /// m6g.4xlarge
    #[serde(rename = "m6g.4xlarge")]
    M6g4xlarge,

    /// m6g.8xlarge
    #[serde(rename = "m6g.8xlarge")]
    M6g8xlarge,

    /// m6g.large
    #[serde(rename = "m6g.large")]
    M6glarge,

    /// m6g.medium
    #[serde(rename = "m6g.medium")]
    M6gmedium,

    /// m6g.metal
    #[serde(rename = "m6g.metal")]
    M6gmetal,

    /// m6g.xlarge
    #[serde(rename = "m6g.xlarge")]
    M6gxlarge,

    /// m6gd.12xlarge
    #[serde(rename = "m6gd.12xlarge")]
    M6gd12xlarge,

    /// m6gd.16xlarge
    #[serde(rename = "m6gd.16xlarge")]
    M6gd16xlarge,

    /// m6gd.2xlarge
    #[serde(rename = "m6gd.2xlarge")]
    M6gd2xlarge,

    /// m6gd.4xlarge
    #[serde(rename = "m6gd.4xlarge")]
    M6gd4xlarge,

    /// m6gd.8xlarge
    #[serde(rename = "m6gd.8xlarge")]
    M6gd8xlarge,

    /// m6gd.large
    #[serde(rename = "m6gd.large")]
    M6gdlarge,

    /// m6gd.medium
    #[serde(rename = "m6gd.medium")]
    M6gdmedium,

    /// m6gd.metal
    #[serde(rename = "m6gd.metal")]
    M6gdmetal,

    /// m6gd.xlarge
    #[serde(rename = "m6gd.xlarge")]
    M6gdxlarge,

    /// m6i.12xlarge
    #[serde(rename = "m6i.12xlarge")]
    M6i12xlarge,

    /// m6i.16xlarge
    #[serde(rename = "m6i.16xlarge")]
    M6i16xlarge,

    /// m6i.24xlarge
    #[serde(rename = "m6i.24xlarge")]
    M6i24xlarge,

    /// m6i.2xlarge
    #[serde(rename = "m6i.2xlarge")]
    M6i2xlarge,

    /// m6i.32xlarge
    #[serde(rename = "m6i.32xlarge")]
    M6i32xlarge,

    /// m6i.4xlarge
    #[serde(rename = "m6i.4xlarge")]
    M6i4xlarge,

    /// m6i.8xlarge
    #[serde(rename = "m6i.8xlarge")]
    M6i8xlarge,

    /// m6i.large
    #[serde(rename = "m6i.large")]
    M6ilarge,

    /// m6i.metal
    #[serde(rename = "m6i.metal")]
    M6imetal,

    /// m6i.xlarge
    #[serde(rename = "m6i.xlarge")]
    M6ixlarge,

    /// m6id.12xlarge
    #[serde(rename = "m6id.12xlarge")]
    M6id12xlarge,

    /// m6id.16xlarge
    #[serde(rename = "m6id.16xlarge")]
    M6id16xlarge,

    /// m6id.24xlarge
    #[serde(rename = "m6id.24xlarge")]
    M6id24xlarge,

    /// m6id.2xlarge
    #[serde(rename = "m6id.2xlarge")]
    M6id2xlarge,

    /// m6id.32xlarge
    #[serde(rename = "m6id.32xlarge")]
    M6id32xlarge,

    /// m6id.4xlarge
    #[serde(rename = "m6id.4xlarge")]
    M6id4xlarge,

    /// m6id.8xlarge
    #[serde(rename = "m6id.8xlarge")]
    M6id8xlarge,

    /// m6id.large
    #[serde(rename = "m6id.large")]
    M6idlarge,

    /// m6id.metal
    #[serde(rename = "m6id.metal")]
    M6idmetal,

    /// m6id.xlarge
    #[serde(rename = "m6id.xlarge")]
    M6idxlarge,

    /// m6idn.12xlarge
    #[serde(rename = "m6idn.12xlarge")]
    M6idn12xlarge,

    /// m6idn.16xlarge
    #[serde(rename = "m6idn.16xlarge")]
    M6idn16xlarge,

    /// m6idn.24xlarge
    #[serde(rename = "m6idn.24xlarge")]
    M6idn24xlarge,

    /// m6idn.2xlarge
    #[serde(rename = "m6idn.2xlarge")]
    M6idn2xlarge,

    /// m6idn.32xlarge
    #[serde(rename = "m6idn.32xlarge")]
    M6idn32xlarge,

    /// m6idn.4xlarge
    #[serde(rename = "m6idn.4xlarge")]
    M6idn4xlarge,

    /// m6idn.8xlarge
    #[serde(rename = "m6idn.8xlarge")]
    M6idn8xlarge,

    /// m6idn.large
    #[serde(rename = "m6idn.large")]
    M6idnlarge,

    /// m6idn.metal
    #[serde(rename = "m6idn.metal")]
    M6idnmetal,

    /// m6idn.xlarge
    #[serde(rename = "m6idn.xlarge")]
    M6idnxlarge,

    /// m6in.12xlarge
    #[serde(rename = "m6in.12xlarge")]
    M6in12xlarge,

    /// m6in.16xlarge
    #[serde(rename = "m6in.16xlarge")]
    M6in16xlarge,

    /// m6in.24xlarge
    #[serde(rename = "m6in.24xlarge")]
    M6in24xlarge,

    /// m6in.2xlarge
    #[serde(rename = "m6in.2xlarge")]
    M6in2xlarge,

    /// m6in.32xlarge
    #[serde(rename = "m6in.32xlarge")]
    M6in32xlarge,

    /// m6in.4xlarge
    #[serde(rename = "m6in.4xlarge")]
    M6in4xlarge,

    /// m6in.8xlarge
    #[serde(rename = "m6in.8xlarge")]
    M6in8xlarge,

    /// m6in.large
    #[serde(rename = "m6in.large")]
    M6inlarge,

    /// m6in.metal
    #[serde(rename = "m6in.metal")]
    M6inmetal,

    /// m6in.xlarge
    #[serde(rename = "m6in.xlarge")]
    M6inxlarge,

    /// mac1.metal
    #[serde(rename = "mac1.metal")]
    Mac1metal,

    /// mac2.metal
    #[serde(rename = "mac2.metal")]
    Mac2metal,

    /// p2.16xlarge
    #[serde(rename = "p2.16xlarge")]
    P216xlarge,

    /// p2.8xlarge
    #[serde(rename = "p2.8xlarge")]
    P28xlarge,

    /// p2.xlarge
    #[serde(rename = "p2.xlarge")]
    P2xlarge,

    /// p3.16xlarge
    #[serde(rename = "p3.16xlarge")]
    P316xlarge,

    /// p3.2xlarge
    #[serde(rename = "p3.2xlarge")]
    P32xlarge,

    /// p3.8xlarge
    #[serde(rename = "p3.8xlarge")]
    P38xlarge,

    /// p3dn.24xlarge
    #[serde(rename = "p3dn.24xlarge")]
    P3dn24xlarge,

    /// p4d.24xlarge
    #[serde(rename = "p4d.24xlarge")]
    P4d24xlarge,

    /// p4de.24xlarge
    #[serde(rename = "p4de.24xlarge")]
    P4de24xlarge,

    /// r3.2xlarge
    #[serde(rename = "r3.2xlarge")]
    R32xlarge,

    /// r3.4xlarge
    #[serde(rename = "r3.4xlarge")]
    R34xlarge,

    /// r3.8xlarge
    #[serde(rename = "r3.8xlarge")]
    R38xlarge,

    /// r3.large
    #[serde(rename = "r3.large")]
    R3large,

    /// r3.xlarge
    #[serde(rename = "r3.xlarge")]
    R3xlarge,

    /// r4.16xlarge
    #[serde(rename = "r4.16xlarge")]
    R416xlarge,

    /// r4.2xlarge
    #[serde(rename = "r4.2xlarge")]
    R42xlarge,

    /// r4.4xlarge
    #[serde(rename = "r4.4xlarge")]
    R44xlarge,

    /// r4.8xlarge
    #[serde(rename = "r4.8xlarge")]
    R48xlarge,

    /// r4.large
    #[serde(rename = "r4.large")]
    R4large,

    /// r4.xlarge
    #[serde(rename = "r4.xlarge")]
    R4xlarge,

    /// r5.12xlarge
    #[serde(rename = "r5.12xlarge")]
    R512xlarge,

    /// r5.16xlarge
    #[serde(rename = "r5.16xlarge")]
    R516xlarge,

    /// r5.24xlarge
    #[serde(rename = "r5.24xlarge")]
    R524xlarge,

    /// r5.2xlarge
    #[serde(rename = "r5.2xlarge")]
    R52xlarge,

    /// r5.4xlarge
    #[serde(rename = "r5.4xlarge")]
    R54xlarge,

    /// r5.8xlarge
    #[serde(rename = "r5.8xlarge")]
    R58xlarge,

    /// r5.large
    #[serde(rename = "r5.large")]
    R5large,

    /// r5.metal
    #[serde(rename = "r5.metal")]
    R5metal,

    /// r5.xlarge
    #[serde(rename = "r5.xlarge")]
    R5xlarge,

    /// r5a.12xlarge
    #[serde(rename = "r5a.12xlarge")]
    R5a12xlarge,

    /// r5a.16xlarge
    #[serde(rename = "r5a.16xlarge")]
    R5a16xlarge,

    /// r5a.24xlarge
    #[serde(rename = "r5a.24xlarge")]
    R5a24xlarge,

    /// r5a.2xlarge
    #[serde(rename = "r5a.2xlarge")]
    R5a2xlarge,

    /// r5a.4xlarge
    #[serde(rename = "r5a.4xlarge")]
    R5a4xlarge,

    /// r5a.8xlarge
    #[serde(rename = "r5a.8xlarge")]
    R5a8xlarge,

    /// r5a.large
    #[serde(rename = "r5a.large")]
    R5alarge,

    /// r5a.xlarge
    #[serde(rename = "r5a.xlarge")]
    R5axlarge,

    /// r5ad.12xlarge
    #[serde(rename = "r5ad.12xlarge")]
    R5ad12xlarge,

    /// r5ad.16xlarge
    #[serde(rename = "r5ad.16xlarge")]
    R5ad16xlarge,

    /// r5ad.24xlarge
    #[serde(rename = "r5ad.24xlarge")]
    R5ad24xlarge,

    /// r5ad.2xlarge
    #[serde(rename = "r5ad.2xlarge")]
    R5ad2xlarge,

    /// r5ad.4xlarge
    #[serde(rename = "r5ad.4xlarge")]
    R5ad4xlarge,

    /// r5ad.8xlarge
    #[serde(rename = "r5ad.8xlarge")]
    R5ad8xlarge,

    /// r5ad.large
    #[serde(rename = "r5ad.large")]
    R5adlarge,

    /// r5ad.xlarge
    #[serde(rename = "r5ad.xlarge")]
    R5adxlarge,

    /// r5b.12xlarge
    #[serde(rename = "r5b.12xlarge")]
    R5b12xlarge,

    /// r5b.16xlarge
    #[serde(rename = "r5b.16xlarge")]
    R5b16xlarge,

    /// r5b.24xlarge
    #[serde(rename = "r5b.24xlarge")]
    R5b24xlarge,

    /// r5b.2xlarge
    #[serde(rename = "r5b.2xlarge")]
    R5b2xlarge,

    /// r5b.4xlarge
    #[serde(rename = "r5b.4xlarge")]
    R5b4xlarge,

    /// r5b.8xlarge
    #[serde(rename = "r5b.8xlarge")]
    R5b8xlarge,

    /// r5b.large
    #[serde(rename = "r5b.large")]
    R5blarge,

    /// r5b.metal
    #[serde(rename = "r5b.metal")]
    R5bmetal,

    /// r5b.xlarge
    #[serde(rename = "r5b.xlarge")]
    R5bxlarge,

    /// r5d.12xlarge
    #[serde(rename = "r5d.12xlarge")]
    R5d12xlarge,

    /// r5d.16xlarge
    #[serde(rename = "r5d.16xlarge")]
    R5d16xlarge,

    /// r5d.24xlarge
    #[serde(rename = "r5d.24xlarge")]
    R5d24xlarge,

    /// r5d.2xlarge
    #[serde(rename = "r5d.2xlarge")]
    R5d2xlarge,

    /// r5d.4xlarge
    #[serde(rename = "r5d.4xlarge")]
    R5d4xlarge,

    /// r5d.8xlarge
    #[serde(rename = "r5d.8xlarge")]
    R5d8xlarge,

    /// r5d.large
    #[serde(rename = "r5d.large")]
    R5dlarge,

    /// r5d.metal
    #[serde(rename = "r5d.metal")]
    R5dmetal,

    /// r5d.xlarge
    #[serde(rename = "r5d.xlarge")]
    R5dxlarge,

    /// r5dn.12xlarge
    #[serde(rename = "r5dn.12xlarge")]
    R5dn12xlarge,

    /// r5dn.16xlarge
    #[serde(rename = "r5dn.16xlarge")]
    R5dn16xlarge,

    /// r5dn.24xlarge
    #[serde(rename = "r5dn.24xlarge")]
    R5dn24xlarge,

    /// r5dn.2xlarge
    #[serde(rename = "r5dn.2xlarge")]
    R5dn2xlarge,

    /// r5dn.4xlarge
    #[serde(rename = "r5dn.4xlarge")]
    R5dn4xlarge,

    /// r5dn.8xlarge
    #[serde(rename = "r5dn.8xlarge")]
    R5dn8xlarge,

    /// r5dn.large
    #[serde(rename = "r5dn.large")]
    R5dnlarge,

    /// r5dn.metal
    #[serde(rename = "r5dn.metal")]
    R5dnmetal,

    /// r5dn.xlarge
    #[serde(rename = "r5dn.xlarge")]
    R5dnxlarge,

    /// r5n.12xlarge
    #[serde(rename = "r5n.12xlarge")]
    R5n12xlarge,

    /// r5n.16xlarge
    #[serde(rename = "r5n.16xlarge")]
    R5n16xlarge,

    /// r5n.24xlarge
    #[serde(rename = "r5n.24xlarge")]
    R5n24xlarge,

    /// r5n.2xlarge
    #[serde(rename = "r5n.2xlarge")]
    R5n2xlarge,

    /// r5n.4xlarge
    #[serde(rename = "r5n.4xlarge")]
    R5n4xlarge,

    /// r5n.8xlarge
    #[serde(rename = "r5n.8xlarge")]
    R5n8xlarge,

    /// r5n.large
    #[serde(rename = "r5n.large")]
    R5nlarge,

    /// r5n.metal
    #[serde(rename = "r5n.metal")]
    R5nmetal,

    /// r5n.xlarge
    #[serde(rename = "r5n.xlarge")]
    R5nxlarge,

    /// r6a.12xlarge
    #[serde(rename = "r6a.12xlarge")]
    R6a12xlarge,

    /// r6a.16xlarge
    #[serde(rename = "r6a.16xlarge")]
    R6a16xlarge,

    /// r6a.24xlarge
    #[serde(rename = "r6a.24xlarge")]
    R6a24xlarge,

    /// r6a.2xlarge
    #[serde(rename = "r6a.2xlarge")]
    R6a2xlarge,

    /// r6a.32xlarge
    #[serde(rename = "r6a.32xlarge")]
    R6a32xlarge,

    /// r6a.48xlarge
    #[serde(rename = "r6a.48xlarge")]
    R6a48xlarge,

    /// r6a.4xlarge
    #[serde(rename = "r6a.4xlarge")]
    R6a4xlarge,

    /// r6a.8xlarge
    #[serde(rename = "r6a.8xlarge")]
    R6a8xlarge,

    /// r6a.large
    #[serde(rename = "r6a.large")]
    R6alarge,

    /// r6a.metal
    #[serde(rename = "r6a.metal")]
    R6ametal,

    /// r6a.xlarge
    #[serde(rename = "r6a.xlarge")]
    R6axlarge,

    /// r6g.12xlarge
    #[serde(rename = "r6g.12xlarge")]
    R6g12xlarge,

    /// r6g.16xlarge
    #[serde(rename = "r6g.16xlarge")]
    R6g16xlarge,

    /// r6g.2xlarge
    #[serde(rename = "r6g.2xlarge")]
    R6g2xlarge,

    /// r6g.4xlarge
    #[serde(rename = "r6g.4xlarge")]
    R6g4xlarge,

    /// r6g.8xlarge
    #[serde(rename = "r6g.8xlarge")]
    R6g8xlarge,

    /// r6g.large
    #[serde(rename = "r6g.large")]
    R6glarge,

    /// r6g.medium
    #[serde(rename = "r6g.medium")]
    R6gmedium,

    /// r6g.metal
    #[serde(rename = "r6g.metal")]
    R6gmetal,

    /// r6g.xlarge
    #[serde(rename = "r6g.xlarge")]
    R6gxlarge,

    /// r6gd.12xlarge
    #[serde(rename = "r6gd.12xlarge")]
    R6gd12xlarge,

    /// r6gd.16xlarge
    #[serde(rename = "r6gd.16xlarge")]
    R6gd16xlarge,

    /// r6gd.2xlarge
    #[serde(rename = "r6gd.2xlarge")]
    R6gd2xlarge,

    /// r6gd.4xlarge
    #[serde(rename = "r6gd.4xlarge")]
    R6gd4xlarge,

    /// r6gd.8xlarge
    #[serde(rename = "r6gd.8xlarge")]
    R6gd8xlarge,

    /// r6gd.large
    #[serde(rename = "r6gd.large")]
    R6gdlarge,

    /// r6gd.medium
    #[serde(rename = "r6gd.medium")]
    R6gdmedium,

    /// r6gd.metal
    #[serde(rename = "r6gd.metal")]
    R6gdmetal,

    /// r6gd.xlarge
    #[serde(rename = "r6gd.xlarge")]
    R6gdxlarge,

    /// r6i.12xlarge
    #[serde(rename = "r6i.12xlarge")]
    R6i12xlarge,

    /// r6i.16xlarge
    #[serde(rename = "r6i.16xlarge")]
    R6i16xlarge,

    /// r6i.24xlarge
    #[serde(rename = "r6i.24xlarge")]
    R6i24xlarge,

    /// r6i.2xlarge
    #[serde(rename = "r6i.2xlarge")]
    R6i2xlarge,

    /// r6i.32xlarge
    #[serde(rename = "r6i.32xlarge")]
    R6i32xlarge,

    /// r6i.4xlarge
    #[serde(rename = "r6i.4xlarge")]
    R6i4xlarge,

    /// r6i.8xlarge
    #[serde(rename = "r6i.8xlarge")]
    R6i8xlarge,

    /// r6i.large
    #[serde(rename = "r6i.large")]
    R6ilarge,

    /// r6i.metal
    #[serde(rename = "r6i.metal")]
    R6imetal,

    /// r6i.xlarge
    #[serde(rename = "r6i.xlarge")]
    R6ixlarge,

    /// r6id.12xlarge
    #[serde(rename = "r6id.12xlarge")]
    R6id12xlarge,

    /// r6id.16xlarge
    #[serde(rename = "r6id.16xlarge")]
    R6id16xlarge,

    /// r6id.24xlarge
    #[serde(rename = "r6id.24xlarge")]
    R6id24xlarge,

    /// r6id.2xlarge
    #[serde(rename = "r6id.2xlarge")]
    R6id2xlarge,

    /// r6id.32xlarge
    #[serde(rename = "r6id.32xlarge")]
    R6id32xlarge,

    /// r6id.4xlarge
    #[serde(rename = "r6id.4xlarge")]
    R6id4xlarge,

    /// r6id.8xlarge
    #[serde(rename = "r6id.8xlarge")]
    R6id8xlarge,

    /// r6id.large
    #[serde(rename = "r6id.large")]
    R6idlarge,

    /// r6id.metal
    #[serde(rename = "r6id.metal")]
    R6idmetal,

    /// r6id.xlarge
    #[serde(rename = "r6id.xlarge")]
    R6idxlarge,

    /// r6idn.12xlarge
    #[serde(rename = "r6idn.12xlarge")]
    R6idn12xlarge,

    /// r6idn.16xlarge
    #[serde(rename = "r6idn.16xlarge")]
    R6idn16xlarge,

    /// r6idn.24xlarge
    #[serde(rename = "r6idn.24xlarge")]
    R6idn24xlarge,

    /// r6idn.2xlarge
    #[serde(rename = "r6idn.2xlarge")]
    R6idn2xlarge,

    /// r6idn.32xlarge
    #[serde(rename = "r6idn.32xlarge")]
    R6idn32xlarge,

    /// r6idn.4xlarge
    #[serde(rename = "r6idn.4xlarge")]
    R6idn4xlarge,

    /// r6idn.8xlarge
    #[serde(rename = "r6idn.8xlarge")]
    R6idn8xlarge,

    /// r6idn.large
    #[serde(rename = "r6idn.large")]
    R6idnlarge,

    /// r6idn.metal
    #[serde(rename = "r6idn.metal")]
    R6idnmetal,

    /// r6idn.xlarge
    #[serde(rename = "r6idn.xlarge")]
    R6idnxlarge,

    /// r6in.12xlarge
    #[serde(rename = "r6in.12xlarge")]
    R6in12xlarge,

    /// r6in.16xlarge
    #[serde(rename = "r6in.16xlarge")]
    R6in16xlarge,

    /// r6in.24xlarge
    #[serde(rename = "r6in.24xlarge")]
    R6in24xlarge,

    /// r6in.2xlarge
    #[serde(rename = "r6in.2xlarge")]
    R6in2xlarge,

    /// r6in.32xlarge
    #[serde(rename = "r6in.32xlarge")]
    R6in32xlarge,

    /// r6in.4xlarge
    #[serde(rename = "r6in.4xlarge")]
    R6in4xlarge,

    /// r6in.8xlarge
    #[serde(rename = "r6in.8xlarge")]
    R6in8xlarge,

    /// r6in.large
    #[serde(rename = "r6in.large")]
    R6inlarge,

    /// r6in.metal
    #[serde(rename = "r6in.metal")]
    R6inmetal,

    /// r6in.xlarge
    #[serde(rename = "r6in.xlarge")]
    R6inxlarge,

    /// t1.micro
    #[serde(rename = "t1.micro")]
    T1micro,

    /// t2.2xlarge
    #[serde(rename = "t2.2xlarge")]
    T22xlarge,

    /// t2.large
    #[serde(rename = "t2.large")]
    T2large,

    /// t2.medium
    #[serde(rename = "t2.medium")]
    T2medium,

    /// t2.micro
    #[serde(rename = "t2.micro")]
    T2micro,

    /// t2.nano
    #[serde(rename = "t2.nano")]
    T2nano,

    /// t2.small
    #[serde(rename = "t2.small")]
    T2small,

    /// t2.xlarge
    #[serde(rename = "t2.xlarge")]
    T2xlarge,

    /// t3.2xlarge
    #[serde(rename = "t3.2xlarge")]
    T32xlarge,

    /// t3.large
    #[serde(rename = "t3.large")]
    T3large,

    /// t3.medium
    #[serde(rename = "t3.medium")]
    T3medium,

    /// t3.micro
    #[serde(rename = "t3.micro")]
    T3micro,

    /// t3.nano
    #[serde(rename = "t3.nano")]
    T3nano,

    /// t3.small
    #[serde(rename = "t3.small")]
    T3small,

    /// t3.xlarge
    #[serde(rename = "t3.xlarge")]
    T3xlarge,

    /// t3a.2xlarge
    #[serde(rename = "t3a.2xlarge")]
    T3a2xlarge,

    /// t3a.large
    #[serde(rename = "t3a.large")]
    T3alarge,

    /// t3a.medium
    #[serde(rename = "t3a.medium")]
    T3amedium,

    /// t3a.micro
    #[serde(rename = "t3a.micro")]
    T3amicro,

    /// t3a.nano
    #[serde(rename = "t3a.nano")]
    T3anano,

    /// t3a.small
    #[serde(rename = "t3a.small")]
    T3asmall,

    /// t3a.xlarge
    #[serde(rename = "t3a.xlarge")]
    T3axlarge,

    /// t4g.2xlarge
    #[serde(rename = "t4g.2xlarge")]
    T4g2xlarge,

    /// t4g.large
    #[serde(rename = "t4g.large")]
    T4glarge,

    /// t4g.medium
    #[serde(rename = "t4g.medium")]
    T4gmedium,

    /// t4g.micro
    #[serde(rename = "t4g.micro")]
    T4gmicro,

    /// t4g.nano
    #[serde(rename = "t4g.nano")]
    T4gnano,

    /// t4g.small
    #[serde(rename = "t4g.small")]
    T4gsmall,

    /// t4g.xlarge
    #[serde(rename = "t4g.xlarge")]
    T4gxlarge,

    /// trn1.2xlarge
    #[serde(rename = "trn1.2xlarge")]
    Trn12xlarge,

    /// trn1.32xlarge
    #[serde(rename = "trn1.32xlarge")]
    Trn132xlarge,

    /// trn1n.32xlarge
    #[serde(rename = "trn1n.32xlarge")]
    Trn1n32xlarge,

    /// u-12tb1.112xlarge
    #[serde(rename = "u-12tb1.112xlarge")]
    U12tb1112xlarge,

    /// u-12tb1.metal
    #[serde(rename = "u-12tb1.metal")]
    U12tb1metal,

    /// u-18tb1.112xlarge
    #[serde(rename = "u-18tb1.112xlarge")]
    U18tb1112xlarge,

    /// u-18tb1.metal
    #[serde(rename = "u-18tb1.metal")]
    U18tb1metal,

    /// u-24tb1.112xlarge
    #[serde(rename = "u-24tb1.112xlarge")]
    U24tb1112xlarge,

    /// u-24tb1.metal
    #[serde(rename = "u-24tb1.metal")]
    U24tb1metal,

    /// u-3tb1.56xlarge
    #[serde(rename = "u-3tb1.56xlarge")]
    U3tb156xlarge,

    /// u-6tb1.112xlarge
    #[serde(rename = "u-6tb1.112xlarge")]
    U6tb1112xlarge,

    /// u-6tb1.56xlarge
    #[serde(rename = "u-6tb1.56xlarge")]
    U6tb156xlarge,

    /// u-6tb1.metal
    #[serde(rename = "u-6tb1.metal")]
    U6tb1metal,

    /// u-9tb1.112xlarge
    #[serde(rename = "u-9tb1.112xlarge")]
    U9tb1112xlarge,

    /// u-9tb1.metal
    #[serde(rename = "u-9tb1.metal")]
    U9tb1metal,

    /// vt1.24xlarge
    #[serde(rename = "vt1.24xlarge")]
    Vt124xlarge,

    /// vt1.3xlarge
    #[serde(rename = "vt1.3xlarge")]
    Vt13xlarge,

    /// vt1.6xlarge
    #[serde(rename = "vt1.6xlarge")]
    Vt16xlarge,

    /// x1.16xlarge
    #[serde(rename = "x1.16xlarge")]
    X116xlarge,

    /// x1.32xlarge
    #[serde(rename = "x1.32xlarge")]
    X132xlarge,

    /// x1e.16xlarge
    #[serde(rename = "x1e.16xlarge")]
    X1e16xlarge,

    /// x1e.2xlarge
    #[serde(rename = "x1e.2xlarge")]
    X1e2xlarge,

    /// x1e.32xlarge
    #[serde(rename = "x1e.32xlarge")]
    X1e32xlarge,

    /// x1e.4xlarge
    #[serde(rename = "x1e.4xlarge")]
    X1e4xlarge,

    /// x1e.8xlarge
    #[serde(rename = "x1e.8xlarge")]
    X1e8xlarge,

    /// x1e.xlarge
    #[serde(rename = "x1e.xlarge")]
    X1exlarge,

    /// x2gd.12xlarge
    #[serde(rename = "x2gd.12xlarge")]
    X2gd12xlarge,

    /// x2gd.16xlarge
    #[serde(rename = "x2gd.16xlarge")]
    X2gd16xlarge,

    /// x2gd.2xlarge
    #[serde(rename = "x2gd.2xlarge")]
    X2gd2xlarge,

    /// x2gd.4xlarge
    #[serde(rename = "x2gd.4xlarge")]
    X2gd4xlarge,

    /// x2gd.8xlarge
    #[serde(rename = "x2gd.8xlarge")]
    X2gd8xlarge,

    /// x2gd.large
    #[serde(rename = "x2gd.large")]
    X2gdlarge,

    /// x2gd.medium
    #[serde(rename = "x2gd.medium")]
    X2gdmedium,

    /// x2gd.metal
    #[serde(rename = "x2gd.metal")]
    X2gdmetal,

    /// x2gd.xlarge
    #[serde(rename = "x2gd.xlarge")]
    X2gdxlarge,

    /// x2idn.16xlarge
    #[serde(rename = "x2idn.16xlarge")]
    X2idn16xlarge,

    /// x2idn.24xlarge
    #[serde(rename = "x2idn.24xlarge")]
    X2idn24xlarge,

    /// x2idn.32xlarge
    #[serde(rename = "x2idn.32xlarge")]
    X2idn32xlarge,

    /// x2idn.metal
    #[serde(rename = "x2idn.metal")]
    X2idnmetal,

    /// x2iedn.16xlarge
    #[serde(rename = "x2iedn.16xlarge")]
    X2iedn16xlarge,

    /// x2iedn.24xlarge
    #[serde(rename = "x2iedn.24xlarge")]
    X2iedn24xlarge,

    /// x2iedn.2xlarge
    #[serde(rename = "x2iedn.2xlarge")]
    X2iedn2xlarge,

    /// x2iedn.32xlarge
    #[serde(rename = "x2iedn.32xlarge")]
    X2iedn32xlarge,

    /// x2iedn.4xlarge
    #[serde(rename = "x2iedn.4xlarge")]
    X2iedn4xlarge,

    /// x2iedn.8xlarge
    #[serde(rename = "x2iedn.8xlarge")]
    X2iedn8xlarge,

    /// x2iedn.metal
    #[serde(rename = "x2iedn.metal")]
    X2iednmetal,

    /// x2iedn.xlarge
    #[serde(rename = "x2iedn.xlarge")]
    X2iednxlarge,

    /// x2iezn.12xlarge
    #[serde(rename = "x2iezn.12xlarge")]
    X2iezn12xlarge,

    /// x2iezn.2xlarge
    #[serde(rename = "x2iezn.2xlarge")]
    X2iezn2xlarge,

    /// x2iezn.4xlarge
    #[serde(rename = "x2iezn.4xlarge")]
    X2iezn4xlarge,

    /// x2iezn.6xlarge
    #[serde(rename = "x2iezn.6xlarge")]
    X2iezn6xlarge,

    /// x2iezn.8xlarge
    #[serde(rename = "x2iezn.8xlarge")]
    X2iezn8xlarge,

    /// x2iezn.metal
    #[serde(rename = "x2iezn.metal")]
    X2ieznmetal,

    /// z1d.12xlarge
    #[serde(rename = "z1d.12xlarge")]
    Z1d12xlarge,

    /// z1d.2xlarge
    #[serde(rename = "z1d.2xlarge")]
    Z1d2xlarge,

    /// z1d.3xlarge
    #[serde(rename = "z1d.3xlarge")]
    Z1d3xlarge,

    /// z1d.6xlarge
    #[serde(rename = "z1d.6xlarge")]
    Z1d6xlarge,

    /// z1d.large
    #[serde(rename = "z1d.large")]
    Z1dlarge,

    /// z1d.metal
    #[serde(rename = "z1d.metal")]
    Z1dmetal,

    /// z1d.xlarge
    #[serde(rename = "z1d.xlarge")]
    Z1dxlarge,

}

impl Default for LaunchTemplateDataInstanceTypeEnum {
    fn default() -> Self {
        LaunchTemplateDataInstanceTypeEnum::A12xlarge
    }
}



/// Specifies an elastic inference accelerator.
///
/// LaunchTemplateElasticInferenceAccelerator is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateElasticInferenceAccelerator {


    /// 
    /// The number of elastic inference accelerators to attach to the instance.
    /// 
    /// Default: 1
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// 
    /// The type of elastic inference accelerator. The possible values are eia1.medium,       eia1.large, and eia1.xlarge.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}




/// Specifies the tags to apply to the launch template during creation.
///
/// LaunchTemplateTagSpecification is a property of AWS::EC2::LaunchTemplate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateTagSpecification {


    /// 
    /// The type of resource. To tag the launch template, ResourceType must be       launch-template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-reservation | capacity-reservation-fleet | carrier-gateway | client-vpn-endpoint | coip-pool | customer-gateway | dedicated-host | dhcp-options | egress-only-internet-gateway | elastic-gpu | elastic-ip | export-image-task | export-instance-task | fleet | fpga-image | host-reservation | image | import-image-task | import-snapshot-task | instance | instance-event-window | internet-gateway | ipam | ipam-pool | ipam-resource-discovery | ipam-resource-discovery-association | ipam-scope | ipv4pool-ec2 | ipv6pool-ec2 | key-pair | launch-template | local-gateway | local-gateway-route-table | local-gateway-route-table-virtual-interface-group-association | local-gateway-route-table-vpc-association | local-gateway-virtual-interface | local-gateway-virtual-interface-group | natgateway | network-acl | network-insights-access-scope | network-insights-access-scope-analysis | network-insights-analysis | network-insights-path | network-interface | placement-group | prefix-list | replace-root-volume-task | reserved-instances | route-table | security-group | security-group-rule | snapshot | spot-fleet-request | spot-instances-request | subnet | subnet-cidr-reservation | traffic-mirror-filter | traffic-mirror-filter-rule | traffic-mirror-session | traffic-mirror-target | transit-gateway | transit-gateway-attachment | transit-gateway-connect-peer | transit-gateway-multicast-domain | transit-gateway-policy-table | transit-gateway-route-table | transit-gateway-route-table-announcement | verified-access-endpoint | verified-access-group | verified-access-instance | verified-access-policy | verified-access-trust-provider | volume | vpc | vpc-block-public-access-exclusion | vpc-endpoint | vpc-endpoint-connection | vpc-endpoint-connection-device-type | vpc-endpoint-service | vpc-endpoint-service-permission | vpc-flow-log | vpc-peering-connection | vpn-connection | vpn-connection-device-type | vpn-gateway
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<LaunchTemplateTagSpecificationResourceTypeEnum>,


    /// 
    /// The tags for the resource.
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
pub enum LaunchTemplateTagSpecificationResourceTypeEnum {

    /// capacity-reservation
    #[serde(rename = "capacity-reservation")]
    Capacityreservation,

    /// capacity-reservation-fleet
    #[serde(rename = "capacity-reservation-fleet")]
    Capacityreservationfleet,

    /// carrier-gateway
    #[serde(rename = "carrier-gateway")]
    Carriergateway,

    /// client-vpn-endpoint
    #[serde(rename = "client-vpn-endpoint")]
    Clientvpnendpoint,

    /// coip-pool
    #[serde(rename = "coip-pool")]
    Coippool,

    /// customer-gateway
    #[serde(rename = "customer-gateway")]
    Customergateway,

    /// dedicated-host
    #[serde(rename = "dedicated-host")]
    Dedicatedhost,

    /// dhcp-options
    #[serde(rename = "dhcp-options")]
    Dhcpoptions,

    /// egress-only-internet-gateway
    #[serde(rename = "egress-only-internet-gateway")]
    Egressonlyinternetgateway,

    /// elastic-gpu
    #[serde(rename = "elastic-gpu")]
    Elasticgpu,

    /// elastic-ip
    #[serde(rename = "elastic-ip")]
    Elasticip,

    /// export-image-task
    #[serde(rename = "export-image-task")]
    Exportimagetask,

    /// export-instance-task
    #[serde(rename = "export-instance-task")]
    Exportinstancetask,

    /// fleet
    #[serde(rename = "fleet")]
    Fleet,

    /// fpga-image
    #[serde(rename = "fpga-image")]
    Fpgaimage,

    /// host-reservation
    #[serde(rename = "host-reservation")]
    Hostreservation,

    /// image
    #[serde(rename = "image")]
    Image,

    /// import-image-task
    #[serde(rename = "import-image-task")]
    Importimagetask,

    /// import-snapshot-task
    #[serde(rename = "import-snapshot-task")]
    Importsnapshottask,

    /// instance
    #[serde(rename = "instance")]
    Instance,

    /// instance-event-window
    #[serde(rename = "instance-event-window")]
    Instanceeventwindow,

    /// internet-gateway
    #[serde(rename = "internet-gateway")]
    Internetgateway,

    /// ipam
    #[serde(rename = "ipam")]
    Ipam,

    /// ipam-pool
    #[serde(rename = "ipam-pool")]
    Ipampool,

    /// ipam-resource-discovery
    #[serde(rename = "ipam-resource-discovery")]
    Ipamresourcediscovery,

    /// ipam-resource-discovery-association
    #[serde(rename = "ipam-resource-discovery-association")]
    Ipamresourcediscoveryassociation,

    /// ipam-scope
    #[serde(rename = "ipam-scope")]
    Ipamscope,

    /// ipv4pool-ec2
    #[serde(rename = "ipv4pool-ec2")]
    Ipv4poolec2,

    /// ipv6pool-ec2
    #[serde(rename = "ipv6pool-ec2")]
    Ipv6poolec2,

    /// key-pair
    #[serde(rename = "key-pair")]
    Keypair,

    /// launch-template
    #[serde(rename = "launch-template")]
    Launchtemplate,

    /// local-gateway
    #[serde(rename = "local-gateway")]
    Localgateway,

    /// local-gateway-route-table
    #[serde(rename = "local-gateway-route-table")]
    Localgatewayroutetable,

    /// local-gateway-route-table-virtual-interface-group-association
    #[serde(rename = "local-gateway-route-table-virtual-interface-group-association")]
    Localgatewayroutetablevirtualinterfacegroupassociation,

    /// local-gateway-route-table-vpc-association
    #[serde(rename = "local-gateway-route-table-vpc-association")]
    Localgatewayroutetablevpcassociation,

    /// local-gateway-virtual-interface
    #[serde(rename = "local-gateway-virtual-interface")]
    Localgatewayvirtualinterface,

    /// local-gateway-virtual-interface-group
    #[serde(rename = "local-gateway-virtual-interface-group")]
    Localgatewayvirtualinterfacegroup,

    /// natgateway
    #[serde(rename = "natgateway")]
    Natgateway,

    /// network-acl
    #[serde(rename = "network-acl")]
    Networkacl,

    /// network-insights-access-scope
    #[serde(rename = "network-insights-access-scope")]
    Networkinsightsaccessscope,

    /// network-insights-access-scope-analysis
    #[serde(rename = "network-insights-access-scope-analysis")]
    Networkinsightsaccessscopeanalysis,

    /// network-insights-analysis
    #[serde(rename = "network-insights-analysis")]
    Networkinsightsanalysis,

    /// network-insights-path
    #[serde(rename = "network-insights-path")]
    Networkinsightspath,

    /// network-interface
    #[serde(rename = "network-interface")]
    Networkinterface,

    /// placement-group
    #[serde(rename = "placement-group")]
    Placementgroup,

    /// prefix-list
    #[serde(rename = "prefix-list")]
    Prefixlist,

    /// replace-root-volume-task
    #[serde(rename = "replace-root-volume-task")]
    Replacerootvolumetask,

    /// reserved-instances
    #[serde(rename = "reserved-instances")]
    Reservedinstances,

    /// route-table
    #[serde(rename = "route-table")]
    Routetable,

    /// security-group
    #[serde(rename = "security-group")]
    Securitygroup,

    /// security-group-rule
    #[serde(rename = "security-group-rule")]
    Securitygrouprule,

    /// snapshot
    #[serde(rename = "snapshot")]
    Snapshot,

    /// spot-fleet-request
    #[serde(rename = "spot-fleet-request")]
    Spotfleetrequest,

    /// spot-instances-request
    #[serde(rename = "spot-instances-request")]
    Spotinstancesrequest,

    /// subnet
    #[serde(rename = "subnet")]
    Subnet,

    /// subnet-cidr-reservation
    #[serde(rename = "subnet-cidr-reservation")]
    Subnetcidrreservation,

    /// traffic-mirror-filter
    #[serde(rename = "traffic-mirror-filter")]
    Trafficmirrorfilter,

    /// traffic-mirror-filter-rule
    #[serde(rename = "traffic-mirror-filter-rule")]
    Trafficmirrorfilterrule,

    /// traffic-mirror-session
    #[serde(rename = "traffic-mirror-session")]
    Trafficmirrorsession,

    /// traffic-mirror-target
    #[serde(rename = "traffic-mirror-target")]
    Trafficmirrortarget,

    /// transit-gateway
    #[serde(rename = "transit-gateway")]
    Transitgateway,

    /// transit-gateway-attachment
    #[serde(rename = "transit-gateway-attachment")]
    Transitgatewayattachment,

    /// transit-gateway-connect-peer
    #[serde(rename = "transit-gateway-connect-peer")]
    Transitgatewayconnectpeer,

    /// transit-gateway-multicast-domain
    #[serde(rename = "transit-gateway-multicast-domain")]
    Transitgatewaymulticastdomain,

    /// transit-gateway-policy-table
    #[serde(rename = "transit-gateway-policy-table")]
    Transitgatewaypolicytable,

    /// transit-gateway-route-table
    #[serde(rename = "transit-gateway-route-table")]
    Transitgatewayroutetable,

    /// transit-gateway-route-table-announcement
    #[serde(rename = "transit-gateway-route-table-announcement")]
    Transitgatewayroutetableannouncement,

    /// verified-access-endpoint
    #[serde(rename = "verified-access-endpoint")]
    Verifiedaccessendpoint,

    /// verified-access-group
    #[serde(rename = "verified-access-group")]
    Verifiedaccessgroup,

    /// verified-access-instance
    #[serde(rename = "verified-access-instance")]
    Verifiedaccessinstance,

    /// verified-access-policy
    #[serde(rename = "verified-access-policy")]
    Verifiedaccesspolicy,

    /// verified-access-trust-provider
    #[serde(rename = "verified-access-trust-provider")]
    Verifiedaccesstrustprovider,

    /// volume
    #[serde(rename = "volume")]
    Volume,

    /// vpc
    #[serde(rename = "vpc")]
    Vpc,

    /// vpc-block-public-access-exclusion
    #[serde(rename = "vpc-block-public-access-exclusion")]
    Vpcblockpublicaccessexclusion,

    /// vpc-endpoint
    #[serde(rename = "vpc-endpoint")]
    Vpcendpoint,

    /// vpc-endpoint-connection
    #[serde(rename = "vpc-endpoint-connection")]
    Vpcendpointconnection,

    /// vpc-endpoint-connection-device-type
    #[serde(rename = "vpc-endpoint-connection-device-type")]
    Vpcendpointconnectiondevicetype,

    /// vpc-endpoint-service
    #[serde(rename = "vpc-endpoint-service")]
    Vpcendpointservice,

    /// vpc-endpoint-service-permission
    #[serde(rename = "vpc-endpoint-service-permission")]
    Vpcendpointservicepermission,

    /// vpc-flow-log
    #[serde(rename = "vpc-flow-log")]
    Vpcflowlog,

    /// vpc-peering-connection
    #[serde(rename = "vpc-peering-connection")]
    Vpcpeeringconnection,

    /// vpn-connection
    #[serde(rename = "vpn-connection")]
    Vpnconnection,

    /// vpn-connection-device-type
    #[serde(rename = "vpn-connection-device-type")]
    Vpnconnectiondevicetype,

    /// vpn-gateway
    #[serde(rename = "vpn-gateway")]
    Vpngateway,

}

impl Default for LaunchTemplateTagSpecificationResourceTypeEnum {
    fn default() -> Self {
        LaunchTemplateTagSpecificationResourceTypeEnum::Capacityreservation
    }
}



/// Specifies a license configuration for an instance.
///
/// LicenseSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LicenseSpecification {


    /// 
    /// The Amazon Resource Name (ARN) of the license configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: Option<String>,

}




/// The maintenance options of your instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceOptions {


    /// 
    /// Disables the automatic recovery behavior of your instance or sets it to       default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: default | disabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoRecovery")]
    pub auto_recovery: Option<MaintenanceOptionsAutoRecoveryEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MaintenanceOptionsAutoRecoveryEnum {

    /// default
    #[serde(rename = "default")]
    Default,

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

}

impl Default for MaintenanceOptionsAutoRecoveryEnum {
    fn default() -> Self {
        MaintenanceOptionsAutoRecoveryEnum::Default
    }
}



/// The minimum and maximum amount of memory per vCPU, in GiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemoryGiBPerVCpu {


    /// 
    /// The maximum amount of memory per vCPU, in GiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum amount of memory per vCPU, in GiB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}




/// The minimum and maximum amount of memory, in MiB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemoryMiB {


    /// 
    /// The maximum amount of memory, in MiB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum amount of memory, in MiB. To specify no minimum limit, specify     0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}




/// The metadata options for the instance. For more information, see Instance metadata and user data in the     Amazon EC2 User Guide.
///
/// MetadataOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetadataOptions {


    /// 
    /// Enables or disables the HTTP metadata endpoint on your instances. If the parameter is       not specified, the default state is enabled.
    /// 
    /// NoteIf you specify a value of disabled, you will not be able to access         your instance metadata.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpEndpoint")]
    pub http_endpoint: Option<MetadataOptionsHttpEndpointEnum>,


    /// 
    /// Enables or disables the IPv6 endpoint for the instance metadata service.
    /// 
    /// Default: disabled
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpProtocolIpv6")]
    pub http_protocol_ipv6: Option<MetadataOptionsHttpProtocolIpv6Enum>,


    /// 
    /// The desired HTTP PUT response hop limit for instance metadata requests. The larger the       number, the further instance metadata requests can travel.
    /// 
    /// Default: 1
    /// 
    /// Possible values: Integers from 1 to 64
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<i64>,


    /// 
    /// IMDSv2 uses token-backed sessions. Set the use of HTTP tokens to optional       (in other words, set the use of IMDSv2 to optional) or         required (in other words, set the use of IMDSv2 to         required).
    /// 
    /// optional - When IMDSv2 is optional, you can choose to retrieve instance metadata with or without       a session token in your request. If you retrieve the IAM role credentials       without a token, the IMDSv1 role credentials are returned. If you retrieve the IAM role credentials       using a valid session token, the IMDSv2 role credentials are returned.                        required - When IMDSv2 is required, you must send a session token       with any instance metadata retrieval requests. In this state, retrieving the IAM role       credentials always returns IMDSv2 credentials; IMDSv1 credentials are not available.
    /// 
    /// Default: optional
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: optional | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpTokens")]
    pub http_tokens: Option<MetadataOptionsHttpTokensEnum>,


    /// 
    /// Set to enabled to allow access to instance tags from the instance       metadata. Set to disabled to turn off access to instance tags from the       instance metadata. For more information, see Work with         instance tags using the instance metadata.
    /// 
    /// Default: disabled
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMetadataTags")]
    pub instance_metadata_tags: Option<MetadataOptionsInstanceMetadataTagsEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MetadataOptionsHttpEndpointEnum {

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,

}

impl Default for MetadataOptionsHttpEndpointEnum {
    fn default() -> Self {
        MetadataOptionsHttpEndpointEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MetadataOptionsHttpProtocolIpv6Enum {

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,

}

impl Default for MetadataOptionsHttpProtocolIpv6Enum {
    fn default() -> Self {
        MetadataOptionsHttpProtocolIpv6Enum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MetadataOptionsHttpTokensEnum {

    /// optional
    #[serde(rename = "optional")]
    Optional,

    /// required
    #[serde(rename = "required")]
    Required,

}

impl Default for MetadataOptionsHttpTokensEnum {
    fn default() -> Self {
        MetadataOptionsHttpTokensEnum::Optional
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MetadataOptionsInstanceMetadataTagsEnum {

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,

}

impl Default for MetadataOptionsInstanceMetadataTagsEnum {
    fn default() -> Self {
        MetadataOptionsInstanceMetadataTagsEnum::Disabled
    }
}



/// Specifies whether detailed monitoring is enabled for an instance. For more information     about detailed monitoring, see Enable or turn off detailed       monitoring for your instances in the Amazon EC2 User       Guide.
///
/// Monitoring is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Monitoring {


    /// 
    /// Specify true to enable detailed monitoring. Otherwise, basic monitoring       is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}




/// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkBandwidthGbps {


    /// 
    /// The maximum amount of network bandwidth, in Gbps. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum amount of network bandwidth, in Gbps. If this parameter is not specified, there is no minimum     limit.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}




/// Specifies the parameters for a network interface.
///
/// NetworkInterface is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInterface {


    /// 
    /// Indicates whether to associate a Carrier IP address with eth0 for a new network       interface.
    /// 
    /// Use this option when you launch an instance in a Wavelength Zone and want to associate       a Carrier IP address with the network interface. For more information about Carrier IP       addresses, see Carrier IP addresses in the         AWS Wavelength Developer       Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociateCarrierIpAddress")]
    pub associate_carrier_ip_address: Option<bool>,


    /// 
    /// Associates a public IPv4 address with eth0 for a new network interface.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,


    /// 
    /// Indicates whether the network interface is deleted when the instance is       terminated.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// A description for the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The device index for the network interface attachment.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceIndex")]
    pub device_index: Option<i64>,


    /// 
    /// The IDs of one or more security groups.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,


    /// 
    /// The type of network interface. To create an Elastic Fabric Adapter (EFA), specify         efa. For more information, see Elastic Fabric Adapter in the         Amazon Elastic Compute Cloud User Guide.
    /// 
    /// If you are not creating an EFA, specify interface or omit this       parameter.
    /// 
    /// Valid values: interface | efa
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<String>,


    /// 
    /// The number of IPv4 prefixes to be automatically assigned to the network interface. You       cannot use this option if you use the Ipv4Prefix option.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv4PrefixCount")]
    pub ipv4_prefix_count: Option<i64>,


    /// 
    /// One or more IPv4 prefixes to be assigned to the network interface. You cannot use this       option if you use the Ipv4PrefixCount option.
    /// 
    /// Required: No
    ///
    /// Type: List of Ipv4PrefixSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv4Prefixes")]
    pub ipv4_prefixes: Option<Vec<Ipv4PrefixSpecification>>,


    /// 
    /// The number of IPv6 addresses to assign to a network interface. Amazon EC2       automatically selects the IPv6 addresses from the subnet range. You can't use this       option if specifying specific IPv6 addresses.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<i64>,


    /// 
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You       can't use this option if you're specifying a number of IPv6 addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of Ipv6Add
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<Ipv6Add>>,


    /// 
    /// The number of IPv6 prefixes to be automatically assigned to the network interface. You       cannot use this option if you use the Ipv6Prefix option.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6PrefixCount")]
    pub ipv6_prefix_count: Option<i64>,


    /// 
    /// One or more IPv6 prefixes to be assigned to the network interface. You cannot use this       option if you use the Ipv6PrefixCount option.
    /// 
    /// Required: No
    ///
    /// Type: List of Ipv6PrefixSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Prefixes")]
    pub ipv6_prefixes: Option<Vec<Ipv6PrefixSpecification>>,


    /// 
    /// The index of the network card. Some instance types support multiple network cards.       The primary network interface must be assigned to network card index 0.       The default is network card index 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkCardIndex")]
    pub network_card_index: Option<i64>,


    /// 
    /// The ID of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The primary private IPv4 address of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,


    /// 
    /// One or more private IPv4 addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of PrivateIpAdd
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<PrivateIpAdd>>,


    /// 
    /// The number of secondary private IPv4 addresses to assign to a network       interface.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<i64>,


    /// 
    /// The ID of the subnet for the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,

}




/// The minimum and maximum number of network interfaces.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInterfaceCount {


    /// 
    /// The maximum number of network interfaces. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum number of network interfaces. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}




/// Specifies the placement of an instance.
///
/// Placement is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Placement {


    /// 
    /// The affinity setting for an instance on a Dedicated Host.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,


    /// 
    /// The Availability Zone for the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The Group Id of a placement group. You must specify the Placement Group Group Id to launch an instance in a shared placement       group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,


    /// 
    /// The name of the placement group for the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,


    /// 
    /// The ID of the Dedicated Host for the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,


    /// 
    /// The ARN of the host resource group in which to launch the instances. If you specify a       host resource group ARN, omit the Tenancy parameter or       set it to host.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,


    /// 
    /// The number of the partition the instance should launch in. Valid only if the placement       group strategy is set to partition.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<i64>,


    /// 
    /// Reserved for future use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpreadDomain")]
    pub spread_domain: Option<String>,


    /// 
    /// The tenancy of the instance. An instance with a       tenancy of dedicated runs on single-tenant hardware.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dedicated | default | host
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<PlacementTenancyEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PlacementTenancyEnum {

    /// dedicated
    #[serde(rename = "dedicated")]
    Dedicated,

    /// default
    #[serde(rename = "default")]
    Default,

    /// host
    #[serde(rename = "host")]
    Host,

}

impl Default for PlacementTenancyEnum {
    fn default() -> Self {
        PlacementTenancyEnum::Dedicated
    }
}



/// The hostname type for EC2 instances launched into this subnet and how DNS A and AAAA record queries should be handled. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateDnsNameOptions {


    /// 
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA       records.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,


    /// 
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A       records.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsARecord")]
    pub enable_resource_name_dns_arecord: Option<bool>,


    /// 
    /// The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be      based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based      on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance     IPv4 address or the instance ID. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ip-name | resource-name
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostnameType")]
    pub hostname_type: Option<PrivateDnsNameOptionsHostnameTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PrivateDnsNameOptionsHostnameTypeEnum {

    /// ip-name
    #[serde(rename = "ip-name")]
    Ipname,

    /// resource-name
    #[serde(rename = "resource-name")]
    Resourcename,

}

impl Default for PrivateDnsNameOptionsHostnameTypeEnum {
    fn default() -> Self {
        PrivateDnsNameOptionsHostnameTypeEnum::Ipname
    }
}



/// Specifies a secondary private IPv4 address for a network interface.
///
/// PrivateIpAdd is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateIpAdd {


    /// 
    /// Indicates whether the private IPv4 address is the primary private IPv4 address. Only       one IPv4 address can be designated as primary.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Primary")]
    pub primary: Option<bool>,


    /// 
    /// The private IPv4 address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,

}




/// Specifies options for Spot Instances.
///
/// SpotOptions is a property of AWS::EC2::LaunchTemplate InstanceMarketOptions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpotOptions {


    /// 
    /// Deprecated.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockDurationMinutes")]
    pub block_duration_minutes: Option<i64>,


    /// 
    /// The behavior when a Spot Instance is interrupted. The default is         terminate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: hibernate | stop | terminate
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceInterruptionBehavior")]
    pub instance_interruption_behavior: Option<SpotOptionsInstanceInterruptionBehaviorEnum>,


    /// 
    /// The maximum hourly price you're willing to pay for the Spot Instances. We do not       recommend using this parameter because it can lead to increased interruptions. If you do       not specify this parameter, you will pay the current Spot price.
    /// 
    /// ImportantIf you specify a maximum price, your Spot Instances will be interrupted more         frequently than if you do not specify this parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxPrice")]
    pub max_price: Option<String>,


    /// 
    /// The Spot Instance request type.
    /// 
    /// If you are using Spot Instances with an Auto Scaling group, use one-time     requests, as the Amazon EC2 Auto Scaling service handles requesting new Spot Instances     whenever the group is below its desired capacity.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: one-time | persistent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotInstanceType")]
    pub spot_instance_type: Option<SpotOptionsSpotInstanceTypeEnum>,


    /// 
    /// The end date of the request, in UTC format         (YYYY-MM-DDTHH:MM:SSZ). Supported only for       persistent requests.
    /// 
    /// For a persistent request, the request remains active until the             ValidUntil date and time is reached. Otherwise, the request           remains active until you cancel it.               For a one-time request, ValidUntil is not supported. The request           remains active until all instances launch or you cancel the request.
    /// 
    /// Default: 7 days from the current date
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidUntil")]
    pub valid_until: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SpotOptionsInstanceInterruptionBehaviorEnum {

    /// hibernate
    #[serde(rename = "hibernate")]
    Hibernate,

    /// stop
    #[serde(rename = "stop")]
    Stop,

    /// terminate
    #[serde(rename = "terminate")]
    Terminate,

}

impl Default for SpotOptionsInstanceInterruptionBehaviorEnum {
    fn default() -> Self {
        SpotOptionsInstanceInterruptionBehaviorEnum::Hibernate
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SpotOptionsSpotInstanceTypeEnum {

    /// one-time
    #[serde(rename = "one-time")]
    Onetime,

    /// persistent
    #[serde(rename = "persistent")]
    Persistent,

}

impl Default for SpotOptionsSpotInstanceTypeEnum {
    fn default() -> Self {
        SpotOptionsSpotInstanceTypeEnum::Onetime
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




/// Specifies the tags to apply to a resource when the resource is created for the launch     template.
///
/// TagSpecification is a property type of TagSpecifications. TagSpecifications is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagSpecification {


    /// 
    /// The type of resource to tag.
    /// 
    /// The Valid Values are all the resource types that can be tagged. However,       when creating a launch template, you can specify tags for the following resource types       only: instance | volume | elastic-gpu |         network-interface | spot-instances-request
    /// 
    /// To tag a resource after it has been created, see CreateTags.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-reservation | capacity-reservation-fleet | carrier-gateway | client-vpn-endpoint | coip-pool | customer-gateway | dedicated-host | dhcp-options | egress-only-internet-gateway | elastic-gpu | elastic-ip | export-image-task | export-instance-task | fleet | fpga-image | host-reservation | image | import-image-task | import-snapshot-task | instance | instance-event-window | internet-gateway | ipam | ipam-pool | ipam-resource-discovery | ipam-resource-discovery-association | ipam-scope | ipv4pool-ec2 | ipv6pool-ec2 | key-pair | launch-template | local-gateway | local-gateway-route-table | local-gateway-route-table-virtual-interface-group-association | local-gateway-route-table-vpc-association | local-gateway-virtual-interface | local-gateway-virtual-interface-group | natgateway | network-acl | network-insights-access-scope | network-insights-access-scope-analysis | network-insights-analysis | network-insights-path | network-interface | placement-group | prefix-list | replace-root-volume-task | reserved-instances | route-table | security-group | security-group-rule | snapshot | spot-fleet-request | spot-instances-request | subnet | subnet-cidr-reservation | traffic-mirror-filter | traffic-mirror-filter-rule | traffic-mirror-session | traffic-mirror-target | transit-gateway | transit-gateway-attachment | transit-gateway-connect-peer | transit-gateway-multicast-domain | transit-gateway-policy-table | transit-gateway-route-table | transit-gateway-route-table-announcement | verified-access-endpoint | verified-access-group | verified-access-instance | verified-access-policy | verified-access-trust-provider | volume | vpc | vpc-block-public-access-exclusion | vpc-endpoint | vpc-endpoint-connection | vpc-endpoint-connection-device-type | vpc-endpoint-service | vpc-endpoint-service-permission | vpc-flow-log | vpc-peering-connection | vpn-connection | vpn-connection-device-type | vpn-gateway
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<TagSpecificationResourceTypeEnum>,


    /// 
    /// The tags to apply to the resource.
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
pub enum TagSpecificationResourceTypeEnum {

    /// capacity-reservation
    #[serde(rename = "capacity-reservation")]
    Capacityreservation,

    /// capacity-reservation-fleet
    #[serde(rename = "capacity-reservation-fleet")]
    Capacityreservationfleet,

    /// carrier-gateway
    #[serde(rename = "carrier-gateway")]
    Carriergateway,

    /// client-vpn-endpoint
    #[serde(rename = "client-vpn-endpoint")]
    Clientvpnendpoint,

    /// coip-pool
    #[serde(rename = "coip-pool")]
    Coippool,

    /// customer-gateway
    #[serde(rename = "customer-gateway")]
    Customergateway,

    /// dedicated-host
    #[serde(rename = "dedicated-host")]
    Dedicatedhost,

    /// dhcp-options
    #[serde(rename = "dhcp-options")]
    Dhcpoptions,

    /// egress-only-internet-gateway
    #[serde(rename = "egress-only-internet-gateway")]
    Egressonlyinternetgateway,

    /// elastic-gpu
    #[serde(rename = "elastic-gpu")]
    Elasticgpu,

    /// elastic-ip
    #[serde(rename = "elastic-ip")]
    Elasticip,

    /// export-image-task
    #[serde(rename = "export-image-task")]
    Exportimagetask,

    /// export-instance-task
    #[serde(rename = "export-instance-task")]
    Exportinstancetask,

    /// fleet
    #[serde(rename = "fleet")]
    Fleet,

    /// fpga-image
    #[serde(rename = "fpga-image")]
    Fpgaimage,

    /// host-reservation
    #[serde(rename = "host-reservation")]
    Hostreservation,

    /// image
    #[serde(rename = "image")]
    Image,

    /// import-image-task
    #[serde(rename = "import-image-task")]
    Importimagetask,

    /// import-snapshot-task
    #[serde(rename = "import-snapshot-task")]
    Importsnapshottask,

    /// instance
    #[serde(rename = "instance")]
    Instance,

    /// instance-event-window
    #[serde(rename = "instance-event-window")]
    Instanceeventwindow,

    /// internet-gateway
    #[serde(rename = "internet-gateway")]
    Internetgateway,

    /// ipam
    #[serde(rename = "ipam")]
    Ipam,

    /// ipam-pool
    #[serde(rename = "ipam-pool")]
    Ipampool,

    /// ipam-resource-discovery
    #[serde(rename = "ipam-resource-discovery")]
    Ipamresourcediscovery,

    /// ipam-resource-discovery-association
    #[serde(rename = "ipam-resource-discovery-association")]
    Ipamresourcediscoveryassociation,

    /// ipam-scope
    #[serde(rename = "ipam-scope")]
    Ipamscope,

    /// ipv4pool-ec2
    #[serde(rename = "ipv4pool-ec2")]
    Ipv4poolec2,

    /// ipv6pool-ec2
    #[serde(rename = "ipv6pool-ec2")]
    Ipv6poolec2,

    /// key-pair
    #[serde(rename = "key-pair")]
    Keypair,

    /// launch-template
    #[serde(rename = "launch-template")]
    Launchtemplate,

    /// local-gateway
    #[serde(rename = "local-gateway")]
    Localgateway,

    /// local-gateway-route-table
    #[serde(rename = "local-gateway-route-table")]
    Localgatewayroutetable,

    /// local-gateway-route-table-virtual-interface-group-association
    #[serde(rename = "local-gateway-route-table-virtual-interface-group-association")]
    Localgatewayroutetablevirtualinterfacegroupassociation,

    /// local-gateway-route-table-vpc-association
    #[serde(rename = "local-gateway-route-table-vpc-association")]
    Localgatewayroutetablevpcassociation,

    /// local-gateway-virtual-interface
    #[serde(rename = "local-gateway-virtual-interface")]
    Localgatewayvirtualinterface,

    /// local-gateway-virtual-interface-group
    #[serde(rename = "local-gateway-virtual-interface-group")]
    Localgatewayvirtualinterfacegroup,

    /// natgateway
    #[serde(rename = "natgateway")]
    Natgateway,

    /// network-acl
    #[serde(rename = "network-acl")]
    Networkacl,

    /// network-insights-access-scope
    #[serde(rename = "network-insights-access-scope")]
    Networkinsightsaccessscope,

    /// network-insights-access-scope-analysis
    #[serde(rename = "network-insights-access-scope-analysis")]
    Networkinsightsaccessscopeanalysis,

    /// network-insights-analysis
    #[serde(rename = "network-insights-analysis")]
    Networkinsightsanalysis,

    /// network-insights-path
    #[serde(rename = "network-insights-path")]
    Networkinsightspath,

    /// network-interface
    #[serde(rename = "network-interface")]
    Networkinterface,

    /// placement-group
    #[serde(rename = "placement-group")]
    Placementgroup,

    /// prefix-list
    #[serde(rename = "prefix-list")]
    Prefixlist,

    /// replace-root-volume-task
    #[serde(rename = "replace-root-volume-task")]
    Replacerootvolumetask,

    /// reserved-instances
    #[serde(rename = "reserved-instances")]
    Reservedinstances,

    /// route-table
    #[serde(rename = "route-table")]
    Routetable,

    /// security-group
    #[serde(rename = "security-group")]
    Securitygroup,

    /// security-group-rule
    #[serde(rename = "security-group-rule")]
    Securitygrouprule,

    /// snapshot
    #[serde(rename = "snapshot")]
    Snapshot,

    /// spot-fleet-request
    #[serde(rename = "spot-fleet-request")]
    Spotfleetrequest,

    /// spot-instances-request
    #[serde(rename = "spot-instances-request")]
    Spotinstancesrequest,

    /// subnet
    #[serde(rename = "subnet")]
    Subnet,

    /// subnet-cidr-reservation
    #[serde(rename = "subnet-cidr-reservation")]
    Subnetcidrreservation,

    /// traffic-mirror-filter
    #[serde(rename = "traffic-mirror-filter")]
    Trafficmirrorfilter,

    /// traffic-mirror-filter-rule
    #[serde(rename = "traffic-mirror-filter-rule")]
    Trafficmirrorfilterrule,

    /// traffic-mirror-session
    #[serde(rename = "traffic-mirror-session")]
    Trafficmirrorsession,

    /// traffic-mirror-target
    #[serde(rename = "traffic-mirror-target")]
    Trafficmirrortarget,

    /// transit-gateway
    #[serde(rename = "transit-gateway")]
    Transitgateway,

    /// transit-gateway-attachment
    #[serde(rename = "transit-gateway-attachment")]
    Transitgatewayattachment,

    /// transit-gateway-connect-peer
    #[serde(rename = "transit-gateway-connect-peer")]
    Transitgatewayconnectpeer,

    /// transit-gateway-multicast-domain
    #[serde(rename = "transit-gateway-multicast-domain")]
    Transitgatewaymulticastdomain,

    /// transit-gateway-policy-table
    #[serde(rename = "transit-gateway-policy-table")]
    Transitgatewaypolicytable,

    /// transit-gateway-route-table
    #[serde(rename = "transit-gateway-route-table")]
    Transitgatewayroutetable,

    /// transit-gateway-route-table-announcement
    #[serde(rename = "transit-gateway-route-table-announcement")]
    Transitgatewayroutetableannouncement,

    /// verified-access-endpoint
    #[serde(rename = "verified-access-endpoint")]
    Verifiedaccessendpoint,

    /// verified-access-group
    #[serde(rename = "verified-access-group")]
    Verifiedaccessgroup,

    /// verified-access-instance
    #[serde(rename = "verified-access-instance")]
    Verifiedaccessinstance,

    /// verified-access-policy
    #[serde(rename = "verified-access-policy")]
    Verifiedaccesspolicy,

    /// verified-access-trust-provider
    #[serde(rename = "verified-access-trust-provider")]
    Verifiedaccesstrustprovider,

    /// volume
    #[serde(rename = "volume")]
    Volume,

    /// vpc
    #[serde(rename = "vpc")]
    Vpc,

    /// vpc-block-public-access-exclusion
    #[serde(rename = "vpc-block-public-access-exclusion")]
    Vpcblockpublicaccessexclusion,

    /// vpc-endpoint
    #[serde(rename = "vpc-endpoint")]
    Vpcendpoint,

    /// vpc-endpoint-connection
    #[serde(rename = "vpc-endpoint-connection")]
    Vpcendpointconnection,

    /// vpc-endpoint-connection-device-type
    #[serde(rename = "vpc-endpoint-connection-device-type")]
    Vpcendpointconnectiondevicetype,

    /// vpc-endpoint-service
    #[serde(rename = "vpc-endpoint-service")]
    Vpcendpointservice,

    /// vpc-endpoint-service-permission
    #[serde(rename = "vpc-endpoint-service-permission")]
    Vpcendpointservicepermission,

    /// vpc-flow-log
    #[serde(rename = "vpc-flow-log")]
    Vpcflowlog,

    /// vpc-peering-connection
    #[serde(rename = "vpc-peering-connection")]
    Vpcpeeringconnection,

    /// vpn-connection
    #[serde(rename = "vpn-connection")]
    Vpnconnection,

    /// vpn-connection-device-type
    #[serde(rename = "vpn-connection-device-type")]
    Vpnconnectiondevicetype,

    /// vpn-gateway
    #[serde(rename = "vpn-gateway")]
    Vpngateway,

}

impl Default for TagSpecificationResourceTypeEnum {
    fn default() -> Self {
        TagSpecificationResourceTypeEnum::Capacityreservation
    }
}



/// The minimum and maximum amount of total local storage, in GB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TotalLocalStorageGB {


    /// 
    /// The maximum amount of total local storage, in GB. To specify no maximum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum amount of total local storage, in GB. To specify no minimum limit, omit this     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}




/// The minimum and maximum number of vCPUs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VCpuCount {


    /// 
    /// The maximum number of vCPUs. To specify no maximum limit, omit this parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<i64>,


    /// 
    /// The minimum number of vCPUs. To specify no minimum limit, specify 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<i64>,

}


