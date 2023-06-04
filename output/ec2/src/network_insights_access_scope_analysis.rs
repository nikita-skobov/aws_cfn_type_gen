/// Describes a Network Access Scope analysis.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNetworkInsightsAccessScopeAnalysis {
    ///
    /// The ID of the Network Access Scope.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInsightsAccessScopeId")]
    pub network_insights_access_scope_id: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_end_date: CfnNetworkInsightsAccessScopeAnalysisenddate,

    #[serde(skip_serializing)]
    pub att_findings_found: CfnNetworkInsightsAccessScopeAnalysisfindingsfound,

    #[serde(skip_serializing)]
    pub att_network_insights_access_scope_analysis_arn:
        CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisarn,

    #[serde(skip_serializing)]
    pub att_network_insights_access_scope_analysis_id:
        CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisid,

    #[serde(skip_serializing)]
    pub att_start_date: CfnNetworkInsightsAccessScopeAnalysisstartdate,

    #[serde(skip_serializing)]
    pub att_status: CfnNetworkInsightsAccessScopeAnalysisstatus,

    #[serde(skip_serializing)]
    pub att_status_message: CfnNetworkInsightsAccessScopeAnalysisstatusmessage,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisenddate;
impl CfnNetworkInsightsAccessScopeAnalysisenddate {
    pub fn att_name(&self) -> &'static str {
        r#"EndDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisfindingsfound;
impl CfnNetworkInsightsAccessScopeAnalysisfindingsfound {
    pub fn att_name(&self) -> &'static str {
        r#"FindingsFound"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisarn;
impl CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisarn {
    pub fn att_name(&self) -> &'static str {
        r#"NetworkInsightsAccessScopeAnalysisArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisid;
impl CfnNetworkInsightsAccessScopeAnalysisnetworkinsightsaccessscopeanalysisid {
    pub fn att_name(&self) -> &'static str {
        r#"NetworkInsightsAccessScopeAnalysisId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisstartdate;
impl CfnNetworkInsightsAccessScopeAnalysisstartdate {
    pub fn att_name(&self) -> &'static str {
        r#"StartDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisstatus;
impl CfnNetworkInsightsAccessScopeAnalysisstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAccessScopeAnalysisstatusmessage;
impl CfnNetworkInsightsAccessScopeAnalysisstatusmessage {
    pub fn att_name(&self) -> &'static str {
        r#"StatusMessage"#
    }
}

impl cfn_resources::CfnResource for CfnNetworkInsightsAccessScopeAnalysis {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkInsightsAccessScopeAnalysis"
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
