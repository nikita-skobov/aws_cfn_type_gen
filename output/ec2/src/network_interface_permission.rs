

/// Specifies a permission for an Amazon EC2 network interface. For example, you can grant       an AWS authorized partner account permission to attach the specified       network interface to an instance in their account.
#[derive(Default, serde::Serialize)]
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
    pub permission: String,


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

}
