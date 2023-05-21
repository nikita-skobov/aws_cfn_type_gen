

/// Associate an identity provider configuration to a cluster.
///
/// If you want to authenticate identities using an identity provider, you can create an       identity provider configuration and associate it to your cluster. After configuring       authentication to your cluster you can create Kubernetes roles and         clusterroles to assign permissions to the roles, and then bind the       roles to the identities using Kubernetes rolebindings and         clusterrolebindings. For more information see Using RBAC         Authorization in the Kubernetes documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIdentityProviderConfig {


    /// 
    /// The cluster that the configuration is associated to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,


    /// 
    /// The name of the configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityProviderConfigName")]
    pub identity_provider_config_name: Option<String>,


    /// 
    /// An object representing an OpenID Connect (OIDC) identity provider       configuration.
    /// 
    /// Required: No
    ///
    /// Type: OidcIdentityProviderConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Oidc")]
    pub oidc: Option<OidcIdentityProviderConfig>,


    /// 
    /// The metadata to apply to the provider configuration to assist with categorization and       organization. Each tag consists of a key and an optional value. You define both.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The type of the identity provider configuration. The only type available is         oidc.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

}



impl cfn_resources::CfnResource for CfnIdentityProviderConfig {
    fn type_string(&self) -> &'static str {
        "AWS::EKS::IdentityProviderConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.oidc.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object representing the configuration for an OpenID Connect (OIDC) identity       provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OidcIdentityProviderConfig {


    /// 
    /// This is also known as audience. The ID of the client application       that makes authentication requests to the OIDC identity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// The JSON web token (JWT) claim that the provider uses to return your groups.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupsClaim")]
    pub groups_claim: Option<String>,


    /// 
    /// The prefix that is prepended to group claims to prevent clashes with existing names       (such as system: groups). For example, the value oidc: creates       group names like oidc:engineering and oidc:infra. The prefix       can't contain system:
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupsPrefix")]
    pub groups_prefix: Option<String>,


    /// 
    /// The URL of the OIDC identity provider that allows the API server to discover public       signing keys for verifying tokens.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IssuerUrl")]
    pub issuer_url: String,


    /// 
    /// The key-value pairs that describe required claims in the identity token. If set, each       claim is verified to be present in the token with a matching value.
    /// 
    /// Required: No
    ///
    /// Type: List of RequiredClaim
    ///
    /// Update requires: Replacement
    #[serde(rename = "RequiredClaims")]
    pub required_claims: Option<Vec<RequiredClaim>>,


    /// 
    /// The JSON Web token (JWT) claim that is used as the username.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UsernameClaim")]
    pub username_claim: Option<String>,


    /// 
    /// The prefix that is prepended to username claims to prevent clashes with existing       names. The prefix can't contain system:
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UsernamePrefix")]
    pub username_prefix: Option<String>,

}



impl cfn_resources::CfnResource for OidcIdentityProviderConfig {
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

/// A key-value pair that describes a required claim in the identity token. If set, each       claim is verified to be present in the token with a matching value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RequiredClaim {


    /// 
    /// The key to match from the token.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the key from the token.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for RequiredClaim {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Tag {
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