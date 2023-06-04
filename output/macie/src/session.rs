/// The AWS::Macie::Session resource represents the Amazon Macie       service and certain configuration settings for an Amazon Macie account in a       specific AWS Region. It enables Macie to become       operational for a specific account in a specific Region. An account can have only one       session in each Region.
///
/// You must create an AWS::Macie::Session resource for an account before you       can create other types of resources for the account. Use a DependsOn         attribute to ensure that an AWS::Macie::Session resource is       created before other Macie resources are created for an account. For       example, "DependsOn": "Session".
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSession {
    ///
    /// Specifies how often Amazon Macie publishes updates to policy findings for       the account. This includes publishing updates to AWS Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events). Valid values are:
    ///
    /// FIFTEEN_MINUTES               ONE_HOUR               SIX_HOURS
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub finding_publishing_frequency: Option<cfn_resources::StrVal>,

    ///
    /// The status of Amazon Macie for the account. Valid values are:         ENABLED, start or resume all Macie activities for the       account; and, PAUSED, suspend all Macie activities for the       account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_aws_account_id: CfnSessionawsaccountid,

    #[serde(skip_serializing)]
    pub att_service_role: CfnSessionservicerole,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSessionawsaccountid;
impl CfnSessionawsaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"AwsAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSessionservicerole;
impl CfnSessionservicerole {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceRole"#
    }
}

impl cfn_resources::CfnResource for CfnSession {
    fn type_string(&self) -> &'static str {
        "AWS::Macie::Session"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
