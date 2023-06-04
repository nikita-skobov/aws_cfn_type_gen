/// Creates a pull through cache rule. A pull through cache rule provides a way to cache       images from an external public registry in your Amazon ECR private registry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<cfn_resources::StrVal>,

    ///
    /// The upstream registry URL associated with the pull through cache rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UpstreamRegistryUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnPullThroughCacheRule {
    fn type_string(&self) -> &'static str {
        "AWS::ECR::PullThroughCacheRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ecr_repository_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20 as _ {
                    return Err(format!("Max validation failed on field 'ecr_repository_prefix'. {} is greater than 20", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.ecr_repository_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 2 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ecr_repository_prefix'. {} is less than 2",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
