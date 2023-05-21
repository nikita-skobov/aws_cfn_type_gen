

/// The AWS::EMRServerless::Application resource specifies an EMR Serverless       application. An application uses open source analytics frameworks to run jobs that       process data. To create an application, you must specify the release version for the       open source framework version you want to use and the type of application you want, such       as Apache Spark or Apache Hive. After you create an application, you can submit data       processing jobs or interactive requests to it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {


    /// The CPU architecture type of the application. Allowed values: X86_64 or ARM64
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architecture")]
    pub architecture: Option<ApplicationArchitectureEnum>,


    /// 
    /// The configuration for an application to automatically start on job submission.
    /// 
    /// Required: No
    ///
    /// Type: AutoStartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoStartConfiguration")]
    pub auto_start_configuration: Option<AutoStartConfiguration>,


    /// 
    /// The configuration for an application to automatically stop after a certain amount of       time being idle.
    /// 
    /// Required: No
    ///
    /// Type: AutoStopConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoStopConfiguration")]
    pub auto_stop_configuration: Option<AutoStopConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ImageConfigurationInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfigurationInput>,


    /// 
    /// The initial capacity of the application.
    /// 
    /// Required: No
    ///
    /// Type: List of InitialCapacityConfigKeyValuePair
    ///
    /// Update requires: No interruption
    #[serde(rename = "InitialCapacity")]
    pub initial_capacity: Option<Vec<InitialCapacityConfigKeyValuePair>>,


    /// 
    /// The maximum capacity of the application. This is cumulative across all workers at any     given point in time during the lifespan of the application is created. No new resources     will be created once any one of the defined limits is hit.
    /// 
    /// Required: No
    ///
    /// Type: MaximumAllowedResources
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumCapacity")]
    pub maximum_capacity: Option<MaximumAllowedResources>,


    /// 
    /// The name of the application.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 64
    /// 
    /// Pattern: ^[A-Za-z0-9._\\/#-]+$
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The network configuration for customer VPC connectivity for the application.
    /// 
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,


    /// 
    /// The EMR release version associated with the application.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 64
    /// 
    /// Pattern: ^[A-Za-z0-9._/-]+$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReleaseLabel")]
    pub release_label: String,


    /// 
    /// The tags assigned to the application.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The type of application, such as Spark or Hive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Map of WorkerTypeSpecificationInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerTypeSpecifications")]
    pub worker_type_specifications: Option<std::collections::HashMap<String, WorkerTypeSpecificationInput>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationArchitectureEnum {

    /// X86_64 or ARM64
    #[serde(rename = "X86_64 or ARM64")]
    X8664orarm64,

}

impl Default for ApplicationArchitectureEnum {
    fn default() -> Self {
        ApplicationArchitectureEnum::X8664orarm64
    }
}


impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::EMRServerless::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.auto_start_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.auto_stop_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.image_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.maximum_capacity.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.network_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.release_label;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'release_label'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.release_label;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'release_label'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The conﬁguration for an application to automatically start on job submission.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoStartConfiguration {


    /// 
    /// Enables the application to automatically start on job submission. Defaults to       true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}



impl cfn_resources::CfnResource for AutoStartConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The conﬁguration for an application to automatically stop after a certain amount of       time being idle.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoStopConfiguration {


    /// 
    /// Enables the application to automatically stop after a certain amount of time being       idle. Defaults to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The amount of idle time in minutes after which your application will automatically       stop. Defaults to 15 minutes.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 10080
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleTimeoutMinutes")]
    pub idle_timeout_minutes: Option<i64>,

}



impl cfn_resources::CfnResource for AutoStopConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.idle_timeout_minutes {

        if *the_val > 10080 as _ {
            return Err(format!("Max validation failed on field 'idle_timeout_minutes'. {} is greater than 10080", the_val));
        }

        }
        
        if let Some(the_val) = &self.idle_timeout_minutes {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'idle_timeout_minutes'. {} is less than 1", the_val));
        }

        }
        
        Ok(())
    }
}

/// The ImageConfigurationInput property type specifies Property description not available. for an AWS::EMRServerless::Application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageConfigurationInput {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageUri")]
    pub image_uri: Option<String>,

}



impl cfn_resources::CfnResource for ImageConfigurationInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The initial capacity configuration per worker.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InitialCapacityConfig {


    /// 
    /// The resource configuration of the initial capacity configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: WorkerConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerConfiguration")]
    pub worker_configuration: WorkerConfiguration,


    /// 
    /// The number of workers in the initial capacity configuration.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 1000000
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerCount")]
    pub worker_count: i64,

}



impl cfn_resources::CfnResource for InitialCapacityConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.worker_configuration.validate()?;

        let the_val = &self.worker_count;

        if *the_val > 1000000 as _ {
            return Err(format!("Max validation failed on field 'worker_count'. {} is greater than 1000000", the_val));
        }

        
        let the_val = &self.worker_count;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'worker_count'. {} is less than 1", the_val));
        }

        
        Ok(())
    }
}

/// The initial capacity configuration per worker.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InitialCapacityConfigKeyValuePair {


    /// 
    /// The worker type for an analytics framework. For Spark applications, the key can either       be set to Driver or Executor. For Hive applications, it can be       set to HiveDriver or TezTask.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 50
    /// 
    /// Pattern: ^[a-zA-Z]+[-_]*[a-zA-Z]+$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the initial capacity configuration per worker.
    /// 
    /// Required: Yes
    ///
    /// Type: InitialCapacityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: InitialCapacityConfig,

}



impl cfn_resources::CfnResource for InitialCapacityConfigKeyValuePair {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.key;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'key'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'key'. {} is less than 1", the_val.len()));
        }

        
        self.value.validate()?;

        Ok(())
    }
}

/// The maximum allowed cumulative resources for an application. No new resources will be     created once the limit is hit.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaximumAllowedResources {


    /// 
    /// The maximum allowed CPU for an application.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(vCPU|vcpu|VCPU)?$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cpu")]
    pub cpu: String,


    /// 
    /// The maximum allowed disk for an application.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(GB|gb|gB|Gb)$"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Disk")]
    pub disk: Option<String>,


    /// 
    /// The maximum allowed resources for an application.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(GB|gb|gB|Gb)?$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    pub memory: String,

}



impl cfn_resources::CfnResource for MaximumAllowedResources {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cpu;

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'cpu'. {} is greater than 15", the_val.len()));
        }

        
        let the_val = &self.cpu;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'cpu'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.disk {

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'disk'. {} is greater than 15", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.disk {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'disk'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.memory;

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'memory'. {} is greater than 15", the_val.len()));
        }

        
        let the_val = &self.memory;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'memory'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The network configuration for customer VPC connectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {


    /// 
    /// The array of security group Ids for customer VPC connectivity.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 32
    /// 
    /// Pattern: ^[-0-9a-zA-Z]+
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The array of subnet Ids for customer VPC connectivity.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 32
    /// 
    /// Pattern: ^[-0-9a-zA-Z]+
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for NetworkConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.security_group_ids {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'security_group_ids'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.security_group_ids {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'security_group_ids'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.subnet_ids {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'subnet_ids'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.subnet_ids {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'subnet_ids'. {} is less than 1", the_val.len()));
        }

        }
        
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The resource configuration of the initial capacity configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WorkerConfiguration {


    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(vCPU|vcpu|VCPU)?$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cpu")]
    pub cpu: String,


    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(GB|gb|gB|Gb)$"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Disk")]
    pub disk: Option<String>,


    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 15
    /// 
    /// Pattern: ^[1-9][0-9]*(\\s)?(GB|gb|gB|Gb)?$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    pub memory: String,

}



impl cfn_resources::CfnResource for WorkerConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cpu;

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'cpu'. {} is greater than 15", the_val.len()));
        }

        
        let the_val = &self.cpu;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'cpu'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.disk {

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'disk'. {} is greater than 15", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.disk {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'disk'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.memory;

        if the_val.len() > 15 as _ {
            return Err(format!("Max validation failed on field 'memory'. {} is greater than 15", the_val.len()));
        }

        
        let the_val = &self.memory;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'memory'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The WorkerTypeSpecificationInput property type specifies Property description not available. for an AWS::EMRServerless::Application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WorkerTypeSpecificationInput {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ImageConfigurationInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfigurationInput>,

}



impl cfn_resources::CfnResource for WorkerTypeSpecificationInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.image_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}