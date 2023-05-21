

/// Describes a Verified Access trust provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVerifiedAccessTrustProvider {


    /// 
    /// The options for an OpenID Connect-compatible user-identity trust provider.
    /// 
    /// Required: No
    ///
    /// Type: OidcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "OidcOptions")]
    pub oidc_options: Option<OidcOptions>,


    /// 
    /// The tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The type of device-based trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: crowdstrike | jamf
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceTrustProviderType")]
    pub device_trust_provider_type: Option<String>,


    /// 
    /// The type of user-based trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: iam-identity-center | oidc
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserTrustProviderType")]
    pub user_trust_provider_type: Option<String>,


    /// 
    /// The type of Verified Access trust provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: device | user
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrustProviderType")]
    pub trust_provider_type: String,


    /// 
    /// The options for device-identity trust provider.
    /// 
    /// Required: No
    ///
    /// Type: DeviceOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceOptions")]
    pub device_options: Option<DeviceOptions>,


    /// 
    /// The identifier to be used when working with policy rules.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyReferenceName")]
    pub policy_reference_name: String,


    /// 
    /// A description for the AWS Verified Access trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnVerifiedAccessTrustProvider {
    fn type_string() -> &'static str {
        "AWS::EC2::VerifiedAccessTrustProvider"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes the options for an AWS Verified Access device-identity based trust provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeviceOptions {


    /// 
    /// The ID of the tenant application with the device-identity provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TenantId")]
    pub tenant_id: Option<String>,

}


/// Describes the options for an OpenID Connect-compatible user-identity trust     provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OidcOptions {


    /// 
    /// The OIDC authorization endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: Option<String>,


    /// 
    /// The OpenID Connect (OIDC) scope specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: Option<String>,


    /// 
    /// The client identifier.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,


    /// 
    /// The OIDC user info endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: Option<String>,


    /// 
    /// The OIDC token endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: Option<String>,


    /// 
    /// The client secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,


    /// 
    /// The OIDC issuer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
