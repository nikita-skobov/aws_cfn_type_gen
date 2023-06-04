/// Specifies a VPN connection between a virtual private gateway and a VPN customer gateway     or a transit gateway and a VPN customer gateway.
///
/// To specify a VPN connection between a transit gateway and customer gateway, use the     TransitGatewayId and CustomerGatewayId properties.
///
/// To specify a VPN connection between a virtual private gateway and customer gateway, use     the VpnGatewayId and CustomerGatewayId properties.
///
/// For more information, see AWS Site-to-Site VPN in the     AWS Site-to-Site VPN User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVPNConnection {
    ///
    /// The ID of the customer gateway at your end of the VPN connection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomerGatewayId")]
    pub customer_gateway_id: cfn_resources::StrVal,

    ///
    /// Indicates whether the VPN connection uses static routes only. Static routes must be used     for devices that don't support BGP.
    ///
    /// If you are creating a VPN connection for a device that does not support Border Gateway     Protocol (BGP), you must specify true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StaticRoutesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_routes_only: Option<bool>,

    ///
    /// Any tags assigned to the VPN connection.
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
    /// The ID of the transit gateway associated with the VPN connection.
    ///
    /// You must specify either TransitGatewayId or VpnGatewayId, but     not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The type of VPN connection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ipsec.1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: VPNConnectionTypeEnum,

    ///
    /// The ID of the virtual private gateway at the AWS side of the VPN     connection.
    ///
    /// You must specify either TransitGatewayId or VpnGatewayId, but     not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpnGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The tunnel options for the VPN connection.
    ///
    /// Required: No
    ///
    /// Type: List of VpnTunnelOptionsSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpnTunnelOptionsSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_tunnel_options_specifications: Option<Vec<VpnTunnelOptionsSpecification>>,

    #[serde(skip_serializing)]
    pub att_vpn_connection_id: CfnVPNConnectionvpnconnectionid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VPNConnectionTypeEnum {
    /// ipsec.1
    #[serde(rename = "ipsec.1")]
    Ipsec1,
}

impl Default for VPNConnectionTypeEnum {
    fn default() -> Self {
        VPNConnectionTypeEnum::Ipsec1
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVPNConnectionvpnconnectionid;
impl CfnVPNConnectionvpnconnectionid {
    pub fn att_name(&self) -> &'static str {
        r#"VpnConnectionId"#
    }
}

impl cfn_resources::CfnResource for CfnVPNConnection {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPNConnection"
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

/// The tunnel options for a single VPN tunnel.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct VpnTunnelOptionsSpecification {
    ///
    /// The pre-shared key (PSK) to establish initial authentication between the virtual       private gateway and customer gateway.
    ///
    /// Constraints: Allowed characters are alphanumeric characters, periods (.), and       underscores (_). Must be between 8 and 64 characters in length and cannot start with       zero (0).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreSharedKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_shared_key: Option<cfn_resources::StrVal>,

    ///
    /// The range of inside IP addresses for the tunnel. Any specified CIDR blocks must be     unique across all VPN connections that use the same virtual private gateway.
    ///
    /// Constraints: A size /30 CIDR block from the 169.254.0.0/16 range. The     following CIDR blocks are reserved and cannot be used:
    ///
    /// 169.254.0.0/30                          169.254.1.0/30                          169.254.2.0/30                          169.254.3.0/30                          169.254.4.0/30                          169.254.5.0/30                          169.254.169.252/30
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TunnelInsideCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_inside_cidr: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpnTunnelOptionsSpecification {
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
