/// Describes a Verified Access group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVerifiedAccessGroup {
    ///
    /// A description for the AWS Verified Access group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The Verified Access policy document.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<cfn_resources::StrVal>,

    ///
    /// The status of the Verified Access policy.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_enabled: Option<bool>,

    ///
    /// The tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the AWS Verified Access instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessInstanceId")]
    pub verified_access_instance_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnVerifiedAccessGroupcreationtime,

    #[serde(skip_serializing)]
    pub att_last_updated_time: CfnVerifiedAccessGrouplastupdatedtime,

    #[serde(skip_serializing)]
    pub att_owner: CfnVerifiedAccessGroupowner,

    #[serde(skip_serializing)]
    pub att_verified_access_group_arn: CfnVerifiedAccessGroupverifiedaccessgrouparn,

    #[serde(skip_serializing)]
    pub att_verified_access_group_id: CfnVerifiedAccessGroupverifiedaccessgroupid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessGroupcreationtime;
impl CfnVerifiedAccessGroupcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessGrouplastupdatedtime;
impl CfnVerifiedAccessGrouplastupdatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastUpdatedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessGroupowner;
impl CfnVerifiedAccessGroupowner {
    pub fn att_name(&self) -> &'static str {
        r#"Owner"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessGroupverifiedaccessgrouparn;
impl CfnVerifiedAccessGroupverifiedaccessgrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"VerifiedAccessGroupArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessGroupverifiedaccessgroupid;
impl CfnVerifiedAccessGroupverifiedaccessgroupid {
    pub fn att_name(&self) -> &'static str {
        r#"VerifiedAccessGroupId"#
    }
}

impl cfn_resources::CfnResource for CfnVerifiedAccessGroup {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VerifiedAccessGroup"
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
