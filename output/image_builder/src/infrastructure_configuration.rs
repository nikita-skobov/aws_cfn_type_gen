/// The infrastructure configuration allows you to specify the infrastructure within which     to build and test your image. In the infrastructure configuration, you can specify instance     types, subnets, and security groups to associate with your instance. You can also associate     an Amazon EC2 key pair with the instance used to build your image. This allows you to log     on to your instance to troubleshoot if your build fails and you set     terminateInstanceOnFailure to false.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInfrastructureConfiguration {
    ///
    /// The description of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The instance metadata option settings for the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: InstanceMetadataOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMetadataOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<InstanceMetadataOptions>,

    ///
    /// The instance profile of the infrastructure configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceProfileName")]
    pub instance_profile_name: cfn_resources::StrVal,

    ///
    /// The instance types of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,

    ///
    /// The Amazon EC2 key pair of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<cfn_resources::StrVal>,

    ///
    /// The logging configuration defines where Image Builder uploads your logs.
    ///
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    ///
    /// The name of the infrastructure configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[-_A-Za-z-0-9][-_A-Za-z0-9 ]{1,126}[-_A-Za-z-0-9]$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The tags attached to the resource created by Image Builder.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The security group IDs of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The Amazon Resource Name (ARN) of the SNS topic for the infrastructure 			configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<cfn_resources::StrVal>,

    ///
    /// The subnet ID of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<cfn_resources::StrVal>,

    ///
    /// The tags of the infrastructure configuration.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The terminate instance on failure configuration of the infrastructure 			configuration.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminateInstanceOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_instance_on_failure: Option<bool>,

    #[serde(skip_serializing)]
    pub att_arn: CfnInfrastructureConfigurationarn,

    #[serde(skip_serializing)]
    pub att_name: CfnInfrastructureConfigurationname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInfrastructureConfigurationarn;
impl CfnInfrastructureConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInfrastructureConfigurationname;
impl CfnInfrastructureConfigurationname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnInfrastructureConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::ImageBuilder::InfrastructureConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.instance_metadata_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.instance_profile_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'instance_profile_name'. {} is greater than 256", s.len()));
            }
        }

        let the_val = &self.instance_profile_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_profile_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.key_pair {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key_pair'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key_pair {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key_pair'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.logging.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.sns_topic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'sns_topic_arn'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sns_topic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sns_topic_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subnet_id'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'subnet_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The instance metadata options that apply to the HTTP requests that pipeline builds use 			to launch EC2 build and test instances. For more information about instance metadata 			options, see Configure the instance metadata options in the 				        Amazon EC2 User Guide       for Linux instances, or Configure the instance metadata options in the 				        Amazon EC2 Windows Guide       for Windows instances.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InstanceMetadataOptions {
    ///
    /// Limit the number of hops that an instance metadata request can traverse to reach its 			destination. The default is one hop. However, if HTTP tokens are required, container 			image builds need a minimum of two hops.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i64>,

    ///
    /// Indicates whether a signed token header is required for instance metadata retrieval 			requests. The values affect the response as follows:
    ///
    /// required – When you retrieve the IAM 					role credentials, version 2.0 credentials are returned in all cases.                        optional – You can include a signed 					token header in your request to retrieve instance metadata, or you can leave it 					out. If you include it, version 2.0 credentials are returned for the IAM role. 					Otherwise, version 1.0 credentials are returned.
    ///
    /// The default setting is optional.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: optional|required
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InstanceMetadataOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.http_put_response_hop_limit {
            if *the_val > 64 as _ {
                return Err(format!("Max validation failed on field 'http_put_response_hop_limit'. {} is greater than 64", the_val));
            }
        }

        if let Some(the_val) = &self.http_put_response_hop_limit {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'http_put_response_hop_limit'. {} is less than 1", the_val));
            }
        }

        Ok(())
    }
}

/// Logging configuration defines where Image Builder uploads your logs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Logging {
    ///
    /// The Amazon S3 logging configuration.
    ///
    /// Required: No
    ///
    /// Type: S3Logs
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3Logs>,
}

impl cfn_resources::CfnResource for Logging {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_logs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Amazon S3 logging configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3Logs {
    ///
    /// The S3 bucket in which to store the logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon S3 path to the bucket where the logs are stored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Logs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.s3_bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_bucket_name'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_bucket_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_key_prefix'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_key_prefix'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
