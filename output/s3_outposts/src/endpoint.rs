

/// This AWS::S3Outposts::Endpoint resource specifies an endpoint and associates it with the specified Outpost.
///
/// Amazon S3 on Outposts access points simplify managing data access at scale for shared    datasets in S3 on Outposts. S3 on Outposts uses endpoints to connect to S3 on Outposts buckets    so that you can perform actions within your virtual private cloud (VPC). For more information,    see     Accessing S3 on Outposts using VPC-only access points.
/// 
#[derive(Default, serde::Serialize)]
pub struct CfnEndpoint {


    /// 
    /// The ID of the security group to use with the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,


    /// 
    /// The ID of the Outpost.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,


    /// 
    /// The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses    are allocated from this pool for the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^ipv4pool-coip-([0-9a-f]{17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    pub customer_owned_ipv4_pool: Option<String>,


    /// 
    /// The container for the type of connectivity used to access the Amazon S3 on Outposts    endpoint. To use the Amazon VPC, choose Private. To use the endpoint    with an on-premises network, choose CustomerOwnedIp. If you choose     CustomerOwnedIp, you must also provide the customer-owned IP address pool (CoIP    pool).
    /// 
    /// NotePrivate is the default access type value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CustomerOwnedIp | Private
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessType")]
    pub access_type: Option<String>,


    /// 
    /// The ID of the subnet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,

}


/// The container for the network interface.
#[derive(Default, serde::Serialize)]
pub struct NetworkInterface {


    /// 
    /// The ID for the network interface.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

}