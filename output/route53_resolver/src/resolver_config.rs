/// A complex type that contains information about a Resolver configuration for a VPC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResolverConfig {
    ///
    /// Represents the desired status of AutodefinedReverse. The only supported value on creation is DISABLE.       Deletion of this resource will return AutodefinedReverse to its default value of ENABLED.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutodefinedReverseFlag")]
    pub autodefined_reverse_flag: cfn_resources::StrVal,

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
    pub resource_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnResolverConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.resource_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_id'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.resource_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
