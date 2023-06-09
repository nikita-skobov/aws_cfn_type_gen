/// A self-service action association consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnServiceActionAssociation {
    ///
    /// The product identifier. For example, prod-abcdzk7xy33qa.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProductId")]
    pub product_id: cfn_resources::StrVal,

    ///
    /// The identifier of the provisioning artifact. For example, pa-4abcdjnxjj6ne.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: cfn_resources::StrVal,

    ///
    /// The self-service action identifier. For example, act-fs7abcd89wxyz.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnServiceActionAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::ServiceActionAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'product_id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'product_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.provisioning_artifact_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'provisioning_artifact_id'. {} is greater than 100", s.len()));
            }
        }

        let the_val = &self.provisioning_artifact_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'provisioning_artifact_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.service_action_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'service_action_id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.service_action_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'service_action_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
