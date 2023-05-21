/// The AWS::Cognito::IdentityPoolRoleAttachment resource manages the role    configuration for an Amazon Cognito identity pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIdentityPoolRoleAttachment {
    ///
    /// An identity pool ID in the format REGION:GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,

    ///
    /// How users for a specific identity provider are mapped to roles. This is a string to the     RoleMapping object map. The string identifies the identity provider. For    example: graph.facebook.com or     cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id.
    ///
    /// If the IdentityProvider field isn't provided in this object, the string is    used as the identity provider name.
    ///
    /// For more information, see the RoleMapping property.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleMappings")]
    pub role_mappings: Option<serde_json::Value>,

    ///
    /// The map of the roles associated with this pool. For a given role, the key is either    "authenticated" or "unauthenticated". The value is the role ARN.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for CfnIdentityPoolRoleAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::IdentityPoolRoleAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Defines how to map a claim to a role ARN.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MappingRule {
    ///
    /// The claim name that must be present in the token. For example: "isAdmin" or "paid".
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Claim")]
    pub claim: String,

    ///
    /// The match condition that specifies how closely the claim value in the IdP token must match     Value.
    ///
    /// Valid values are: Equals, Contains, StartsWith, and     NotEqual.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchType")]
    pub match_type: String,

    ///
    /// The Amazon Resource Name (ARN) of the role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

    ///
    /// A brief string that the claim must match. For example, "paid" or "yes".
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for MappingRule {
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

/// RoleMapping is a property of the AWS::Cognito::IdentityPoolRoleAttachment resource that defines the role-mapping    attributes of an Amazon Cognito identity pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoleMapping {
    ///
    /// Specifies the action to be taken if either no rules match the claim value for the Rules    type, or there is no cognito:preferred_role claim and there are multiple     cognito:roles matches for the Token type. If you specify Token or Rules as the    Type, AmbiguousRoleResolution is required.
    ///
    /// Valid values are AuthenticatedRole or Deny.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmbiguousRoleResolution")]
    pub ambiguous_role_resolution: Option<String>,

    ///
    /// Identifier for the identity provider for which the role is mapped. For example:     graph.facebook.com or     cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id     (http://cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id). This    is the identity provider that is used by the user for authentication.
    ///
    /// If the identity provider property isn't provided, the key of the entry in the     RoleMappings map is used as the identity provider.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: Option<String>,

    ///
    /// The rules to be used for mapping users to roles. If you specify "Rules" as the    role-mapping type, RulesConfiguration is required.
    ///
    /// Required: No
    ///
    /// Type: RulesConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "RulesConfiguration")]
    pub rules_configuration: Option<RulesConfigurationType>,

    ///
    /// The role-mapping type. Token uses cognito:roles and     cognito:preferred_role claims from the Amazon Cognito identity provider token    to map groups to roles. Rules attempts to match claims from the token to map to a    role.
    ///
    /// Valid values are Token or Rules.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,
}

impl cfn_resources::CfnResource for RoleMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.rules_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// RulesConfigurationType is a subproperty of the RoleMapping property that defines the rules to be used for mapping users to    roles.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RulesConfigurationType {
    ///
    /// The rules. You can specify up to 25 rules per identity provider.
    ///
    /// Required: Yes
    ///
    /// Type: List of MappingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<MappingRule>,
}

impl cfn_resources::CfnResource for RulesConfigurationType {
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
