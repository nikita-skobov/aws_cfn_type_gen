/// Creates or updates a resource policy that allows other AWS services to put log events to    this account. An account can have up to 10 resource policies per AWS    Region.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourcePolicy {
    ///
    /// The details of the policy. It must be formatted in JSON, and you must use backslashes to escape characters that need to be escaped in JSON strings, such as double quote marks.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5120
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: cfn_resources::StrVal,

    ///
    /// The name of the resource policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    pub policy_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::Logs::ResourcePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.policy_document;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 5120 as _ {
                return Err(format!(
                    "Max validation failed on field 'policy_document'. {} is greater than 5120",
                    s.len()
                ));
            }
        }

        let the_val = &self.policy_document;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'policy_document'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
