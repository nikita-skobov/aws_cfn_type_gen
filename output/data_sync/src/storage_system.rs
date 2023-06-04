/// The AWS::DataSync::StorageSystem resource creates an AWS resource for an on-premises storage system that you want DataSync Discovery to collect     information about. For more information, see discovering your storage with DataSync Discovery.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageSystem {
    ///
    /// Specifies the Amazon Resource Name (ARN) of the DataSync agent that connects to    and reads from your on-premises storage system's management interface.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,

    ///
    /// Specifies the ARN of the Amazon CloudWatch log group for monitoring and logging    discovery job events.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 562
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):logs:[a-z\-0-9]+:[0-9]{12}:log-group:([^:\*]*)(:\*)?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies a familiar name for your on-premises storage system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\p{L}\p{M}\p{N}\s+=._:@\/-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the server name and network port required to connect with the management    interface of your on-premises storage system.
    ///
    /// Required: Yes
    ///
    /// Type: ServerConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerConfiguration")]
    pub server_configuration: ServerConfiguration,

    ///
    /// Specifies the user name and password for accessing your on-premises storage system's    management interface.
    ///
    /// Required: No
    ///
    /// Type: ServerCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_credentials: Option<ServerCredentials>,

    ///
    /// Specifies the type of on-premises storage system that you want DataSync Discovery to collect    information about.
    ///
    /// NoteDataSync Discovery currently supports NetApp     Fabric-Attached     Storage (FAS) and All Flash FAS (AFF) systems running ONTAP 9.7 or     later.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NetAppONTAP
    ///
    /// Update requires: No interruption
    #[serde(rename = "SystemType")]
    pub system_type: StorageSystemSystemTypeEnum,

    ///
    /// Specifies labels that help you categorize, filter, and search for your AWS    resources. We recommend creating at least a name tag for your on-premises storage    system.
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

    #[serde(skip_serializing)]
    pub att_connectivity_status: CfnStorageSystemconnectivitystatus,

    #[serde(skip_serializing)]
    pub att_secrets_manager_arn: CfnStorageSystemsecretsmanagerarn,

    #[serde(skip_serializing)]
    pub att_storage_system_arn: CfnStorageSystemstoragesystemarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StorageSystemSystemTypeEnum {
    /// NetAppONTAP
    #[serde(rename = "NetAppONTAP")]
    Netappontap,
}

impl Default for StorageSystemSystemTypeEnum {
    fn default() -> Self {
        StorageSystemSystemTypeEnum::Netappontap
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageSystemconnectivitystatus;
impl CfnStorageSystemconnectivitystatus {
    pub fn att_name(&self) -> &'static str {
        r#"ConnectivityStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageSystemsecretsmanagerarn;
impl CfnStorageSystemsecretsmanagerarn {
    pub fn att_name(&self) -> &'static str {
        r#"SecretsManagerArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageSystemstoragesystemarn;
impl CfnStorageSystemstoragesystemarn {
    pub fn att_name(&self) -> &'static str {
        r#"StorageSystemArn"#
    }
}

impl cfn_resources::CfnResource for CfnStorageSystem {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::StorageSystem"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.agent_arns;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'agent_arns'. {} is greater than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.cloud_watch_log_group_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 562 as _ {
                    return Err(format!("Max validation failed on field 'cloud_watch_log_group_arn'. {} is greater than 562", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.server_configuration.validate()?;

        self.server_credentials
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

/// The network settings that DataSync Discovery uses to connect with your on-premises storage system's    management interface.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ServerConfiguration {
    ///
    /// The domain name or IP address of your storage system's management interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerHostname")]
    pub server_hostname: cfn_resources::StrVal,

    ///
    /// The network port for accessing the storage system's management interface.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i64>,
}

impl cfn_resources::CfnResource for ServerConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.server_hostname;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'server_hostname'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.server_port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'server_port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.server_port {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'server_port'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The credentials that provide DataSync Discovery read access to your on-premises storage system's    management interface.
///
/// DataSync Discovery stores these credentials in AWS Secrets Manager. For more    information, see Accessing your on-premises     storage system.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ServerCredentials {
    ///
    /// Specifies the password for your storage system's management interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(?!.*[:\"][^:"]*$).+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// Specifies the user name for your storage system's management interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(?!.*[:\"][^:"]*$).+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ServerCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'password'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.username;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'username'. {} is greater than 1024",
                    s.len()
                ));
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
