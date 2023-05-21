

/// Creates a resource group with the specified name and description. You can optionally       include either a resource query or a service configuration. For more information about       constructing a resource query, see Build queries and groups in         AWS Resource Groups in the         AWS Resource Groups User Guide. For more information       about service-linked groups and service configurations, see Service configurations for Resource Groups.
///
/// Minimum permissions
///
/// To run this command, you must have the following permissions:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGroup {


    /// 
    /// A list of the Amazon Resource Names (ARNs) of AWS resources that you       want to add to the specified group.
    /// 
    /// Note                                   You can specify the group membership either by using a list of               Resources or by using a ResourceQuery, but not             both.                   You can include a Resources property only if you also specify             a Configuration property.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,


    /// 
    /// The resource query structure that is used to dynamically determine which AWS resources are members of the associated resource group. For more       information about queries and how to construct them, see Build queries and groups in           AWS Resource Groups in the AWS Resource Groups User         Guide
    /// 
    /// Note                                   You can include either a ResourceQuery or a               Configuration, but not both.                   You can specify the group's membership either by using a               ResourceQuery or by using a list of Resources,             but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: ResourceQuery
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceQuery")]
    pub resource_query: Option<ResourceQuery>,


    /// 
    /// The name of a resource group. The name must be unique within the AWS       Region in which you create the resource. To create multiple resource groups based on the       same CloudFormation stack, you must generate unique names for each.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The service configuration currently associated with the resource group and in effect       for the members of the resource group. A Configuration consists of one or       more ConfigurationItem entries. For information about service       configurations for resource groups and how to construct them, see Service         configurations for resource groups in the AWS Resource Groups         User Guide.
    /// 
    /// NoteYou can include either a Configuration or a           ResourceQuery, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of ConfigurationItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: Option<Vec<ConfigurationItem>>,


    /// 
    /// The tag key and value pairs that are attached to the resource group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The description of the resource group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnGroup {
    fn type_string() -> &'static str {
        "AWS::ResourceGroups::Group"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// One of the items in the service configuration assigned to a resource group. A service       configuration can consist of one or more items. For details service configurations and       how to construct them, see Service configurations for resource         groups in the AWS Resource Groups User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationItem {


    /// 
    /// A collection of parameters for this configuration item. For the list of parameters       that you can use with each configuration item Type, see Supported resource types and parameters in the AWS Resource Groups User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfigurationParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ConfigurationParameter>>,


    /// 
    /// Specifies the type of configuration item. Each item must have a unique value for type.       For the list of the types that you can specify for a configuration item, see Supported resource types and parameters in the AWS Resource Groups User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The query used to dynamically define the members of a group. For more information       about how to construct a query, see Build queries and groups in           AWS Resource Groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceQuery {


    /// 
    /// Specifies the type of resource query that determines this group's membership. There       are two valid query types:
    /// 
    /// TAG_FILTERS_1_0 indicates that the group is a tag-based group.           To complete the group membership, you must include the TagFilters           property to specify the tag filters to use in the query.               CLOUDFORMATION_STACK_1_0, the default, indicates that the           group is a CloudFormation stack-based group. Group membership is based on the           CloudFormation stack. You must specify the StackIdentifier property           in the query to define which stack to associate the group with, or leave it           empty to default to the stack where the group is defined.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The query that defines the membership of the group. This is a structure with       properties that depend on the Type.
    /// 
    /// The Query structure must be included in the following scenarios:
    /// 
    /// When the Type is TAG_FILTERS_1_0, you must specify a             Query structure that contains a TagFilters list of           tags. Resources with tags that match those in the TagFilter list           become members of the resource group.               When the Type is CLOUDFORMATION_STACK_1_0 then this           field is required only when you must specify a CloudFormation stack other than           the one you are defining. To do this, the Query structure must           contain the StackIdentifier property. If you don't specify either a             Query structure or a StackIdentifier within that             Query, then it defaults to the CloudFormation stack that you're           currently constructing.
    /// 
    /// Required: No
    ///
    /// Type: Query
    ///
    /// Update requires: No interruption
    #[serde(rename = "Query")]
    pub query: Option<Query>,

}


/// Specifies a single tag key and optional values that you can use to specify membership       in a tag-based group. An AWS resource that doesn't have a matching tag       key and value is rejected as a member of the group.
///
/// A TagFilter object includes two properties: Key (a string)       and Values (a list of strings). Only resources in the account that are       tagged with a matching key-value pair are members of the group. The Values       property of TagFilter is optional, but specifying it narrows the query       results.
///
/// As an example, suppose the TagFilters string is [{"Key": "Stage",         "Values": ["Test", "Beta"]}, {"Key": "Storage"}]. In this case, only       resources with all of the following tags are members of the group:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFilter {


    /// 
    /// A string that defines a tag key. Only resources in the account that are tagged with a       specified tag key are members of the tag-based resource group.
    /// 
    /// This field is required when the ResourceQuery structure's         Type property is TAG_FILTERS_1_0. You must specify at       least one tag key.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// A list of tag values that can be included in the tag-based resource group. This is       optional. If you don't specify a value or values for a key, then an AWS       resource with any value for that key is a member.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}


/// One parameter for a group configuration item. For details about service configurations       and how to construct them, see Service configurations for resource         groups in the AWS Resource Groups User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationParameter {


    /// 
    /// The name of the group configuration parameter. For the list of parameters that you can       use with each configuration item type, see Supported resource         types and parameters in the AWS Resource Groups User         Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value or values to be used for the specified parameter. For the list of values you       can use with each parameter, see Supported resource         types and parameters.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}


/// Specifies details within a ResourceQuery structure that determines the       membership of the resource group. The contents required in the Query       structure are determined by the Type property of the containing         ResourceQuery structure.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Query {


    /// 
    /// Specifies limits to the types of resources that can be included in the resource group.       For example, if ResourceTypeFilters is ["AWS::EC2::Instance",         "AWS::DynamoDB::Table"], only EC2 instances or DynamoDB tables can be members       of this resource group. The default value is ["AWS::AllSupported"].
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypeFilters")]
    pub resource_type_filters: Option<Vec<String>>,


    /// 
    /// A list of key-value pair objects that limit which resources can be members of the       resource group. This property is required when the ResourceQuery.Type       property is TAG_FILTERS_1_0.
    /// 
    /// A resource must have a tag that matches every filter that is provided in the         TagFilters list.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,


    /// 
    /// Specifies the ARN of a CloudFormation stack. All supported resources of the       CloudFormation stack are members of the resource group. If you don't specify an ARN,       this parameter defaults to the current stack that you are defining, which means that all       the resources of the current stack are grouped.
    /// 
    /// You can specify a value for StackIdentifier only when the         ResourceQuery.Type property is       CLOUDFORMATION_STACK_1_0.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackIdentifier")]
    pub stack_identifier: Option<String>,

}
