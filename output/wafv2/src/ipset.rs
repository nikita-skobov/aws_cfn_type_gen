/// Use an AWS::WAFv2::IPSet to identify web requests that originate from specific IP addresses or ranges of IP addresses. For example, if you're receiving a lot of requests from a ranges of IP addresses, you can configure AWS WAF to block them using an IP set that lists those IP addresses.
///
/// You use an IP set by providing its Amazon Resource Name (ARN) to the rule statement IPSetReferenceStatement, when you add a rule to a rule group or web ACL.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnIPSet {
    ///
    /// Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports all IPv4 and IPv6 CIDR ranges except for /0.
    ///
    /// Example address strings:
    ///
    /// To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify 192.0.2.44/32.               To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify         192.0.2.0/24.               To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify 1111:0000:0000:0000:0000:0000:0000:0111/128.               To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify 1111:0000:0000:0000:0000:0000:0000:0000/64.
    ///
    /// For more information about CIDR notation, see the Wikipedia entry Classless Inter-Domain Routing.
    ///
    /// Example JSON Addresses specifications:
    ///
    /// Empty array: "Addresses": []                       Array with one address: "Addresses": ["192.0.2.44/32"]                       Array with three addresses: "Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]                       INVALID specification: "Addresses": [""] INVALID
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,

    ///
    /// A description of the IP set that helps with identification.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\w+=:#@/\-,\.][\w+=:#@/\-,\.\s]+[\w+=:#@/\-,\.]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The version of the IP addresses, either IPV4 or IPV6.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: IPV4 | IPV6
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPAddressVersion")]
    pub ipaddress_version: IPSetIPAddressVersionEnum,

    ///
    /// The name of the IP set. You cannot change the name of an IPSet after you create it.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\w\-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether this is for an Amazon CloudFront distribution or for a regional application. A regional      application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, an      AWS AppSync GraphQL API, an Amazon Cognito user pool, or an AWS App Runner service. Valid Values are CLOUDFRONT and REGIONAL.
    ///
    /// NoteFor CLOUDFRONT, you must create your WAFv2 resources in the US East (N. Virginia) Region, us-east-1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Scope")]
    pub scope: cfn_resources::StrVal,

    ///
    /// Key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each AWS resource.
    ///
    /// NoteTo modify tags on existing resources, use the AWS WAF APIs or command line interface. With AWS CloudFormation, you can only add tags to AWS WAF resources during resource creation.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnIPSetarn,

    #[serde(skip_serializing)]
    pub att_id: CfnIPSetid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IPSetIPAddressVersionEnum {
    /// IPV4
    #[serde(rename = "IPV4")]
    Ipv4,

    /// IPV6
    #[serde(rename = "IPV6")]
    Ipv6,
}

impl Default for IPSetIPAddressVersionEnum {
    fn default() -> Self {
        IPSetIPAddressVersionEnum::Ipv4
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPSetarn;
impl CfnIPSetarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPSetid;
impl CfnIPSetid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnIPSet {
    fn type_string(&self) -> &'static str {
        "AWS::WAFv2::IPSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

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
