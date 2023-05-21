/// Use the AWS::SageMaker::Endpoint resource to create an endpoint using the       specified configuration in the request. Amazon SageMaker uses the endpoint to provision       resources and deploy models. You create the endpoint configuration with the AWS::SageMaker::EndpointConfig resource. For more information, see Deploy a         Model on Amazon SageMaker Hosting Services in the Amazon SageMaker         Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpoint {
    ///
    /// The deployment configuration for an endpoint, which contains the desired deployment       strategy and rollback configurations.
    ///
    /// Required: No
    ///
    /// Type: DeploymentConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentConfig")]
    pub deployment_config: Option<DeploymentConfig>,

    ///
    /// The name of the AWS::SageMaker::EndpointConfig resource that specifies the configuration       for the endpoint. For more information, see CreateEndpointConfig.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,

    ///
    /// The name of the endpoint.The name must be unique within an AWS       Region in your AWS account. The name is case-insensitive in         CreateEndpoint, but the case is preserved and must be matched in https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<String>,

    ///
    /// When you are updating endpoint resources with RetainAllVariantProperties whose value is set to true,         ExcludeRetainedVariantProperties specifies the list of type VariantProperty to override with the values provided by         EndpointConfig. If you don't specify a value for         ExcludeAllVariantProperties, no variant properties are overridden.       Don't use this property when creating new endpoint resources or when         RetainAllVariantProperties is set to false.
    ///
    /// Required: No
    ///
    /// Type: List of VariantProperty
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeRetainedVariantProperties")]
    pub exclude_retained_variant_properties: Option<Vec<VariantProperty>>,

    ///
    /// When updating endpoint resources, enables or disables the retention of variant       properties, such as the instance count or the variant weight. To retain the variant       properties of an endpoint when updating it, set RetainAllVariantProperties       to true. To use the variant properties specified in a new         EndpointConfig call when updating an endpoint, set         RetainAllVariantProperties to false. Use this property       only when updating endpoint resources, not when creating new endpoint resources.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainAllVariantProperties")]
    pub retain_all_variant_properties: Option<bool>,

    ///
    /// Specifies whether to reuse the last deployment configuration. The default value is       false (the configuration is not reused).
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainDeploymentConfig")]
    pub retain_deployment_config: Option<bool>,

    ///
    /// A list of key-value pairs to apply to this resource.
    ///
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags in the         AWS Billing and Cost Management User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::Endpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.deployment_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.endpoint_config_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'endpoint_config_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.endpoint_name {
            if the_val.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.exclude_retained_variant_properties {
            if the_val.len() > 3 as _ {
                return Err(format!("Max validation failed on field 'exclude_retained_variant_properties'. {} is greater than 3", the_val.len()));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An Amazon CloudWatch alarm configured to monitor metrics on an endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Alarm {
    ///
    /// The name of a CloudWatch alarm in your account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(?!\s*$).+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,
}

impl cfn_resources::CfnResource for Alarm {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.alarm_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'alarm_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.alarm_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'alarm_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Automatic rollback configuration for handling endpoint deployment failures and       recovery.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoRollbackConfig {
    ///
    /// List of CloudWatch alarms in your account that are configured to monitor metrics on an       endpoint. If any alarms are tripped during a deployment, SageMaker rolls back the       deployment.
    ///
    /// Required: Yes
    ///
    /// Type: List of Alarm
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alarms")]
    pub alarms: Vec<Alarm>,
}

impl cfn_resources::CfnResource for AutoRollbackConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.alarms;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'alarms'. {} is greater than 10",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Update policy for a blue/green deployment. If this update policy is specified, SageMaker       creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips       traffic to the new fleet according to the specified traffic routing configuration. Only       one update policy should be used in the deployment configuration. If no update policy is       specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting       by default.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlueGreenUpdatePolicy {
    ///
    /// Maximum execution timeout for the deployment. Note that the timeout value should be       larger than the total waiting time specified in TerminationWaitInSeconds       and WaitIntervalInSeconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 600
    ///
    /// Maximum: 14400
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionTimeoutInSeconds")]
    pub maximum_execution_timeout_in_seconds: Option<i64>,

    ///
    /// Additional waiting time in seconds after the completion of an endpoint deployment       before terminating the old endpoint fleet. Default is 0.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 3600
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminationWaitInSeconds")]
    pub termination_wait_in_seconds: Option<i64>,

    ///
    /// Defines the traffic routing strategy to shift traffic from the old fleet to the new       fleet during an endpoint deployment.
    ///
    /// Required: Yes
    ///
    /// Type: TrafficRoutingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficRoutingConfiguration")]
    pub traffic_routing_configuration: TrafficRoutingConfig,
}

impl cfn_resources::CfnResource for BlueGreenUpdatePolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.maximum_execution_timeout_in_seconds {
            if *the_val > 14400 as _ {
                return Err(format!("Max validation failed on field 'maximum_execution_timeout_in_seconds'. {} is greater than 14400", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_execution_timeout_in_seconds {
            if *the_val < 600 as _ {
                return Err(format!("Min validation failed on field 'maximum_execution_timeout_in_seconds'. {} is less than 600", the_val));
            }
        }

        if let Some(the_val) = &self.termination_wait_in_seconds {
            if *the_val > 3600 as _ {
                return Err(format!("Max validation failed on field 'termination_wait_in_seconds'. {} is greater than 3600", the_val));
            }
        }

        if let Some(the_val) = &self.termination_wait_in_seconds {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'termination_wait_in_seconds'. {} is less than 0", the_val));
            }
        }

        self.traffic_routing_configuration.validate()?;

        Ok(())
    }
}

/// Specifies the endpoint capacity to activate for production.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacitySize {
    ///
    /// Specifies the endpoint capacity type.
    ///
    /// INSTANCE_COUNT: The endpoint activates based on the number of           instances.                        CAPACITY_PERCENT: The endpoint activates based on the specified           percentage of capacity.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CAPACITY_PERCENT | INSTANCE_COUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: CapacitySizeTypeEnum,

    ///
    /// Defines the capacity size, either as a number of instances or a capacity       percentage.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CapacitySizeTypeEnum {
    /// CAPACITY_PERCENT
    #[serde(rename = "CAPACITY_PERCENT")]
    Capacitypercent,

    /// INSTANCE_COUNT
    #[serde(rename = "INSTANCE_COUNT")]
    Instancecount,
}

impl Default for CapacitySizeTypeEnum {
    fn default() -> Self {
        CapacitySizeTypeEnum::Capacitypercent
    }
}

impl cfn_resources::CfnResource for CapacitySize {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.value;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// The deployment configuration for an endpoint, which contains the desired deployment       strategy and rollback configurations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentConfig {
    ///
    /// Automatic rollback configuration for handling endpoint deployment failures and       recovery.
    ///
    /// Required: No
    ///
    /// Type: AutoRollbackConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoRollbackConfiguration")]
    pub auto_rollback_configuration: Option<AutoRollbackConfig>,

    ///
    /// Update policy for a blue/green deployment. If this update policy is specified, SageMaker       creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips       traffic to the new fleet according to the specified traffic routing configuration. Only       one update policy should be used in the deployment configuration. If no update policy is       specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting       by default.
    ///
    /// Required: Yes
    ///
    /// Type: BlueGreenUpdatePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlueGreenUpdatePolicy")]
    pub blue_green_update_policy: BlueGreenUpdatePolicy,
}

impl cfn_resources::CfnResource for DeploymentConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_rollback_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.blue_green_update_policy.validate()?;

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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

/// Defines the traffic routing strategy during an endpoint deployment to shift traffic from the       old fleet to the new fleet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrafficRoutingConfig {
    ///
    /// Batch size for the first step to turn on traffic on the new endpoint fleet. Value must be less than       or equal to 50% of the variant's total instance count.
    ///
    /// Required: No
    ///
    /// Type: CapacitySize
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanarySize")]
    pub canary_size: Option<CapacitySize>,

    ///
    /// Batch size for each step to turn on traffic on the new endpoint fleet. Value must be       10-50% of the variant's total instance count.
    ///
    /// Required: No
    ///
    /// Type: CapacitySize
    ///
    /// Update requires: No interruption
    #[serde(rename = "LinearStepSize")]
    pub linear_step_size: Option<CapacitySize>,

    ///
    /// Traffic routing strategy type.
    ///
    /// ALL_AT_ONCE: Endpoint traffic shifts to the new fleet         in a single step.                               CANARY: Endpoint traffic shifts to the new fleet         in two steps. The first step is the canary, which is a small portion of the traffic. The         second step is the remainder of the traffic.                               LINEAR: Endpoint traffic shifts to the new fleet in         n steps of a configurable size.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_AT_ONCE | CANARY | LINEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: TrafficRoutingConfigTypeEnum,

    ///
    /// The waiting time (in seconds) between incremental steps to turn on traffic on the       new endpoint fleet.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 3600
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaitIntervalInSeconds")]
    pub wait_interval_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TrafficRoutingConfigTypeEnum {
    /// ALL_AT_ONCE
    #[serde(rename = "ALL_AT_ONCE")]
    Allatonce,

    /// CANARY
    #[serde(rename = "CANARY")]
    Canary,

    /// LINEAR
    #[serde(rename = "LINEAR")]
    Linear,
}

impl Default for TrafficRoutingConfigTypeEnum {
    fn default() -> Self {
        TrafficRoutingConfigTypeEnum::Allatonce
    }
}

impl cfn_resources::CfnResource for TrafficRoutingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.canary_size
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.linear_step_size
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.wait_interval_in_seconds {
            if *the_val > 3600 as _ {
                return Err(format!("Max validation failed on field 'wait_interval_in_seconds'. {} is greater than 3600", the_val));
            }
        }

        if let Some(the_val) = &self.wait_interval_in_seconds {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'wait_interval_in_seconds'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Specifies a production variant property type for an Endpoint.
///
/// If you are updating an Endpoint with the RetainAllVariantProperties option set to true, the         VarientProperty objects listed in ExcludeRetainedVariantProperties override the existing variant properties       of the Endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VariantProperty {
    ///
    /// The type of variant property. The supported values are:
    ///
    /// DesiredInstanceCount: Overrides the existing variant instance           counts using the InitialInstanceCount values in the ProductionVariants.               DesiredWeight: Overrides the existing variant weights using the             InitialVariantWeight values in the ProductionVariants.               DataCaptureConfig: (Not currently supported.)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariantPropertyType")]
    pub variant_property_type: Option<String>,
}

impl cfn_resources::CfnResource for VariantProperty {
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
