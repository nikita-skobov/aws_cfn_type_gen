

/// The AWS::AppRunner::Service resource is an AWS App Runner resource type that specifies an App Runner service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnService {


    /// 
    /// The Amazon Resource Name (ARN) of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner    associates the latest revision of a default auto scaling configuration.
    /// 
    /// Specify an ARN with a name and a revision number to associate that revision. For example:      arn:aws:apprunner:us-east-1:123456789012:autoscalingconfiguration/high-availability/3
    /// 
    /// Specify just the name to associate the latest revision. For example:     arn:aws:apprunner:us-east-1:123456789012:autoscalingconfiguration/high-availability
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: arn:aws(-[\w]+)*:[a-z0-9-\\.]{0,63}:[a-z0-9-\\.]{0,63}:[0-9]{12}:(\w|\/|-){1,1011}
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingConfigurationArn")]
    pub auto_scaling_configuration_arn: Option<String>,


    /// 
    /// An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default,    App Runner uses an AWS managed key.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,


    /// 
    /// The settings for the health check that AWS App Runner performs to monitor the health of the App Runner service.
    /// 
    /// Required: No
    ///
    /// Type: HealthCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckConfiguration")]
    pub health_check_configuration: Option<HealthCheckConfiguration>,


    /// 
    /// The runtime configuration of instances (scaling units) of your service.
    /// 
    /// Required: No
    ///
    /// Type: InstanceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceConfiguration")]
    pub instance_configuration: Option<InstanceConfiguration>,


    /// 
    /// Configuration settings related to network traffic of the web application that the App Runner service runs.
    /// 
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,


    /// 
    /// The observability configuration of your service.
    /// 
    /// Required: No
    ///
    /// Type: ServiceObservabilityConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObservabilityConfiguration")]
    pub observability_configuration: Option<ServiceObservabilityConfiguration>,


    /// 
    /// A name for the App Runner service. It must be unique across all the running App Runner services in your AWS account in the AWS Region.
    /// 
    /// If you don't specify a name, AWS CloudFormation generates a name for your service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9-_]{3,39}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,


    /// 
    /// The source to deploy to the App Runner service. It can be a code or an image repository.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceConfiguration")]
    pub source_configuration: SourceConfiguration,


    /// 
    /// An optional list of metadata items that you can associate with the App Runner service resource. A tag is a key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnService {
    fn type_string() -> &'static str {
        "AWS::AppRunner::Service"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes resources needed to authenticate access to some source repositories. The specific resource depends on the repository provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthenticationConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that grants the App Runner service access to a source repository. It's required for ECR image repositories    (but not for ECR Public repositories).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 29
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):iam::[0-9]{12}:(role|role\/service-role)\/[\w+=,.@\-/]{1,1000}
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessRoleArn")]
    pub access_role_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the App Runner connection that enables the App Runner service to connect to a source repository. It's required for GitHub code    repositories.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: arn:aws(-[\w]+)*:[a-z0-9-\\.]{0,63}:[a-z0-9-\\.]{0,63}:[0-9]{12}:(\w|\/|-){1,1011}
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,

}




/// Describes the configuration that AWS App Runner uses to build and run an App Runner service from a source code repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CodeConfiguration {


    /// 
    /// The basic configuration for building and running the App Runner service. Use it to quickly launch an App Runner service without providing a     apprunner.yaml file in the source code repository (or ignoring the file if it exists).
    /// 
    /// Required: No
    ///
    /// Type: CodeConfigurationValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeConfigurationValues")]
    pub code_configuration_values: Option<CodeConfigurationValues>,


    /// 
    /// The source of the App Runner configuration. Values are interpreted as follows:
    /// 
    /// REPOSITORY – App Runner reads configuration values from the apprunner.yaml file in the source code repository and      ignores CodeConfigurationValues.                        API – App Runner uses configuration values provided in CodeConfigurationValues and ignores the       apprunner.yaml file in the source code repository.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: API | REPOSITORY
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationSource")]
    pub configuration_source: CodeConfigurationConfigurationSourceEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CodeConfigurationConfigurationSourceEnum {

    /// API
    #[serde(rename = "API")]
    Api,

    /// REPOSITORY
    #[serde(rename = "REPOSITORY")]
    Repository,

}

impl Default for CodeConfigurationConfigurationSourceEnum {
    fn default() -> Self {
        CodeConfigurationConfigurationSourceEnum::Api
    }
}



/// Describes the basic configuration needed for building and running an AWS App Runner service. This type doesn't support the full set of possible    configuration options. Fur full configuration capabilities, use a apprunner.yaml file in the source code repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CodeConfigurationValues {


    /// 
    /// The command App Runner runs to build your application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [^\x0a\x0d]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildCommand")]
    pub build_command: Option<String>,


    /// 
    /// The port that your application listens to in the container.
    /// 
    /// Default: 8080
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,


    /// 
    /// A runtime environment type for building and running an App Runner service.    It represents a    programming language runtime.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CORRETTO_11 | CORRETTO_8 | DOTNET_6 | GO_1 | NODEJS_12 | NODEJS_14 | NODEJS_16 | PHP_81 | PYTHON_3 | RUBY_31
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    pub runtime: CodeConfigurationValuesRuntimeEnum,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    pub runtime_environment_secrets: Option<Vec<KeyValuePair>>,


    /// 
    /// The environment variables that are available to your running AWS App Runner service. An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeEnvironmentVariables")]
    pub runtime_environment_variables: Option<Vec<KeyValuePair>>,


    /// 
    /// The command App Runner runs to start your application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [^\x0a\x0d]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartCommand")]
    pub start_command: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CodeConfigurationValuesRuntimeEnum {

    /// CORRETTO_11
    #[serde(rename = "CORRETTO_11")]
    Corretto11,

    /// CORRETTO_8
    #[serde(rename = "CORRETTO_8")]
    Corretto8,

    /// DOTNET_6
    #[serde(rename = "DOTNET_6")]
    Dotnet6,

    /// GO_1
    #[serde(rename = "GO_1")]
    Go1,

    /// NODEJS_12
    #[serde(rename = "NODEJS_12")]
    Nodejs12,

    /// NODEJS_14
    #[serde(rename = "NODEJS_14")]
    Nodejs14,

    /// NODEJS_16
    #[serde(rename = "NODEJS_16")]
    Nodejs16,

    /// PHP_81
    #[serde(rename = "PHP_81")]
    Php81,

    /// PYTHON_3
    #[serde(rename = "PYTHON_3")]
    Python3,

    /// RUBY_31
    #[serde(rename = "RUBY_31")]
    Ruby31,

}

impl Default for CodeConfigurationValuesRuntimeEnum {
    fn default() -> Self {
        CodeConfigurationValuesRuntimeEnum::Corretto11
    }
}



/// Describes a source code repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CodeRepository {


    /// 
    /// Configuration for building and running the service from a source code repository.
    /// 
    /// Note        CodeConfiguration is required only for CreateService request.
    /// 
    /// Required: No
    ///
    /// Type: CodeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeConfiguration")]
    pub code_configuration: Option<CodeConfiguration>,


    /// 
    /// The location of the repository that contains the source code.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,


    /// 
    /// The version that should be used within the source code repository.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceCodeVersion
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceCodeVersion")]
    pub source_code_version: SourceCodeVersion,

}




/// Describes configuration settings related to outbound network traffic of an AWS App Runner service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EgressConfiguration {


    /// 
    /// The type of egress configuration.
    /// 
    /// Set to DEFAULT for access to resources hosted on public networks.
    /// 
    /// Set to VPC to associate your service to a custom VPC specified by VpcConnectorArn.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT | VPC
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressType")]
    pub egress_type: EgressConfigurationEgressTypeEnum,


    /// 
    /// The Amazon Resource Name (ARN) of the App Runner VPC connector that you want to associate with your App Runner service. Only valid when EgressType =     VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: arn:aws(-[\w]+)*:[a-z0-9-\\.]{0,63}:[a-z0-9-\\.]{0,63}:[0-9]{12}:(\w|\/|-){1,1011}
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConnectorArn")]
    pub vpc_connector_arn: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EgressConfigurationEgressTypeEnum {

    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

    /// VPC
    #[serde(rename = "VPC")]
    Vpc,

}

impl Default for EgressConfigurationEgressTypeEnum {
    fn default() -> Self {
        EgressConfigurationEgressTypeEnum::Default
    }
}



/// Describes a custom encryption key that AWS App Runner uses to encrypt copies of the source repository and service logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfiguration {


    /// 
    /// The ARN of the KMS key that's used for encryption.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws(-[\w]+)*:kms:[a-z\-]+-[0-9]{1}:[0-9]{12}:key\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKey")]
    pub kms_key: String,

}




/// Describes the settings for the health check that AWS App Runner performs to monitor the health of a service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheckConfiguration {


    /// 
    /// The number of consecutive checks that must succeed before App Runner decides that the service is healthy.
    /// 
    /// Default: 1
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: Option<i64>,


    /// 
    /// The time interval, in seconds, between health checks.
    /// 
    /// Default: 5
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,


    /// 
    /// The URL that health check requests are sent to.
    /// 
    /// Path is only applicable when you set Protocol to HTTP.
    /// 
    /// Default: "/"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,


    /// 
    /// The IP protocol that App Runner uses to perform health checks for your service.
    /// 
    /// If you set Protocol to HTTP, App Runner sends health check requests to the HTTP path specified by Path.
    /// 
    /// Default: TCP
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP | TCP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<HealthCheckConfigurationProtocolEnum>,


    /// 
    /// The time, in seconds, to wait for a health check response before deciding it failed.
    /// 
    /// Default: 2
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


    /// 
    /// The number of consecutive checks that must fail before App Runner decides that the service is unhealthy.
    /// 
    /// Default: 5
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HealthCheckConfigurationProtocolEnum {

    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

}

impl Default for HealthCheckConfigurationProtocolEnum {
    fn default() -> Self {
        HealthCheckConfigurationProtocolEnum::Http
    }
}



/// Describes the configuration that AWS App Runner uses to run an App Runner service using an image pulled from a source image repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageConfiguration {


    /// 
    /// The port that your application listens to in the container.
    /// 
    /// Default: 8080
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    pub runtime_environment_secrets: Option<Vec<KeyValuePair>>,


    /// 
    /// Environment variables that are available to your running App Runner service. An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeEnvironmentVariables")]
    pub runtime_environment_variables: Option<Vec<KeyValuePair>>,


    /// 
    /// An optional command that App Runner runs to start the application in the source image. If specified, this command overrides the Docker image’s default start    command.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [^\x0a\x0d]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartCommand")]
    pub start_command: Option<String>,

}




/// Describes a source image repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageRepository {


    /// 
    /// Configuration for running the identified image.
    /// 
    /// Required: No
    ///
    /// Type: ImageConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfiguration>,


    /// 
    /// The identifier of an image.
    /// 
    /// For an image in Amazon Elastic Container Registry (Amazon ECR), this is an image name. For the image name format, see Pulling an image in the Amazon ECR User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ([0-9]{12}.dkr.ecr.[a-z\-]+-[0-9]{1}.amazonaws.com\/((?:[a-z0-9]+(?:[._-][a-z0-9]+)*\/)*[a-z0-9]+(?:[._-][a-z0-9]+)*)(:([\w\d+\-=._:\/@])+|@([\w\d\:]+))?)|(^public\.ecr\.aws\/.+\/((?:[a-z0-9]+(?:[._-][a-z0-9]+)*\/)*[a-z0-9]+(?:[._-][a-z0-9]+)*)(:([\w\d+\-=._:\/@])+|@([\w\d\:]+))?)
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageIdentifier")]
    pub image_identifier: String,


    /// 
    /// The type of the image repository. This reflects the repository provider and whether the repository is private or public.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ECR | ECR_PUBLIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageRepositoryType")]
    pub image_repository_type: ImageRepositoryImageRepositoryTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ImageRepositoryImageRepositoryTypeEnum {

    /// ECR
    #[serde(rename = "ECR")]
    Ecr,

    /// ECR_PUBLIC
    #[serde(rename = "ECR_PUBLIC")]
    Ecrpublic,

}

impl Default for ImageRepositoryImageRepositoryTypeEnum {
    fn default() -> Self {
        ImageRepositoryImageRepositoryTypeEnum::Ecr
    }
}



/// Network configuration settings for inbound network traffic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IngressConfiguration {


    /// 
    /// Specifies whether your App Runner service is publicly accessible. To make the service publicly accessible set it to True. To make the service    privately accessible, from only within an Amazon VPC set it to False.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsPubliclyAccessible")]
    pub is_publicly_accessible: bool,

}




/// Describes the runtime configuration of an AWS App Runner service instance (scaling unit).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceConfiguration {


    /// 
    /// The number of CPU units reserved for each instance of your App Runner service.
    /// 
    /// Default: 1 vCPU
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 9
    ///
    /// Pattern: 256|512|1024|2048|4096|(0.25|0.5|1|2|4) vCPU
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cpu")]
    pub cpu: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of an IAM role that provides permissions to your App Runner service. These are permissions that your code needs when it calls    any AWS APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 29
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):iam::[0-9]{12}:(role|role\/service-role)\/[\w+=,.@\-/]{1,1000}
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceRoleArn")]
    pub instance_role_arn: Option<String>,


    /// 
    /// The amount of memory, in MB or GB, reserved for each instance of your App Runner service.
    /// 
    /// Default: 2 GB
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 6
    ///
    /// Pattern: 512|1024|2048|3072|4096|6144|8192|10240|12288|(0.5|1|2|3|4|6|8|10|12) GB
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    pub memory: Option<String>,

}




/// Describes a key-value pair, which is a string-to-string mapping.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeyValuePair {


    /// 
    /// The key name string to map to a value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value string to which the key name is mapped.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}




/// Describes configuration settings related to network traffic of an AWS App Runner service. Consists of embedded objects for each configurable network    feature.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {


    /// 
    /// Network configuration settings for outbound message traffic.
    /// 
    /// Required: No
    ///
    /// Type: EgressConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressConfiguration")]
    pub egress_configuration: Option<EgressConfiguration>,


    /// 
    /// Network configuration settings for inbound message traffic.
    /// 
    /// Required: No
    ///
    /// Type: IngressConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressConfiguration")]
    pub ingress_configuration: Option<IngressConfiguration>,

}




/// Describes the observability configuration of an AWS App Runner service. These are additional observability features, like tracing, that you choose to    enable. They're configured in a separate resource that you associate with your service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceObservabilityConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the observability configuration that is associated with the service. Specified only when     ObservabilityEnabled is true.
    /// 
    /// Specify an ARN with a name and a revision number to associate that revision. For example:      arn:aws:apprunner:us-east-1:123456789012:observabilityconfiguration/xray-tracing/3
    /// 
    /// Specify just the name to associate the latest revision. For example:     arn:aws:apprunner:us-east-1:123456789012:observabilityconfiguration/xray-tracing
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: arn:aws(-[\w]+)*:[a-z0-9-\\.]{0,63}:[a-z0-9-\\.]{0,63}:[0-9]{12}:(\w|\/|-){1,1011}
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObservabilityConfigurationArn")]
    pub observability_configuration_arn: Option<String>,


    /// 
    /// When true, an observability configuration resource is associated with the service, and an ObservabilityConfigurationArn is    specified.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObservabilityEnabled")]
    pub observability_enabled: bool,

}




/// Identifies a version of code that AWS App Runner refers to within a source code repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceCodeVersion {


    /// 
    /// The type of version identifier.
    /// 
    /// For a git-based repository, branches represent versions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BRANCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: SourceCodeVersionTypeEnum,


    /// 
    /// A source code version.
    /// 
    /// For a git-based repository, a branch name maps to a specific version. App Runner uses the most recent commit to the branch.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceCodeVersionTypeEnum {

    /// BRANCH
    #[serde(rename = "BRANCH")]
    Branch,

}

impl Default for SourceCodeVersionTypeEnum {
    fn default() -> Self {
        SourceCodeVersionTypeEnum::Branch
    }
}



/// Describes the source deployed to an AWS App Runner service. It can be a code or an image repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceConfiguration {


    /// 
    /// Describes the resources that are needed to authenticate access to some source repositories.
    /// 
    /// Required: No
    ///
    /// Type: AuthenticationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: Option<AuthenticationConfiguration>,


    /// 
    /// If true, continuous integration from the source repository is enabled for the App Runner service. Each repository change (including any source    code commit or new image version) starts a deployment.
    /// 
    /// Default: App Runner sets to false for a source image that uses an ECR Public repository or an ECR repository that's in an AWS account other than the one that the service is in. App Runner sets to true in all other cases (which currently include a source code    repository or a source image using a same-account ECR repository).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoDeploymentsEnabled")]
    pub auto_deployments_enabled: Option<bool>,


    /// 
    /// The description of a source code repository.
    /// 
    /// You must provide either this member or ImageRepository (but not both).
    /// 
    /// Required: No
    ///
    /// Type: CodeRepository
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeRepository")]
    pub code_repository: Option<CodeRepository>,


    /// 
    /// The description of a source image    repository.
    /// 
    /// You must provide either this member or CodeRepository (but not both).
    /// 
    /// Required: No
    ///
    /// Type: ImageRepository
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageRepository")]
    pub image_repository: Option<ImageRepository>,

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


