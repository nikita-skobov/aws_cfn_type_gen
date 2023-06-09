/// The AWS::Lightsail::Instance resource specifies an Amazon Lightsail instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnInstance {
    ///
    /// An array of add-ons for the instance.
    ///
    /// NoteIf the instance has an add-on enabled when performing a delete instance request, the       add-on is automatically disabled before the instance is deleted.
    ///
    /// Required: No
    ///
    /// Type: List of AddOn
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOn>>,

    ///
    /// The Availability Zone for the instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The blueprint ID for the instance (for example, os_amlinux_2016_03).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "BlueprintId")]
    pub blueprint_id: cfn_resources::StrVal,

    ///
    /// The bundle ID for the instance (for example, micro_1_0).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "BundleId")]
    pub bundle_id: cfn_resources::StrVal,

    ///
    /// The hardware properties for the instance, such as the vCPU count, attached disks, and     amount of RAM.
    ///
    /// ImportantThe instance restarts when performing an attach disk or detach disk request. This       resets the public IP address of your instance if a static IP isn't attached to       it.
    ///
    /// Required: No
    ///
    /// Type: Hardware
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<Hardware>,

    ///
    /// The name of the instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \w[\w\-]*\w
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceName")]
    pub instance_name: cfn_resources::StrVal,

    ///
    /// The name of the key pair to use for the instance.
    ///
    /// If no key pair name is specified, the Regional Lightsail default key     pair is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<cfn_resources::StrVal>,

    ///
    /// The location for the instance, such as the AWS Region and Availability     Zone.
    ///
    /// NoteThe Location property is read-only and should not be specified in a       create instance or update instance request.
    ///
    /// Required: No
    ///
    /// Type: Location
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    ///
    /// The public ports and the monthly amount of data transfer allocated for the     instance.
    ///
    /// Required: No
    ///
    /// Type: Networking
    ///
    /// Update requires: No interruption
    #[serde(rename = "Networking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<Networking>,

    ///
    /// The status code and the state (for example, running) of the     instance.
    ///
    /// NoteThe State property is read-only and should not be specified in a create       instance or update instance request.
    ///
    /// Required: No
    ///
    /// Type: State
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    ///
    /// NoteThe Value of Tags is optional for Lightsail resources.
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
    /// The optional launch script for the instance.
    ///
    /// Specify a launch script to configure an instance with additional user data. For example,     you might want to specify apt-get -y update as a launch script.
    ///
    /// NoteDepending on the blueprint of your instance, the command to get software on your       instance varies. Amazon Linux and CentOS use yum, Debian and Ubuntu use        apt-get, and FreeBSD uses pkg.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_instance_arn: CfnInstanceinstancearn,

    #[serde(skip_serializing)]
    pub att_location_availability_zone: CfnInstancelocationavailabilityzone,

    #[serde(skip_serializing)]
    pub att_location_region_name: CfnInstancelocationregionname,

    #[serde(skip_serializing)]
    pub att_networking_monthly_transfer_gb_per_month_allocated:
        CfnInstancenetworkingmonthlytransfergbpermonthallocated,

    #[serde(skip_serializing)]
    pub att_private_ip_address: CfnInstanceprivateipaddress,

    #[serde(skip_serializing)]
    pub att_public_ip_address: CfnInstancepublicipaddress,

    #[serde(skip_serializing)]
    pub att_resource_type: CfnInstanceresourcetype,

    #[serde(skip_serializing)]
    pub att_ssh_key_name: CfnInstancesshkeyname,

    #[serde(skip_serializing)]
    pub att_state_name: CfnInstancestatename,

    #[serde(skip_serializing)]
    pub att_support_code: CfnInstancesupportcode,

    #[serde(skip_serializing)]
    pub att_user_name: CfnInstanceusername,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceinstancearn;
impl CfnInstanceinstancearn {
    pub fn att_name(&self) -> &'static str {
        r#"InstanceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancelocationavailabilityzone;
impl CfnInstancelocationavailabilityzone {
    pub fn att_name(&self) -> &'static str {
        r#"Location.AvailabilityZone"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancelocationregionname;
impl CfnInstancelocationregionname {
    pub fn att_name(&self) -> &'static str {
        r#"Location.RegionName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancenetworkingmonthlytransfergbpermonthallocated;
impl CfnInstancenetworkingmonthlytransfergbpermonthallocated {
    pub fn att_name(&self) -> &'static str {
        r#"Networking.MonthlyTransfer.GbPerMonthAllocated"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceprivateipaddress;
impl CfnInstanceprivateipaddress {
    pub fn att_name(&self) -> &'static str {
        r#"PrivateIpAddress"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancepublicipaddress;
impl CfnInstancepublicipaddress {
    pub fn att_name(&self) -> &'static str {
        r#"PublicIpAddress"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceresourcetype;
impl CfnInstanceresourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancesshkeyname;
impl CfnInstancesshkeyname {
    pub fn att_name(&self) -> &'static str {
        r#"SshKeyName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancestatename;
impl CfnInstancestatename {
    pub fn att_name(&self) -> &'static str {
        r#"State.Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstancesupportcode;
impl CfnInstancesupportcode {
    pub fn att_name(&self) -> &'static str {
        r#"SupportCode"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceusername;
impl CfnInstanceusername {
    pub fn att_name(&self) -> &'static str {
        r#"UserName"#
    }
}

impl cfn_resources::CfnResource for CfnInstance {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::Instance"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hardware
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.networking
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.state.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// AddOn is a property of the AWS::Lightsail::Instance resource. It describes the add-ons for an     instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddOn {
    ///
    /// The add-on type (for example, AutoSnapshot).
    ///
    /// NoteAutoSnapshot is the only add-on that can be enabled for an       instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddOnType")]
    pub add_on_type: cfn_resources::StrVal,

    ///
    /// The parameters for the automatic snapshot add-on, such as the daily time when an     automatic snapshot will be created.
    ///
    /// Required: No
    ///
    /// Type: AutoSnapshotAddOn
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSnapshotAddOnRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_add_on_request: Option<AutoSnapshotAddOn>,

    ///
    /// The status of the add-on.
    ///
    /// Valid Values: Enabled | Disabled
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AddOnStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AddOnStatusEnum {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for AddOnStatusEnum {
    fn default() -> Self {
        AddOnStatusEnum::Enabled
    }
}

impl cfn_resources::CfnResource for AddOn {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_snapshot_add_on_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// AutoSnapshotAddOn is a property of the AddOn property. It describes the automatic snapshot add-on for an     instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AutoSnapshotAddOn {
    ///
    /// The daily time when an automatic snapshot will be created.
    ///
    /// Constraints:
    ///
    /// Must be in HH:00 format, and in an hourly increment.            Specified in Coordinated Universal Time (UTC).            The snapshot will be automatically created between the time specified and up to 45        minutes after.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotTimeOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time_of_day: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AutoSnapshotAddOn {
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

/// Disk is a property of the Hardware property. It describes a disk attached to an instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Disk {
    ///
    /// The resources to which the disk is attached.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AttachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<cfn_resources::StrVal>,

    ///
    /// (Deprecated) The attachment state of the disk.
    ///
    /// NoteIn releases prior to November 14, 2017, this parameter returned attached       for system disks in the API response. It is now deprecated, but still included in the       response. Use isAttached instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AttachmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<cfn_resources::StrVal>,

    ///
    /// The unique name of the disk.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DiskName")]
    pub disk_name: cfn_resources::StrVal,

    ///
    /// The input/output operations per second (IOPS) of the disk.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    ///
    /// A Boolean value indicating whether this disk is a system disk (has an operating system     loaded on it).
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IsSystemDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_disk: Option<bool>,

    ///
    /// The disk path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Path")]
    pub path: cfn_resources::StrVal,

    ///
    /// The size of the disk in GB.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "SizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Disk {
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

/// Hardware is a property of the AWS::Lightsail::Instance resource. It describes the hardware properties for the     instance, such as the vCPU count, attached disks, and amount of RAM.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Hardware {
    ///
    /// The number of vCPUs the instance has.
    ///
    /// NoteThe CpuCount property is read-only and should not be specified in a       create instance or update instance request.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,

    ///
    /// The disks attached to the instance.
    ///
    /// The instance restarts when performing an attach disk or detach disk request. This resets     the public IP address of your instance if a static IP isn't attached to it.
    ///
    /// Required: No
    ///
    /// Type: List of Disk
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,

    ///
    /// The amount of RAM in GB on the instance (for example, 1.0).
    ///
    /// NoteThe RamSizeInGb property is read-only and should not be specified in a       create instance or update instance request.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RamSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<i64>,
}

impl cfn_resources::CfnResource for Hardware {
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

/// Location is a property of the AWS::Lightsail::Instance resource. It describes the location for an     instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Location {
    ///
    /// The Availability Zone for the instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The name of the AWS Region for the instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Location {
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

/// MonthlyTransfer is a property of the Networking property. It describes the amount of allocated monthly data transfer     (in GB) for an instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MonthlyTransfer {
    ///
    /// The amount of allocated monthly data transfer (in GB) for an instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GbPerMonthAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_per_month_allocated: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MonthlyTransfer {
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

/// Networking is a property of the AWS::Lightsail::Instance resource. It describes the public ports and the     monthly amount of data transfer allocated for the instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Networking {
    ///
    /// The monthly amount of data transfer, in GB, allocated for the instance
    ///
    /// Required: No
    ///
    /// Type: MonthlyTransfer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "MonthlyTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_transfer: Option<MonthlyTransfer>,

    ///
    /// An array of ports to open on the instance.
    ///
    /// Required: Yes
    ///
    /// Type: List of Port
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,
}

impl cfn_resources::CfnResource for Networking {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.monthly_transfer
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Port is a property of the Networking property. It describes information about ports for an     instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Port {
    ///
    /// The access direction (inbound or outbound).
    ///
    /// NoteLightsail currently supports only inbound access       direction.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_direction: Option<cfn_resources::StrVal>,

    ///
    /// The location from which access is allowed. For example, Anywhere       (0.0.0.0/0), or Custom if a specific IP address or range of IP     addresses is allowed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_from: Option<cfn_resources::StrVal>,

    ///
    /// The type of access (Public or Private).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<cfn_resources::StrVal>,

    ///
    /// An alias that defines access for a preconfigured range of IP addresses.
    ///
    /// The only alias currently supported is lightsail-connect, which allows IP     addresses of the browser-based RDP/SSH client in the Lightsail console to     connect to your instance.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrListAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_list_aliases: Option<Vec<String>>,

    ///
    /// The IPv4 address, or range of IPv4 addresses (in CIDR notation) that are allowed to     connect to an instance through the ports, and the protocol.
    ///
    /// NoteThe ipv6Cidrs parameter lists the IPv6 addresses that are allowed to       connect to an instance.
    ///
    /// Examples:
    ///
    /// To allow the IP address 192.0.2.44, specify 192.0.2.44        or 192.0.2.44/32.            To allow the IP addresses 192.0.2.0 to 192.0.2.255,        specify 192.0.2.0/24.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,

    ///
    /// The common name of the port information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<cfn_resources::StrVal>,

    ///
    /// The first port in a range of open ports on an instance.
    ///
    /// Allowed ports:
    ///
    /// TCP and UDP - 0 to 65535            ICMP - The ICMP type for IPv4 addresses. For example, specify 8 as        the fromPort (ICMP type), and -1 as the toPort        (ICMP code), to enable ICMP Ping.            ICMPv6 - The ICMP type for IPv6 addresses. For example, specify 128        as the fromPort (ICMPv6 type), and 0 as toPort        (ICMPv6 code).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,

    ///
    /// The IPv6 address, or range of IPv6 addresses (in CIDR notation) that are allowed to     connect to an instance through the ports, and the protocol. Only devices with an IPv6     address can connect to an instance through IPv6; otherwise, IPv4 should be used.
    ///
    /// NoteThe cidrs parameter lists the IPv4 addresses that are allowed to connect       to an instance.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Cidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidrs: Option<Vec<String>>,

    ///
    /// The IP protocol name.
    ///
    /// The name can be one of the following:
    ///
    /// tcp - Transmission Control Protocol (TCP) provides reliable, ordered,        and error-checked delivery of streamed data between applications running on hosts        communicating by an IP network. If you have an application that doesn't require        reliable data stream service, use UDP instead.            all - All transport layer protocol types.            udp - With User Datagram Protocol (UDP), computer applications can        send messages (or datagrams) to other hosts on an Internet Protocol (IP) network.        Prior communications are not required to set up transmission channels or data paths.        Applications that don't require reliable data stream service can use UDP, which        provides a connectionless datagram service that emphasizes reduced latency over        reliability. If you do require reliable data stream service, use TCP instead.            icmp - Internet Control Message Protocol (ICMP) is used to send error        messages and operational information indicating success or failure when communicating        with an instance. For example, an error is indicated when an instance could not be        reached. When you specify icmp as the protocol, you must        specify the ICMP type using the fromPort parameter, and ICMP code using        the toPort parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// The last port in a range of open ports on an instance.
    ///
    /// Allowed ports:
    ///
    /// TCP and UDP - 0 to 65535            ICMP - The ICMP code for IPv4 addresses. For example, specify 8 as        the fromPort (ICMP type), and -1 as the toPort        (ICMP code), to enable ICMP Ping.            ICMPv6 - The ICMP code for IPv6 addresses. For example, specify 128        as the fromPort (ICMPv6 type), and 0 as toPort        (ICMPv6 code).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

impl cfn_resources::CfnResource for Port {
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

/// State is a property of the AWS::Lightsail::Instance resource. It describes the status code and the state     (for example, running) of an instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct State {
    ///
    /// The status code of the instance.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,

    ///
    /// The state of the instance (for example, running or     pending).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for State {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
