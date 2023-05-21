/// Creates a new VPC connection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPCConnection {
    ///
    /// The availability status of the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVAILABLE | PARTIALLY_AVAILABLE | UNAVAILABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<VPCConnectionAvailabilityStatusEnum>,

    ///
    /// The AWS account ID of the account where you want to create a new VPC 			connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,

    ///
    /// A list of IP addresses of DNS resolver endpoints for the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsResolvers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,

    ///
    /// The display name for the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///
    /// The ARN of the         IAM role associated with the VPC       connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    ///
    /// The Amazon EC2 security group IDs associated with the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of subnet IDs for the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// A map of the key-value pairs for the resource tag or tags assigned to the VPC 			connection.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the VPC connection that you're creating. This ID is a unique identifier for each AWS Region in an         AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "VPCConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcconnection_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VPCConnectionAvailabilityStatusEnum {
    /// AVAILABLE
    #[serde(rename = "AVAILABLE")]
    Available,

    /// PARTIALLY_AVAILABLE
    #[serde(rename = "PARTIALLY_AVAILABLE")]
    Partiallyavailable,

    /// UNAVAILABLE
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
}

impl Default for VPCConnectionAvailabilityStatusEnum {
    fn default() -> Self {
        VPCConnectionAvailabilityStatusEnum::Available
    }
}

impl cfn_resources::CfnResource for CfnVPCConnection {
    fn type_string(&self) -> &'static str {
        "AWS::QuickSight::VPCConnection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'aws_account_id'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'aws_account_id'. {} is less than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_ids {
            if the_val.len() > 15 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_ids'. {} is greater than 15",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vpcconnection_id {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpcconnection_id'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vpcconnection_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpcconnection_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The structure that contains information about a network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInterface {
    ///
    /// The availability zone that the network interface resides in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    ///
    /// An error message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    ///
    /// The network interface ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^eni-[0-9a-z]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,

    ///
    /// The status of the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ATTACHMENT_FAILED_ROLLBACK_FAILED | AVAILABLE | CREATING | CREATION_FAILED | DELETED | DELETING | DELETION_FAILED | DELETION_SCHEDULED | UPDATE_FAILED | UPDATING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NetworkInterfaceStatusEnum>,

    ///
    /// The subnet ID associated with the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^subnet-[0-9a-z]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkInterfaceStatusEnum {
    /// ATTACHMENT_FAILED_ROLLBACK_FAILED
    #[serde(rename = "ATTACHMENT_FAILED_ROLLBACK_FAILED")]
    Attachmentfailedrollbackfailed,

    /// AVAILABLE
    #[serde(rename = "AVAILABLE")]
    Available,

    /// CREATING
    #[serde(rename = "CREATING")]
    Creating,

    /// CREATION_FAILED
    #[serde(rename = "CREATION_FAILED")]
    Creationfailed,

    /// DELETED
    #[serde(rename = "DELETED")]
    Deleted,

    /// DELETING
    #[serde(rename = "DELETING")]
    Deleting,

    /// DELETION_FAILED
    #[serde(rename = "DELETION_FAILED")]
    Deletionfailed,

    /// DELETION_SCHEDULED
    #[serde(rename = "DELETION_SCHEDULED")]
    Deletionscheduled,

    /// UPDATE_FAILED
    #[serde(rename = "UPDATE_FAILED")]
    Updatefailed,

    /// UPDATING
    #[serde(rename = "UPDATING")]
    Updating,
}

impl Default for NetworkInterfaceStatusEnum {
    fn default() -> Self {
        NetworkInterfaceStatusEnum::Attachmentfailedrollbackfailed
    }
}

impl cfn_resources::CfnResource for NetworkInterface {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.network_interface_id {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'network_interface_id'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_id'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'subnet_id'. {} is less than 1",
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
