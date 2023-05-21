/// Specifies a permission for an Amazon EC2 network interface. For example, you can grant       an AWS authorized partner account permission to attach the specified       network interface to an instance in their account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNetworkInterfacePermission {
    ///
    /// The AWS account ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,

    ///
    /// The ID of the network interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

    ///
    /// The type of permission to grant: INSTANCE-ATTACH or       EIP-ASSOCIATE.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EIP-ASSOCIATE | INSTANCE-ATTACH
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permission")]
    pub permission: NetworkInterfacePermissionPermissionEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkInterfacePermissionPermissionEnum {
    /// EIP-ASSOCIATE
    #[serde(rename = "EIP-ASSOCIATE")]
    Eipassociate,

    /// INSTANCE-ATTACH
    #[serde(rename = "INSTANCE-ATTACH")]
    Instanceattach,
}

impl Default for NetworkInterfacePermissionPermissionEnum {
    fn default() -> Self {
        NetworkInterfacePermissionPermissionEnum::Eipassociate
    }
}

impl cfn_resources::CfnResource for CfnNetworkInterfacePermission {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkInterfacePermission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
