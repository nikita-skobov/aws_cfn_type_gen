

/// Associates an application to an entitlement.
#[derive(Default, serde::Serialize)]
pub struct CfnApplicationEntitlementAssociation {


    /// The name of the entitlement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntitlementName")]
    pub entitlement_name: String,


    /// The identifier of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,


    /// The name of the stack.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackName")]
    pub stack_name: String,

}
