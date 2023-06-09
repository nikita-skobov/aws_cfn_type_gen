/// Specifies a customer gateway.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCustomerGateway {
    ///
    /// For devices that support BGP, the customer gateway's BGP ASN.
    ///
    /// Default: 65000
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "BgpAsn")]
    pub bgp_asn: i64,

    ///
    /// The name of customer gateway device.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<cfn_resources::StrVal>,

    ///
    /// IPv4 address for the customer gateway device's outside interface. The address must be static.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddress")]
    pub ip_address: cfn_resources::StrVal,

    ///
    /// One or more tags for the customer gateway.
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
    /// The type of VPN connection that this customer gateway supports       (ipsec.1).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ipsec.1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: CustomerGatewayTypeEnum,

    #[serde(skip_serializing)]
    pub att_customer_gateway_id: CfnCustomerGatewaycustomergatewayid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CustomerGatewayTypeEnum {
    /// ipsec.1
    #[serde(rename = "ipsec.1")]
    Ipsec1,
}

impl Default for CustomerGatewayTypeEnum {
    fn default() -> Self {
        CustomerGatewayTypeEnum::Ipsec1
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCustomerGatewaycustomergatewayid;
impl CfnCustomerGatewaycustomergatewayid {
    pub fn att_name(&self) -> &'static str {
        r#"CustomerGatewayId"#
    }
}

impl cfn_resources::CfnResource for CfnCustomerGateway {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::CustomerGateway"
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
