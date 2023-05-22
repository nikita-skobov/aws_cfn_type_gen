/// Creates a dashboard in an AWS IoT SiteWise Monitor project.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDashboard {
    ///
    /// The dashboard definition specified in a JSON literal. For detailed information, see       Creating dashboards (CLI) in the AWS IoT SiteWise User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardDefinition")]
    pub dashboard_definition: cfn_resources::StrVal,

    ///
    /// A description for the dashboard.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardDescription")]
    pub dashboard_description: cfn_resources::StrVal,

    ///
    /// A friendly name for the dashboard.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardName")]
    pub dashboard_name: cfn_resources::StrVal,

    ///
    /// The ID of the project in which to create the dashboard.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of key-value pairs that contain metadata for the dashboard. For more information,       see Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_dashboard_arn: CfnDashboarddashboardarn,

    #[serde(skip_serializing)]
    pub att_dashboard_id: CfnDashboarddashboardid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDashboarddashboardarn;
impl CfnDashboarddashboardarn {
    pub fn att_name(&self) -> &'static str {
        r#"DashboardArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDashboarddashboardid;
impl CfnDashboarddashboardid {
    pub fn att_name(&self) -> &'static str {
        r#"DashboardId"#
    }
}

impl cfn_resources::CfnResource for CfnDashboard {
    fn type_string(&self) -> &'static str {
        "AWS::IoTSiteWise::Dashboard"
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
