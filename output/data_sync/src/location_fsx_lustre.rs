/// The AWS::DataSync::LocationFSxLustre resource specifies an endpoint for an Amazon FSx for Lustre file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationFSxLustre {
    ///
    /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
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
    pub fsx_filesystem_arn: Option<String>,

    ///
    /// The ARNs of the security groups that are used to configure the FSx for Lustre file system.
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
    /// A subdirectory in the location's path. This subdirectory in the FSx for Lustre    file system is used to read data from the FSx for Lustre source location or write    data to the FSx for Lustre destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\$\p{Zs}]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,

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

impl cfn_resources::CfnResource for CfnLocationFSxLustre {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationFSxLustre"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.fsx_filesystem_arn {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'fsx_filesystem_arn'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.security_group_arns;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'security_group_arns'. {} is greater than 5",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.subdirectory {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'subdirectory'. {} is greater than 4096",
                    the_val.len()
                ));
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
