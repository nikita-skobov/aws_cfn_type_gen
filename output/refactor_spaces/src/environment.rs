/// Creates an AWS Migration Hub Refactor Spaces environment. The caller owns the environment resource, and all    Refactor Spaces applications, services, and routes created within the environment. They are referred    to as the environment owner. The environment owner has cross-account    visibility and control of Refactor Spaces resources that are added to the environment by other    accounts that the environment is shared with.
///
/// When creating an environment with a CreateEnvironment:NetworkFabricType of TRANSIT_GATEWAY, Refactor Spaces    provisions a transit gateway to enable services in VPCs to communicate directly across    accounts. If CreateEnvironment:NetworkFabricType is NONE, Refactor Spaces does not create    a transit gateway and you must use your network infrastructure to route traffic to services    with private URL endpoints.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEnvironment {
    ///
    /// A description of the environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The network fabric type of the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkFabricType")]
    pub network_fabric_type: cfn_resources::StrVal,

    ///
    /// The tags assigned to the environment.
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
    pub att_arn: CfnEnvironmentarn,

    #[serde(skip_serializing)]
    pub att_environment_identifier: CfnEnvironmentenvironmentidentifier,

    #[serde(skip_serializing)]
    pub att_transit_gateway_id: CfnEnvironmenttransitgatewayid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEnvironmentarn;
impl CfnEnvironmentarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEnvironmentenvironmentidentifier;
impl CfnEnvironmentenvironmentidentifier {
    pub fn att_name(&self) -> &'static str {
        r#"EnvironmentIdentifier"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEnvironmenttransitgatewayid;
impl CfnEnvironmenttransitgatewayid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayId"#
    }
}

impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string(&self) -> &'static str {
        "AWS::RefactorSpaces::Environment"
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
