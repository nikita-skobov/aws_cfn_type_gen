/// The AWS::DataSync::LocationHDFS resource specifies an endpoint for a Hadoop Distributed File System (HDFS).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLocationHDFS {
    ///
    /// The Amazon Resource Names (ARNs) of the agents that are used to connect to the HDFS    cluster.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: cfn_resources::StrVal,

    ///
    /// The size of data blocks to write into the HDFS cluster. The block size must be a multiple    of 512 bytes. The default block size is 128 mebibytes (MiB).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1048576
    ///
    /// Maximum: 1073741824
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,

    ///
    /// The Kerberos key table (keytab) that contains mappings between the defined Kerberos     principal and the encrypted keys. Provide the base64-encoded file text. If       KERBEROS is specified for AuthType, this value is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KerberosKeytab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<cfn_resources::StrVal>,

    ///
    /// The krb5.conf file that contains the Kerberos configuration information.     You can load the krb5.conf by providing a string of the file's contents or an       Amazon S3 presigned URL of the file. IfKERBEROS is specified for       AuthType, this value is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KerberosKrb5Conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_krb5_conf: Option<cfn_resources::StrVal>,

    ///
    /// The Kerberos principal with access to the files and folders on the HDFS cluster.
    ///
    /// NoteIf KERBEROS is specified for AuthenticationType, this     parameter is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "KerberosPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<cfn_resources::StrVal>,

    ///
    /// The URI of the HDFS cluster's Key Management Server (KMS).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^kms:\/\/http[s]?@(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])(;(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9]))*:[0-9]{1,5}\/kms$
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyProviderUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_provider_uri: Option<cfn_resources::StrVal>,

    ///
    /// The NameNode that manages the HDFS namespace. The NameNode performs operations such as    opening, closing, and renaming files and directories. The NameNode contains the information to    map blocks of data to the DataNodes. You can use only one NameNode.
    ///
    /// Required: Yes
    ///
    /// Type: List of NameNode
    ///
    /// Update requires: No interruption
    #[serde(rename = "NameNodes")]
    pub name_nodes: Vec<NameNode>,

    ///
    /// The Quality of Protection (QOP) configuration specifies the Remote Procedure Call (RPC)    and data transfer protection settings configured on the Hadoop Distributed File System (HDFS)    cluster. If QopConfiguration isn't specified, RpcProtection and     DataTransferProtection default to PRIVACY. If you set     RpcProtection or DataTransferProtection, the other parameter    assumes the same value.
    ///
    /// Required: No
    ///
    /// Type: QopConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "QopConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qop_configuration: Option<QopConfiguration>,

    ///
    /// The number of DataNodes to replicate the data to when writing to the HDFS cluster. By    default, data is replicated to three DataNodes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i64>,

    ///
    /// The user name used to identify the client on the host operating system.
    ///
    /// NoteIf SIMPLE is specified for AuthenticationType, this parameter     is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[_.A-Za-z0-9][-_.A-Za-z0-9]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_user: Option<cfn_resources::StrVal>,

    ///
    /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write    data to the HDFS cluster. If the subdirectory isn't specified, it will default to     /.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\$\p{Zs}]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// The key-value pair that represents the tag that you want to add to the location. The value    can be an empty string. We recommend using tags to name your resources.
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
    pub att_location_arn: CfnLocationHDFSlocationarn,

    #[serde(skip_serializing)]
    pub att_location_uri: CfnLocationHDFSlocationuri,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationHDFSlocationarn;
impl CfnLocationHDFSlocationarn {
    pub fn att_name(&self) -> &'static str {
        r#"LocationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationHDFSlocationuri;
impl CfnLocationHDFSlocationuri {
    pub fn att_name(&self) -> &'static str {
        r#"LocationUri"#
    }
}

impl cfn_resources::CfnResource for CfnLocationHDFS {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationHDFS"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.agent_arns;

        if the_val.len() > 4 as _ {
            return Err(format!(
                "Max validation failed on field 'agent_arns'. {} is greater than 4",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.block_size {
            if *the_val > 1073741824 as _ {
                return Err(format!(
                    "Max validation failed on field 'block_size'. {} is greater than 1073741824",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.block_size {
            if *the_val < 1048576 as _ {
                return Err(format!(
                    "Min validation failed on field 'block_size'. {} is less than 1048576",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.kerberos_principal {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'kerberos_principal'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.kerberos_principal {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kerberos_principal'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_provider_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'kms_key_provider_uri'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_provider_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_provider_uri'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.qop_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.replication_factor {
            if *the_val > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'replication_factor'. {} is greater than 512",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.replication_factor {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'replication_factor'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.simple_user {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'simple_user'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.simple_user {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'simple_user'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.subdirectory {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 4096 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subdirectory'. {} is greater than 4096",
                        s.len()
                    ));
                }
            }
        }

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

/// The    NameNode of the Hadoop Distributed File System (HDFS). The NameNode manages the file system's    namespace and performs operations such as opening, closing, and renaming files and    directories. The NameNode also contains the information to map blocks of data to the    DataNodes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NameNode {
    ///
    /// The hostname of the NameNode in the HDFS cluster. This value is the IP address or Domain    Name Service (DNS) name of the NameNode. An agent that's installed on-premises uses this    hostname to communicate with the NameNode in the network.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: cfn_resources::StrVal,

    ///
    /// The port that the NameNode uses to listen to client requests.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65536
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: i64,
}

impl cfn_resources::CfnResource for NameNode {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.hostname;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'hostname'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.hostname;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'hostname'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.port;

        if *the_val > 65536 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65536",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// The    Quality of Protection (QOP) configuration specifies the Remote Procedure Call (RPC) and data transfer privacy settings    configured on the Hadoop Distributed File System (HDFS) cluster.
///
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QopConfiguration {
    ///
    /// The data transfer protection setting configured on the HDFS cluster. This setting    corresponds to your dfs.data.transfer.protection setting in the     hdfs-site.xml file on your Hadoop cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHENTICATION | DISABLED | INTEGRITY | PRIVACY
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransferProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_protection: Option<QopConfigurationDataTransferProtectionEnum>,

    ///
    /// The    Remote Procedure Call (RPC) protection setting configured on the HDFS cluster. This setting    corresponds to your hadoop.rpc.protection setting in your     core-site.xml file on your Hadoop cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHENTICATION | DISABLED | INTEGRITY | PRIVACY
    ///
    /// Update requires: No interruption
    #[serde(rename = "RpcProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_protection: Option<QopConfigurationRpcProtectionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum QopConfigurationDataTransferProtectionEnum {
    /// AUTHENTICATION
    #[serde(rename = "AUTHENTICATION")]
    Authentication,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// INTEGRITY
    #[serde(rename = "INTEGRITY")]
    Integrity,

    /// PRIVACY
    #[serde(rename = "PRIVACY")]
    Privacy,
}

impl Default for QopConfigurationDataTransferProtectionEnum {
    fn default() -> Self {
        QopConfigurationDataTransferProtectionEnum::Authentication
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum QopConfigurationRpcProtectionEnum {
    /// AUTHENTICATION
    #[serde(rename = "AUTHENTICATION")]
    Authentication,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// INTEGRITY
    #[serde(rename = "INTEGRITY")]
    Integrity,

    /// PRIVACY
    #[serde(rename = "PRIVACY")]
    Privacy,
}

impl Default for QopConfigurationRpcProtectionEnum {
    fn default() -> Self {
        QopConfigurationRpcProtectionEnum::Authentication
    }
}

impl cfn_resources::CfnResource for QopConfiguration {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
