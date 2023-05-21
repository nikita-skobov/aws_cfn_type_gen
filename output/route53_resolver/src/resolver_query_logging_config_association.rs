

/// The AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation resource is a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResolverQueryLoggingConfigAssociation {


    /// 
    /// The ID of the Amazon VPC that is associated with the query logging configuration.
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


    /// 
    /// The ID of the query logging configuration that a VPC is associated with.
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
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: Option<String>,

}

impl cfn_resources::CfnResource for CfnResolverQueryLoggingConfigAssociation {
    fn type_string() -> &'static str {
        "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
