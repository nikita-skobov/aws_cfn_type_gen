/// The AWS::ElastiCache::ParameterGroup type creates a new cache parameter group. Cache parameter groups control the parameters for a cache cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnParameterGroup {
    ///
    /// The name of the cache parameter group family that this cache parameter group is compatible with.
    ///
    /// Valid values are:   memcached1.4 |    memcached1.5 |       memcached1.6 |   redis2.6 |   redis2.8 |   redis3.2 |   redis4.0 |   redis5.0 |       redis6.x |      redis7
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CacheParameterGroupFamily")]
    pub cache_parameter_group_family: cfn_resources::StrVal,

    ///
    /// The description for this cache parameter group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: cfn_resources::StrVal,

    ///
    /// A comma-delimited list of parameter name/value pairs.
    ///
    /// For example:
    ///
    /// "Properties" : {  "cas_disabled" : "1",  "chunk_size_growth_factor" : "1.02" }
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

    /// A tag that can be added to an ElastiCache parameter group.    Tags are composed of a Key/Value pair. You can use tags to categorize and track all your parameter groups. A tag with a null Value is permitted.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnParameterGroup {
    fn type_string(&self) -> &'static str {
        "AWS::ElastiCache::ParameterGroup"
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
