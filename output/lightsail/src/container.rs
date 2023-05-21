/// The AWS::Lightsail::Container resource specifies a container     service.
///
/// A Lightsail container service is a compute resource to which you can     deploy containers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnContainer {
    ///
    /// An object that describes the current container deployment of the container     service.
    ///
    /// Required: No
    ///
    /// Type: ContainerServiceDeployment
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ContainerServiceDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_service_deployment: Option<ContainerServiceDeployment>,

    ///
    /// A Boolean value indicating whether the container service is disabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,

    ///
    /// The power specification of the container service.
    ///
    /// The power specifies the amount of RAM, the number of vCPUs, and the base price of the     container service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: large | medium | micro | nano | small | xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "Power")]
    pub power: ContainerPowerEnum,

    ///
    /// The public domain name of the container service, such as example.com and       www.example.com.
    ///
    /// You can specify up to four public domain names for a container service. The domain names     that you specify are used when you create a deployment with a container that is configured as the     public endpoint of your container service.
    ///
    /// If you don't specify public domain names, then you can use the default domain of the     container service.
    ///
    /// ImportantYou must create and validate an SSL/TLS certificate before you can use public domain       names with your container service. Use the AWS::Lightsail::Certificate resource to create a certificate for the public       domain names that you want to use with your container service.
    ///
    /// Required: No
    ///
    /// Type: List of PublicDomainName
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicDomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_domain_names: Option<Vec<PublicDomainName>>,

    ///
    /// The scale specification of the container service.
    ///
    /// The scale specifies the allocated compute nodes of the container service.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scale")]
    pub scale: i64,

    ///
    /// The name of the container service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceName")]
    pub service_name: String,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    ///
    /// NoteThe Value of Tags is optional for Lightsail resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ContainerPowerEnum {
    /// large
    #[serde(rename = "large")]
    Large,

    /// medium
    #[serde(rename = "medium")]
    Medium,

    /// micro
    #[serde(rename = "micro")]
    Micro,

    /// nano
    #[serde(rename = "nano")]
    Nano,

    /// small
    #[serde(rename = "small")]
    Small,

    /// xlarge
    #[serde(rename = "xlarge")]
    Xlarge,
}

impl Default for ContainerPowerEnum {
    fn default() -> Self {
        ContainerPowerEnum::Large
    }
}

impl cfn_resources::CfnResource for CfnContainer {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::Container"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_service_deployment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.scale;

        if *the_val > 20 as _ {
            return Err(format!(
                "Max validation failed on field 'scale'. {} is greater than 20",
                the_val
            ));
        }

        let the_val = &self.scale;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'scale'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Container is a property of the ContainerServiceDeployment property. It describes the settings of a container     that will be launched, or that is launched, to an Amazon Lightsail container     service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Container {
    ///
    /// The launch command for the container.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    ///
    /// The name of the container.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    ///
    /// The environment variables of the container.
    ///
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,

    ///
    /// The name of the image used for the container.
    ///
    /// Container images that are sourced from (registered and stored on) your container service     start with a colon (:). For example, if your container service name is container-service-1,      the container image label is mystaticsite, and you want to use the third version (3) of the    registered container image, then you should specify :container-service-1.mystaticsite.3. To use the latest      version of a container image, specify latest instead of a version number (for example,    :container-service-1.mystaticsite.latest). Your container service will automatically use the highest numbered      version of the registered container image.
    ///
    /// Container images that are sourced from a public registry like Docker Hub don’t start with a     colon. For example, nginx:latest or nginx.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    ///
    /// An object that describes the open firewall ports and protocols of the container.
    ///
    /// Required: No
    ///
    /// Type: List of PortInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<PortInfo>>,
}

impl cfn_resources::CfnResource for Container {
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

/// ContainerServiceDeployment is a property of the AWS::Lightsail::Container resource. It describes a container deployment     configuration of a container service.
///
/// A deployment specifies the settings, such as the ports and launch command, of containers     that are deployed to your container service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerServiceDeployment {
    ///
    /// An object that describes the configuration for the containers of the deployment.
    ///
    /// Required: No
    ///
    /// Type: List of Container
    ///
    /// Update requires: No interruption
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,

    ///
    /// An object that describes the endpoint of the deployment.
    ///
    /// Required: No
    ///
    /// Type: PublicEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_endpoint: Option<PublicEndpoint>,
}

impl cfn_resources::CfnResource for ContainerServiceDeployment {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.public_endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// EnvironmentVariable is a property of the Container property. It describes the environment variables of a container on a container service which are key-value parameters that     provide dynamic configuration of the application or script run by the container.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EnvironmentVariable {
    ///
    /// The environment variable value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    ///
    /// The environment variable key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
}

impl cfn_resources::CfnResource for EnvironmentVariable {
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

/// HealthCheckConfig is a property of the PublicEndpoint property. It describes the healthcheck configuration of a     container deployment on a container service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheckConfig {
    ///
    /// The number of consecutive health check successes required before moving the container     to the Healthy state. The default value is 2.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i64>,

    ///
    /// The approximate interval, in seconds, between health checks of an individual container.     You can specify between 5 and 300 seconds. The default value is       5.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,

    ///
    /// The path on the container on which to perform the health check. The default value is       /.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    ///
    /// The HTTP codes to use when checking for a successful response from a container. You can     specify values between 200 and 499. You can specify multiple     values (for example, 200,202) or a range of values (for example,       200-299).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuccessCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_codes: Option<String>,

    ///
    /// The amount of time, in seconds, during which no response means a failed health check.     You can specify between 2 and 60 seconds. The default value is       2.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,

    ///
    /// The number of consecutive health check failures required before moving the container to     the Unhealthy state. The default value is 2.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i64>,
}

impl cfn_resources::CfnResource for HealthCheckConfig {
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

/// PortInfo is a property of the Container property. It describes the ports to open and the protocols to use for     a container on a Amazon Lightsail container service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortInfo {
    ///
    /// The open firewall ports of the container.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    ///
    /// The protocol name for the open ports.
    ///
    /// Allowed values: HTTP | HTTPS | TCP | UDP
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<PortInfoProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PortInfoProtocolEnum {
    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// HTTPS
    #[serde(rename = "HTTPS")]
    Https,

    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

    /// UDP
    #[serde(rename = "UDP")]
    Udp,
}

impl Default for PortInfoProtocolEnum {
    fn default() -> Self {
        PortInfoProtocolEnum::Http
    }
}

impl cfn_resources::CfnResource for PortInfo {
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

/// PublicDomainName is a property of the AWS::Lightsail::Container resource. It describes the public domain names to use     with a container service, such as example.com and     www.example.com. It also describes the certificates to use with a container     service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicDomainName {
    ///
    /// The name of the certificate for the public domains.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,

    ///
    /// The public domain names to use with the container service.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for PublicDomainName {
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

/// PublicEndpoint is a property of the ContainerServiceDeployment property. It describes describes the settings of the     public endpoint of a container on a container service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicEndpoint {
    ///
    /// The name of the container entry of the deployment that the endpoint configuration     applies to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    ///
    /// The port of the specified container to which traffic is forwarded to.
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
    /// An object that describes the health check configuration of the container.
    ///
    /// Required: No
    ///
    /// Type: HealthCheckConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
}

impl cfn_resources::CfnResource for PublicEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.health_check_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
