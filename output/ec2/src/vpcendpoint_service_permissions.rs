

/// Grant or revoke permissions for service consumers (users, IAM roles, and AWS accounts) to connect to a VPC endpoint service.
///
/// If you grant permissions to all principals, the service is public. Any users who know     the name of a public service can send a request to attach an endpoint. If the service does     not require manual approval, attachments are automatically approved.
#[derive(Default, serde::Serialize)]
pub struct CfnVPCEndpointServicePermissions {


    /// 
    /// The Amazon Resource Names (ARN) of one or more principals (users, IAM roles, and       AWS accounts). Permissions are granted to the principals in this list.     To grant permissions to all principals, specify an asterisk (*). Permissions are revoked     for principals not in this list. If the list is empty, then all permissions are revoked.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedPrincipals")]
    pub allowed_principals: Option<Vec<String>>,


    /// 
    /// The ID of the service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceId")]
    pub service_id: String,

}
