

/// A complex type that contains information about a Resolver configuration for a VPC.
#[derive(Default, serde::Serialize)]
pub struct CfnResolverConfig {


    /// 
    /// The ID of the Amazon Virtual Private Cloud VPC that you're configuring Resolver for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: String,


    /// 
    /// Represents the desired status of AutodefinedReverse. The only supported value on creation is DISABLE.       Deletion of this resource will return AutodefinedReverse to its default value of ENABLED.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutodefinedReverseFlag")]
    pub autodefined_reverse_flag: String,

}
