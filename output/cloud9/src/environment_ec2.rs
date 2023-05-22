/// The AWS::Cloud9::EnvironmentEC2 resource creates an Amazon EC2 development environment in AWS Cloud9. For more information, see Creating an Environment in the AWS Cloud9 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentEC2 {
    ///
    /// The number of minutes until the running instance is shut down after the environment was last used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 20160
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutomaticStopTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_stop_time_minutes: Option<i64>,

    ///
    /// The connection type used for connecting to an Amazon EC2 environment. Valid values are CONNECT_SSH (default) and CONNECT_SSM (connected through AWS Systems Manager).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONNECT_SSH | CONNECT_SSM
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<EnvironmentEC2ConnectionTypeEnum>,

    ///
    /// The description of the environment to create.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance. To choose an AMI for the instance, you must specify a valid AMI alias or a valid AWS Systems Manager path.
    ///
    /// The default AMI is used if the parameter isn't explicitly assigned a value in the request.
    ///
    /// AMI aliases
    ///
    /// Amazon Linux (default): amazonlinux-1-x86_64 Amazon Linux 2: amazonlinux-2-x86_64Ubuntu 18.04: ubuntu-18.04-x86_64
    ///
    /// SSM paths
    ///
    /// Amazon Linux (default): resolve:ssm:/aws/service/cloud9/amis/amazonlinux-1-x86_64Amazon Linux 2: resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64Ubuntu 18.04: resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<cfn_resources::StrVal>,

    ///
    /// The type of instance to connect to the environment (for example, t2.micro).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^[a-z][1-9][.][a-z0-9]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,

    ///
    /// The name of the environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the environment owner. This ARN can be the ARN of any AWS Identity and Access Management principal. If this value is not specified, the ARN defaults to this environment's creator.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):(iam|sts)::\d+:(root|(user\/[\w+=/:,.@-]{1,64}|federated-user\/[\w+=/:,.@-]{2,32}|assumed-role\/[\w+=:,.@-]{1,64}\/[\w+=,.@-]{1,64}))$
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<cfn_resources::StrVal>,

    ///
    /// Any AWS CodeCommit source code repositories to be cloned into the development environment.
    ///
    /// Required: No
    ///
    /// Type: List of Repository
    ///
    /// Update requires: Replacement
    #[serde(rename = "Repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

    ///
    /// The ID of the subnet in Amazon Virtual Private Cloud (Amazon VPC) that AWS Cloud9 will use to communicate with the Amazon Elastic Compute Cloud (Amazon EC2) instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 15
    ///
    /// Maximum: 24
    ///
    /// Pattern: ^(subnet-[0-9a-f]{8}|subnet-[0-9a-f]{17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs that will be associated with the new AWS Cloud9 development    environment.
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

    #[serde(skip_serializing)]
    pub att_arn: CfnEnvironmentEC2arn,

    #[serde(skip_serializing)]
    pub att_name: CfnEnvironmentEC2name,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EnvironmentEC2ConnectionTypeEnum {
    /// CONNECT_SSH
    #[serde(rename = "CONNECT_SSH")]
    Connectssh,

    /// CONNECT_SSM
    #[serde(rename = "CONNECT_SSM")]
    Connectssm,
}

impl Default for EnvironmentEC2ConnectionTypeEnum {
    fn default() -> Self {
        EnvironmentEC2ConnectionTypeEnum::Connectssh
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentEC2arn;
impl CfnEnvironmentEC2arn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentEC2name;
impl CfnEnvironmentEC2name {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnEnvironmentEC2 {
    fn type_string(&self) -> &'static str {
        "AWS::Cloud9::EnvironmentEC2"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.automatic_stop_time_minutes {
            if *the_val > 20160 as _ {
                return Err(format!("Max validation failed on field 'automatic_stop_time_minutes'. {} is greater than 20160", the_val));
            }
        }

        if let Some(the_val) = &self.automatic_stop_time_minutes {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'automatic_stop_time_minutes'. {} is less than 0", the_val));
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 200",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.image_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'image_id'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_type'. {} is greater than 20",
                    s.len()
                ));
            }
        }

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 5 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_type'. {} is less than 5",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 24 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subnet_id'. {} is greater than 24",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 15 as _ {
                    return Err(format!(
                        "Min validation failed on field 'subnet_id'. {} is less than 15",
                        s.len()
                    ));
                }
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

        Ok(())
    }
}

/// The Repository property type specifies an AWS CodeCommit source code repository to be cloned into an AWS Cloud9 development environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Repository {
    ///
    /// The path within the development environment's default file system location to clone the AWS CodeCommit repository into. For example, /REPOSITORY_NAME would clone the repository into the /home/USER_NAME/environment/REPOSITORY_NAME directory in the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathComponent")]
    pub path_component: cfn_resources::StrVal,

    ///
    /// The clone URL of the AWS CodeCommit repository to be cloned. For example, for an AWS CodeCommit repository this might be https://git-codecommit.us-east-2.amazonaws.com/v1/repos/REPOSITORY_NAME.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Repository {
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
