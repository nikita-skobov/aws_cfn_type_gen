/// Creates an instance in a specified stack. For more information, see Adding an     Instance to a Layer.
///
/// Required Permissions: To use this action, an IAM user must have a Manage permissions    level for the stack, or an attached policy that explicitly grants permissions. For more    information on user permissions, see Managing User     Permissions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstance {
    ///
    /// The default AWS OpsWorks Stacks agent version. You have the following options:
    ///
    /// INHERIT - Use the stack's default agent version setting.                        version_number - Use the specified agent version.     This value overrides the stack's default setting.     To update the agent version, edit the instance configuration and specify a     new version.       AWS OpsWorks Stacks installs that version on the instance.
    ///
    /// The default setting is INHERIT. To specify an agent version,    you must use the complete version number, not the abbreviated number shown on the console.    For a list of available agent version numbers, call DescribeAgentVersions.      AgentVersion cannot be set to Chef 12.2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<cfn_resources::StrVal>,

    ///
    /// A custom AMI ID to be used to create the instance. The AMI should be based on one of the    supported operating systems.    For more information, see    Using Custom AMIs.
    ///
    /// NoteIf you specify a custom AMI, you must set Os to Custom.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<cfn_resources::StrVal>,

    ///
    /// The instance architecture. The default option is x86_64. Instance types do not    necessarily support both architectures. For a list of the architectures that are supported by    the different instance types, see Instance Families and     Types.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: i386 | x86_64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<InstanceArchitectureEnum>,

    ///
    /// For load-based or time-based instances, the type. Windows stacks can use only time-based instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: load | timer
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<InstanceAutoScalingTypeEnum>,

    ///
    /// The Availability Zone of the AWS OpsWorks instance, such as     us-east-2a.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// An array of BlockDeviceMapping objects that specify the instance's block    devices. For more information, see Block     Device Mapping. Note that block device mappings are not supported for custom AMIs.
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
    /// Whether to create an Amazon EBS-optimized instance.
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
    /// A list of Elastic IP addresses to associate with the instance.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<Vec<String>>,

    ///
    /// The instance host name. The following are character limits for instance host names.
    ///
    /// Linux-based instances: 63 characters               Windows-based instances: 15 characters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<cfn_resources::StrVal>,

    ///
    /// Whether to install operating system and package updates when the instance boots. The default    value is true. To control when updates are installed, set this value to     false. You must then update your instances manually by using     CreateDeployment to run the update_dependencies stack command or    by manually running yum (Amazon Linux) or apt-get (Ubuntu) on the    instances.
    ///
    /// NoteWe strongly recommend using the default value of true to ensure that your     instances have the latest security updates.
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
    /// The instance type, such as t2.micro. For a list of supported instance types,    open the stack in the console, choose Instances, and choose + Instance.    The Size list contains the currently supported types.      For more information, see Instance     Families and Types. The parameter values that you use to specify the various types are    in the API Name column of the Available Instance Types table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,

    ///
    /// An array that contains the instance's layer IDs.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LayerIds")]
    pub layer_ids: Vec<String>,

    ///
    /// The instance's operating system, which must be set to one of the following.
    ///
    /// A supported Linux operating system: An Amazon Linux version, such as Amazon Linux 2, Amazon Linux 2018.03, Amazon Linux 2017.09, Amazon Linux 2017.03, Amazon Linux 2016.09,        Amazon Linux 2016.03, Amazon Linux 2015.09, or Amazon Linux 2015.03.               A supported Ubuntu operating system, such as Ubuntu 18.04 LTS, Ubuntu 16.04 LTS, Ubuntu 14.04 LTS, or Ubuntu 12.04 LTS.                        CentOS Linux 7                                Red Hat Enterprise Linux 7                       A supported Windows operating system, such as Microsoft Windows Server 2012 R2 Base, Microsoft Windows Server 2012 R2 with SQL Server Express, 			  Microsoft Windows Server 2012 R2 with SQL Server Standard, or Microsoft Windows Server 2012 R2 with SQL Server Web.               A custom AMI: Custom.
    ///
    /// Not all operating systems are supported with all versions of Chef. For more information about the supported operating systems,      see AWS OpsWorks Stacks Operating Systems.
    ///
    /// The default option is the current Amazon Linux version. If you set this parameter to     Custom, you must use the CreateInstance action's AmiId parameter to    specify the custom AMI that you want to use. Block device mappings are not supported if the value is Custom.      For more information about how to use custom AMIs with AWS OpsWorks Stacks, see Using     Custom AMIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<cfn_resources::StrVal>,

    ///
    /// The instance root device type. For more information, see Storage for the Root Device.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ebs | instance-store
    ///
    /// Update requires: Replacement
    #[serde(rename = "RootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<InstanceRootDeviceTypeEnum>,

    ///
    /// The instance's Amazon EC2 key-pair name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<cfn_resources::StrVal>,

    ///
    /// The stack ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackId")]
    pub stack_id: cfn_resources::StrVal,

    ///
    /// The ID of the instance's subnet. If the stack is running in a VPC, you can use this parameter to override the stack's      default subnet ID value and direct AWS OpsWorks Stacks to launch the instance in a different subnet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<cfn_resources::StrVal>,

    ///
    /// The instance's tenancy option. The default option is no tenancy, or if the instance is running in a VPC,      inherit tenancy settings from the VPC. The following are valid values for this parameter:       dedicated, default, or host. Because there are costs associated with changes      in tenancy options, we recommend that you research tenancy options before choosing them for your instances.      For more information about dedicated hosts, see      Dedicated Hosts Overview and      Amazon EC2 Dedicated Hosts.      For more information about dedicated instances, see      Dedicated Instances and      Amazon EC2 Dedicated Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<cfn_resources::StrVal>,

    ///
    /// The time-based scaling configuration for the instance.
    ///
    /// Required: No
    ///
    /// Type: TimeBasedAutoScaling
    ///
    /// Allowed values: load | timer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeBasedAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_auto_scaling: Option<TimeBasedAutoScaling>,

    ///
    /// The instance's virtualization type, paravirtual or hvm.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<cfn_resources::StrVal>,

    ///
    /// A list of AWS OpsWorks volume IDs to associate with the instance. For more     information, see AWS::OpsWorks::Volume.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_availability_zone: CfnInstanceavailabilityzone,

    #[serde(skip_serializing)]
    pub att_private_dns_name: CfnInstanceprivatednsname,

    #[serde(skip_serializing)]
    pub att_private_ip: CfnInstanceprivateip,

    #[serde(skip_serializing)]
    pub att_public_dns_name: CfnInstancepublicdnsname,

    #[serde(skip_serializing)]
    pub att_public_ip: CfnInstancepublicip,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceArchitectureEnum {
    /// i386
    #[serde(rename = "i386")]
    I386,

    /// x86_64
    #[serde(rename = "x86_64")]
    X8664,
}

impl Default for InstanceArchitectureEnum {
    fn default() -> Self {
        InstanceArchitectureEnum::I386
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceAutoScalingTypeEnum {
    /// load
    #[serde(rename = "load")]
    Load,

    /// timer
    #[serde(rename = "timer")]
    Timer,
}

impl Default for InstanceAutoScalingTypeEnum {
    fn default() -> Self {
        InstanceAutoScalingTypeEnum::Load
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceRootDeviceTypeEnum {
    /// ebs
    #[serde(rename = "ebs")]
    Ebs,

    /// instance-store
    #[serde(rename = "instance-store")]
    Instancestore,
}

impl Default for InstanceRootDeviceTypeEnum {
    fn default() -> Self {
        InstanceRootDeviceTypeEnum::Ebs
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceavailabilityzone;
impl CfnInstanceavailabilityzone {
    pub fn att_name(&self) -> &'static str {
        r#"AvailabilityZone"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceprivatednsname;
impl CfnInstanceprivatednsname {
    pub fn att_name(&self) -> &'static str {
        r#"PrivateDnsName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceprivateip;
impl CfnInstanceprivateip {
    pub fn att_name(&self) -> &'static str {
        r#"PrivateIp"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancepublicdnsname;
impl CfnInstancepublicdnsname {
    pub fn att_name(&self) -> &'static str {
        r#"PublicDnsName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancepublicip;
impl CfnInstancepublicip {
    pub fn att_name(&self) -> &'static str {
        r#"PublicIp"#
    }
}

impl cfn_resources::CfnResource for CfnInstance {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::Instance"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes a block device mapping. This data type maps directly to the Amazon EC2 BlockDeviceMapping data type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BlockDeviceMapping {
    ///
    /// The device name that is exposed to the instance, such as /dev/sdh. For the root    device, you can use the explicit device name or you can set this parameter to      ROOT_DEVICE and AWS OpsWorks Stacks will provide the correct device name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<cfn_resources::StrVal>,

    ///
    /// An EBSBlockDevice that defines how to configure an Amazon EBS volume when the       instance is launched. You can specify either the VirtualName or Ebs, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: EbsBlockDevice
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ebs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsBlockDevice>,

    ///
    /// Suppresses the specified device included in the AMI's block device mapping.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<cfn_resources::StrVal>,

    ///
    /// The virtual device name. For more information, see BlockDeviceMapping.       You can specify either the VirtualName or Ebs, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<cfn_resources::StrVal>,
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

/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EbsBlockDevice {
    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-deleteontermination
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-iops
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-snapshotid
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<cfn_resources::StrVal>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumesize
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumetype
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for EbsBlockDevice {
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

/// Describes an instance's time-based auto scaling configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimeBasedAutoScaling {
    ///
    /// The schedule for Friday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Friday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Monday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Monday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Saturday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Saturday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Sunday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sunday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Thursday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Thursday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Tuesday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tuesday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<std::collections::HashMap<String, String>>,

    ///
    /// The schedule for Wednesday.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Wednesday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for TimeBasedAutoScaling {
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
