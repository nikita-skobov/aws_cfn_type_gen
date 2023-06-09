/// Contains one or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /24, /32, /48, /56, /64, and /128.
///
/// To specify an individual IP address, you specify the four-part IP address followed by a       /32, for example, 192.0.2.0/32. To block a range of IP addresses, you can     specify /8 or any range between /16 through /32 (for IPv4) or /24, /32, /48, /56, /64, or     /128 (for IPv6). For more information about CIDR notation, see the Wikipedia entry Classless       Inter-Domain Routing.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnIPSet {
    ///
    /// The IP address type (IPV4 or IPV6) and the IP address range (in CIDR notation) that web requests originate from. 			If the WebACL is associated with an Amazon CloudFront distribution and the viewer did not use an HTTP proxy or a load balancer to send the request, this is the value of the c-ip field in the CloudFront access logs.
    ///
    /// Required: No
    ///
    /// Type: List of IPSetDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPSetDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipset_descriptors: Option<Vec<IPSetDescriptor>>,

    ///
    /// The name of the IPSet. You can't change the name of an IPSet after you create it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnIPSet {
    fn type_string(&self) -> &'static str {
        "AWS::WAF::IPSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the IP address type (IPV4 or IPV6) and the IP address range (in CIDR format) that web requests originate from.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IPSetDescriptor {
    ///
    /// Specify IPV4 or IPV6.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: IPV4 | IPV6
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: IPSetDescriptorTypeEnum,

    ///
    /// Specify an IPv4 address by using CIDR notation. For example:
    ///
    /// To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify 192.0.2.44/32.               To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify 				192.0.2.0/24.
    ///
    /// For more information about CIDR notation, see the Wikipedia entry 	   Classless Inter-Domain Routing.
    ///
    /// Specify an IPv6 address by using CIDR notation. For example:
    ///
    /// To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify 1111:0000:0000:0000:0000:0000:0000:0111/128.               To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify 1111:0000:0000:0000:0000:0000:0000:0000/64.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IPSetDescriptorTypeEnum {
    /// IPV4
    #[serde(rename = "IPV4")]
    Ipv4,

    /// IPV6
    #[serde(rename = "IPV6")]
    Ipv6,
}

impl Default for IPSetDescriptorTypeEnum {
    fn default() -> Self {
        IPSetDescriptorTypeEnum::Ipv4
    }
}

impl cfn_resources::CfnResource for IPSetDescriptor {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
