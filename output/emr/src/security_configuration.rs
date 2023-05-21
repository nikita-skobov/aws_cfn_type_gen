/// Use a SecurityConfiguration resource to configure data encryption, Kerberos authentication (available in Amazon EMR release version 5.10.0 and later), and Amazon S3 authorization for EMRFS (available in EMR 5.10.0 and later). You can re-use a security configuration for any number of clusters in your account. For more information and example security configuration JSON objects, see Create a Security Configuration in the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecurityConfiguration {
    ///
    /// The name of the security configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The security configuration details in JSON format.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: serde_json::Value,
}

impl cfn_resources::CfnResource for CfnSecurityConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::SecurityConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 10280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
