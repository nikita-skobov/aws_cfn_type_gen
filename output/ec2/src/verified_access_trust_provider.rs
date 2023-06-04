/// Describes a Verified Access trust provider.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVerifiedAccessTrustProvider {
    ///
    /// A description for the AWS Verified Access trust provider.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The options for device-identity trust provider.
    ///
    /// Required: No
    ///
    /// Type: DeviceOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_options: Option<DeviceOptions>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_trust_provider_type: Option<VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum>,

    ///
    /// The options for an OpenID Connect-compatible user-identity trust provider.
    ///
    /// Required: No
    ///
    /// Type: OidcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "OidcOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub oidc_options: Option<OidcOptions>,

    ///
    /// The identifier to be used when working with policy rules.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyReferenceName")]
    pub policy_reference_name: cfn_resources::StrVal,

    ///
    /// The tags.
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
    pub trust_provider_type: VerifiedAccessTrustProviderTrustProviderTypeEnum,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_trust_provider_type: Option<VerifiedAccessTrustProviderUserTrustProviderTypeEnum>,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnVerifiedAccessTrustProvidercreationtime,

    #[serde(skip_serializing)]
    pub att_last_updated_time: CfnVerifiedAccessTrustProviderlastupdatedtime,

    #[serde(skip_serializing)]
    pub att_verified_access_trust_provider_id:
        CfnVerifiedAccessTrustProviderverifiedaccesstrustproviderid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum {
    /// crowdstrike
    #[serde(rename = "crowdstrike")]
    Crowdstrike,

    /// jamf
    #[serde(rename = "jamf")]
    Jamf,
}

impl Default for VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum::Crowdstrike
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerifiedAccessTrustProviderTrustProviderTypeEnum {
    /// device
    #[serde(rename = "device")]
    Device,

    /// user
    #[serde(rename = "user")]
    User,
}

impl Default for VerifiedAccessTrustProviderTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderTrustProviderTypeEnum::Device
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerifiedAccessTrustProviderUserTrustProviderTypeEnum {
    /// iam-identity-center
    #[serde(rename = "iam-identity-center")]
    Iamidentitycenter,

    /// oidc
    #[serde(rename = "oidc")]
    Oidc,
}

impl Default for VerifiedAccessTrustProviderUserTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderUserTrustProviderTypeEnum::Iamidentitycenter
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessTrustProvidercreationtime;
impl CfnVerifiedAccessTrustProvidercreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessTrustProviderlastupdatedtime;
impl CfnVerifiedAccessTrustProviderlastupdatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastUpdatedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessTrustProviderverifiedaccesstrustproviderid;
impl CfnVerifiedAccessTrustProviderverifiedaccesstrustproviderid {
    pub fn att_name(&self) -> &'static str {
        r#"VerifiedAccessTrustProviderId"#
    }
}

impl cfn_resources::CfnResource for CfnVerifiedAccessTrustProvider {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VerifiedAccessTrustProvider"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.device_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.oidc_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the options for an AWS Verified Access device-identity based trust provider.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tenant_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DeviceOptions {
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

/// Describes the options for an OpenID Connect-compatible user-identity trust     provider.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authorization_endpoint: Option<cfn_resources::StrVal>,

    ///
    /// The client identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub client_id: Option<cfn_resources::StrVal>,

    ///
    /// The client secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub client_secret: Option<cfn_resources::StrVal>,

    ///
    /// The OIDC issuer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub issuer: Option<cfn_resources::StrVal>,

    ///
    /// The OpenID Connect (OIDC) scope specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub scope: Option<cfn_resources::StrVal>,

    ///
    /// The OIDC token endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenEndpoint")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub token_endpoint: Option<cfn_resources::StrVal>,

    ///
    /// The OIDC user info endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserInfoEndpoint")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_info_endpoint: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OidcOptions {
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
