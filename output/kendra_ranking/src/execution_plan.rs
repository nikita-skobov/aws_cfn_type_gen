

/// Creates a rescore execution plan. A rescore execution       plan is an Amazon Kendra Intelligent Ranking resource       used for provisioning the Rescore API. You set       the number of capacity units that you require for       Amazon Kendra Intelligent Ranking to rescore or re-rank       a search service's results.
///
/// For an example of using the       CreateRescoreExecutionPlan API, including using       the Python and Java SDKs, see Semantically         ranking a search service's results.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnExecutionPlan {


    /// 
    /// A name for the rescore execution plan.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A description for the rescore execution plan.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// You can set additional capacity units to meet the       needs of your rescore execution plan. You are given a single       capacity unit by default. If you want to use the default       capacity, you don't set additional capacity units. For more       information on the default capacity and additional capacity       units, see Adjusting         capacity.
    /// 
    /// Required: No
    ///
    /// Type: CapacityUnitsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,


    /// 
    /// A list of key-value pairs that identify or categorize your       rescore execution plan. You can also use tags to help control       access to the rescore execution plan. Tag keys and values can       consist of Unicode letters, digits, white space. They can also       consist of underscore, period, colon, equal, plus, and asperand.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnExecutionPlan {
    fn type_string() -> &'static str {
        "AWS::KendraRanking::ExecutionPlan"
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




/// Sets additional capacity units configured for your       rescore execution plan. A rescore execution plan is an       Amazon Kendra Intelligent Ranking resource used for       provisioning the Rescore API. You can add and       remove capacity units to fit your usage requirements.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityUnitsConfiguration {


    /// 
    /// The amount of extra capacity for your rescore execution       plan.
    /// 
    /// A single extra capacity unit for a rescore execution       plan provides 0.01 rescore requests per second. You can add       up to 1000 extra capacity units.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "RescoreCapacityUnits")]
    pub rescore_capacity_units: i64,

}


