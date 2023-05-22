/// This AWS::S3Outposts::Endpoint resource specifies an endpoint and associates it with the specified Outpost.
///
/// Amazon S3 on Outposts access points simplify managing data access at scale for shared    datasets in S3 on Outposts. S3 on Outposts uses endpoints to connect to S3 on Outposts buckets    so that you can perform actions within your virtual private cloud (VPC). For more information,    see     Accessing S3 on Outposts using VPC-only access points.
///
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpoint {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<EndpointAccessTypeEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ipv4_pool: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the Outpost.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostId")]
    pub outpost_id: cfn_resources::StrVal,

    ///
    /// The ID of the security group to use with the endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: cfn_resources::StrVal,

    ///
    /// The ID of the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnEndpointarn,

    #[serde(skip_serializing)]
    pub att_cidr_block: CfnEndpointcidrblock,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnEndpointcreationtime,

    #[serde(skip_serializing)]
    pub att_id: CfnEndpointid,

    #[serde(skip_serializing)]
    pub att_status: CfnEndpointstatus,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EndpointAccessTypeEnum {
    /// CustomerOwnedIp
    #[serde(rename = "CustomerOwnedIp")]
    Customerownedip,

    /// Private
    #[serde(rename = "Private")]
    Private,
}

impl Default for EndpointAccessTypeEnum {
    fn default() -> Self {
        EndpointAccessTypeEnum::Customerownedip
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointarn;
impl CfnEndpointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointcidrblock;
impl CfnEndpointcidrblock {
    pub fn att_name(&self) -> &'static str {
        r#"CidrBlock"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointcreationtime;
impl CfnEndpointcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointid;
impl CfnEndpointid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointstatus;
impl CfnEndpointstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::S3Outposts::Endpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The container for the network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub network_interface_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NetworkInterface {
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
