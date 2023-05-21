

/// The AWS::Macie::Session resource represents the Amazon Macie       service and certain configuration settings for an Amazon Macie account in a       specific AWS Region. It enables Macie to become       operational for a specific account in a specific Region. An account can have only one       session in each Region.
///
/// You must create an AWS::Macie::Session resource for an account before you       can create other types of resources for the account. Use a DependsOn         attribute to ensure that an AWS::Macie::Session resource is       created before other Macie resources are created for an account. For       example, "DependsOn": "Session".
#[derive(Default, serde::Serialize)]
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
    pub finding_publishing_frequency: Option<String>,


    /// 
    /// The status of Amazon Macie for the account. Valid values are:         ENABLED, start or resume all Macie activities for the       account; and, PAUSED, suspend all Macie activities for the       account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}