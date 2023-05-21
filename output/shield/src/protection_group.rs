

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html
#[derive(Default, serde::Serialize)]
pub struct CfnProtectionGroup {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-members
    #[serde(rename = "Members")]
    pub members: Option<Vec<String>>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-resourcetype
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-protectiongroupid
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-pattern
    #[serde(rename = "Pattern")]
    pub pattern: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-aggregation
    #[serde(rename = "Aggregation")]
    pub aggregation: String,

}


/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-value
    #[serde(rename = "Value")]
    pub value: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-key
    #[serde(rename = "Key")]
    pub key: String,

}