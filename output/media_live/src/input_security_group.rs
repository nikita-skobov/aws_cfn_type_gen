/// The AWS::MediaLive::InputSecurityGroup is a MediaLive resource       type that creates an input security group.
///
/// A MediaLive input security group is associated with a MediaLive       input. The input security group is an "allow list" of IP addresses       that controls whether an external IP address can push content to the       associated MediaLive input.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
}

impl cfn_resources::CfnResource for CfnInputSecurityGroup {
    fn type_string(&self) -> &'static str {
        "AWS::MediaLive::InputSecurityGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An IPv4 CIDR range to include in this input security group.
///
/// The parent of this entity is InputSecurityGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub cidr: Option<String>,
}

impl cfn_resources::CfnResource for InputWhitelistRuleCidr {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
