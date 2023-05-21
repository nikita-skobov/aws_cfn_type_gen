

/// The details about the configuration aggregator, including 			information about source accounts, regions, and metadata of the 			aggregator.
#[derive(Default, serde::Serialize)]
pub struct CfnConfigurationAggregator {


    /// 
    /// Provides a list of source accounts and regions to be 			aggregated.
    /// 
    /// Required: No
    ///
    /// Type: List of AccountAggregationSource
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountAggregationSources")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,


    /// 
    /// The name of the aggregator.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: Option<String>,


    /// 
    /// Provides an organization and list of regions to be 			aggregated.
    /// 
    /// Required: No
    ///
    /// Type: OrganizationAggregationSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationAggregationSource")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,


    /// 
    /// An array of tag object.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// A collection of accounts and regions.
#[derive(Default, serde::Serialize)]
pub struct AccountAggregationSource {


    /// 
    /// The 12-digit account ID of the account being aggregated.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,


    /// 
    /// If true, aggregate existing AWS Config regions and future 			regions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllAwsRegions")]
    pub all_aws_regions: Option<bool>,


    /// 
    /// The source regions being aggregated.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegions")]
    pub aws_regions: Option<Vec<String>>,

}


/// This object contains regions to set up the aggregator and an IAM 			role to retrieve organization details.
#[derive(Default, serde::Serialize)]
pub struct OrganizationAggregationSource {


    /// 
    /// The source regions being aggregated.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegions")]
    pub aws_regions: Option<Vec<String>>,


    /// 
    /// ARN of the IAM role used to retrieve AWS Organizations details 			associated with the aggregator account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// If true, aggregate existing AWS Config regions and future 			regions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllAwsRegions")]
    pub all_aws_regions: Option<bool>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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