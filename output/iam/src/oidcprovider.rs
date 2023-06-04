/// Creates or updates an IAM entity to describe an identity provider (IdP)     that supports OpenID Connect (OIDC).
///
/// The OIDC provider that you create with this operation can be used as a principal in a     role's trust policy. Such a policy establishes a trust relationship between AWS and the OIDC provider.
///
/// When you create the IAM OIDC provider, you specify the     following:
///
/// You get all of this information from the OIDC IdP that you want to use to access AWS.
///
/// When you update the IAM OIDC provider, you specify the     following:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOIDCProvider {
    ///
    /// A list of client IDs (also known as audiences) that are associated with the specified       IAM OIDC provider resource object. For more information, see CreateOpenIDConnectProvider.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientIdList")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub client_id_list: Option<Vec<String>>,

    ///
    /// A list of tags that are attached to the specified IAM OIDC provider.     The returned list of tags is sorted by tag key. For more information about tagging, see       Tagging IAM resources in the IAM User       Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A list of certificate thumbprints that are associated with the specified IAM OIDC provider resource object. For more information, see CreateOpenIDConnectProvider.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThumbprintList")]
    pub thumbprint_list: Vec<String>,

    ///
    /// The URL that the IAM OIDC provider resource object is associated with.     For more information, see CreateOpenIDConnectProvider.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub url: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnOIDCProviderarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOIDCProviderarn;
impl CfnOIDCProviderarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnOIDCProvider {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::OIDCProvider"
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
