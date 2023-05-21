/// The AWS::OSIS::Pipeline resource creates an Amazon OpenSearch Ingestion pipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPipeline {
    ///
    /// Key-value pairs that represent log publishing settings.
    ///
    /// Required: No
    ///
    /// Type: LogPublishingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<LogPublishingOptions>,

    ///
    /// The maximum pipeline capacity, in Ingestion Compute Units (ICUs).
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxUnits")]
    pub max_units: i64,

    ///
    /// The minimum pipeline capacity, in Ingestion Compute Units (ICUs).
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinUnits")]
    pub min_units: i64,

    ///
    /// The Data Prepper pipeline configuration in YAML format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineConfigurationBody")]
    pub pipeline_configuration_body: String,

    ///
    /// The name of the pipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,

    ///
    /// List of tags to add to the pipeline upon creation.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Options that specify the subnets and security groups for an OpenSearch Ingestion  VPC endpoint.
    ///
    /// Required: No
    ///
    /// Type: VpcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcOptions")]
    pub vpc_options: Option<VpcOptions>,
}

impl cfn_resources::CfnResource for CfnPipeline {
    fn type_string(&self) -> &'static str {
        "AWS::OSIS::Pipeline"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.log_publishing_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The CloudWatchLogDestination property type specifies Property description not available. for an AWS::OSIS::Pipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogDestination {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,
}

impl cfn_resources::CfnResource for CloudWatchLogDestination {
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

/// Container for the values required to configure logging for the pipeline. If you don't  specify these values, OpenSearch Ingestion will not publish logs from your application to  CloudWatch Logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogPublishingOptions {
    ///
    /// The destination for OpenSearch Ingestion logs sent to Amazon CloudWatch Logs. This  parameter is required if IsLoggingEnabled is set to true.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchLogDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogDestination")]
    pub cloud_watch_log_destination: Option<CloudWatchLogDestination>,

    ///
    /// Whether logs should be published.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsLoggingEnabled")]
    pub is_logging_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for LogPublishingOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_log_destination
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An OpenSearch Ingestion-managed VPC endpoint that will access one or more  pipelines.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcEndpoint {
    ///
    /// The unique identifier of the endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,

    ///
    /// The ID for your VPC. AWS PrivateLink generates this value when you create a  VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

    ///
    /// Information about the VPC, including associated subnets and security groups.
    ///
    /// Required: No
    ///
    /// Type: VpcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcOptions")]
    pub vpc_options: Option<VpcOptions>,
}

impl cfn_resources::CfnResource for VpcEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Options that specify the subnets and security groups for an OpenSearch Ingestion  VPC endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcOptions {
    ///
    /// A list of security groups associated with the VPC endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of subnet IDs associated with the VPC endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for VpcOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_ids {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_ids'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
