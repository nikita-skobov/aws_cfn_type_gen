

/// The AWS::AppConfig::Deployment resource starts a deployment. Starting a    deployment in AWS AppConfig calls the StartDeployment API action. This    call includes the IDs of the AWS AppConfig application, the environment, the    configuration profile, and (optionally) the configuration data version to deploy. The call    also includes the ID of the deployment strategy to use, which determines how the configuration    data is deployed.
///
/// AWS AppConfig monitors the distribution to all hosts and reports status. If a    distribution fails, then AWS AppConfig rolls back the configuration.
///
/// AWS AppConfig requires that you create resources and deploy a configuration in the    following order:
///
/// For more information, see AWS AppConfig in the      AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeployment {


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
    pub description: Option<String>,


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
    pub configuration_profile_id: String,


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
    pub deployment_strategy_id: String,


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
    pub kms_key_identifier: Option<String>,


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
    pub environment_id: String,


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
    pub application_id: String,


    /// 
    /// Metadata to assign to the deployment. Tags help organize and categorize your AWS AppConfig resources. Each tag consists of a key and an optional value, both of which     you define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tags
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,


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
    pub configuration_version: String,

}

impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string() -> &'static str {
        "AWS::AppConfig::Deployment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Metadata to assign to the deployment strategy. Tags help organize and categorize your       AWS AppConfig resources. Each tag consists of a key and an optional value, both of     which you define.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tags {


    /// 
    /// The tag value can be up to 256 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The key-value string map. The valid character set is [a-zA-Z+-=._:/]. The tag    key can be up to 128 characters and must not start with aws:.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}
