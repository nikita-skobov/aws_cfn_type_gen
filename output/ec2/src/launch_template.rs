

/// Specifies the properties for creating a launch template.
///
/// The minimum required properties for specifying a launch template are as follows:
///
/// A launch template can contain some or all of the configuration information to launch an     instance. When you launch an instance using a launch template, instance properties that are     not specified in the launch template use default values, except the ImageId     property, which has no default value. If you do not specify an AMI ID for the launch     template ImageId property, you must specify an AMI ID for the instance       ImageId property.
///
/// For more information, see Launch an instance from a       launch template in the Amazon EC2 User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnLaunchTemplate {


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


/// The hostname type for EC2 instances launched into this subnet and how DNS A and AAAA record queries should be handled. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
#[derive(Default, serde::Serialize)]
pub struct PrivateDnsNameOptions {


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
    pub hostname_type: Option<String>,


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
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA       records.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,

}


/// Specifies an IPv6 address in an Amazon EC2 launch template.
///
/// Ipv6Add is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Default, serde::Serialize)]
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


/// Specifies the placement of an instance.
///
/// Placement is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
pub struct Placement {


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
    pub tenancy: Option<String>,

}


/// Specifies the CPU options for an instance. For more information, see Optimize       CPU options in the Amazon Elastic Compute Cloud User     Guide.
///
/// CpuOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
    pub amd_sev_snp: Option<String>,


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


/// Specifies the market (purchasing) option for an instance.
///
/// InstanceMarketOptions is a property of the AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
    pub market_type: Option<String>,


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


/// The minimum and maximum amount of total accelerator memory, in MiB.
#[derive(Default, serde::Serialize)]
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


/// The minimum and maximum number of network interfaces.
#[derive(Default, serde::Serialize)]
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


/// Specifies an IAM instance profile, which is a container for an IAM role for your     instance. You can use an IAM role to distribute your AWS credentials to     your instances.
///
/// If you are creating the launch template for use with an Amazon EC2 Auto Scaling group,     you can specify either the name or the ARN of the instance profile, but not both.
///
/// IamInstanceProfile is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
pub struct IamInstanceProfile {


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

}


/// Specifies an instance's Capacity Reservation targeting option. You can specify only one     option at a time.
///
/// CapacityReservationSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
    pub capacity_reservation_preference: Option<String>,


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


/// Specifies a target Capacity Reservation.
///
/// CapacityReservationTarget is a property of the Amazon EC2 LaunchTemplate LaunchTemplateData property type.
#[derive(Default, serde::Serialize)]
pub struct CapacityReservationTarget {


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

}


/// The minimum and maximum baseline bandwidth to Amazon EBS, in Mbps. For more information, see       Amazon       EBS–optimized instances in the Amazon EC2 User Guide.
#[derive(Default, serde::Serialize)]
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


/// Indicates whether the instance is enabled for AWS Nitro       Enclaves.
#[derive(Default, serde::Serialize)]
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


/// Specifies an elastic inference accelerator.
///
/// LaunchTemplateElasticInferenceAccelerator is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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


/// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps).
#[derive(Default, serde::Serialize)]
pub struct NetworkBandwidthGbps {


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

}


/// Information about a block device mapping for an Amazon EC2 launch template.
///
/// BlockDeviceMapping is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
pub struct BlockDeviceMapping {


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
    /// The device name (for example, /dev/sdh or xvdh).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

}


/// The minimum and maximum amount of total local storage, in GB.
#[derive(Default, serde::Serialize)]
pub struct TotalLocalStorageGB {


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

}


/// Specifies whether detailed monitoring is enabled for an instance. For more information     about detailed monitoring, see Enable or turn off detailed       monitoring for your instances in the Amazon EC2 User       Guide.
///
/// Monitoring is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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


/// Parameters for a block device for an EBS volume in an Amazon EC2 launch template.
///
/// Ebs is a property of AWS::EC2::LaunchTemplate BlockDeviceMapping.
#[derive(Default, serde::Serialize)]
pub struct Ebs {


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
    pub volume_type: Option<String>,


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

}


/// Specifies the credit option for CPU usage of a T2, T3, or T3a instance.
///
/// CreditSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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


/// Specifies the tags to apply to a resource when the resource is created for the launch     template.
///
/// TagSpecification is a property type of TagSpecifications. TagSpecifications is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
    pub resource_type: Option<String>,


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


/// The metadata options for the instance. For more information, see Instance metadata and user data in the     Amazon EC2 User Guide.
///
/// MetadataOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
pub struct MetadataOptions {


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
    pub http_protocol_ipv6: Option<String>,


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
    pub http_tokens: Option<String>,


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
    pub instance_metadata_tags: Option<String>,


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
    pub http_endpoint: Option<String>,

}


/// The minimum and maximum amount of memory per vCPU, in GiB.
#[derive(Default, serde::Serialize)]
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


/// Specifies an IPv4 prefix for a network interface.
///
/// Ipv4PrefixSpecification is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Default, serde::Serialize)]
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


/// The maintenance options of your instance.
#[derive(Default, serde::Serialize)]
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
    pub auto_recovery: Option<String>,

}


/// Specifies the tags to apply to the launch template during creation.
///
/// LaunchTemplateTagSpecification is a property of AWS::EC2::LaunchTemplate.
#[derive(Default, serde::Serialize)]
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
    pub resource_type: Option<String>,


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


/// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips)     on an instance.
#[derive(Default, serde::Serialize)]
pub struct AcceleratorCount {


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

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The minimum and maximum number of vCPUs.
#[derive(Default, serde::Serialize)]
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


/// Specifies a secondary private IPv4 address for a network interface.
///
/// PrivateIpAdd is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Default, serde::Serialize)]
pub struct PrivateIpAdd {


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

}


/// Specifies an IPv6 prefix for a network interface.
///
/// Ipv6PrefixSpecification is a property of AWS::EC2::LaunchTemplate NetworkInterface.
#[derive(Default, serde::Serialize)]
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


/// The attributes for the instance types. When you specify instance attributes, Amazon EC2 will     identify instance types with these attributes.
///
/// When you specify multiple attributes, you get instance types that satisfy all of the     specified attributes. If you specify multiple values for an attribute, you get instance     types that satisfy any of the specified values.
///
/// To limit the list of instance types from which Amazon EC2 can identify matching instance types,      you can use one of the following parameters, but not both in the same request:
///
/// For more information, see Attribute-based instance type selection for EC2 Fleet, Attribute-based instance type selection for Spot Fleet, and Spot        placement score in the Amazon EC2 User Guide.
#[derive(Default, serde::Serialize)]
pub struct InstanceRequirements {


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
    /// The minimum and maximum number of vCPUs.
    /// 
    /// Required: No
    ///
    /// Type: VCpuCount
    ///
    /// Update requires: No interruption
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCount>,


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
    pub local_storage: Option<String>,


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
    pub bare_metal: Option<String>,


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
    pub burstable_performance: Option<String>,


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

}


/// The minimum and maximum amount of memory, in MiB.
#[derive(Default, serde::Serialize)]
pub struct MemoryMiB {


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

}


/// Specifies a license configuration for an instance.
///
/// LicenseSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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


/// Specifies options for Spot Instances.
///
/// SpotOptions is a property of AWS::EC2::LaunchTemplate InstanceMarketOptions.
#[derive(Default, serde::Serialize)]
pub struct SpotOptions {


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
    pub instance_interruption_behavior: Option<String>,


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
    pub spot_instance_type: Option<String>,

}


/// Specifies a specification for an Elastic GPU for an Amazon EC2 launch template.
///
/// ElasticGpuSpecification is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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


/// The information to include in the launch template.
#[derive(Default, serde::Serialize)]
pub struct LaunchTemplateData {


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
    pub instance_initiated_shutdown_behavior: Option<String>,


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
    pub instance_type: Option<String>,


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

}


/// Specifies the parameters for a network interface.
///
/// NetworkInterface is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
    /// The ID of the subnet for the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


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
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You       can't use this option if you're specifying a number of IPv6 addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of Ipv6Add
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<Ipv6Add>>,

}


/// Specifies whether your instance is configured for hibernation. This parameter is valid     only if the instance meets the hibernation       prerequisites. For more information, see Hibernate Your Instance in the       Amazon EC2 User Guide.
///
/// HibernationOptions is a property of AWS::EC2::LaunchTemplate LaunchTemplateData.
#[derive(Default, serde::Serialize)]
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
