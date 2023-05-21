

/// Creates a pull through cache rule. A pull through cache rule provides a way to cache       images from an external public registry in your Amazon ECR private registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPullThroughCacheRule {


    /// 
    /// The Amazon ECR repository prefix associated with the pull through cache rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 20
    ///
    /// Pattern: [a-z0-9]+(?:[._-][a-z0-9]+)*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EcrRepositoryPrefix")]
    pub ecr_repository_prefix: Option<String>,


    /// 
    /// The upstream registry URL associated with the pull through cache rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UpstreamRegistryUrl")]
    pub upstream_registry_url: Option<String>,

}



impl cfn_resources::CfnResource for CfnPullThroughCacheRule {
    fn type_string() -> &'static str {
        "AWS::ECR::PullThroughCacheRule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
