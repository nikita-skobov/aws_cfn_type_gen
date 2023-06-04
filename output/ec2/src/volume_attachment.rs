/// Attaches an Amazon EBS volume to a running instance and exposes it to the instance with     the specified device name.
///
/// Before this resource can be deleted (and therefore the volume detached), you must first     unmount the volume in the instance. Failure to do so results in the volume being stuck in     the busy state while it is trying to detach, which could possibly damage the file system or     the data it contains.
///
/// If an Amazon EBS volume is the root device of an instance, it cannot be detached while     the instance is in the "running" state. To detach the root volume, stop the instance     first.
///
/// If the root volume is detached from an instance with an AWS Marketplace product     code, then the product codes from that volume are no longer associated with the     instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVolumeAttachment {
    ///
    /// The device name (for example, /dev/sdh or xvdh).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Device")]
    pub device: cfn_resources::StrVal,

    ///
    /// The ID of the instance to which the volume attaches. This value can be a reference to an       AWS::EC2::Instance resource, or it can be the physical ID of an     existing EC2 instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceId")]
    pub instance_id: cfn_resources::StrVal,

    ///
    /// The ID of the Amazon EBS volume. The volume and instance must be within the same     Availability Zone. This value can be a reference to an AWS::EC2::Volume resource, or it can be the volume ID of an     existing Amazon EBS volume.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeId")]
    pub volume_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnVolumeAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VolumeAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
