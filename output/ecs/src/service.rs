/// The AWS::ECS::Service resource creates an Amazon Elastic Container Service (Amazon ECS) service  that runs and maintains the requested number of tasks and associated load balancers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnService {
    ///
    /// The capacity provider strategy to use for the service.
    ///
    /// If a capacityProviderStrategy is specified, the launchType 			parameter must be omitted. If no capacityProviderStrategy or 				launchType is specified, the 				defaultCapacityProviderStrategy for the cluster is used.
    ///
    /// A capacity provider strategy may contain a maximum of 6 capacity providers.
    ///
    /// Required: No
    ///
    /// Type: List of CapacityProviderStrategyItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,

    ///
    /// The short name or full Amazon Resource Name (ARN) of the cluster that you run your service on. 			If you do not specify a cluster, the default cluster is assumed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<cfn_resources::StrVal>,

    ///
    /// Optional deployment parameters that control how many tasks run during the deployment 			and the ordering of stopping and starting tasks.
    ///
    /// Required: No
    ///
    /// Type: DeploymentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,

    ///
    /// The deployment controller to use for the service. If no deployment controller is 			specified, the default value of ECS is used.
    ///
    /// Required: No
    ///
    /// Type: DeploymentController
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,

    ///
    /// The number of instantiations of the specified task definition to place and keep running in your service.
    ///
    /// For new services, if a desired count is not specified, a default value of 1 is used. When using the   DAEMON scheduling strategy, the desired count is not required.
    ///
    /// For existing services, if a desired count is not specified, it is omitted from the operation.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,

    ///
    /// Specifies whether to turn on Amazon ECS managed tags for the tasks within the service. For 			more information, see Tagging your Amazon ECS 				resources in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecsmanaged_tags: Option<bool>,

    ///
    /// Determines whether the execute command functionality is turned on for the service. If 				true, the execute command functionality is turned on for all containers 			in tasks as part of the service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,

    ///
    /// The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy 			Elastic Load Balancing target health checks after a task has first started. This is only used when your 			service is configured to use a load balancer. If your service has a load balancer 			defined and you don't specify a health check grace period value, the default value of 				0 is used.
    ///
    /// If you do not use an Elastic Load Balancing, we recommend that you use the startPeriod in 			the task definition health check parameters. For more information, see Health 				check.
    ///
    /// If your service's tasks take a while to start and respond to Elastic Load Balancing health checks, you 			can specify a health check grace period of up to 			2,147,483,647 			seconds (about 69 years). During that time, the Amazon ECS service 			scheduler ignores health check status. This grace period can prevent the service 			scheduler from marking tasks as unhealthy and stopping them before they have time to 			come up.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,

    ///
    /// The launch type on which to run your service. For more information, see Amazon ECS Launch Types in the Amazon   Elastic Container Service Developer Guide.
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
    pub launch_type: Option<ServiceLaunchTypeEnum>,

    ///
    /// A list of load balancer objects to associate with the service. If you specify the Role property,   LoadBalancers must be specified as well. For information about the number of load balancers that you  can specify per service, see Service Load Balancing in the   Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of LoadBalancer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,

    ///
    /// The network configuration for the service. This parameter is required for task definitions that use the   awsvpc network mode to receive their own elastic network interface, and it is not supported for other  network modes. For more information, see Task Networking in the Amazon Elastic   Container Service Developer Guide.
    ///
    /// Required: Conditional
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    ///
    /// An array of placement constraint objects to use for tasks in your service. You can 			specify a maximum of 10 constraints for each task. This limit includes constraints in 			the task definition and those specified at runtime.
    ///
    /// Required: No
    ///
    /// Type: List of PlacementConstraint
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,

    ///
    /// The placement strategy objects to use for tasks in your service. You can specify a 			maximum of 5 strategy rules for each service.
    ///
    /// Required: No
    ///
    /// Type: List of PlacementStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementStrategies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategies: Option<Vec<PlacementStrategy>>,

    ///
    /// The platform version that your tasks in the service are running on. A platform version 			is specified only for tasks using the Fargate launch type. If one isn't 			specified, the LATEST platform version is used. For more information, see 				AWS Fargate platform 				versions in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to propagate the tags from the task definition to the task. If no 			value is specified, the tags aren't propagated. Tags can only be propagated to the task 			during task creation. To add tags to a task after task creation, use the TagResource API action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SERVICE | TASK_DEFINITION
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<ServicePropagateTagsEnum>,

    ///
    /// The name or full Amazon Resource Name (ARN) of the IAM role that allows Amazon ECS to make calls to your 			load balancer on your behalf. This parameter is only permitted if you are using a load 			balancer with your service and your task definition doesn't use the awsvpc 			network mode. If you specify the role parameter, you must also specify a 			load balancer object with the loadBalancers parameter.
    ///
    /// ImportantIf your account has already created the Amazon ECS service-linked role, that role is 				used for your service unless you specify a role here. The service-linked role is 				required if your task definition uses the awsvpc network mode or if the 				service is configured to use service discovery, an external deployment controller, 				multiple target groups, or Elastic Inference accelerators in which case you don't 				specify a role here. For more information, see Using 					service-linked roles for Amazon ECS in the Amazon Elastic Container Service Developer Guide.
    ///
    /// If your specified role has a path other than /, then you must either 			specify the full role ARN (this is recommended) or prefix the role name with the path. 			For example, if a role with the name bar has a path of /foo/ 			then you would specify /foo/bar as the role name. For more information, see 				Friendly names and paths in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<cfn_resources::StrVal>,

    ///
    /// The scheduling strategy to use for the service. For more information, see Services.
    ///
    /// There are two service scheduler strategies available:
    ///
    /// REPLICA-The replica scheduling strategy places and 					maintains the desired number of tasks across your cluster. By default, the 					service scheduler spreads tasks across Availability Zones. You can use task 					placement strategies and constraints to customize task placement decisions. This 					scheduler strategy is required if the service uses the CODE_DEPLOY 					or EXTERNAL deployment controller types.                        DAEMON-The daemon scheduling strategy deploys exactly one 					task on each active container instance that meets all of the task placement 					constraints that you specify in your cluster. The service scheduler also 					evaluates the task placement constraints for running tasks and will stop tasks 					that don't meet the placement constraints. When you're using this strategy, you 					don't need to specify a desired number of tasks, a task placement strategy, or 					use Service Auto Scaling policies.        NoteTasks using the Fargate launch type or the 							CODE_DEPLOY or EXTERNAL deployment controller 						types don't support the DAEMON scheduling strategy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAEMON | REPLICA
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<ServiceSchedulingStrategyEnum>,

    ///
    /// The configuration for this service to discover and connect to 	services, and be discovered by, and connected from, other services within a namespace.
    ///
    /// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ServiceConnectConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceConnectConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,

    ///
    /// The name of your service. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. Service names must be unique within 			a cluster, but you can have similarly named services in multiple clusters within a 			Region or across multiple Regions.
    ///
    /// ImportantThe stack update fails if you change any properties that require replacement and the       ServiceName is configured. This is because AWS CloudFormation       creates the replacement service first, but each ServiceName must be unique in       the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<cfn_resources::StrVal>,

    ///
    /// The details of the service discovery registry to associate with this service. For more 			information, see Service 				discovery.
    ///
    /// NoteEach service may be associated with one service registry. Multiple service 				registries for each service isn't supported.
    ///
    /// Required: No
    ///
    /// Type: List of ServiceRegistry
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,

    ///
    /// The metadata that you apply to the service to help you categorize and organize them. 			Each tag consists of a key and an optional value, both of which you define. When a 			service is deleted, the tags are deleted as well.
    ///
    /// The following basic restrictions apply to tags:
    ///
    /// Maximum number of tags per resource - 50               For each resource, each tag key must be unique, and each tag key can have only           one value.               Maximum key length - 128 Unicode characters in UTF-8               Maximum value length - 256 Unicode characters in UTF-8               If your tagging schema is used across multiple services and resources,           remember that other services may have restrictions on allowed characters.           Generally allowed characters are: letters, numbers, and spaces representable in           UTF-8, and the following characters: + - = . _ : / @.               Tag keys and values are case-sensitive.               Do not use aws:, AWS:, or any upper or lowercase           combination of such as a prefix for either keys or values as it is reserved for           AWS use. You cannot edit or delete tag keys or values with this prefix. Tags with           this prefix do not count against your tags per resource limit.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The family and revision (family:revision) or 			full ARN of the task definition to run in your service. If a revision 			isn't specified, the latest ACTIVE revision is used.
    ///
    /// A task definition must be specified if the service uses either the ECS or 				CODE_DEPLOY deployment controllers.
    ///
    /// For more information about deployment types, see Amazon ECS deployment types.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_name: CfnServicename,

    #[serde(skip_serializing)]
    pub att_service_arn: CfnServiceservicearn,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ServiceLaunchTypeEnum {
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

impl Default for ServiceLaunchTypeEnum {
    fn default() -> Self {
        ServiceLaunchTypeEnum::Ec2
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ServicePropagateTagsEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SERVICE
    #[serde(rename = "SERVICE")]
    Service,

    /// TASK_DEFINITION
    #[serde(rename = "TASK_DEFINITION")]
    Taskdefinition,
}

impl Default for ServicePropagateTagsEnum {
    fn default() -> Self {
        ServicePropagateTagsEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ServiceSchedulingStrategyEnum {
    /// DAEMON
    #[serde(rename = "DAEMON")]
    Daemon,

    /// REPLICA
    #[serde(rename = "REPLICA")]
    Replica,
}

impl Default for ServiceSchedulingStrategyEnum {
    fn default() -> Self {
        ServiceSchedulingStrategyEnum::Daemon
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServicename;
impl CfnServicename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceservicearn;
impl CfnServiceservicearn {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceArn"#
    }
}

impl cfn_resources::CfnResource for CfnService {
    fn type_string(&self) -> &'static str {
        "AWS::ECS::Service"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.deployment_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.deployment_controller
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.service_connect_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The IDs of the subnets associated with the task or service. There's a limit of 16 			subnets that can be specified per AwsVpcConfiguration.
    ///
    /// NoteAll specified subnets must be from the same VPC.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
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

/// The details of a capacity provider strategy. A capacity provider strategy can be set when using the   RunTask or CreateService APIs or as the default capacity provider strategy for a cluster  with the CreateCluster API.
///
/// Only capacity providers that are already associated with a cluster and have an ACTIVE or   UPDATING status can be used in a capacity provider strategy. The   PutClusterCapacityProviders API is used to associate a capacity provider with a cluster.
///
/// If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be  created. New Auto Scaling group capacity providers can be created with the CreateCapacityProvider API  operation.
///
/// To use an AWS Fargate capacity provider, specify either the FARGATE or   FARGATE_SPOT capacity providers. The AWS Fargate capacity providers are available to  all accounts and only need to be associated with a cluster to be used in a capacity provider strategy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityProviderStrategyItem {
    ///
    /// The base value designates how many tasks, at a minimum, to run on 			the specified capacity provider. Only one capacity provider in a capacity provider 			strategy can have a base defined. If no value is specified, the 			default value of 0 is used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,

    ///
    /// The short name of the capacity provider.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<cfn_resources::StrVal>,

    ///
    /// The weight value designates the relative percentage of the total 			number of tasks launched that should use the specified capacity provider. The 				weight value is taken into consideration after the base 			value, if defined, is satisfied.
    ///
    /// If no weight value is specified, the default value of 0 is 			used. When multiple capacity providers are specified within a capacity provider 			strategy, at least one of the capacity providers must have a weight value greater than 			zero and any capacity providers with a weight of 0 can't be used to place 			tasks. If you specify multiple capacity providers in a strategy that all have a weight 			of 0, any RunTask or CreateService actions using 			the capacity provider strategy will fail.
    ///
    /// An example scenario for using weights is defining a strategy that contains two 			capacity providers and both have a weight of 1, then when the 				base is satisfied, the tasks will be split evenly across the two 			capacity providers. Using that same logic, if you specify a weight of 1 for 				capacityProviderA and a weight of 4 for 				capacityProviderB, then for every one task that's run using 				capacityProviderA, four tasks would use 				capacityProviderB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

impl cfn_resources::CfnResource for CapacityProviderStrategyItem {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.base {
            if *the_val > 100000 as _ {
                return Err(format!(
                    "Max validation failed on field 'base'. {} is greater than 100000",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.base {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'base'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.weight {
            if *the_val > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'weight'. {} is greater than 1000",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.weight {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'weight'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// One of the methods which provide a way for you to quickly identify when a deployment 			has failed, and then to optionally roll back the failure to the last working 			deployment.
///
/// When the alarms are generated, Amazon ECS sets the service deployment to failed. Set the rollback 			parameter to have Amazon ECS to roll back your service to the last completed deployment 			after a failure.
///
/// You can only use the DeploymentAlarms method to detect failures when the 				DeploymentController is set to ECS (rolling 			update).
///
/// For more information, see Rolling 				update in the         Amazon Elastic Container Service Developer Guide       .
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentAlarms {
    ///
    /// One or more CloudWatch alarm names. Use a "," to separate the alarms.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmNames")]
    pub alarm_names: Vec<String>,

    ///
    /// Determines whether to use the CloudWatch alarm option in the service deployment process.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: bool,

    ///
    /// Determines whether to configure Amazon ECS to roll back the service if a service deployment 			fails. If rollback is used, when a service deployment fails, the service is rolled back 			to the last deployment that completed successfully.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rollback")]
    pub rollback: bool,
}

impl cfn_resources::CfnResource for DeploymentAlarms {
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

/// The deployment circuit breaker determines whether a 			service deployment will fail if the service can't reach a steady state. If it is turned on, a 			service deployment will transition to a failed state and stop launching new tasks. You 			can also configure Amazon ECS to roll back your service to the last completed deployment 			after a failure. For more information, see Rolling 				update in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentCircuitBreaker {
    ///
    /// Determines whether to use the deployment circuit breaker logic for the service.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: bool,

    ///
    /// Determines whether to configure Amazon ECS to roll back the service if a service deployment 			fails. If rollback is on, when a service deployment fails, the service is rolled back to 			the last deployment that completed successfully.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rollback")]
    pub rollback: bool,
}

impl cfn_resources::CfnResource for DeploymentCircuitBreaker {
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

/// The DeploymentConfiguration property specifies optional deployment parameters that control how many  tasks run during the deployment and the ordering of stopping and starting tasks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentConfiguration {
    ///
    /// Information about the CloudWatch alarms.
    ///
    /// Required: No
    ///
    /// Type: DeploymentAlarms
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<DeploymentAlarms>,

    ///
    /// NoteThe deployment circuit breaker can only be used for services using the rolling 				update (ECS) deployment type.
    ///
    /// The deployment circuit breaker determines whether a 			service deployment will fail if the service can't reach a steady state. If you use the deployment 			circuit breaker, a service deployment will transition to a failed state and 			stop launching new tasks. If you use the rollback option, when a service deployment fails, the 			service is rolled back to the last deployment that completed successfully. For more information, see Rolling 				update in the Amazon Elastic Container Service Developer 					Guide
    ///
    /// Required: No
    ///
    /// Type: DeploymentCircuitBreaker
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentCircuitBreaker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_circuit_breaker: Option<DeploymentCircuitBreaker>,

    ///
    /// If a service is using the rolling update (ECS) deployment type, the 				maximumPercent parameter represents an upper limit on the number of 			your service's tasks that are allowed in the RUNNING or 				PENDING state during a deployment, as a percentage of the 				desiredCount (rounded down to the nearest integer). This parameter 			enables you to define the deployment batch size. For example, if your service is using 			the REPLICA service scheduler and has a desiredCount of four 			tasks and a maximumPercent value of 200%, the scheduler may start four new 			tasks before stopping the four older tasks (provided that the cluster resources required 			to do this are available). The default maximumPercent value for a service 			using the REPLICA service scheduler is 200%.
    ///
    /// If a service is using either the blue/green (CODE_DEPLOY) or 				EXTERNAL deployment types and tasks that use the EC2 			launch type, the maximum percent value is set to the 			default value and is used to define the upper limit on the number of the tasks in the 			service that remain in the RUNNING state while the container instances are 			in the DRAINING state. If the tasks in the service use the 			Fargate launch type, the maximum percent value is not used, although it is 			returned when describing your service.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<i64>,

    ///
    /// If a service is using the rolling update (ECS) deployment type, the 				minimumHealthyPercent represents a lower limit on the number of your 			service's tasks that must remain in the RUNNING state during a deployment, 			as a percentage of the desiredCount (rounded up to the nearest integer). 			This parameter enables you to deploy without using additional cluster capacity. For 			example, if your service has a desiredCount of four tasks and a 				minimumHealthyPercent of 50%, the service scheduler may stop two 			existing tasks to free up cluster capacity before starting two new tasks.
    ///
    /// For services that do not use a load balancer, the following 			should be noted:
    ///
    /// A service is considered healthy if all essential containers within the tasks 					in the service pass their health checks.               If a task has no essential containers with a health check defined, the service 					scheduler will wait for 40 seconds after a task reaches a RUNNING 					state before the task is counted towards the minimum healthy percent 					total.               If a task has one or more essential containers with a health check defined, 					the service scheduler will wait for the task to reach a healthy status before 					counting it towards the minimum healthy percent total. A task is considered 					healthy when all essential containers within the task have passed their health 					checks. The amount of time the service scheduler can wait for is determined by 					the container health check settings.
    ///
    /// For services are that do use a load balancer, the following 			should be noted:
    ///
    /// If a task has no essential containers with a health check defined, the service 					scheduler will wait for the load balancer target group health check to return a 					healthy status before counting the task towards the minimum healthy percent 					total.               If a task has an essential container with a health check defined, the service 					scheduler will wait for both the task to reach a healthy status and the load 					balancer target group health check to return a healthy status before counting 					the task towards the minimum healthy percent total.
    ///
    /// If a service is using either the blue/green (CODE_DEPLOY) or 				EXTERNAL deployment types and is running tasks that use the 			EC2 launch type, the minimum healthy 				percent value is set to the default value and is used to define the lower 			limit on the number of the tasks in the service that remain in the RUNNING 			state while the container instances are in the DRAINING state. If a service 			is using either the blue/green (CODE_DEPLOY) or EXTERNAL 			deployment types and is running tasks that use the Fargate launch type, 			the minimum healthy percent value is not used, although it is returned when describing 			your service.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumHealthyPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_percent: Option<i64>,
}

impl cfn_resources::CfnResource for DeploymentConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.alarms.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.deployment_circuit_breaker
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The deployment controller to use for the service. For more information, see Amazon ECS deployment types in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentController {
    ///
    /// The deployment controller type to use. There are three deployment controller types available:
    ///
    /// ECS      The rolling update (ECS) deployment type involves replacing the current running version of the    container with the latest version. The number of containers Amazon ECS adds or removes from the service during a    rolling update is controlled by adjusting the minimum and maximum number of healthy tasks allowed during a service    deployment, as specified in the DeploymentConfiguration.        CODE_DEPLOY      The blue/green (CODE_DEPLOY) deployment type uses the blue/green deployment model powered by    AWS CodeDeploy, which allows you to verify a new deployment of a service before sending production    traffic to it.        EXTERNAL      The external (EXTERNAL) deployment type enables you to use any third-party deployment controller    for full control over the deployment process for an Amazon ECS service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CODE_DEPLOY | ECS | EXTERNAL
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<DeploymentControllerTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DeploymentControllerTypeEnum {
    /// CODE_DEPLOY
    #[serde(rename = "CODE_DEPLOY")]
    Codedeploy,

    /// ECS
    #[serde(rename = "ECS")]
    Ecs,

    /// EXTERNAL
    #[serde(rename = "EXTERNAL")]
    External,
}

impl Default for DeploymentControllerTypeEnum {
    fn default() -> Self {
        DeploymentControllerTypeEnum::Codedeploy
    }
}

impl cfn_resources::CfnResource for DeploymentController {
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

/// The LoadBalancer property specifies details on a load balancer that is used with a service.
///
/// If the service is using the CODE_DEPLOY deployment controller, the service is required to use  either an Application Load Balancer or Network Load Balancer. When you are creating an AWS CodeDeploy  deployment group, you specify two target groups (referred to as a targetGroupPair). Each target group  binds to a separate task set in the deployment. The load balancer can also have up to two listeners, a required  listener for production traffic and an optional listener that allows you to test new revisions of the service before  routing production traffic to it.
///
/// Services with tasks that use the awsvpc network mode (for example, those with the Fargate launch  type) only support Application Load Balancers and Network Load Balancers. Classic Load Balancers are not supported.  Also, when you create any target groups for these services, you must choose ip as the target type, not   instance. Tasks that use the awsvpc network mode are associated with an elastic network  interface, not an Amazon EC2 instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoadBalancer {
    ///
    /// The name of the container (as it appears in a container definition) to associate with 			the load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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

/// The log configuration for the container. This parameter maps to LogConfig 			in the Create a container section of the Docker Remote API and the 				--log-driver option to docker 					run.
///
/// By default, containers use the same logging driver that the Docker daemon uses. 			However, the container might use a different logging driver than the Docker daemon by 			specifying a log driver configuration in the container definition. For more information 			about the options for different supported log drivers, see Configure logging 				drivers in the Docker documentation.
///
/// Understand the following when specifying a log configuration for your 			containers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogConfiguration {
    ///
    /// The log driver to use for the container.
    ///
    /// For tasks on AWS Fargate, the supported log drivers are awslogs, 				splunk, and awsfirelens.
    ///
    /// For tasks hosted on Amazon EC2 instances, the supported log drivers are 				awslogs, fluentd, gelf, 				json-file, journald, 				logentries,syslog, splunk, and 				awsfirelens.
    ///
    /// For more information about using the awslogs log driver, see Using 				the awslogs log driver in the Amazon Elastic Container Service Developer Guide.
    ///
    /// For more information about using the awsfirelens log driver, see Custom log routing in the Amazon Elastic Container Service Developer Guide.
    ///
    /// NoteIf you have a custom driver that isn't listed, you can fork the Amazon ECS container 				agent project that's available 					on GitHub and customize it to work with that driver. We encourage you to 				submit pull requests for changes that you would like to have included. However, we 				don't currently provide support for running modified copies of this software.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: awsfirelens | awslogs | fluentd | gelf | journald | json-file | splunk | syslog
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<LogConfigurationLogDriverEnum>,

    ///
    /// The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,

    ///
    /// The secrets to pass to the log configuration. For more information, see Specifying 				sensitive data in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Secret
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LogConfigurationLogDriverEnum {
    /// awsfirelens
    #[serde(rename = "awsfirelens")]
    Awsfirelens,

    /// awslogs
    #[serde(rename = "awslogs")]
    Awslogs,

    /// fluentd
    #[serde(rename = "fluentd")]
    Fluentd,

    /// gelf
    #[serde(rename = "gelf")]
    Gelf,

    /// journald
    #[serde(rename = "journald")]
    Journald,

    /// json-file
    #[serde(rename = "json-file")]
    Jsonfile,

    /// splunk
    #[serde(rename = "splunk")]
    Splunk,

    /// syslog
    #[serde(rename = "syslog")]
    Syslog,
}

impl Default for LogConfigurationLogDriverEnum {
    fn default() -> Self {
        LogConfigurationLogDriverEnum::Awsfirelens
    }
}

impl cfn_resources::CfnResource for LogConfiguration {
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

/// The NetworkConfiguration property specifies an object representing the network configuration for a  task or service.
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
    /// Update requires: No interruption
    #[serde(rename = "AwsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

impl cfn_resources::CfnResource for NetworkConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.awsvpc_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The PlacementConstraint property specifies an object representing a constraint on task placement in  the task definition. For more information, see Task Placement Constraints in the   Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PlacementConstraint {
    ///
    /// A cluster query language expression to apply to the constraint. The expression can 			have a maximum length of 2000 characters. You can't specify an expression if the 			constraint type is distinctInstance. For more information, see Cluster query language in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<cfn_resources::StrVal>,

    ///
    /// The type of constraint. Use distinctInstance to ensure that each task in 			a particular group is running on a different container instance. Use 				memberOf to restrict the selection to a group of valid 			candidates.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: distinctInstance | memberOf
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: PlacementConstraintTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PlacementConstraintTypeEnum {
    /// distinctInstance
    #[serde(rename = "distinctInstance")]
    Distinctinstance,

    /// memberOf
    #[serde(rename = "memberOf")]
    Memberof,
}

impl Default for PlacementConstraintTypeEnum {
    fn default() -> Self {
        PlacementConstraintTypeEnum::Distinctinstance
    }
}

impl cfn_resources::CfnResource for PlacementConstraint {
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

/// The PlacementStrategy property specifies the task placement strategy for a task or service. For  more information, see Task Placement Strategies in the   Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PlacementStrategy {
    ///
    /// The field to apply the placement strategy against. For the spread placement strategy, valid values  are instanceId (or host, which has the same effect), or any platform or custom attribute  that is applied to a container instance, such as attribute:ecs.availability-zone. For the   binpack placement strategy, valid values are CPU and MEMORY. For the   random placement strategy, this field is not used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The type of placement strategy. The random placement strategy randomly 			places tasks on available candidates. The spread placement strategy spreads 			placement across available candidates evenly based on the field parameter. 			The binpack strategy places tasks on available candidates that have the 			least available amount of the resource that's specified with the field 			parameter. For example, if you binpack on memory, a task is placed on the instance with 			the least amount of remaining memory but still enough to run the task.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: binpack | random | spread
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: PlacementStrategyTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PlacementStrategyTypeEnum {
    /// binpack
    #[serde(rename = "binpack")]
    Binpack,

    /// random
    #[serde(rename = "random")]
    Random,

    /// spread
    #[serde(rename = "spread")]
    Spread,
}

impl Default for PlacementStrategyTypeEnum {
    fn default() -> Self {
        PlacementStrategyTypeEnum::Binpack
    }
}

impl cfn_resources::CfnResource for PlacementStrategy {
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

/// An object representing the secret to expose to your container. Secrets can be exposed 			to a container in the following ways:
///
/// For more information, see Specifying 				sensitive data in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Secret {
    ///
    /// The name of the secret.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The secret to expose to the container. The supported values are either the full ARN 			of the AWS Secrets Manager secret or the full ARN of the parameter in the SSM Parameter 			Store.
    ///
    /// For information about the require AWS Identity and Access Management permissions, see Required IAM permissions for Amazon ECS secrets (for Secrets Manager) or Required IAM permissions for Amazon ECS secrets (for Systems Manager Parameter 			store) in the Amazon Elastic Container Service Developer Guide.
    ///
    /// NoteIf the SSM Parameter Store parameter exists in the same Region as the task 				you're launching, then you can use either the full ARN or name of the parameter. 				If the parameter exists in a different Region, then the full ARN must be 				specified.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueFrom")]
    pub value_from: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Secret {
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

/// Each alias ("endpoint") is a fully-qualified name and port number that other tasks 			("clients") can use to connect to this service.
///
/// Each name and port mapping must be unique within the namespace.
///
/// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceConnectClientAlias {
    ///
    /// The dnsName is the name that you use in the applications of client tasks 			to connect to this service. The name must be a valid DNS name but doesn't need to be 			fully-qualified. The name can include up to 127 characters. The name can include 			lowercase letters, numbers, underscores (_), hyphens (-), and periods (.). The name 			can't start with a hyphen.
    ///
    /// If this parameter isn't specified, the default value of discoveryName.namespace is used. If the discoveryName isn't specified, the port mapping name from the task definition is used in portName.namespace.
    ///
    /// To avoid changing your applications in client Amazon ECS services, set this to the same 			name that the client application uses by default. For example, a few common names are 				database, db, or the lowercase name of a database, such as 				mysql or redis. For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<cfn_resources::StrVal>,

    ///
    /// The listening port number for the Service Connect proxy. This port is available 			inside of all of the tasks within the same namespace.
    ///
    /// To avoid changing your applications in client Amazon ECS services, set this to the same 			port that the client application uses by default. For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: i64,
}

impl cfn_resources::CfnResource for ServiceConnectClientAlias {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// The Service Connect configuration of your Amazon ECS service. The configuration for this 			service to discover and connect to services, and be discovered by, and connected from, 			other services within a namespace.
///
/// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceConnectConfiguration {
    ///
    /// Specifies whether to use Service Connect with this service.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The log configuration for the container. This parameter maps to LogConfig 			in the Create a container section of the Docker Remote API and the 				--log-driver option to docker 					run.
    ///
    /// By default, containers use the same logging driver that the Docker daemon uses. 			However, the container might use a different logging driver than the Docker daemon by 			specifying a log driver configuration in the container definition. For more information 			about the options for different supported log drivers, see Configure logging 				drivers in the Docker documentation.
    ///
    /// Understand the following when specifying a log configuration for your 			containers.
    ///
    /// Amazon ECS currently supports a subset of the logging drivers available to the 					Docker daemon (shown in the valid values below). Additional log drivers may be 					available in future releases of the Amazon ECS container agent.               This parameter requires version 1.18 of the Docker Remote API or greater on 					your container instance.               For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must 					register the available logging drivers with the 						ECS_AVAILABLE_LOGGING_DRIVERS environment variable before 					containers placed on that instance can use these log configuration options. For 					more information, see Amazon ECS container agent configuration in the 					Amazon Elastic Container Service Developer Guide.               For tasks that are on AWS Fargate, because you don't have access to the 					underlying infrastructure your tasks are hosted on, any additional software 					needed must be installed outside of the task. For example, the Fluentd output 					aggregators or a remote host running Logstash to send Gelf logs to.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,

    ///
    /// The namespace name or full Amazon Resource Name (ARN) of the AWS Cloud Map namespace for use with Service Connect. The namespace must be in 			the same AWS Region as the Amazon ECS service and cluster. The type of namespace doesn't 			affect Service Connect. For more information about AWS Cloud Map, see Working with Services in the 			        AWS Cloud Map Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,

    ///
    /// The list of Service Connect service objects. These are names and aliases (also known 			as endpoints) that are used by other Amazon ECS services to connect to this service.
    ///
    /// This field is not required for a "client" Amazon ECS service that's a member of a namespace 			only to connect to other services within the namespace. An example of this would be a 			frontend application that accepts incoming requests from either a load balancer that's 			attached to the service or by other means.
    ///
    /// An object selects a port from the task definition, assigns a name for the AWS Cloud Map 			service, and a list of aliases (endpoints) and ports for client applications to refer to 			this service.
    ///
    /// Required: No
    ///
    /// Type: List of ServiceConnectService
    ///
    /// Update requires: No interruption
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceConnectService>>,
}

impl cfn_resources::CfnResource for ServiceConnectConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.log_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Service Connect service object configuration. For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceConnectService {
    ///
    /// The list of client aliases for this Service Connect service. You use these to assign 			names that can be used by client applications. The maximum number of client aliases that 			you can have in this list is 1.
    ///
    /// Each alias ("endpoint") is a fully-qualified name and port number that other Amazon ECS 			tasks ("clients") can use to connect to this service.
    ///
    /// Each name and port mapping must be unique within the namespace.
    ///
    /// For each ServiceConnectService, you must provide at least one 				clientAlias with one port.
    ///
    /// Required: No
    ///
    /// Type: List of ServiceConnectClientAlias
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_aliases: Option<Vec<ServiceConnectClientAlias>>,

    ///
    /// The discoveryName is the name of the new AWS Cloud Map service that Amazon ECS creates 			for this Amazon ECS service. This must be unique within the AWS Cloud Map namespace. The name can contain up to 64 characters. The name can include lowercase letters, 			numbers, underscores (_), and hyphens (-). The name can't start with a hyphen.
    ///
    /// If the discoveryName isn't specified, the port mapping name from the task definition is used in portName.namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DiscoveryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_name: Option<cfn_resources::StrVal>,

    ///
    /// The port number for the Service Connect proxy to listen on.
    ///
    /// Use the value of this field to bypass the proxy for traffic on the port number 			specified in the named portMapping in the task definition of this 			application, and then use it in your VPC security groups to allow traffic into the proxy 			for this Amazon ECS service.
    ///
    /// In awsvpc mode and Fargate, the default value is the container port 			number. The container port number is in the portMapping in the task 			definition. In bridge mode, the default value is the ephemeral port of the 			Service Connect proxy.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressPortOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_port_override: Option<i64>,

    ///
    /// The portName must match the name of one of the portMappings 			from all the containers in the task definition of this Amazon ECS service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortName")]
    pub port_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ServiceConnectService {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ingress_port_override {
            if *the_val > 65535 as _ {
                return Err(format!("Max validation failed on field 'ingress_port_override'. {} is greater than 65535", the_val));
            }
        }

        if let Some(the_val) = &self.ingress_port_override {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'ingress_port_override'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The ServiceRegistry property specifies details of the service registry. For more information, see   Service Discovery  in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceRegistry {
    ///
    /// The container name value to be used for your service discovery service. It's already 			specified in the task definition. If the task definition that your service task 			specifies uses the bridge or host network mode, you must 			specify a containerName and containerPort combination from the 			task definition. If the task definition that your service task specifies uses the 				awsvpc network mode and a type SRV DNS record is used, you must specify 			either a containerName and containerPort combination or a 				port value. However, you can't specify both.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
