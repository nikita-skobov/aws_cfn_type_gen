/// The AWS::AppConfig::Deployment resource starts a deployment. Starting a    deployment in AWS AppConfig calls the StartDeployment API action. This    call includes the IDs of the AWS AppConfig application, the environment, the    configuration profile, and (optionally) the configuration data version to deploy. The call    also includes the ID of the deployment strategy to use, which determines how the configuration    data is deployed.
///
/// AWS AppConfig monitors the distribution to all hosts and reports status. If a    distribution fails, then AWS AppConfig rolls back the configuration.
///
/// AWS AppConfig requires that you create resources and deploy a configuration in the    following order:
///
/// For more information, see AWS AppConfig in the      AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDeployment {
    ///
    /// The application ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationId")]
    pub application_id: cfn_resources::StrVal,

    ///
    /// The configuration profile ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: cfn_resources::StrVal,

    ///
    /// The configuration version to deploy. If deploying an AWS AppConfig hosted     configuration version, you can specify either the version number or version label. For all     other configurations, you must specify the version number.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationVersion")]
    pub configuration_version: cfn_resources::StrVal,

    ///
    /// The deployment strategy ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: cfn_resources::StrVal,

    ///
    /// A description of the deployment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The environment ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentId")]
    pub environment_id: cfn_resources::StrVal,

    ///
    /// The AWS KMS key identifier (key ID, key alias, or key ARN). AWS AppConfig uses this ID to encrypt the configuration data using a customer managed key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Metadata to assign to the deployment. Tags help organize and categorize your AWS AppConfig resources. Each tag consists of a key and an optional value, both of which     you define.
    ///
    /// Required: No
    ///
    /// Type: List of Tags
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tags>>,
}

impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string(&self) -> &'static str {
        "AWS::AppConfig::Deployment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.configuration_version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!("Max validation failed on field 'configuration_version'. {} is greater than 1024", s.len()));
            }
        }

        let the_val = &self.configuration_version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'configuration_version'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'kms_key_identifier'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_identifier'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Metadata to assign to the deployment strategy. Tags help organize and categorize your       AWS AppConfig resources. Each tag consists of a key and an optional value, both of     which you define.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tags {
    ///
    /// The key-value string map. The valid character set is [a-zA-Z+-=._:/]. The tag    key can be up to 128 characters and must not start with aws:.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The tag value can be up to 256 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Tags {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
