/// The AWS::MediaLive::InputSecurityGroup is a MediaLive resource       type that creates an input security group.
///
/// A MediaLive input security group is associated with a MediaLive       input. The input security group is an "allow list" of IP addresses       that controls whether an external IP address can push content to the       associated MediaLive input.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInputSecurityGroup {
    ///
    /// A collection of tags for this input security group. Each tag is a       key-value pair.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// The list of IPv4 CIDR addresses to include in the input security       group as "allowed" addresses.
    ///
    /// Required: No
    ///
    /// Type: List of InputWhitelistRuleCidr
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhitelistRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnInputSecurityGrouparn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInputSecurityGrouparn;
impl CfnInputSecurityGrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnInputSecurityGroup {
    fn type_string(&self) -> &'static str {
        "AWS::MediaLive::InputSecurityGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An IPv4 CIDR range to include in this input security group.
///
/// The parent of this entity is InputSecurityGroup.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InputWhitelistRuleCidr {
    ///
    /// An IPv4 CIDR range to include in this input security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InputWhitelistRuleCidr {
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
