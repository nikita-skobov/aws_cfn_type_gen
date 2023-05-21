

/// Specifies a placement group in which to launch instances. The strategy of the placement     group determines how the instances are organized within the group.
///
/// A cluster placement group is a logical grouping of instances within a     single Availability Zone that benefit from low network latency, high network throughput. A     spread placement group places instances on distinct hardware. A     partition placement group places groups of instances in different     partitions, where instances in one partition do not share the same hardware with instances     in another partition.
///
/// For more information, see Placement Groups in the     Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPlacementGroup {


    /// 
    /// The tags to apply to the new placement group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The placement strategy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: cluster | partition | spread
    ///
    /// Update requires: Replacement
    #[serde(rename = "Strategy")]
    pub strategy: Option<PlacementGroupStrategyEnum>,


    /// 
    /// The number of partitions. Valid only when Strategy is       set to partition.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "PartitionCount")]
    pub partition_count: Option<i64>,


    /// 
    /// Determines how placement groups spread instances.
    /// 
    /// Host – You can use host only with Outpost placement           groups.               Rack – No usage restrictions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: host | rack
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpreadLevel")]
    pub spread_level: Option<PlacementGroupSpreadLevelEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PlacementGroupStrategyEnum {

    /// cluster
    #[serde(rename = "cluster")]
    Cluster,

    /// partition
    #[serde(rename = "partition")]
    Partition,

    /// spread
    #[serde(rename = "spread")]
    Spread,

}

impl Default for PlacementGroupStrategyEnum {
    fn default() -> Self {
        PlacementGroupStrategyEnum::Cluster
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PlacementGroupSpreadLevelEnum {

    /// host
    #[serde(rename = "host")]
    Host,

    /// rack
    #[serde(rename = "rack")]
    Rack,

}

impl Default for PlacementGroupSpreadLevelEnum {
    fn default() -> Self {
        PlacementGroupSpreadLevelEnum::Host
    }
}


impl cfn_resources::CfnResource for CfnPlacementGroup {
    fn type_string() -> &'static str {
        "AWS::EC2::PlacementGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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


