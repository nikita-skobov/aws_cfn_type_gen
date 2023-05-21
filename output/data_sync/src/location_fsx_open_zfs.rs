/// The AWS::DataSync::LocationFSxOpenZFS resource specifies an endpoint for an Amazon FSx for OpenZFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationFSxOpenZFS {
    ///
    /// The Amazon Resource Name (ARN) of the FSx for OpenZFS file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):fsx:[a-z\-0-9]*:[0-9]{12}:file-system/fs-.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_filesystem_arn: Option<cfn_resources::StrVal>,

    ///
    /// The type of protocol that AWS DataSync uses to access your file system.
    ///
    /// Required: Yes
    ///
    /// Type: Protocol
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: Protocol,

    ///
    /// The ARNs of the security groups that are used to configure the FSx for OpenZFS file system.
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$
    ///
    /// Length constraints: Maximum length of 128.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,

    ///
    /// A subdirectory in the location's path that must begin with /fsx. DataSync uses this subdirectory to read or write data (depending on whether the file    system is a source or destination location).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,4096}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// The key-value pair that represents a tag that you want to add to the resource. The value    can be an empty string. This value helps you manage, filter, and search for your resources. We    recommend that you create a name tag for your location.
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
}

impl cfn_resources::CfnResource for CfnLocationFSxOpenZFS {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationFSxOpenZFS"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.fsx_filesystem_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'fsx_filesystem_arn'. {} is greater than 128", s.len()));
                }
            }
        }

        self.protocol.validate()?;

        let the_val = &self.security_group_arns;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'security_group_arns'. {} is greater than 5",
                the_val.len()
            ));
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

/// Represents the mount options that are available for DataSync to access a Network File System (NFS) location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MountOptions {
    ///
    /// The specific NFS version that you want DataSync to use to mount your NFS share. If the server refuses to use the version specified, the sync will fail. If you don't specify a version, DataSync defaults to AUTOMATIC. That is, DataSync automatically selects a version based on negotiation with the NFS server.
    ///
    /// You can specify the following NFS versions:
    ///
    /// NFSv3: Stateless protocol version that allows for asynchronous        writes on the server.            NFSv4.0: Stateful, firewall-friendly protocol version that supports        delegations and pseudo file systems.            NFSv4.1: Stateful protocol version that supports sessions,        directory delegations, and parallel data processing. Version 4.1 also includes all        features available in version 4.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | NFS3 | NFS4_0 | NFS4_1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<MountOptionsVersionEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
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

/// Represents the Network File System (NFS) protocol that AWS DataSync uses to access your Amazon FSx for OpenZFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NFS {
    ///
    /// Represents the mount options that are available for DataSync to access an NFS location.
    ///
    /// Required: Yes
    ///
    /// Type: MountOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountOptions")]
    pub mount_options: MountOptions,
}

impl cfn_resources::CfnResource for NFS {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.mount_options.validate()?;

        Ok(())
    }
}

/// Represents the protocol that AWS DataSync uses to access your Amazon FSx for OpenZFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Protocol {
    ///
    /// Represents the Network File System (NFS) protocol that DataSync uses to access your FSx for OpenZFS file system.
    ///
    /// Required: No
    ///
    /// Type: NFS
    ///
    /// Update requires: Replacement
    #[serde(rename = "NFS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFS>,
}

impl cfn_resources::CfnResource for Protocol {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.nfs.as_ref().map_or(Ok(()), |val| val.validate())?;

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
