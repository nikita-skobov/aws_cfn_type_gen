

/// Specifies an EC2 instance.
///
/// If an Elastic IP address is attached to your instance, AWS CloudFormation     reattaches the Elastic IP address after it updates the instance. For more information about     updating stacks, see AWS CloudFormation Stacks Updates.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnInstance {


    /// 
    /// The name of an existing placement group that you want to launch the instance into     (cluster | partition | spread).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlacementGroupName")]
    pub placement_group_name: Option<String>,


    /// 
    /// If you set this parameter to true, you can't terminate the instance using       the Amazon EC2 console, CLI, or API; otherwise, you can. To change this attribute after       launch, use ModifyInstanceAttribute. Alternatively, if you set         InstanceInitiatedShutdownBehavior to terminate, you can       terminate the instance by running the shutdown command from the instance.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableApiTermination")]
    pub disable_api_termination: Option<bool>,


    /// 
    /// The tags to add to the instance. These tags are not applied to the EBS volumes, such as     the root volume, unless PropagateTagsToVolumeOnCreation     is true.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


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
    /// The tenancy of the instance. An instance with a tenancy of dedicated      runs on single-tenant hardware.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dedicated | default | host
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,


    /// 
    /// The SSM       document and parameter values in AWS Systems Manager to associate with this     instance. To use this property, you must specify an IAM instance profile role for the     instance. For more information, see Create an       IAM instance profile for Systems Manager in the AWS Systems Manager       User Guide.
    /// 
    /// NoteYou can currently associate only one document with an instance.
    /// 
    /// Required: No
    ///
    /// Type: List of SsmAssociation
    ///
    /// Update requires: No interruption
    #[serde(rename = "SsmAssociations")]
    pub ssm_associations: Option<Vec<SsmAssociation>>,


    /// 
    /// Indicates whether the instance is optimized for Amazon EBS I/O. This optimization       provides dedicated throughput to Amazon EBS and an optimized configuration stack to       provide optimal Amazon EBS I/O performance. This optimization isn't available with all       instance types. Additional usage charges apply when using an EBS-optimized       instance.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,


    /// 
    /// The CPU options for the instance. For more information, see Optimize CPU options in     the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: CpuOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "CpuOptions")]
    pub cpu_options: Option<CpuOptions>,


    /// 
    /// Indicates whether to assign the tags from the instance to all of the volumes attached to     the instance at launch. If you specify true and you assign tags to the     instance, those tags are automatically assigned to all of the volumes that you attach to     the instance at launch. If you specify false, those tags are not assigned to     the attached volumes.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateTagsToVolumeOnCreation")]
    pub propagate_tags_to_volume_on_creation: Option<bool>,


    /// 
    /// The primary IPv4 address. You must specify a value from the IPv4 address range     of the subnet.
    /// 
    /// Only one private IP address can be designated as primary. You can't specify this option     if you've specified the option to designate a private IP address as the primary IP address     in a network interface specification. You cannot specify this option if you're launching     more than one instance in the request.
    /// 
    /// You cannot specify this option and the network interfaces option in the same     request.
    /// 
    /// If you make an update to an instance that requires replacement, you must assign a new     private IP address. During a replacement, AWS CloudFormation creates a new instance     but doesn't delete the old instance until the stack has successfully updated. If the stack     update fails, AWS CloudFormation uses the old instance to roll back the stack to the     previous working state. The old and new instances cannot have the same private IP     address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,


    /// 
    /// The Availability Zone of the instance.
    /// 
    /// If not specified, an Availability Zone will be automatically chosen for you based on the     load balancing criteria for the Region.
    /// 
    /// This parameter is not supported by DescribeImageAttribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The instance type. For more information, see Instance types in the         Amazon EC2 User Guide.
    /// 
    /// Default: m1.small
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: a1.2xlarge | a1.4xlarge | a1.large | a1.medium | a1.metal | a1.xlarge | c1.medium | c1.xlarge | c3.2xlarge | c3.4xlarge | c3.8xlarge | c3.large | c3.xlarge | c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.metal | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c5ad.12xlarge | c5ad.16xlarge | c5ad.24xlarge | c5ad.2xlarge | c5ad.4xlarge | c5ad.8xlarge | c5ad.large | c5ad.xlarge | c5d.12xlarge | c5d.18xlarge | c5d.24xlarge | c5d.2xlarge | c5d.4xlarge | c5d.9xlarge | c5d.large | c5d.metal | c5d.xlarge | c5n.18xlarge | c5n.2xlarge | c5n.4xlarge | c5n.9xlarge | c5n.large | c5n.metal | c5n.xlarge | c6a.12xlarge | c6a.16xlarge | c6a.24xlarge | c6a.2xlarge | c6a.32xlarge | c6a.48xlarge | c6a.4xlarge | c6a.8xlarge | c6a.large | c6a.metal | c6a.xlarge | c6g.12xlarge | c6g.16xlarge | c6g.2xlarge | c6g.4xlarge | c6g.8xlarge | c6g.large | c6g.medium | c6g.metal | c6g.xlarge | c6gd.12xlarge | c6gd.16xlarge | c6gd.2xlarge | c6gd.4xlarge | c6gd.8xlarge | c6gd.large | c6gd.medium | c6gd.metal | c6gd.xlarge | c6gn.12xlarge | c6gn.16xlarge | c6gn.2xlarge | c6gn.4xlarge | c6gn.8xlarge | c6gn.large | c6gn.medium | c6gn.xlarge | c6i.12xlarge | c6i.16xlarge | c6i.24xlarge | c6i.2xlarge | c6i.32xlarge | c6i.4xlarge | c6i.8xlarge | c6i.large | c6i.metal | c6i.xlarge | c6id.12xlarge | c6id.16xlarge | c6id.24xlarge | c6id.2xlarge | c6id.32xlarge | c6id.4xlarge | c6id.8xlarge | c6id.large | c6id.metal | c6id.xlarge | c6in.12xlarge | c6in.16xlarge | c6in.24xlarge | c6in.2xlarge | c6in.32xlarge | c6in.4xlarge | c6in.8xlarge | c6in.large | c6in.metal | c6in.xlarge | c7g.12xlarge | c7g.16xlarge | c7g.2xlarge | c7g.4xlarge | c7g.8xlarge | c7g.large | c7g.medium | c7g.xlarge | cc1.4xlarge | cc2.8xlarge | cg1.4xlarge | cr1.8xlarge | d2.2xlarge | d2.4xlarge | d2.8xlarge | d2.xlarge | d3.2xlarge | d3.4xlarge | d3.8xlarge | d3.xlarge | d3en.12xlarge | d3en.2xlarge | d3en.4xlarge | d3en.6xlarge | d3en.8xlarge | d3en.xlarge | dl1.24xlarge | f1.16xlarge | f1.2xlarge | f1.4xlarge | g2.2xlarge | g2.8xlarge | g3.16xlarge | g3.4xlarge | g3.8xlarge | g3s.xlarge | g4ad.16xlarge | g4ad.2xlarge | g4ad.4xlarge | g4ad.8xlarge | g4ad.xlarge | g4dn.12xlarge | g4dn.16xlarge | g4dn.2xlarge | g4dn.4xlarge | g4dn.8xlarge | g4dn.metal | g4dn.xlarge | g5.12xlarge | g5.16xlarge | g5.24xlarge | g5.2xlarge | g5.48xlarge | g5.4xlarge | g5.8xlarge | g5.xlarge | g5g.16xlarge | g5g.2xlarge | g5g.4xlarge | g5g.8xlarge | g5g.metal | g5g.xlarge | h1.16xlarge | h1.2xlarge | h1.4xlarge | h1.8xlarge | hi1.4xlarge | hpc6a.48xlarge | hpc6id.32xlarge | hs1.8xlarge | i2.2xlarge | i2.4xlarge | i2.8xlarge | i2.xlarge | i3.16xlarge | i3.2xlarge | i3.4xlarge | i3.8xlarge | i3.large | i3.metal | i3.xlarge | i3en.12xlarge | i3en.24xlarge | i3en.2xlarge | i3en.3xlarge | i3en.6xlarge | i3en.large | i3en.metal | i3en.xlarge | i4g.16xlarge | i4g.2xlarge | i4g.4xlarge | i4g.8xlarge | i4g.large | i4g.xlarge | i4i.16xlarge | i4i.2xlarge | i4i.32xlarge | i4i.4xlarge | i4i.8xlarge | i4i.large | i4i.metal | i4i.xlarge | im4gn.16xlarge | im4gn.2xlarge | im4gn.4xlarge | im4gn.8xlarge | im4gn.large | im4gn.xlarge | inf1.24xlarge | inf1.2xlarge | inf1.6xlarge | inf1.xlarge | inf2.24xlarge | inf2.48xlarge | inf2.8xlarge | inf2.xlarge | is4gen.2xlarge | is4gen.4xlarge | is4gen.8xlarge | is4gen.large | is4gen.medium | is4gen.xlarge | m1.large | m1.medium | m1.small | m1.xlarge | m2.2xlarge | m2.4xlarge | m2.xlarge | m3.2xlarge | m3.large | m3.medium | m3.xlarge | m4.10xlarge | m4.16xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.metal | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | m5ad.12xlarge | m5ad.16xlarge | m5ad.24xlarge | m5ad.2xlarge | m5ad.4xlarge | m5ad.8xlarge | m5ad.large | m5ad.xlarge | m5d.12xlarge | m5d.16xlarge | m5d.24xlarge | m5d.2xlarge | m5d.4xlarge | m5d.8xlarge | m5d.large | m5d.metal | m5d.xlarge | m5dn.12xlarge | m5dn.16xlarge | m5dn.24xlarge | m5dn.2xlarge | m5dn.4xlarge | m5dn.8xlarge | m5dn.large | m5dn.metal | m5dn.xlarge | m5n.12xlarge | m5n.16xlarge | m5n.24xlarge | m5n.2xlarge | m5n.4xlarge | m5n.8xlarge | m5n.large | m5n.metal | m5n.xlarge | m5zn.12xlarge | m5zn.2xlarge | m5zn.3xlarge | m5zn.6xlarge | m5zn.large | m5zn.metal | m5zn.xlarge | m6a.12xlarge | m6a.16xlarge | m6a.24xlarge | m6a.2xlarge | m6a.32xlarge | m6a.48xlarge | m6a.4xlarge | m6a.8xlarge | m6a.large | m6a.metal | m6a.xlarge | m6g.12xlarge | m6g.16xlarge | m6g.2xlarge | m6g.4xlarge | m6g.8xlarge | m6g.large | m6g.medium | m6g.metal | m6g.xlarge | m6gd.12xlarge | m6gd.16xlarge | m6gd.2xlarge | m6gd.4xlarge | m6gd.8xlarge | m6gd.large | m6gd.medium | m6gd.metal | m6gd.xlarge | m6i.12xlarge | m6i.16xlarge | m6i.24xlarge | m6i.2xlarge | m6i.32xlarge | m6i.4xlarge | m6i.8xlarge | m6i.large | m6i.metal | m6i.xlarge | m6id.12xlarge | m6id.16xlarge | m6id.24xlarge | m6id.2xlarge | m6id.32xlarge | m6id.4xlarge | m6id.8xlarge | m6id.large | m6id.metal | m6id.xlarge | m6idn.12xlarge | m6idn.16xlarge | m6idn.24xlarge | m6idn.2xlarge | m6idn.32xlarge | m6idn.4xlarge | m6idn.8xlarge | m6idn.large | m6idn.metal | m6idn.xlarge | m6in.12xlarge | m6in.16xlarge | m6in.24xlarge | m6in.2xlarge | m6in.32xlarge | m6in.4xlarge | m6in.8xlarge | m6in.large | m6in.metal | m6in.xlarge | mac1.metal | mac2.metal | p2.16xlarge | p2.8xlarge | p2.xlarge | p3.16xlarge | p3.2xlarge | p3.8xlarge | p3dn.24xlarge | p4d.24xlarge | p4de.24xlarge | r3.2xlarge | r3.4xlarge | r3.8xlarge | r3.large | r3.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.metal | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r5ad.12xlarge | r5ad.16xlarge | r5ad.24xlarge | r5ad.2xlarge | r5ad.4xlarge | r5ad.8xlarge | r5ad.large | r5ad.xlarge | r5b.12xlarge | r5b.16xlarge | r5b.24xlarge | r5b.2xlarge | r5b.4xlarge | r5b.8xlarge | r5b.large | r5b.metal | r5b.xlarge | r5d.12xlarge | r5d.16xlarge | r5d.24xlarge | r5d.2xlarge | r5d.4xlarge | r5d.8xlarge | r5d.large | r5d.metal | r5d.xlarge | r5dn.12xlarge | r5dn.16xlarge | r5dn.24xlarge | r5dn.2xlarge | r5dn.4xlarge | r5dn.8xlarge | r5dn.large | r5dn.metal | r5dn.xlarge | r5n.12xlarge | r5n.16xlarge | r5n.24xlarge | r5n.2xlarge | r5n.4xlarge | r5n.8xlarge | r5n.large | r5n.metal | r5n.xlarge | r6a.12xlarge | r6a.16xlarge | r6a.24xlarge | r6a.2xlarge | r6a.32xlarge | r6a.48xlarge | r6a.4xlarge | r6a.8xlarge | r6a.large | r6a.metal | r6a.xlarge | r6g.12xlarge | r6g.16xlarge | r6g.2xlarge | r6g.4xlarge | r6g.8xlarge | r6g.large | r6g.medium | r6g.metal | r6g.xlarge | r6gd.12xlarge | r6gd.16xlarge | r6gd.2xlarge | r6gd.4xlarge | r6gd.8xlarge | r6gd.large | r6gd.medium | r6gd.metal | r6gd.xlarge | r6i.12xlarge | r6i.16xlarge | r6i.24xlarge | r6i.2xlarge | r6i.32xlarge | r6i.4xlarge | r6i.8xlarge | r6i.large | r6i.metal | r6i.xlarge | r6id.12xlarge | r6id.16xlarge | r6id.24xlarge | r6id.2xlarge | r6id.32xlarge | r6id.4xlarge | r6id.8xlarge | r6id.large | r6id.metal | r6id.xlarge | r6idn.12xlarge | r6idn.16xlarge | r6idn.24xlarge | r6idn.2xlarge | r6idn.32xlarge | r6idn.4xlarge | r6idn.8xlarge | r6idn.large | r6idn.metal | r6idn.xlarge | r6in.12xlarge | r6in.16xlarge | r6in.24xlarge | r6in.2xlarge | r6in.32xlarge | r6in.4xlarge | r6in.8xlarge | r6in.large | r6in.metal | r6in.xlarge | t1.micro | t2.2xlarge | t2.large | t2.medium | t2.micro | t2.nano | t2.small | t2.xlarge | t3.2xlarge | t3.large | t3.medium | t3.micro | t3.nano | t3.small | t3.xlarge | t3a.2xlarge | t3a.large | t3a.medium | t3a.micro | t3a.nano | t3a.small | t3a.xlarge | t4g.2xlarge | t4g.large | t4g.medium | t4g.micro | t4g.nano | t4g.small | t4g.xlarge | trn1.2xlarge | trn1.32xlarge | trn1n.32xlarge | u-12tb1.112xlarge | u-12tb1.metal | u-18tb1.112xlarge | u-18tb1.metal | u-24tb1.112xlarge | u-24tb1.metal | u-3tb1.56xlarge | u-6tb1.112xlarge | u-6tb1.56xlarge | u-6tb1.metal | u-9tb1.112xlarge | u-9tb1.metal | vt1.24xlarge | vt1.3xlarge | vt1.6xlarge | x1.16xlarge | x1.32xlarge | x1e.16xlarge | x1e.2xlarge | x1e.32xlarge | x1e.4xlarge | x1e.8xlarge | x1e.xlarge | x2gd.12xlarge | x2gd.16xlarge | x2gd.2xlarge | x2gd.4xlarge | x2gd.8xlarge | x2gd.large | x2gd.medium | x2gd.metal | x2gd.xlarge | x2idn.16xlarge | x2idn.24xlarge | x2idn.32xlarge | x2idn.metal | x2iedn.16xlarge | x2iedn.24xlarge | x2iedn.2xlarge | x2iedn.32xlarge | x2iedn.4xlarge | x2iedn.8xlarge | x2iedn.metal | x2iedn.xlarge | x2iezn.12xlarge | x2iezn.2xlarge | x2iezn.4xlarge | x2iezn.6xlarge | x2iezn.8xlarge | x2iezn.metal | z1d.12xlarge | z1d.2xlarge | z1d.3xlarge | z1d.6xlarge | z1d.large | z1d.metal | z1d.xlarge
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The IDs of the security groups. You can specify the IDs of existing security groups and       references to resources created by the stack template.
    /// 
    /// If you specify a network interface, you must specify any security groups as part of       the network interface.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// Enable or disable source/destination checks, which ensure that the instance       is either the source or the destination of any traffic that it receives.       If the value is true, source/destination checks are enabled;       otherwise, they are disabled. The default value is true.       You must disable source/destination checks if the instance runs services       such as network address translation, routing, or firewalls.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceDestCheck")]
    pub source_dest_check: Option<bool>,


    /// 
    /// This property is reserved for internal use. If you use it, the stack fails with this     error: Bad property set: [Testing this property] (Service: AmazonEC2; Status Code:       400; Error Code: InvalidParameterCombination; Request ID:       0XXXXXX-49c7-4b40-8bcc-76885dcXXXXX).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<String>,


    /// 
    /// Indicates whether the instance is associated with a dedicated host. If you want the     instance to always restart on the same host on which it was launched, specify       host. If you want the instance to restart on any available host, but try to     launch onto the last host it ran on (on a best-effort basis), specify     default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,


    /// 
    /// An elastic inference accelerator to associate with the instance. Elastic inference       accelerators are a resource you can attach to your Amazon EC2 instances to accelerate       your Deep Learning (DL) inference workloads.
    /// 
    /// You cannot specify accelerators from different generations in the same request.
    /// 
    /// NoteStarting April 15, 2023, AWS will not onboard new customers to Amazon       Elastic Inference (EI), and will help current customers migrate their workloads to       options that offer better price and performance. After April 15, 2023, new customers       will not be able to launch instances with Amazon EI accelerators in Amazon SageMaker,       Amazon ECS, or Amazon EC2. However, customers who have used Amazon EI at least once during       the past 30-day period are considered current customers and will be able to continue       using the service.
    /// 
    /// Required: No
    ///
    /// Type: List of ElasticInferenceAccelerator
    ///
    /// Update requires: Replacement
    #[serde(rename = "ElasticInferenceAccelerators")]
    pub elastic_inference_accelerators: Option<Vec<ElasticInferenceAccelerator>>,


    /// 
    /// The name of the key pair. You can create a key pair using CreateKeyPair or         ImportKeyPair.
    /// 
    /// ImportantIf you do not specify a key pair, you can't connect to the instance unless you         choose an AMI that is configured to allow users another way to log in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,


    /// 
    /// If you specify host for the Affinity property, the ID of a dedicated host     that the instance is associated with. If you don't specify an ID, Amazon EC2 launches the     instance onto any available, compatible dedicated host in your account. This type of launch     is called an untargeted launch. Note that for untargeted launches, you must have a     compatible, dedicated host available to successfully launch instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,


    /// 
    /// The ID of the RAM disk to select. Some kernels require additional drivers at launch.       Check the kernel requirements for information about whether you need to specify a RAM       disk. To find kernel requirements, go to the AWS Resource Center and       search for the kernel ID.
    /// 
    /// ImportantWe recommend that you use PV-GRUB instead of kernels and RAM disks. For more         information, see PV-GRUB in the           Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "RamdiskId")]
    pub ramdisk_id: Option<String>,


    /// 
    /// The network interfaces to associate with the instance.
    /// 
    /// NoteIf you use this property to point to a network interface, you must terminate the       original interface before attaching a new one to allow the update of the instance to       succeed.If this resource has a public IP address and is also in a VPC that is defined in the       same template, you must use the DependsOn        Attribute to declare a dependency on the VPC-gateway attachment.
    /// 
    /// Required: No
    ///
    /// Type: List of NetworkInterface
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,


    /// 
    /// The options for the instance hostname.
    /// 
    /// Required: No
    ///
    /// Type: PrivateDnsNameOptions
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PrivateDnsNameOptions")]
    pub private_dns_name_options: Option<PrivateDnsNameOptions>,


    /// 
    /// The credit option for CPU usage of the burstable performance instance. Valid values       are standard and unlimited. To change this attribute after       launch, use         ModifyInstanceCreditSpecification. For more information, see Burstable         performance instances in the Amazon EC2 User Guide.
    /// 
    /// Default: standard (T2 instances) or unlimited (T3/T3a/T4g       instances)
    /// 
    /// For T3 instances with host tenancy, only standard is       supported.
    /// 
    /// Required: No
    ///
    /// Type: CreditSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreditSpecification")]
    pub credit_specification: Option<CreditSpecification>,


    /// 
    /// The ID of the kernel.
    /// 
    /// ImportantWe recommend that you use PV-GRUB instead of kernels and RAM disks. For more         information, see PV-GRUB in the           Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,


    /// 
    /// An elastic GPU to associate with the instance. An Elastic GPU is a GPU resource that       you can attach to your Windows instance to accelerate the graphics performance of your       applications. For more information, see Amazon EC2 Elastic GPUs in       the Amazon EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ElasticGpuSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "ElasticGpuSpecifications")]
    pub elastic_gpu_specifications: Option<Vec<ElasticGpuSpecification>>,


    /// 
    /// Indicates whether an instance is enabled for hibernation. For more information, see         Hibernate         your instance in the Amazon EC2 User Guide.
    /// 
    /// You can't enable hibernation and AWS Nitro Enclaves on the same       instance.
    /// 
    /// Required: No
    ///
    /// Type: HibernationOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "HibernationOptions")]
    pub hibernation_options: Option<HibernationOptions>,


    /// 
    /// Indicates whether the instance is enabled for AWS Nitro       Enclaves.
    /// 
    /// Required: No
    ///
    /// Type: EnclaveOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnclaveOptions")]
    pub enclave_options: Option<EnclaveOptions>,


    /// 
    /// The block device mapping entries that defines the block devices to attach to the     instance at launch.
    /// 
    /// By default, the block devices specified in the block device mapping for the AMI are     used. You can override the AMI block device mapping using the instance block device     mapping. For the root volume, you can override only the volume size, volume type, volume     encryption settings, and the DeleteOnTermination setting.
    /// 
    /// ImportantAfter the instance is running, you can modify only the        DeleteOnTermination parameter for the attached volumes without       interrupting the instance. Modifying any other parameter results in instance replacement.
    /// 
    /// Required: No
    ///
    /// Type: List of BlockDeviceMapping
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,


    /// 
    /// The license configurations.
    /// 
    /// Required: No
    ///
    /// Type: List of LicenseSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "LicenseSpecifications")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,


    /// 
    /// The user data script to make available to the instance. User data is limited to 16 KB. 	   You must provide base64-encoded text. For more information, see Fn::Base64.
    /// 
    /// User data runs only at instance launch. For more information, see Run commands on your Linux instance at launch and Run commands on your Windows instance at launch.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,


    /// 
    /// The name of an IAM instance profile. To create a new IAM instance profile, use the       AWS::IAM::InstanceProfile resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<String>,


    /// 
    /// [Default VPC] The names of the security groups. For a nondefault VPC, you     must use security group IDs instead.
    /// 
    /// You cannot specify this option and the network interfaces option in the same request.     The list can contain both the name of existing Amazon EC2 security groups or references to     AWS::EC2::SecurityGroup resources created in the template.
    /// 
    /// Default: Amazon EC2 uses the default security group.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,


    /// 
    /// The volumes to attach to the instance.
    /// 
    /// Required: No
    ///
    /// Type: List of Volume
    ///
    /// Update requires: No interruption
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<Volume>>,


    /// 
    /// The IPv6 addresses from the range of the subnet to associate with the       primary network interface. You cannot specify this option and the option to assign a       number of IPv6 addresses in the same request. You cannot specify this option if you've       specified a minimum number of instances to launch.
    /// 
    /// You cannot specify this option and the network interfaces option in the same       request.
    /// 
    /// Required: No
    ///
    /// Type: List of InstanceIpv6Address
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,


    /// 
    /// Specifies whether detailed monitoring is enabled for the instance. Specify true to     enable detailed monitoring. Otherwise, basic monitoring is enabled. For more information     about detailed monitoring, see Enable or turn off detailed       monitoring for your instances in the Amazon EC2 User       Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Monitoring")]
    pub monitoring: Option<bool>,


    /// 
    /// The ID of the subnet to launch the instance into.
    /// 
    /// If you specify a network interface, you must specify any subnets as part of the       network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The ID of the AMI. An AMI ID is required to launch an instance and must be specified       here or in a launch template.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,


    /// 
    /// The ARN of the host resource group in which to launch the instances. If you specify a     host resource group ARN, omit the Tenancy parameter or set     it to host.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,


    /// 
    /// The launch template to use to launch the instances. Any parameters that you specify in     the AWS CloudFormation template override the same parameters in the launch template.     You can specify either the name or ID of a launch template, but not both.
    /// 
    /// Required: No
    ///
    /// Type: LaunchTemplateSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,


    /// 
    /// The number of IPv6 addresses to associate with the primary network       interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet. You       cannot specify this option and the option to assign specific IPv6 addresses in the same       request. You can specify this option if you've specified a minimum number of instances       to launch.
    /// 
    /// You cannot specify this option and the network interfaces option in the same       request.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<i64>,

}

impl cfn_resources::CfnResource for CfnInstance {
    fn type_string() -> &'static str {
        "AWS::EC2::Instance"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies a secondary private IPv4 address for a network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateIpAddressSpecification {


    /// 
    /// Indicates whether the private IPv4 address is the primary private IPv4 address. Only       one IPv4 address can be designated as primary.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Primary")]
    pub primary: bool,


    /// 
    /// The private IPv4 address.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: String,

}


/// Specifies the IPv6 address for the instance.
///
/// InstanceIpv6Address is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceIpv6Address {


    /// 
    /// The IPv6 address.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: String,

}


/// Specifies a network interface that is to be attached to an instance.
///
/// You can create a network interface when launching an instance. For an example, see the       AWS::EC2::Instance examples.
///
/// Alternatively, you can attach an existing network interface when launching an instance.     For an example, see the AWS::EC2:NetworkInterface examples.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInterface {


    /// 
    /// The IPv6 addresses to assign to the network interface. You cannot specify       this option and the option to assign a number of IPv6 addresses in the same request. You       cannot specify this option if you've specified a minimum number of instances to       launch.
    /// 
    /// Required: No
    ///
    /// Type: List of InstanceIpv6Address
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,


    /// 
    /// Indicates whether to assign a public IPv4 address to an instance. Applies only if     creating a network interface when launching an instance. The network interface must be the     primary network interface. If launching into a default subnet, the default value is       true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,


    /// 
    /// Indicates whether to assign a carrier IP address to the network interface.
    /// 
    /// You can only assign a carrier IP address to a network interface that is in a subnet in       a Wavelength Zone. For more information about carrier IP addresses, see Carrier IP address in the         AWS Wavelength Developer         Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociateCarrierIpAddress")]
    pub associate_carrier_ip_address: Option<bool>,


    /// 
    /// The description of the network interface. Applies only if creating a network interface when launching an instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The position of the network interface in the attachment order. A primary network     interface has a device index of 0.
    /// 
    /// If you create a network interface when launching an instance, you must specify the     device index.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceIndex")]
    pub device_index: String,


    /// 
    /// A number of IPv6 addresses to assign to the network interface. Amazon EC2 chooses       the IPv6 addresses from the range of the subnet. You cannot specify this option and the       option to assign specific IPv6 addresses in the same request. You can specify this       option if you've specified a minimum number of instances to launch.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<i64>,


    /// 
    /// The private IPv4 address of the network interface. Applies only if creating a network     interface when launching an instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,


    /// 
    /// Indicates whether the network interface is deleted when the instance is terminated.     Applies only if creating a network interface when launching an instance.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// One or more private IPv4 addresses to assign to the network interface. Only one private     IPv4 address can be designated as primary.
    /// 
    /// Required: No
    ///
    /// Type: List of PrivateIpAddressSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification>>,


    /// 
    /// The IDs of the security groups for the network interface. Applies only if creating a network interface when launching an instance.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupSet")]
    pub group_set: Option<Vec<String>>,


    /// 
    /// The ID of the network interface, when attaching an existing network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The ID of the subnet associated with the network interface. Applies only if creating a network interface when launching an instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The number of secondary private IPv4 addresses. You can't specify this option and     specify more than one private IP address using the private IP addresses option.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<i64>,

}


/// Suppresses the specified device included in the block device mapping of the AMI. To     suppress a device, specify an empty string.
///
/// NoDevice is a property of the       Amazon EC2 BlockDeviceMapping property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NoDevice {

}


/// Specifies a launch template to use when launching an Amazon EC2 instance.
///
/// You must specify the following:
///
/// LaunchTemplateSpecification is a property of the AWS::EC2::Instance resource.
///
/// For information about creating a launch template, see      AWS::EC2::LaunchTemplate and      Create a launch template     in the Amazon EC2 User Guide.
///
/// For examples of launch templates, see Examples.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateSpecification {


    /// 
    /// The ID of the launch template.
    /// 
    /// You must specify the LaunchTemplateId or the         LaunchTemplateName, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,


    /// 
    /// The name of the launch template.
    /// 
    /// You must specify the LaunchTemplateName or the         LaunchTemplateId, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,


    /// 
    /// The version number of the launch template.
    /// 
    /// Specifying $Latest or $Default for the template version number     is not supported. However, you can specify       LatestVersionNumber or DefaultVersionNumber using the       Fn::GetAtt intrinsic function. For more information, see Fn::GetAtt.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: String,

}


/// Specifies a block device mapping for an instance. You must specify exactly one of the     following properties: VirtualName, Ebs, or     NoDevice.
///
/// BlockDeviceMapping is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlockDeviceMapping {


    /// 
    /// The virtual device name (ephemeralN). The name must be in the form       ephemeralX where X is a number     starting from zero (0). For example, an instance type with 2 available instance store     volumes can specify mappings for ephemeral0 and ephemeral1. The     number of available instance store volumes depends on the instance type. After you connect     to the instance, you must mount the volume.
    /// 
    /// NVMe instance store volumes are automatically enumerated and assigned a device name.     Including them in your block device mapping has no effect.
    /// 
    /// Constraints: For M3 instances, you must specify instance store volumes     in the block device mapping for the instance. When you launch an M3 instance, we ignore any     instance store volumes specified in the block device mapping for the AMI.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,


    /// 
    /// To omit the device from the block device mapping, specify an empty string.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: NoDevice
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoDevice")]
    pub no_device: Option<NoDevice>,


    /// 
    /// The device name (for example, /dev/sdh or xvdh).
    /// 
    /// ImportantAfter the instance is running, this parameter is used to specify the device name of       the block device mapping to update.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceName")]
    pub device_name: String,


    /// 
    /// Parameters used to automatically set up EBS volumes when the instance is     launched.
    /// 
    /// ImportantAfter the instance is running, you can modify only the        DeleteOnTermination parameter for the attached volumes without       interrupting the instance. Modifying any other parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: Ebs
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ebs")]
    pub ebs: Option<Ebs>,

}


/// Specifies the license configuration to use.
///
/// LicenseSpecification is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LicenseSpecification {


    /// 
    /// The Amazon Resource Name (ARN) of the license configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,

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


/// Specifies a block device for an EBS volume.
///
/// Ebs is a property of the       Amazon EC2 BlockDeviceMapping property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ebs {


    /// 
    /// Indicates whether the volume should be encrypted. The effect of setting the encryption     state to true depends on the volume origin (new or from a snapshot), starting     encryption state, ownership, and whether encryption by default is enabled. For more     information, see Encryption by       default in the Amazon Elastic Compute Cloud User        Guide.
    /// 
    /// Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS     encryption. For more information, see Supported instance types.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// The ID of the snapshot.
    /// 
    /// If you specify both SnapshotId and VolumeSize,     VolumeSize must be equal or greater than the size of the snapshot.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// 
    /// The number of I/O operations per second (IOPS). For gp3, io1,     and io2 volumes, this represents the number of IOPS that are provisioned for     the volume. For gp2 volumes, this represents the baseline performance of the     volume and the rate at which the volume accumulates I/O credits for bursting.
    /// 
    /// The following are the supported values for each volume type:
    /// 
    /// gp3: 3,000-16,000 IOPS                    io1: 100-64,000 IOPS                    io2: 100-64,000 IOPS
    /// 
    /// For io1 and io2 volumes, we guarantee 64,000 IOPS only for     Instances built on       the Nitro System. Other instance families guarantee performance up to 32,000     IOPS.
    /// 
    /// This parameter is required for io1 and io2 volumes. The     default for gp3 volumes is 3,000 IOPS. This parameter is not supported for     gp2, st1, sc1, or standard     volumes.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size.     If you specify a snapshot, the default is the snapshot size. You can specify a volume size     that is equal to or larger than the snapshot size.
    /// 
    /// The following are the supported volumes sizes for each volume type:
    /// 
    /// gp2 and gp3:1-16,384                    io1 and io2: 4-16,384                    st1 and sc1: 125-16,384                    standard: 1-1,024
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// 
    /// Indicates whether the EBS volume is deleted on instance termination. For more     information, see Preserving Amazon EBS volumes on instance termination in the Amazon       EC2 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// The identifier of the AWS KMS key to use for Amazon EBS encryption. If     KmsKeyId is specified, the encrypted state must be true. If     the encrypted state is true but you do not specify KmsKeyId, your     KMS key for EBS is used.
    /// 
    /// You can specify the KMS key using any of the following:
    /// 
    /// Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.            Key alias. For example, alias/ExampleAlias.            Key ARN. For example,        arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab.            Alias ARN. For example,        arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The volume type. For more information, see Amazon EBS volume types in the     Amazon EC2 User Guide. If the volume type is io1 or     io2, you must specify the IOPS that the volume supports.
    /// 
    /// ImportantAfter the instance is running, modifying this parameter results in instance replacement.
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


/// Specifies the type of Elastic GPU. An Elastic GPU is a GPU resource that you can attach     to your Amazon EC2 instance to accelerate the graphics performance of your applications.     For more information, see Amazon EC2 Elastic GPUs     in the Amazon EC2 User Guide for Windows Instances.
///
/// ElasticGpuSpecification is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticGpuSpecification {


    /// 
    /// The type of Elastic Graphics accelerator. For more information about the values to specify for       Type, see Elastic Graphics Basics, specifically the Elastic Graphics accelerator column, in the       Amazon Elastic Compute Cloud User Guide for Windows Instances.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// Specifies the credit option for CPU usage of a T instance.
///
/// CreditSpecification is a property of the AWS::EC2::Instance resource.
///
/// For more information, see Burstable performance instances in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreditSpecification {


    /// 
    /// The credit option for CPU usage of the instance.
    /// 
    /// Valid values: standard | unlimited
    /// 
    /// T3 instances with host tenancy do not support the unlimited       CPU credit option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CPUCredits")]
    pub cpucredits: Option<String>,

}


/// Specifies the Elastic Inference Accelerator for the instance.
///
/// ElasticInferenceAccelerator is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticInferenceAccelerator {


    /// 
    /// The number of elastic inference accelerators to attach to the instance.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// 
    /// The type of elastic inference accelerator. The possible values are eia1.medium, eia1.large, eia1.xlarge, eia2.medium, eia2.large, and eia2.xlarge.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// Specifies the hibernation options for the instance.
///
/// HibernationOptions is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HibernationOptions {


    /// 
    /// If you set this parameter to true, your instance is enabled for       hibernation.
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


/// Specifies input parameter values for an SSM document in AWS Systems Manager.
///
/// AssociationParameter is a property of the Amazon EC2 Instance SsmAssociation property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssociationParameter {


    /// 
    /// The value of an input parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Vec<String>,


    /// 
    /// The name of an input parameter that is in the associated SSM document.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}


/// Specifies the SSM document and parameter values in AWS Systems Manager to associate     with an instance.
///
/// SsmAssociations is a property of the AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SsmAssociation {


    /// 
    /// The name of an SSM document to associate with the instance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentName")]
    pub document_name: String,


    /// 
    /// The input parameter values to use with the associated SSM document.
    /// 
    /// Required: No
    ///
    /// Type: List of AssociationParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociationParameters")]
    pub association_parameters: Option<Vec<AssociationParameter>>,

}


/// The type of hostnames to assign to instances in the subnet at launch. For IPv4 only subnets, an     instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance     DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS     names use the instance IPv4 address or the instance ID. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateDnsNameOptions {


    /// 
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsARecord")]
    pub enable_resource_name_dns_arecord: Option<bool>,


    /// 
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,


    /// 
    /// The type of hostnames to assign to instances in the subnet at launch. For IPv4 only subnets, an     instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance     DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS     names use the instance IPv4 address or the instance ID. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
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

}


/// Specifies a volume to attach to an instance.
///
/// Volume is an embedded property of the       AWS::EC2::Instance resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Volume {


    /// 
    /// The ID of the EBS volume. The volume and instance must be within the same Availability    Zone.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeId")]
    pub volume_id: String,


    /// 
    /// The device name (for example, /dev/sdh or xvdh).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Device")]
    pub device: String,

}


/// Specifies the CPU options for the instance. When you specify CPU options, you must     specify both the number of CPU cores and threads per core.
///
/// For more information, see Optimize CPU options in     the Amazon Elastic Compute Cloud User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CpuOptions {


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
    /// The number of threads per CPU core.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThreadsPerCore")]
    pub threads_per_core: Option<i64>,

}
