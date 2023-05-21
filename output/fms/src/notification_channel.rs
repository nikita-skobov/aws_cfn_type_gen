

/// Designates the IAM role and Amazon Simple Notification Service (SNS) topic to use to record SNS logs.
///
/// To perform this action outside of the console, you must configure the SNS topic to allow the    role AWSServiceRoleForFMS to publish SNS logs. For more information, see    Firewall Manager required permissions for API actions in the         AWS Firewall Manager Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnNotificationChannel {


    /// 
    /// The Amazon Resource Name (ARN) of the SNS topic that collects notifications from AWS Firewall Manager.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that allows Amazon SNS to record AWS Firewall Manager activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsRoleName")]
    pub sns_role_name: String,

}