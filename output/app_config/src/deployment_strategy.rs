

/// The AWS::AppConfig::DeploymentStrategy resource creates an AWS AppConfig deployment strategy. A deployment strategy defines important criteria for    rolling out your configuration to the designated targets. A deployment strategy includes: the    overall duration required, a percentage of targets to receive the deployment during each    interval, an algorithm that defines how percentage grows, and bake time.
///
/// AWS AppConfig requires that you create resources and deploy a configuration in the    following order:
///
/// For more information, see AWS AppConfig in the      AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeploymentStrategy {


    /// 
    /// Total amount of time for a deployment to last.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1440
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub deployment_duration_in_minutes: f64,


    /// 
    /// Specifies the amount of time AWS AppConfig monitors for Amazon CloudWatch alarms after the     configuration has been deployed to 100% of its targets, before considering the deployment     to be complete. If an alarm is triggered during this time, AWS AppConfig rolls back     the deployment. You must configure permissions for AWS AppConfig to roll back based     on CloudWatch alarms. For more information, see Configuring permissions for rollback based on Amazon CloudWatch alarms in the               AWS AppConfig User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1440
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalBakeTimeInMinutes")]
    pub final_bake_time_in_minutes: Option<f64>,


    /// 
    /// Save the deployment strategy to a Systems Manager (SSM) document.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SSM_DOCUMENT
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicateTo")]
    pub replicate_to: DeploymentStrategyReplicateToEnum,


    /// 
    /// A description of the deployment strategy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The algorithm used to define how percentage grows over time. AWS AppConfig     supports the following growth types:
    /// 
    /// Linear: For this type, AWS AppConfig processes     the deployment by dividing the total number of targets by the value specified for       Step percentage. For example, a linear deployment that uses a Step       percentage of 10 deploys the configuration to 10 percent of the hosts. After     those deployments are complete, the system deploys the configuration to the next 10     percent. This continues until 100% of the targets have successfully received the     configuration.
    /// 
    /// Exponential: For this type, AWS AppConfig     processes the deployment exponentially using the following formula: G*(2^N).     In this formula, G is the growth factor specified by the user and       N is the number of steps until the configuration is deployed to all     targets. For example, if you specify a growth factor of 2, then the system rolls out the     configuration as follows:
    /// 
    /// 2*(2^0)
    /// 
    /// 2*(2^1)
    /// 
    /// 2*(2^2)
    /// 
    /// Expressed numerically, the deployment rolls out as follows: 2% of the targets, 4% of the     targets, 8% of the targets, and continues until the configuration has been deployed to all     targets.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EXPONENTIAL | LINEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrowthType")]
    pub growth_type: Option<DeploymentStrategyGrowthTypeEnum>,


    /// 
    /// Assigns metadata to an AWS AppConfig resource. Tags help organize and categorize     your AWS AppConfig resources. Each tag consists of a key and an optional value, both     of which you define. You can specify a maximum of 50 tags for a resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tags
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,


    /// 
    /// A name for the deployment strategy.
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
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The percentage of targets to receive a deployed configuration during each     interval.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrowthFactor")]
    pub growth_factor: f64,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DeploymentStrategyReplicateToEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SSM_DOCUMENT
    #[serde(rename = "SSM_DOCUMENT")]
    Ssmdocument,

}

impl Default for DeploymentStrategyReplicateToEnum {
    fn default() -> Self {
        DeploymentStrategyReplicateToEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DeploymentStrategyGrowthTypeEnum {

    /// EXPONENTIAL
    #[serde(rename = "EXPONENTIAL")]
    Exponential,

    /// LINEAR
    #[serde(rename = "LINEAR")]
    Linear,

}

impl Default for DeploymentStrategyGrowthTypeEnum {
    fn default() -> Self {
        DeploymentStrategyGrowthTypeEnum::Exponential
    }
}


impl cfn_resources::CfnResource for CfnDeploymentStrategy {
    fn type_string() -> &'static str {
        "AWS::AppConfig::DeploymentStrategy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Metadata to assign to the deployment strategy. Tags help organize and categorize your       AWS AppConfig resources. Each tag consists of a key and an optional value, both of     which you define.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub key: Option<String>,


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

}


