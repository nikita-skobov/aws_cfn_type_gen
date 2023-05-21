

/// The AWS::Route53Resolver::ResolverDNSSECConfig resource is a complex type that contains information about a configuration for DNSSEC validation.
#[derive(Default, serde::Serialize)]
pub struct CfnResolverDNSSECConfig {


    /// 
    /// The ID of the virtual private cloud (VPC) that you're configuring the DNSSEC validation status for.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

}
