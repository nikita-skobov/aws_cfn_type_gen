

/// Describes an instance's Amazon EBS volume.
#[derive(Default, serde::Serialize)]
pub struct CfnVolume {


    /// 
    /// The Amazon EC2 volume ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2VolumeId")]
    pub ec2_volume_id: String,


    /// 
    /// The volume mount point. For example, "/mnt/disk1".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPoint")]
    pub mount_point: Option<String>,


    /// 
    /// The volume name. Volume names are a maximum of 128 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


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

}