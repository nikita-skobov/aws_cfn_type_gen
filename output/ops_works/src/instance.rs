

/// Creates an instance in a specified stack. For more information, see Adding an     Instance to a Layer.
///
/// Required Permissions: To use this action, an IAM user must have a Manage permissions    level for the stack, or an attached policy that explicitly grants permissions. For more    information on user permissions, see Managing User     Permissions.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub agent_version: Option<String>,


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
    pub ami_id: Option<String>,


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
    pub availability_zone: Option<String>,


    /// 
    /// An array of BlockDeviceMapping objects that specify the instance's block    devices. For more information, see Block     Device Mapping. Note that block device mappings are not supported for custom AMIs.
    /// 
    /// Required: No
    ///
    /// Type: List of BlockDeviceMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "BlockDeviceMappings")]
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
    pub hostname: Option<String>,


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
    pub instance_type: String,


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
    pub os: Option<String>,


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
    pub ssh_key_name: Option<String>,


    /// 
    /// The stack ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackId")]
    pub stack_id: String,


    /// 
    /// The ID of the instance's subnet. If the stack is running in a VPC, you can use this parameter to override the stack's      default subnet ID value and direct AWS OpsWorks Stacks to launch the instance in a different subnet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The instance's tenancy option. The default option is no tenancy, or if the instance is running in a VPC,      inherit tenancy settings from the VPC. The following are valid values for this parameter:       dedicated, default, or host. Because there are costs associated with changes      in tenancy options, we recommend that you research tenancy options before choosing them for your instances.      For more information about dedicated hosts, see      Dedicated Hosts Overview and      Amazon EC2 Dedicated Hosts.      For more information about dedicated instances, see      Dedicated Instances and      Amazon EC2 Dedicated Instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,


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
    pub virtualization_type: Option<String>,


    /// 
    /// A list of AWS OpsWorks volume IDs to associate with the instance. For more     information, see AWS::OpsWorks::Volume.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<String>>,

}


#[derive(Clone, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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


impl cfn_resources::CfnResource for CfnInstance {
    fn type_string() -> &'static str {
        "AWS::OpsWorks::Instance"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes a block device mapping. This data type maps directly to the Amazon EC2 BlockDeviceMapping data type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_name: Option<String>,


    /// 
    /// An EBSBlockDevice that defines how to configure an Amazon EBS volume when the       instance is launched. You can specify either the VirtualName or Ebs, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: EbsBlockDevice
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ebs")]
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
    pub no_device: Option<String>,


    /// 
    /// The virtual device name. For more information, see BlockDeviceMapping.       You can specify either the VirtualName or Ebs, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,

}




/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbsBlockDevice {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-deleteontermination
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-iops
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-snapshotid
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumesize
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumetype
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,

}




/// Describes an instance's time-based auto scaling configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub wednesday: Option<std::collections::HashMap<String, String>>,

}


