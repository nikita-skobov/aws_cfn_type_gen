/// The AWS::Lightsail::Disk resource specifies a disk that can be attached to     an Amazon Lightsail instance that is in the same AWS Region     and Availability Zone.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDisk {
    ///
    /// An array of add-ons for the disk.
    ///
    /// NoteIf the disk has an add-on enabled when performing a delete disk request, the add-on       is automatically disabled before the disk is deleted.
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
    /// The AWS Region and Availability Zone location for the disk (for     example, us-east-1a).
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
    /// The name of the disk.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \w[\w\-]*\w
    ///
    /// Update requires: Replacement
    #[serde(rename = "DiskName")]
    pub disk_name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    ///
    /// The size of the disk in GB.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "SizeInGb")]
    pub size_in_gb: i64,

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

    #[serde(skip_serializing)]
    pub att_attached_to: CfnDiskattachedto,

    #[serde(skip_serializing)]
    pub att_attachment_state: CfnDiskattachmentstate,

    #[serde(skip_serializing)]
    pub att_disk_arn: CfnDiskdiskarn,

    #[serde(skip_serializing)]
    pub att_location_availability_zone: CfnDisklocationavailabilityzone,

    #[serde(skip_serializing)]
    pub att_location_region_name: CfnDisklocationregionname,

    #[serde(skip_serializing)]
    pub att_path: CfnDiskpath,

    #[serde(skip_serializing)]
    pub att_resource_type: CfnDiskresourcetype,

    #[serde(skip_serializing)]
    pub att_state: CfnDiskstate,

    #[serde(skip_serializing)]
    pub att_support_code: CfnDisksupportcode,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskattachedto;
impl CfnDiskattachedto {
    pub fn att_name(&self) -> &'static str {
        r#"AttachedTo"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskattachmentstate;
impl CfnDiskattachmentstate {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentState"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskdiskarn;
impl CfnDiskdiskarn {
    pub fn att_name(&self) -> &'static str {
        r#"DiskArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDisklocationavailabilityzone;
impl CfnDisklocationavailabilityzone {
    pub fn att_name(&self) -> &'static str {
        r#"Location.AvailabilityZone"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDisklocationregionname;
impl CfnDisklocationregionname {
    pub fn att_name(&self) -> &'static str {
        r#"Location.RegionName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskpath;
impl CfnDiskpath {
    pub fn att_name(&self) -> &'static str {
        r#"Path"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskresourcetype;
impl CfnDiskresourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDiskstate;
impl CfnDiskstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDisksupportcode;
impl CfnDisksupportcode {
    pub fn att_name(&self) -> &'static str {
        r#"SupportCode"#
    }
}

impl cfn_resources::CfnResource for CfnDisk {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::Disk"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// AddOn is a property of the AWS::Lightsail::Disk resource. It describes the add-ons for a disk.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddOn {
    ///
    /// The add-on type (for example, AutoSnapshot).
    ///
    /// NoteAutoSnapshot is the only add-on that can be enabled for a disk.
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

/// AutoSnapshotAddOn is a property of the AddOn property. It describes the automatic snapshot add-on for a disk.
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

/// The Location property type specifies Property description not available. for an AWS::Lightsail::Disk.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Location {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    /// Property description not available.
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
