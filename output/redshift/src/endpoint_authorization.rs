

/// Describes an endpoint authorization for authorizing Redshift-managed VPC endpoint access to a cluster across AWS accounts.
#[derive(Default, serde::Serialize)]
pub struct CfnEndpointAuthorization {


    /// 
    /// The virtual private cloud (VPC) identifiers to grant access to.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcIds")]
    pub vpc_ids: Option<Vec<String>>,


    /// 
    /// The cluster identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,


    /// 
    /// Indicates whether to force the revoke action.       If true, the Redshift-managed VPC endpoints associated with the endpoint authorization are also deleted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Force")]
    pub force: Option<bool>,


    /// 
    /// The AWS account ID of either the cluster owner (grantor) or grantee.     If Grantee parameter is true, then the Account value is of the grantor.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "Account")]
    pub account: String,

}
