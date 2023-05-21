

/// The AWS::Lightsail::Disk resource specifies a disk that can be attached to     an Amazon Lightsail instance that is in the same AWS Region     and Availability Zone.
#[derive(Default, serde::Serialize)]
pub struct CfnDisk {


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
    pub tags: Option<Vec<Tag>>,


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
    pub add_ons: Option<Vec<AddOn>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<Location>,


    /// 
    /// The AWS Region and Availability Zone location for the disk (for     example, us-east-1a).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


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
    pub disk_name: String,


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

}


/// The Location property type specifies Property description not available. for an AWS::Lightsail::Disk.
#[derive(Default, serde::Serialize)]
pub struct Location {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionName")]
    pub region_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

}


/// AddOn is a property of the AWS::Lightsail::Disk resource. It describes the add-ons for a disk.
#[derive(Default, serde::Serialize)]
pub struct AddOn {


    /// 
    /// The parameters for the automatic snapshot add-on, such as the daily time when an     automatic snapshot will be created.
    /// 
    /// Required: No
    ///
    /// Type: AutoSnapshotAddOn
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSnapshotAddOnRequest")]
    pub auto_snapshot_add_on_request: Option<AutoSnapshotAddOn>,


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
    pub add_on_type: String,


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
    pub status: Option<String>,

}


/// AutoSnapshotAddOn is a property of the AddOn property. It describes the automatic snapshot add-on for a disk.
#[derive(Default, serde::Serialize)]
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
    pub snapshot_time_of_day: Option<String>,

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
