/// The AWS::AutoScaling::LaunchConfiguration resource specifies the launch    configuration that can be used by an Auto Scaling group to configure Amazon EC2 instances.
///
/// When you update the launch configuration for an Auto Scaling group, CloudFormation deletes    that resource and creates a new launch configuration with the updated properties and a new    name. Existing instances are not affected. To update existing instances when you update the     AWS::AutoScaling::LaunchConfiguration resource, you can specify an UpdatePolicy     attribute for the group. You can find sample update policies for rolling updates in     Auto scaling template     snippets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLaunchConfiguration {
    ///
    /// Specifies whether to assign a public IPv4 address to the group's instances. If the       instance is launched into a default subnet, the default is to assign a public IPv4       address, unless you disabled the option to assign a public IPv4 address on the subnet.       If the instance is launched into a nondefault subnet, the default is not to assign a       public IPv4 address, unless you enabled the option to assign a public IPv4 address on       the subnet.
    ///
    /// If you specify true, each instance in the Auto Scaling group receives a unique       public IPv4 address. For more information, see Launching Auto Scaling instances in a         VPC in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// If you specify this property, you must specify at least one subnet for         VPCZoneIdentifier when you create your group.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,

    ///
    /// The block device mapping entries that define the block devices to attach to the       instances at launch. By default, the block devices specified in the block device mapping       for the AMI are used. For more information, see Block device         mappings in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Required: No
    ///
    /// Type: List of BlockDeviceMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,

    ///
    /// Available for backward compatibility.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClassicLinkVPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpcid: Option<String>,

    ///
    /// Available for backward compatibility.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClassicLinkVPCSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpcsecurity_groups: Option<Vec<String>>,

    ///
    /// Specifies whether the launch configuration is optimized for EBS I/O       (true) or not (false). The optimization provides dedicated       throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O       performance. This optimization is not available with all instance types. Additional fees       are incurred when you enable EBS optimization for an instance type that is not       EBS-optimized by default. For more information, see Amazon EBS-optimized instances in       the Amazon EC2 User Guide for Linux Instances.
    ///
    /// The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,

    ///
    /// The name or the Amazon Resource Name (ARN) of the instance profile associated with the       IAM role for the instance. The instance profile contains the IAM role. For more       information, see IAM role for applications that run         on Amazon EC2 instances in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,

    ///
    /// The ID of the Amazon Machine Image (AMI) that was assigned during registration. For       more information, see Finding a Linux AMI in the         Amazon EC2 User Guide for Linux Instances.
    ///
    /// If you specify InstanceId, an ImageId is not       required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageId")]
    pub image_id: String,

    ///
    /// The ID of the Amazon EC2 instance to use to create the launch configuration. When you use    an instance to create a launch configuration, all properties are derived from the instance    with the exception of BlockDeviceMapping and     AssociatePublicIpAddress. You can override any properties from the instance by    specifying them in the launch configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    ///
    /// Controls whether instances in this group are launched with detailed       (true) or basic (false) monitoring.
    ///
    /// The default value is true (enabled).
    ///
    /// ImportantWhen detailed monitoring is enabled, Amazon CloudWatch generates metrics every minute and         your account is charged a fee. When you disable detailed monitoring, CloudWatch generates         metrics every 5 minutes. For more information, see Configure           Monitoring for Auto Scaling Instances in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<bool>,

    ///
    /// Specifies the instance type of the EC2 instance. For information about available       instance types, see Available         instance types in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// If you specify InstanceId, an InstanceType is not       required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

    ///
    /// The ID of the kernel associated with the AMI.
    ///
    /// NoteWe recommend that you use PV-GRUB instead of kernels and RAM disks. For more         information, see User provided           kernels in the Amazon EC2 User Guide for Linux           Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KernelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,

    ///
    /// The name of the key pair. For more information, see Amazon EC2 key pairs and Linux         instances in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,

    ///
    /// The name of the launch configuration. This name must be unique per Region per       account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,

    ///
    /// The metadata options for the instances. For more information, see Configuring the Instance Metadata Options in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: MetadataOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetadataOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<MetadataOptions>,

    ///
    /// The tenancy of the instance, either default or dedicated. An       instance with dedicated tenancy runs on isolated, single-tenant hardware       and can only be launched into a VPC. To launch dedicated instances into a shared tenancy       VPC (a VPC with the instance placement tenancy attribute set to default),       you must set the value of this property to dedicated. For more information,       see Configuring         instance tenancy with Amazon EC2 Auto Scaling in the       Amazon EC2 Auto Scaling User Guide.
    ///
    /// If you specify PlacementTenancy, you must specify at least one subnet for         VPCZoneIdentifier when you create your group.
    ///
    /// Valid values: default | dedicated
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlacementTenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<String>,

    ///
    /// The ID of the RAM disk to select.
    ///
    /// NoteWe recommend that you use PV-GRUB instead of kernels and RAM disks. For more         information, see User provided           kernels in the Amazon EC2 User Guide for Linux           Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RamDiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_disk_id: Option<String>,

    ///
    /// A list that contains the security groups to assign to the instances in the Auto Scaling    group. The list can contain both the IDs of existing security groups and references to SecurityGroup resources created in the template.
    ///
    /// For more information, see Control traffic to resources using     security groups in the Amazon Virtual Private Cloud User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The maximum hourly price to be paid for any Spot Instance launched to fulfill the       request. Spot Instances are launched when the price you specify exceeds the current Spot       price. For more information, see Request Spot         Instances for fault-tolerant and flexible applications in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Valid Range: Minimum value of 0.001
    ///
    /// NoteWhen you change your maximum price by creating a new launch configuration, running         instances will continue to run as long as the maximum price for those running         instances is higher than the current Spot price.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpotPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<String>,

    ///
    /// The Base64-encoded user data to make available to the launched EC2 instances. For more    information, see Instance metadata and user     data in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 21847
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

impl cfn_resources::CfnResource for CfnLaunchConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::AutoScaling::LaunchConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.launch_configuration_name {
            if the_val.len() > 255 as _ {
                return Err(format!("Max validation failed on field 'launch_configuration_name'. {} is greater than 255", the_val.len()));
            }
        }

        if let Some(the_val) = &self.launch_configuration_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'launch_configuration_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.metadata_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.user_data {
            if the_val.len() > 21847 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_data'. {} is greater than 21847",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// BlockDevice is a property of the EBS property of the AWS::AutoScaling::LaunchConfiguration BlockDeviceMapping property type that    describes an Amazon EBS volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlockDevice {
    ///
    /// Indicates whether the volume is deleted on instance termination. For Amazon EC2 Auto Scaling, the       default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,

    ///
    /// Specifies whether the volume should be encrypted. Encrypted EBS volumes can only be       attached to instances that support Amazon EBS encryption. For more information, see Supported instance types. If your AMI uses encrypted volumes, you can also       only launch it on supported instance types.
    ///
    /// NoteIf you are creating a volume from a snapshot, you cannot create an unencrypted         volume from an encrypted snapshot. Also, you cannot specify a KMS key ID when using         a launch configuration.If you enable encryption by default, the EBS volumes that you create are always         encrypted, either using the AWS managed KMS key or a customer-managed KMS key,         regardless of whether the snapshot was encrypted. For more information, see Use AWS KMS keys to encrypt Amazon EBS volumes in the           Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    ///
    /// The number of input/output (I/O) operations per second (IOPS) to provision for the volume.    For gp3 and io1 volumes, this represents the number of IOPS that are    provisioned for the volume. For gp2 volumes, this represents the baseline    performance of the volume and the rate at which the volume accumulates I/O credits for    bursting.
    ///
    /// The following are the supported values for each volume type:
    ///
    /// gp3: 3,000-16,000 IOPS              io1: 100-64,000 IOPS
    ///
    /// For io1 volumes, we guarantee 64,000 IOPS only for Instances built on the Nitro System. Other instance families guarantee performance    up to 32,000 IOPS.
    ///
    /// Iops is supported when the volume type is gp3 or io1    and required only when the volume type is io1. (Not used with     standard, gp2, st1, or sc1 volumes.)
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    ///
    /// The snapshot ID of the volume to use.
    ///
    /// You must specify either a VolumeSize or a SnapshotId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,

    ///
    /// The throughput (MiBps) to provision for a gp3 volume.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 125
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,

    ///
    /// The volume size, in GiBs. The following are the supported volumes sizes for each       volume type:
    ///
    /// gp2 and gp3: 1-16,384                        io1: 4-16,384                        st1 and sc1: 125-16,384                        standard: 1-1,024
    ///
    /// You must specify either a SnapshotId or a VolumeSize. If you       specify both SnapshotId and VolumeSize, the volume size must       be equal or greater than the size of the snapshot.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16384
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,

    ///
    /// The volume type. For more information, see Amazon EBS volume types in the         Amazon EC2 User Guide for Linux Instances.
    ///
    /// Valid values: standard | io1 | gp2 |         st1 | sc1 | gp3
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

impl cfn_resources::CfnResource for BlockDevice {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.throughput {
            if *the_val > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'throughput'. {} is greater than 1000",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.throughput {
            if *the_val < 125 as _ {
                return Err(format!(
                    "Min validation failed on field 'throughput'. {} is less than 125",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.volume_size {
            if *the_val > 16384 as _ {
                return Err(format!(
                    "Max validation failed on field 'volume_size'. {} is greater than 16384",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.volume_size {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'volume_size'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// BlockDeviceMapping specifies a block device mapping for the     BlockDeviceMappings property of the AWS::AutoScaling::LaunchConfiguration resource.
///
/// Each instance that is launched has an associated root device volume, either an Amazon EBS    volume or an instance store volume. You can use block device mappings to specify additional    EBS volumes or instance store volumes to attach to an instance when it is launched.
///
/// For more information, see Example block device mapping in the Amazon EC2 User Guide for Linux     Instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlockDeviceMapping {
    ///
    /// The device name assigned to the volume (for example, /dev/sdh or         xvdh). For more information, see Device naming on Linux         instances in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// NoteTo define a block device mapping, set the device name and exactly one of the         following properties: Ebs, NoDevice, or           VirtualName.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    pub device_name: String,

    ///
    /// Information to attach an EBS volume to an instance at launch.
    ///
    /// Required: No
    ///
    /// Type: BlockDevice
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ebs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<BlockDevice>,

    ///
    /// Setting this value to true prevents a volume that is included in the       block device mapping of the AMI from being mapped to the specified device name at       launch.
    ///
    /// If NoDevice is true for the root device, instances might       fail the EC2 health check. In that case, Amazon EC2 Auto Scaling launches replacement instances.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "NoDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<bool>,

    ///
    /// The name of the instance store volume (virtual device) to attach to an instance at       launch. The name must be in the form ephemeralX where         X is a number starting from zero (0), for example,         ephemeral0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

impl cfn_resources::CfnResource for BlockDeviceMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ebs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MetadataOptions is a property of AWS::AutoScaling::LaunchConfiguration that describes metadata options for the    instances.
///
/// For more information, see Configure the instance metadata options in the Amazon EC2 Auto Scaling     User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetadataOptions {
    ///
    /// This parameter enables or disables the HTTP metadata endpoint on your instances. If       the parameter is not specified, the default state is enabled.
    ///
    /// NoteIf you specify a value of disabled, you will not be able to access         your instance metadata.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: Replacement
    #[serde(rename = "HttpEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<MetadataOptionsHttpEndpointEnum>,

    ///
    /// The desired HTTP PUT response hop limit for instance metadata requests. The larger the       number, the further instance metadata requests can travel.
    ///
    /// Default: 1
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i64>,

    ///
    /// The state of token usage for your instance metadata requests. If the parameter is not       specified in the request, the default state is optional.
    ///
    /// If the state is optional, you can choose to retrieve instance metadata       with or without a signed token header on your request. If you retrieve the IAM role       credentials without a token, the version 1.0 role credentials are returned. If you       retrieve the IAM role credentials using a valid signed token, the version 2.0 role       credentials are returned.
    ///
    /// If the state is required, you must send a signed token header with any       instance metadata retrieval requests. In this state, retrieving the IAM role credentials       always returns the version 2.0 credentials; the version 1.0 credentials are not       available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: optional | required
    ///
    /// Update requires: Replacement
    #[serde(rename = "HttpTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<MetadataOptionsHttpTokensEnum>,
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

impl cfn_resources::CfnResource for MetadataOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.http_put_response_hop_limit {
            if *the_val > 64 as _ {
                return Err(format!("Max validation failed on field 'http_put_response_hop_limit'. {} is greater than 64", the_val));
            }
        }

        if let Some(the_val) = &self.http_put_response_hop_limit {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'http_put_response_hop_limit'. {} is less than 1", the_val));
            }
        }

        Ok(())
    }
}
