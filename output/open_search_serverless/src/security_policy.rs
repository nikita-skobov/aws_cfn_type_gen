

/// Creates an encryption or network policy to be used by one or more OpenSearch       Serverless collections.
///
/// Network policies specify access to a collection and its OpenSearch Dashboards endpoint       from public networks or specific VPC endpoints. For more information, see Network         access for Amazon OpenSearch Serverless.
///
/// Encryption policies specify a KMS encryption key to assign to particular collections.       For more information, see Encryption         at rest for Amazon OpenSearch Serverless.
#[derive(Default, serde::Serialize)]
pub struct CfnSecurityPolicy {


    /// 
    /// The name of the policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The JSON policy document without any whitespaces.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: String,


    /// 
    /// The description of the security policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The type of security policy. Can be either encryption or network.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

}