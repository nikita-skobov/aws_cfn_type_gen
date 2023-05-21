/// Describes an instance's Amazon EBS volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl cfn_resources::CfnResource for CfnVolume {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::Volume"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
