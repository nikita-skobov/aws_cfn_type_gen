

/// A self-service action association consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.
#[derive(Default, serde::Serialize)]
pub struct CfnServiceActionAssociation {


    /// 
    /// The identifier of the provisioning artifact. For example, pa-4abcdjnxjj6ne.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,


    /// 
    /// The product identifier. For example, prod-abcdzk7xy33qa.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProductId")]
    pub product_id: String,


    /// 
    /// The self-service action identifier. For example, act-fs7abcd89wxyz.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,

}