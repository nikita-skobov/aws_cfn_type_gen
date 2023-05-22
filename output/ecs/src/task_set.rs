/// Create a task set in the specified cluster and service. This is used when a service 			uses the EXTERNAL deployment controller type. For more information, see 				Amazon ECS deployment 				types in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTaskSet {
    ///
    /// The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to create the 			task set in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cluster")]
    pub cluster: cfn_resources::StrVal,

    ///
    /// An optional non-unique tag that identifies this task set in external systems. If the 			task set is associated with a service discovery registry, the tasks in this task set 			will have the ECS_TASK_SET_EXTERNAL_ID       AWS Cloud Map attribute set to the provided 			value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<cfn_resources::StrVal>,

    ///
    /// The launch type that new tasks in the task set uses. For more information, see Amazon ECS 				launch types in the Amazon Elastic Container Service Developer Guide.
    ///
    /// If a launchType is specified, the capacityProviderStrategy 			parameter must be omitted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EC2 | EXTERNAL | FARGATE
    ///
    /// Update requires: Replacement
    #[serde(rename = "LaunchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<TaskSetLaunchTypeEnum>,

    ///
    /// A load balancer object representing the load balancer to use with the task set. The 			supported load balancer types are either an Application Load Balancer or a Network Load Balancer.
    ///
    /// Required: No
    ///
    /// Type: List of LoadBalancer
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,

    ///
    /// The network configuration for the task set.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    ///
    /// The platform version that the tasks in the task set uses. A platform version is 			specified only for tasks using the Fargate launch type. If one isn't 			specified, the LATEST platform version is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<cfn_resources::StrVal>,

    ///
    /// A floating-point percentage of your desired number of tasks to place and keep running 			in the task set.
    ///
    /// Required: No
    ///
    /// Type: Scale
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,

    ///
    /// The short name or full Amazon Resource Name (ARN) of the service to create the task set in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Service")]
    pub service: cfn_resources::StrVal,

    ///
    /// The details of the service discovery registries to assign to this task set. For more 			information, see Service 				discovery.
    ///
    /// Required: No
    ///
    /// Type: List of ServiceRegistry
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,

    ///
    /// The task definition for the tasks in the task set to use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TaskDefinition")]
    pub task_definition: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnTaskSetid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskSetLaunchTypeEnum {
    /// EC2
    #[serde(rename = "EC2")]
    Ec2,

    /// EXTERNAL
    #[serde(rename = "EXTERNAL")]
    External,

    /// FARGATE
    #[serde(rename = "FARGATE")]
    Fargate,
}

impl Default for TaskSetLaunchTypeEnum {
    fn default() -> Self {
        TaskSetLaunchTypeEnum::Ec2
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTaskSetid;
impl CfnTaskSetid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnTaskSet {
    fn type_string(&self) -> &'static str {
        "AWS::ECS::TaskSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.network_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scale.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object representing the networking details for a task or service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsVpcConfiguration {
    ///
    /// Whether the task's elastic network interface receives a public IP address. The default 			value is DISABLED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<AwsVpcConfigurationAssignPublicIpEnum>,

    ///
    /// The IDs of the security groups associated with the task or service. If you don't 			specify a security group, the default security group for the VPC is used. There's a 			limit of 5 security groups that can be specified per 			AwsVpcConfiguration.
    ///
    /// NoteAll specified security groups must be from the same VPC.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The IDs of the subnets associated with the task or service. There's a limit of 16 			subnets that can be specified per AwsVpcConfiguration.
    ///
    /// NoteAll specified subnets must be from the same VPC.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AwsVpcConfigurationAssignPublicIpEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for AwsVpcConfigurationAssignPublicIpEnum {
    fn default() -> Self {
        AwsVpcConfigurationAssignPublicIpEnum::Disabled
    }
}

impl cfn_resources::CfnResource for AwsVpcConfiguration {
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

/// The load balancer configuration to use with a service or task set.
///
/// When you add, update, or remove a load balancer configuration, Amazon ECS starts a new 			deployment with the updated Elastic Load Balancing configuration. This causes tasks to register to and 			deregister from load balancers.
///
/// We recommend that you verify this on a test environment before you update the Elastic Load Balancing 			configuration.
///
/// A service-linked role is required for services that use multiple target groups. For 			more information, see Using 				service-linked roles in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoadBalancer {
    ///
    /// The name of the container (as it appears in a container definition) to associate with 			the load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<cfn_resources::StrVal>,

    ///
    /// The port on the container to associate with the load balancer. This port must 			correspond to a containerPort in the task definition the tasks in the 			service are using. For tasks that use the EC2 launch type, the container 			instance they're launched on must allow ingress traffic on the hostPort of 			the port mapping.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,

    ///
    /// The name of the load balancer to associate with the Amazon ECS service or task set.
    ///
    /// A load balancer name is only specified when using a Classic Load Balancer. If you are using an Application Load Balancer 			or a Network Load Balancer the load balancer name parameter should be omitted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<cfn_resources::StrVal>,

    ///
    /// The full Amazon Resource Name (ARN) of the Elastic Load Balancing target group or groups associated with a service or 			task set.
    ///
    /// A target group ARN is only specified when using an Application Load Balancer or Network Load Balancer. If you're using a 			Classic Load Balancer, omit the target group ARN.
    ///
    /// For services using the ECS deployment controller, you can specify one or 			multiple target groups. For more information, see Registering multiple target groups with a service in 			the Amazon Elastic Container Service Developer Guide.
    ///
    /// For services using the CODE_DEPLOY deployment controller, you're required 			to define two target groups for the load balancer. For more information, see Blue/green deployment with CodeDeploy in the 			Amazon Elastic Container Service Developer Guide.
    ///
    /// ImportantIf your service's task definition uses the awsvpc network mode, you 				must choose ip as the target type, not instance. Do this 				when creating your target groups because tasks that use the awsvpc 				network mode are associated with an elastic network interface, not an Amazon EC2 				instance. This network mode is required for the Fargate launch 				type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LoadBalancer {
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

/// The network configuration for a task or service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {
    ///
    /// The VPC subnets and security groups that are associated with a task.
    ///
    /// NoteAll specified subnets and security groups must be from the same VPC.
    ///
    /// Required: No
    ///
    /// Type: AwsVpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsVpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_vpc_configuration: Option<AwsVpcConfiguration>,
}

impl cfn_resources::CfnResource for NetworkConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.aws_vpc_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A floating-point percentage of the desired number of tasks to place and keep running 			in the task set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scale {
    ///
    /// The unit of measure for the scale value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PERCENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ScaleUnitEnum>,

    ///
    /// The value, specified as a percent total of a service's desiredCount, to 			scale the task set. Accepted values are numbers between 0 and 100.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScaleUnitEnum {
    /// PERCENT
    #[serde(rename = "PERCENT")]
    Percent,
}

impl Default for ScaleUnitEnum {
    fn default() -> Self {
        ScaleUnitEnum::Percent
    }
}

impl cfn_resources::CfnResource for Scale {
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

/// The details for the service registry.
///
/// Each service may be associated with one service registry. Multiple service registries 			for each service are not supported.
///
/// When you add, update, or remove the service registries configuration, Amazon ECS starts a 			new deployment. New tasks are registered and deregistered to the updated service 			registry configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceRegistry {
    ///
    /// The container name value to be used for your service discovery service. It's already 			specified in the task definition. If the task definition that your service task 			specifies uses the bridge or host network mode, you must 			specify a containerName and containerPort combination from the 			task definition. If the task definition that your service task specifies uses the 				awsvpc network mode and a type SRV DNS record is used, you must specify 			either a containerName and containerPort combination or a 				port value. However, you can't specify both.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<cfn_resources::StrVal>,

    ///
    /// The port value to be used for your service discovery service. It's already specified 			in the task definition. If the task definition your service task specifies uses the 				bridge or host network mode, you must specify a 				containerName and containerPort combination from the task 			definition. If the task definition your service task specifies uses the 				awsvpc network mode and a type SRV DNS record is used, you must specify 			either a containerName and containerPort combination or a 				port value. However, you can't specify both.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,

    ///
    /// The port value used if your service discovery service specified an SRV record. This 			field might be used if both the awsvpc network mode and SRV records are 			used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) of the service registry. The currently supported service registry is 			AWS Cloud Map. For more information, see CreateService.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ServiceRegistry {
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
