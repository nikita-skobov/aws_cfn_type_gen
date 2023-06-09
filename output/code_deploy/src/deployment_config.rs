/// The AWS::CodeDeploy::DeploymentConfig resource creates a set of deployment    rules, deployment success conditions, and deployment failure conditions that AWS CodeDeploy uses during a deployment. The deployment configuration specifies, through    the use of a MinimumHealthyHosts value, the number or percentage of instances    that must remain available at any time during a deployment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDeploymentConfig {
    ///
    /// The destination platform type for the deployment (Lambda,         Server, or ECS).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECS | Lambda | Server
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComputePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<DeploymentConfigComputePlatformEnum>,

    ///
    /// A name for the deployment configuration. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the deployment configuration name. For    more information, see Name Type.
    ///
    /// Important If you specify a name, you cannot perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<cfn_resources::StrVal>,

    ///
    /// The minimum number of healthy instances that should be available at any time during the    deployment. There are two parameters expected in the input: type and value.
    ///
    /// The type parameter takes either of the following values:
    ///
    /// HOST_COUNT: The value parameter represents the minimum number of healthy instances as      an absolute value.        FLEET_PERCENT: The value parameter represents the minimum number of healthy instances      as a percentage of the total number of instances in the deployment. If you specify      FLEET_PERCENT, at the start of the deployment, AWS CodeDeploy converts the      percentage to the equivalent number of instance and rounds up fractional instances.
    ///
    /// The value parameter takes an integer.
    ///
    /// For example, to set a minimum of 95% healthy instance, specify a type of FLEET_PERCENT and    a value of 95.
    ///
    /// For more information about instance health, see CodeDeploy Instance     Health in the AWS CodeDeploy User Guide.
    ///
    /// Required: No
    ///
    /// Type: MinimumHealthyHosts
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinimumHealthyHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,

    ///
    /// The configuration that specifies how the deployment traffic is routed.
    ///
    /// Required: No
    ///
    /// Type: TrafficRoutingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrafficRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DeploymentConfigComputePlatformEnum {
    /// ECS
    #[serde(rename = "ECS")]
    Ecs,

    /// Lambda
    #[serde(rename = "Lambda")]
    Lambda,

    /// Server
    #[serde(rename = "Server")]
    Server,
}

impl Default for DeploymentConfigComputePlatformEnum {
    fn default() -> Self {
        DeploymentConfigComputePlatformEnum::Ecs
    }
}

impl cfn_resources::CfnResource for CfnDeploymentConfig {
    fn type_string(&self) -> &'static str {
        "AWS::CodeDeploy::DeploymentConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.deployment_config_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'deployment_config_name'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.deployment_config_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'deployment_config_name'. {} is less than 1", s.len()));
                }
            }
        }

        self.minimum_healthy_hosts
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.traffic_routing_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MinimumHealthyHosts is a property of the DeploymentConfig resource that defines how many instances must remain healthy    during an AWS CodeDeploy deployment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MinimumHealthyHosts {
    ///
    /// The minimum healthy instance type:
    ///
    /// HOST_COUNT: The minimum number of healthy instance as an absolute value.        FLEET_PERCENT: The minimum number of healthy instance as a percentage of the total      number of instance in the deployment.
    ///
    /// In an example of nine instance, if a HOST_COUNT of six is specified, deploy to up to three    instances at a time. The deployment is successful if six or more instances are deployed to    successfully. Otherwise, the deployment fails. If a FLEET_PERCENT of 40 is specified, deploy    to up to five instance at a time. The deployment is successful if four or more instance are    deployed to successfully. Otherwise, the deployment fails.
    ///
    /// NoteIn a call to GetDeploymentConfig, CodeDeployDefault.OneAtATime returns a     minimum healthy instance type of MOST_CONCURRENCY and a value of 1. This means a deployment     to only one instance at a time. (You cannot set the type to MOST_CONCURRENCY, only to     HOST_COUNT or FLEET_PERCENT.) In addition, with CodeDeployDefault.OneAtATime, AWS CodeDeploy attempts to ensure that all instances but one are kept in a healthy state     during the deployment. Although this allows one instance at a time to be taken offline for a     new deployment, it also means that if the deployment to the last instance fails, the overall     deployment is still successful.
    ///
    /// For more information, see AWS CodeDeploy Instance     Health in the AWS CodeDeploy User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FLEET_PERCENT | HOST_COUNT
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: MinimumHealthyHostsTypeEnum,

    ///
    /// The minimum healthy instance value.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MinimumHealthyHostsTypeEnum {
    /// FLEET_PERCENT
    #[serde(rename = "FLEET_PERCENT")]
    Fleetpercent,

    /// HOST_COUNT
    #[serde(rename = "HOST_COUNT")]
    Hostcount,
}

impl Default for MinimumHealthyHostsTypeEnum {
    fn default() -> Self {
        MinimumHealthyHostsTypeEnum::Fleetpercent
    }
}

impl cfn_resources::CfnResource for MinimumHealthyHosts {
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

/// A configuration that shifts traffic from one version of a Lambda function       or Amazon ECS task set to another in two increments. The original and target         Lambda function versions or ECS task sets are specified in the       deployment's AppSpec file.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TimeBasedCanary {
    ///
    /// The number of minutes between the first and second traffic shifts of a         TimeBasedCanary deployment.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "CanaryInterval")]
    pub canary_interval: i64,

    ///
    /// The percentage of traffic to shift in the first increment of a         TimeBasedCanary deployment.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "CanaryPercentage")]
    pub canary_percentage: i64,
}

impl cfn_resources::CfnResource for TimeBasedCanary {
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

/// A configuration that shifts traffic from one version of a Lambda function       or ECS task set to another in equal increments, with an equal number of minutes between       each increment. The original and target Lambda function versions or ECS task       sets are specified in the deployment's AppSpec file.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TimeBasedLinear {
    ///
    /// The number of minutes between each incremental traffic shift of a         TimeBasedLinear deployment.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "LinearInterval")]
    pub linear_interval: i64,

    ///
    /// The percentage of traffic that is shifted at the start of each increment of a         TimeBasedLinear deployment.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "LinearPercentage")]
    pub linear_percentage: i64,
}

impl cfn_resources::CfnResource for TimeBasedLinear {
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

/// The configuration that specifies how traffic is shifted from one version of a Lambda function to another version during an AWS Lambda deployment,       or from one Amazon ECS task set to another during an Amazon ECS       deployment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrafficRoutingConfig {
    ///
    /// A configuration that shifts traffic from one version of a Lambda function       or ECS task set to another in two increments. The original and target Lambda       function versions or ECS task sets are specified in the deployment's AppSpec       file.
    ///
    /// Required: No
    ///
    /// Type: TimeBasedCanary
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeBasedCanary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_canary: Option<TimeBasedCanary>,

    ///
    /// A configuration that shifts traffic from one version of a Lambda function       or Amazon ECS task set to another in equal increments, with an equal number of       minutes between each increment. The original and target Lambda function       versions or Amazon ECS task sets are specified in the deployment's AppSpec       file.
    ///
    /// Required: No
    ///
    /// Type: TimeBasedLinear
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeBasedLinear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_linear: Option<TimeBasedLinear>,

    ///
    /// The type of traffic shifting (TimeBasedCanary or         TimeBasedLinear) used by a deployment configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AllAtOnce | TimeBasedCanary | TimeBasedLinear
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: TrafficRoutingConfigTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TrafficRoutingConfigTypeEnum {
    /// AllAtOnce
    #[serde(rename = "AllAtOnce")]
    Allatonce,

    /// TimeBasedCanary
    #[serde(rename = "TimeBasedCanary")]
    Timebasedcanary,

    /// TimeBasedLinear
    #[serde(rename = "TimeBasedLinear")]
    Timebasedlinear,
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
        self.time_based_canary
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.time_based_linear
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
