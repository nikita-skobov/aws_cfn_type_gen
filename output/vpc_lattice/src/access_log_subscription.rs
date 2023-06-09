/// Enables access logs to be sent to Amazon CloudWatch, Amazon S3, and Amazon Kinesis Data Firehose. The service network owner  can use the access logs to audit the services in the network. The service network owner will only  see access logs from clients and services that are associated with their service network. Access  log entries represent traffic originated from VPCs associated with that network. For more  information, see Access logs in the   Amazon VPC Lattice User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAccessLogSubscription {
    ///
    /// The Amazon Resource Name (ARN) of the destination. The supported destination types are  CloudWatch Log groups, Kinesis Data Firehose delivery streams, and Amazon S3 buckets.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: cfn_resources::StrVal,

    ///
    /// The ID or Amazon Resource Name (ARN) of the service network or service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The tags for the access log subscription.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnAccessLogSubscriptionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnAccessLogSubscriptionid,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnAccessLogSubscriptionresourcearn,

    #[serde(skip_serializing)]
    pub att_resource_id: CfnAccessLogSubscriptionresourceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessLogSubscriptionarn;
impl CfnAccessLogSubscriptionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessLogSubscriptionid;
impl CfnAccessLogSubscriptionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessLogSubscriptionresourcearn;
impl CfnAccessLogSubscriptionresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessLogSubscriptionresourceid;
impl CfnAccessLogSubscriptionresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceId"#
    }
}

impl cfn_resources::CfnResource for CfnAccessLogSubscription {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::AccessLogSubscription"
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
