

/// The AWS::ECS::Cluster resource creates an Amazon Elastic Container Service (Amazon ECS)  cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {


    /// 
    /// The settings to use when creating a cluster. This parameter is used to turn on CloudWatch 			Container Insights for a cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of ClusterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterSettings")]
    pub cluster_settings: Option<Vec<ClusterSettings>>,


    /// 
    /// The default capacity provider strategy for the cluster. When services or tasks are run 			in the cluster with no launch type or capacity provider strategy specified, the default 			capacity provider strategy is used.
    /// 
    /// Required: No
    ///
    /// Type: List of CapacityProviderStrategyItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,


    /// 
    /// Use this parameter to set a default Service Connect namespace. After you set a default 	Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as 	client services in the namespace. This setting only applies to new services that set the enabled parameter to 	true in the ServiceConnectConfiguration. 	You can set the namespace of each service individually in the ServiceConnectConfiguration to override this default 	parameter.
    /// 
    /// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: ServiceConnectDefaults
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceConnectDefaults")]
    pub service_connect_defaults: Option<ServiceConnectDefaults>,


    /// 
    /// A user-generated string that you use to identify your cluster. If you don't specify a name, AWS CloudFormation generates a unique physical ID for the name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,


    /// 
    /// The short name of one or more capacity providers to associate with the cluster. A 			capacity provider must be associated with a cluster before it can be included as part of 			the default capacity provider strategy of the cluster or used in a capacity provider 			strategy when calling the CreateService or RunTask 			actions.
    /// 
    /// If specifying a capacity provider that uses an Auto Scaling group, the capacity 			provider must be created but not associated with another cluster. New Auto Scaling group 			capacity providers can be created with the CreateCapacityProvider API 			operation.
    /// 
    /// To use a AWS Fargate capacity provider, specify either the FARGATE or 				FARGATE_SPOT capacity providers. The AWS Fargate capacity providers are 			available to all accounts and only need to be associated with a cluster to be 			used.
    /// 
    /// The PutCapacityProvider API operation is used to update the 			list of available capacity providers for a cluster after the cluster is created.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProviders")]
    pub capacity_providers: Option<Vec<String>>,


    /// 
    /// The metadata that you apply to the cluster to help you categorize and organize them. 			Each tag consists of a key and an optional value. You define both.
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
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The execute command configuration for the cluster.
    /// 
    /// Required: No
    ///
    /// Type: ClusterConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: Option<ClusterConfiguration>,

}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string() -> &'static str {
        "AWS::ECS::Cluster"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The details of the execute command configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExecuteCommandConfiguration {


    /// 
    /// Specify an AWS Key Management Service key ID to encrypt the data between the local client 			and the container.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The log setting to use for redirecting logs for your execute command results. The 			following log settings are available.
    /// 
    /// NONE: The execute command session is not logged.                        DEFAULT: The awslogs configuration in the task 					definition is used. If no logging parameter is specified, it defaults to this 					value. If no awslogs log driver is configured in the task 					definition, the output won't be logged.                        OVERRIDE: Specify the logging details as a part of 						logConfiguration. If the OVERRIDE logging option 					is specified, the logConfiguration is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT | NONE | OVERRIDE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    pub logging: Option<String>,


    /// 
    /// The log configuration for the results of the execute command actions. The logs can be 			sent to CloudWatch Logs or an Amazon S3 bucket. When logging=OVERRIDE is 			specified, a logConfiguration must be provided.
    /// 
    /// Required: No
    ///
    /// Type: ExecuteCommandLogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<ExecuteCommandLogConfiguration>,

}


/// The settings to use when creating a cluster. This parameter is used to turn on CloudWatch 			Container Insights for a cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusterSettings {


    /// 
    /// The value to set for the cluster setting. The supported values are enabled and 				disabled.
    /// 
    /// If you set name to containerInsights and value 			to enabled, CloudWatch Container Insights will be on for the cluster, otherwise 			it will be off unless the containerInsights account setting is turned on. 			If a cluster value is specified, it will override the containerInsights 			value set with PutAccountSetting or PutAccountSettingDefault.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The name of the cluster setting. The value is containerInsights .
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: containerInsights
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The execute command configuration for the cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusterConfiguration {


    /// 
    /// The details of the execute command configuration.
    /// 
    /// Required: No
    ///
    /// Type: ExecuteCommandConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecuteCommandConfiguration")]
    pub execute_command_configuration: Option<ExecuteCommandConfiguration>,

}


/// Use this parameter to set a default Service Connect namespace. After you set a default 	Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as 	client services in the namespace. This setting only applies to new services that set the enabled parameter to 	true in the ServiceConnectConfiguration. 	You can set the namespace of each service individually in the ServiceConnectConfiguration to override this default 	parameter.
///
/// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceConnectDefaults {


    /// 
    /// The namespace name or full Amazon Resource Name (ARN) of the AWS Cloud Map namespace that's used when you create a service and don't specify 			a Service Connect configuration. The namespace name can include up to 1024 characters. 			The name is case-sensitive. The name can't include hyphens (-), tilde (~), greater than 			(>), less than (<), or slash (/).
    /// 
    /// If you enter an existing namespace name or ARN, then that namespace will be used. 			Any namespace type is supported. The namespace must be in this account and this AWS 			Region.
    /// 
    /// If you enter a new name, a AWS Cloud Map namespace will be created. Amazon ECS creates a 			AWS Cloud Map namespace with the "API calls" method of instance discovery only. This instance 			discovery method is the "HTTP" namespace type in the AWS Command Line Interface. Other types of instance 			discovery aren't used by Service Connect.
    /// 
    /// If you update the service with an empty string "" for the namespace name, 			the cluster configuration for Service Connect is removed. Note that the namespace will 			remain in AWS Cloud Map and must be deleted separately.
    /// 
    /// For more information about AWS Cloud Map, see Working 				with Services in the         AWS Cloud Map Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The log configuration for the results of the execute command actions. The logs can be 			sent to CloudWatch Logs or an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExecuteCommandLogConfiguration {


    /// 
    /// The name of the CloudWatch log group to send logs to.
    /// 
    /// NoteThe CloudWatch log group must already be created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupName")]
    pub cloud_watch_log_group_name: Option<String>,


    /// 
    /// An optional folder in the S3 bucket to place logs in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,


    /// 
    /// Determines whether to use encryption on the CloudWatch logs. If not specified, encryption 			will be off.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchEncryptionEnabled")]
    pub cloud_watch_encryption_enabled: Option<bool>,


    /// 
    /// Determines whether to use encryption on the S3 logs. If not specified, encryption is 			not used.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3EncryptionEnabled")]
    pub s3_encryption_enabled: Option<bool>,


    /// 
    /// The name of the S3 bucket to send logs to.
    /// 
    /// NoteThe S3 bucket must already be created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,

}


/// The CapacityProviderStrategyItem property specifies the details of the default capacity provider  strategy for the cluster. When services or tasks are run in the cluster with no launch type or capacity provider  strategy specified, the default capacity provider strategy is used.
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
    pub base: Option<i64>,


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
    pub weight: Option<i64>,


    /// 
    /// The short name of the capacity provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: Option<String>,

}
