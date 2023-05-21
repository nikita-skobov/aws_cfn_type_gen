/// The AWS::DataSync::LocationEFS resource creates an endpoint for an Amazon EFS file system. AWS DataSync can access this endpoint as a source or destination location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationEFS {
    ///
    /// Specifies the Amazon Resource Name (ARN) of the access point that DataSync uses    to access the Amazon EFS file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):elasticfilesystem:[a-z\-0-9]+:[0-9]{12}:access-point/fsap-[0-9a-f]{8,40}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the subnet and security groups DataSync uses to access your Amazon EFS file system.
    ///
    /// Required: Yes
    ///
    /// Type: Ec2Config
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2Config")]
    pub ec2_config: Ec2Config,

    ///
    /// Specifies the ARN for the Amazon EFS file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):elasticfilesystem:[a-z\-0-9]*:[0-9]{12}:file-system/fs-.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "EfsFilesystemArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_filesystem_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies an AWS Identity and Access Management (IAM) role that DataSync    assumes when mounting the Amazon EFS file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):iam::[0-9]{12}:role/.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether you want DataSync to use Transport Layer Security (TLS) 1.2    encryption when it copies data to or from the Amazon EFS file system.
    ///
    /// If you specify an access point using AccessPointArn or an IAM    role using FileSystemAccessRoleArn, you must set this parameter to     TLS1_2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | TLS1_2
    ///
    /// Update requires: Replacement
    #[serde(rename = "InTransitEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_encryption: Option<LocationEFSInTransitEncryptionEnum>,

    ///
    /// Specifies a mount path for your Amazon EFS file system. This is where DataSync reads or writes data (depending on if this is a source or destination location).    By default, DataSync uses the root directory, but you can also include    subdirectories.
    ///
    /// NoteYou must specify a value with forward slashes (for example,     /path/to/folder).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\p{Zs}]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the key-value pair that represents a tag that you want to add to the    resource. The value can be an empty string. This value helps you manage, filter, and search    for your resources. We recommend that you create a name tag for your location.
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

#[derive(Clone, Debug, serde::Serialize)]
pub enum LocationEFSInTransitEncryptionEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// TLS1_2
    #[serde(rename = "TLS1_2")]
    Tls12,
}

impl Default for LocationEFSInTransitEncryptionEnum {
    fn default() -> Self {
        LocationEFSInTransitEncryptionEnum::None
    }
}

impl cfn_resources::CfnResource for CfnLocationEFS {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationEFS"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_point_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'access_point_arn'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        self.ec2_config.validate()?;

        if let Some(the_val) = &self.efs_filesystem_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'efs_filesystem_arn'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.file_system_access_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'file_system_access_role_arn'. {} is greater than 2048", s.len()));
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

/// The subnet and security groups that AWS DataSync uses to access your Amazon EFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ec2Config {
    ///
    /// Specifies the Amazon Resource Names (ARNs) of the security groups associated with an     Amazon EFS file system's mount target.
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
    /// Specifies the ARN of a subnet where DataSync creates the network interfaces for managing traffic during your transfer.
    ///
    /// The subnet must be located:
    ///
    /// In the same virtual private cloud (VPC) as the Amazon EFS file system.               In the same Availability Zone as at least one mount target for the Amazon EFS file      system.
    ///
    /// NoteYou don't need to specify a subnet that includes a file system mount target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:subnet/.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetArn")]
    pub subnet_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Ec2Config {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.security_group_arns;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'security_group_arns'. {} is greater than 5",
                the_val.len()
            ));
        }

        let the_val = &self.subnet_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_arn'. {} is greater than 128",
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
