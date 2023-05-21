/// The AWS::RDS::DBSecurityGroup resource creates or updates an Amazon RDS       DB security group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBSecurityGroup {
    ///
    /// Ingress rules to be applied to the DB security group.
    ///
    /// Required: Yes
    ///
    /// Type: List of Ingress
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBSecurityGroupIngress")]
    pub dbsecurity_group_ingress: Vec<Ingress>,

    ///
    /// The identifier of an Amazon VPC. This property indicates the VPC that this DB security       group belongs to.
    ///
    /// ImportantThe EC2VpcId property is for backward compatibility with older         regions, and is no longer recommended for providing security information to an RDS         DB instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2VpcId")]
    pub ec2_vpc_id: Option<String>,

    ///
    /// Provides the description of the DB security group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupDescription")]
    pub group_description: String,

    ///
    /// An optional array of key-value pairs to apply to this DB security group.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDBSecurityGroup {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::DBSecurityGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Ingress property type specifies an individual ingress rule within an         AWS::RDS::DBSecurityGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ingress {
    ///
    /// The IP range to authorize.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CIDRIP")]
    pub cidrip: Option<String>,

    ///
    /// Id of the EC2 security group to authorize.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName or EC2SecurityGroupId must be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2SecurityGroupId")]
    pub ec2_security_group_id: Option<String>,

    ///
    /// Name of the EC2 security group to authorize.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName    or EC2SecurityGroupId must be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: Option<String>,

    ///
    /// AWS account number of the owner of the EC2 security group     specified in the EC2SecurityGroupName parameter.     The AWS access key ID isn't an acceptable value.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName or EC2SecurityGroupId must be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,
}

impl cfn_resources::CfnResource for Ingress {
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
