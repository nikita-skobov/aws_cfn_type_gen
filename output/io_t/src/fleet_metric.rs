/// Use the AWS::IoT::FleetMetric resource to declare a fleet metric.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFleetMetric {
    ///
    /// The field to aggregate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<cfn_resources::StrVal>,

    ///
    /// The type of the aggregation query.
    ///
    /// Required: No
    ///
    /// Type: AggregationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<AggregationType>,

    ///
    /// The fleet metric description.
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
    /// The name of the index to search.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the fleet metric to create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

    ///
    /// The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,

    ///
    /// The search query string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<cfn_resources::StrVal>,

    ///
    /// The query version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<cfn_resources::StrVal>,

    ///
    /// Metadata which can be used to manage the fleet metric.
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
    /// Used to support unit transformation such as milliseconds to seconds. Must be a unit supported by CW metric. Default to null.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_metric_arn: CfnFleetMetricmetricarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFleetMetricmetricarn;
impl CfnFleetMetricmetricarn {
    pub fn att_name(&self) -> &'static str {
        r#"MetricArn"#
    }
}

impl cfn_resources::CfnResource for CfnFleetMetric {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::FleetMetric"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.aggregation_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The type of aggregation queries.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AggregationType {
    ///
    /// The name of the aggregation type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// A list of the values of aggregation types.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

impl cfn_resources::CfnResource for AggregationType {
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
