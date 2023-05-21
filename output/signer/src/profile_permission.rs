

/// Adds cross-account permissions to a signing profile.
#[derive(Default, serde::Serialize)]
pub struct CfnProfilePermission {


    /// 
    /// A unique identifier for the cross-account permission statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatementId")]
    pub statement_id: String,


    /// 
    /// The version of the signing profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileVersion")]
    pub profile_version: Option<String>,


    /// 
    /// The AWS principal receiving cross-account permissions. This             may be an IAM role or another AWS account ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: String,


    /// 
    /// The human-readable name of the signing profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileName")]
    pub profile_name: String,


    /// 
    /// The AWS Signer action permitted as part of cross-account             permissions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: String,

}