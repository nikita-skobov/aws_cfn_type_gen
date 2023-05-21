/// Creates an entitlement to control access, based on user attributes, to specific     applications within a stack. Entitlements apply to SAML 2.0 federated user identities.     Amazon AppStream 2.0 user pool and streaming URL users are entitled to all applications in     a stack. Entitlements don't apply to the desktop stream view application or to applications     managed by a dynamic app provider using the Dynamic Application Framework.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEntitlement {
    ///
    /// Specifies whether to entitle all apps or only selected apps.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | ASSOCIATED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppVisibility")]
    pub app_visibility: EntitlementAppVisibilityEnum,

    ///
    /// The attributes of the entitlement.
    ///
    /// Required: Yes
    ///
    /// Type: List of Attribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,

    ///
    /// The description of the entitlement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the entitlement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The name of the stack.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackName")]
    pub stack_name: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EntitlementAppVisibilityEnum {
    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// ASSOCIATED
    #[serde(rename = "ASSOCIATED")]
    Associated,
}

impl Default for EntitlementAppVisibilityEnum {
    fn default() -> Self {
        EntitlementAppVisibilityEnum::All
    }
}

impl cfn_resources::CfnResource for CfnEntitlement {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::Entitlement"
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

        Ok(())
    }
}

/// An attribute that belongs to an entitlement. Application entitlements work by matching a     supported SAML 2.0 attribute name to a value when a user identity federates to an AppStream     2.0 SAML application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Attribute {
    /// A supported AWS IAM SAML PrincipalTag attribute that is matched to a value when a user     identity federates to an AppStream 2.0 SAML application.
    ///
    /// The following are supported values:
    ///
    /// roles department organization groups title costCenter userType
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    /// A value that is matched to a supported SAML attribute name when a user identity federates to an AppStream 2.0 SAML application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Attribute {
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
