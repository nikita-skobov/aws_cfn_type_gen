/// The AWS::DataSync::LocationNFS resource specifies a file system on a     Network File System (NFS) server that can be read from or written to.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLocationNFS {
    ///
    /// The NFS mount options that DataSync can use to mount your NFS share.
    ///
    /// Required: No
    ///
    /// Type: MountOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mount_options: Option<MountOptions>,

    ///
    /// Contains a list of Amazon Resource Names (ARNs) of agents that are used to connect to    an NFS server.
    ///
    /// If you are copying data to or from your AWS Snowcone device, see NFS Server on      AWS Snowcone for more information.
    ///
    /// Required: Yes
    ///
    /// Type: OnPremConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPremConfig")]
    pub on_prem_config: OnPremConfig,

    ///
    /// The name of the NFS server. This value is the IP address or Domain Name Service (DNS)    name of the NFS server. An agent that is installed on-premises uses this hostname to mount the    NFS server in a network.
    ///
    /// If you are copying data to or from your AWS Snowcone device, see NFS Server on      AWS Snowcone for more information.
    ///
    /// NoteThis name must either be DNS-compliant or must be an IP version 4 (IPv4)     address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerHostname")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub server_hostname: Option<cfn_resources::StrVal>,

    ///
    /// The subdirectory in the NFS file system that is used to read data from the NFS source    location or write data to the NFS destination. The NFS path should be a path that's    exported by the NFS server, or a subdirectory of that path. The path should be such that it    can be mounted by other NFS clients in your network.
    ///
    /// To see all the paths exported by your NFS server, run "showmount -e     nfs-server-name" from an NFS client that has access to your server. You can specify    any directory that appears in the results, and any subdirectory of that directory. Ensure that    the NFS export is accessible without Kerberos authentication.
    ///
    /// To transfer all the data in the folder you specified, DataSync needs to have    permissions to read all the data. To ensure this, either configure the NFS export with     no_root_squash, or ensure that the permissions for all of the files that you    want DataSync allow read access for all users. Doing either enables the agent to    read the files. For the agent to access directories, you must additionally enable all execute    access.
    ///
    /// If you are copying data to or from your AWS Snowcone device, see NFS Server on      AWS Snowcone for more information.
    ///
    /// For information about NFS export configuration, see 18.7. The /etc/exports Configuration File in the Red Hat Enterprise Linux    documentation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\p{Zs}]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// The key-value pair that represents the tag that you want to add to the location. The    value can be an empty string. We recommend using tags to name your resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_location_arn: CfnLocationNFSlocationarn,

    #[serde(skip_serializing)]
    pub att_location_uri: CfnLocationNFSlocationuri,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationNFSlocationarn;
impl CfnLocationNFSlocationarn {
    pub fn att_name(&self) -> &'static str {
        r#"LocationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationNFSlocationuri;
impl CfnLocationNFSlocationuri {
    pub fn att_name(&self) -> &'static str {
        r#"LocationUri"#
    }
}

impl cfn_resources::CfnResource for CfnLocationNFS {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationNFS"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.mount_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.on_prem_config.validate()?;

        if let Some(the_val) = &self.server_hostname {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'server_hostname'. {} is greater than 255",
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

/// The NFS mount options that DataSync can use to mount your NFS share.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MountOptions {
    ///
    /// Specifies the NFS version that you want DataSync to use when mounting your NFS    share. If the server refuses to use the version specified, the task fails.
    ///
    /// You can specify the following options:
    ///
    /// AUTOMATIC (default): DataSync chooses NFS version 4.1.                        NFS3: Stateless protocol version that allows for asynchronous writes on      the server.                        NFSv4_0: Stateful, firewall-friendly protocol version that supports      delegations and pseudo file systems.                        NFSv4_1: Stateful protocol version that supports sessions, directory      delegations, and parallel data processing. NFS version 4.1 also includes all features      available in version 4.0.
    ///
    /// NoteDataSync currently only supports NFS version 3 with Amazon FSx for NetApp ONTAP locations.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | NFS3 | NFS4_0 | NFS4_1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version: Option<MountOptionsVersionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MountOptionsVersionEnum {
    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// NFS3
    #[serde(rename = "NFS3")]
    Nfs3,

    /// NFS4_0
    #[serde(rename = "NFS4_0")]
    Nfs40,

    /// NFS4_1
    #[serde(rename = "NFS4_1")]
    Nfs41,
}

impl Default for MountOptionsVersionEnum {
    fn default() -> Self {
        MountOptionsVersionEnum::Automatic
    }
}

impl cfn_resources::CfnResource for MountOptions {
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

/// A list of Amazon Resource Names (ARNs) of agents to use for a Network File System (NFS)    location.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OnPremConfig {
    ///
    /// ARNs of the agents to use for an NFS location.
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
}

impl cfn_resources::CfnResource for OnPremConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
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
