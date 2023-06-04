/// Creates a packaging group.
///
/// The packaging group holds one or more packaging configurations. When you create an asset, you specify the packaging group associated with the asset. The asset has playback endpoints for each packaging configuration within the group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPackagingGroup {
    ///
    /// Parameters for CDN authorization.
    ///
    /// Required: No
    ///
    /// Type: Authorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,

    ///
    /// The configuration parameters for egress access logging.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<LogConfiguration>,

    ///
    /// Unique identifier that you assign to the packaging group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The tags to assign to the packaging group.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnPackagingGrouparn,

    #[serde(skip_serializing)]
    pub att_domain_name: CfnPackagingGroupdomainname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPackagingGrouparn;
impl CfnPackagingGrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPackagingGroupdomainname;
impl CfnPackagingGroupdomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DomainName"#
    }
}

impl cfn_resources::CfnResource for CfnPackagingGroup {
    fn type_string(&self) -> &'static str {
        "AWS::MediaPackage::PackagingGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authorization
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.egress_access_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Parameters for enabling CDN authorization.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Authorization {
    ///
    /// The Amazon Resource Name (ARN) for the secret in AWS Secrets Manager that is used for CDN authorization.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) for the IAM role that allows AWS Elemental MediaPackage to communicate with AWS Secrets Manager.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Authorization {
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

/// Sets a custom Amazon CloudWatch log group name for egress logs. If a log group name isn't specified, the default name is used: /aws/MediaPackage/EgressAccessLogs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LogConfiguration {
    ///
    /// Sets a custom Amazon CloudWatch log group name for egress logs. If a log group name isn't specified, the default name is used: /aws/MediaPackage/EgressAccessLogs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LogConfiguration {
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
