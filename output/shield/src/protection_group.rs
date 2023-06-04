/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProtectionGroup {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-aggregation
    #[serde(rename = "Aggregation")]
    pub aggregation: cfn_resources::StrVal,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-members
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-pattern
    #[serde(rename = "Pattern")]
    pub pattern: cfn_resources::StrVal,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-protectiongroupid
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: cfn_resources::StrVal,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-resourcetype
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<cfn_resources::StrVal>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-tags
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_protection_group_arn: CfnProtectionGroupprotectiongrouparn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProtectionGroupprotectiongrouparn;
impl CfnProtectionGroupprotectiongrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"ProtectionGroupArn"#
    }
}

impl cfn_resources::CfnResource for CfnProtectionGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Shield::ProtectionGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-key
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-value
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
