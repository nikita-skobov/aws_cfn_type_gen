

/// Creates an entitlement to control access, based on user attributes, to specific     applications within a stack. Entitlements apply to SAML 2.0 federated user identities.     Amazon AppStream 2.0 user pool and streaming URL users are entitled to all applications in     a stack. Entitlements don't apply to the desktop stream view application or to applications     managed by a dynamic app provider using the Dynamic Application Framework.
#[derive(Default, serde::Serialize)]
pub struct CfnEntitlement {


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
    pub stack_name: String,


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
    pub name: String,


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
    pub app_visibility: String,


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
    pub description: Option<String>,

}


/// An attribute that belongs to an entitlement. Application entitlements work by matching a     supported SAML 2.0 attribute name to a value when a user identity federates to an AppStream     2.0 SAML application.
#[derive(Default, serde::Serialize)]
pub struct Attribute {


    /// A value that is matched to a supported SAML attribute name when a user identity federates to an AppStream 2.0 SAML application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


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
    pub name: String,

}
