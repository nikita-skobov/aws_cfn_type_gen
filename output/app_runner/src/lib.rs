
pub mod cfn_observability_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnObservabilityConfiguration {
    /// A name for the observability configuration. When you use it for the first time in an AWS Region, App Runner creates revision number 1 of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.
    #[serde(rename = "ObservabilityConfigurationName")]
    pub observability_configuration_name: Option<String>,
    /// A list of metadata items that you can associate with your observability configuration resource. A tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing.
    #[serde(rename = "TraceConfiguration")]
    pub trace_configuration: Option<TraceConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct TraceConfiguration {
    #[serde(rename = "Vendor")]
    pub vendor: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_service {

#[derive(serde::Serialize, Default)]
pub struct CfnService {
    /// No documentation provided by AWS
    #[serde(rename = "ObservabilityConfiguration")]
    pub observability_configuration: Option<ServiceObservabilityConfiguration>,
    /// Network configuration
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// Encryption configuration (KMS key)
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// Autoscaling configuration ARN
    #[serde(rename = "AutoScalingConfigurationArn")]
    pub auto_scaling_configuration_arn: Option<String>,
    /// The AppRunner Service Name.
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    /// Health check configuration
    #[serde(rename = "HealthCheckConfiguration")]
    pub health_check_configuration: Option<HealthCheckConfiguration>,
    /// Source Code configuration
    #[serde(rename = "SourceConfiguration")]
    pub source_configuration: SourceConfiguration,
    /// Instance Configuration
    #[serde(rename = "InstanceConfiguration")]
    pub instance_configuration: Option<InstanceConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct SourceConfiguration {
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: Option<AuthenticationConfiguration>,
    #[serde(rename = "CodeRepository")]
    pub code_repository: Option<CodeRepository>,
    #[serde(rename = "ImageRepository")]
    pub image_repository: Option<ImageRepository>,
    #[serde(rename = "AutoDeploymentsEnabled")]
    pub auto_deployments_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CodeRepository {
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,
    #[serde(rename = "SourceCodeVersion")]
    pub source_code_version: SourceCodeVersion,
    #[serde(rename = "CodeConfiguration")]
    pub code_configuration: Option<CodeConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthenticationConfiguration {
    #[serde(rename = "AccessRoleArn")]
    pub access_role_arn: Option<RoleArn>,
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,

}
pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    pub kms_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceObservabilityConfiguration {
    #[serde(rename = "ObservabilityEnabled")]
    pub observability_enabled: bool,
    #[serde(rename = "ObservabilityConfigurationArn")]
    pub observability_configuration_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "EgressConfiguration")]
    pub egress_configuration: Option<EgressConfiguration>,
    #[serde(rename = "IngressConfiguration")]
    pub ingress_configuration: Option<IngressConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ImageConfiguration {
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    pub runtime_environment_secrets: Option<Vec<KeyValuePair>>,
    #[serde(rename = "RuntimeEnvironmentVariables")]
    pub runtime_environment_variables: Option<Vec<KeyValuePair>>,
    #[serde(rename = "StartCommand")]
    pub start_command: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EgressConfiguration {
    #[serde(rename = "EgressType")]
    pub egress_type: String,
    #[serde(rename = "VpcConnectorArn")]
    pub vpc_connector_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyValuePair {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheckConfiguration {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: Option<usize>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceCodeVersion {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct CodeConfigurationValues {
    #[serde(rename = "RuntimeEnvironmentVariables")]
    pub runtime_environment_variables: Option<Vec<KeyValuePair>>,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "BuildCommand")]
    pub build_command: Option<String>,
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    pub runtime_environment_secrets: Option<Vec<KeyValuePair>>,
    #[serde(rename = "StartCommand")]
    pub start_command: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceConfiguration {
    #[serde(rename = "Cpu")]
    pub cpu: Option<String>,
    #[serde(rename = "Memory")]
    pub memory: Option<String>,
    #[serde(rename = "InstanceRoleArn")]
    pub instance_role_arn: Option<RoleArn>,

}

#[derive(serde::Serialize, Default)]
pub struct CodeConfiguration {
    #[serde(rename = "CodeConfigurationValues")]
    pub code_configuration_values: Option<CodeConfigurationValues>,
    #[serde(rename = "ConfigurationSource")]
    pub configuration_source: String,

}

#[derive(serde::Serialize, Default)]
pub struct IngressConfiguration {
    #[serde(rename = "IsPubliclyAccessible")]
    pub is_publicly_accessible: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ImageRepository {
    #[serde(rename = "ImageIdentifier")]
    pub image_identifier: String,
    #[serde(rename = "ImageRepositoryType")]
    pub image_repository_type: String,
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfiguration>,

}


}

pub mod cfn_vpc_connector {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcConnector {
    /// A name for the VPC connector. If you don't specify a name, AWS CloudFormation generates a name for your VPC connector.
    #[serde(rename = "VpcConnectorName")]
    pub vpc_connector_name: Option<String>,
    /// A list of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    /// A list of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// A list of metadata items that you can associate with your VPC connector resource. A tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_vpc_ingress_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcIngressConnection {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Amazon Resource Name (ARN) of the service.
    #[serde(rename = "ServiceArn")]
    pub service_arn: String,
    /// The configuration of customer’s VPC and related VPC endpoint
    #[serde(rename = "IngressVpcConfiguration")]
    pub ingress_vpc_configuration: IngressVpcConfiguration,
    /// The customer-provided Vpc Ingress Connection name.
    #[serde(rename = "VpcIngressConnectionName")]
    pub vpc_ingress_connection_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct IngressVpcConfiguration {
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}
