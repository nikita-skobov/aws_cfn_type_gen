

/// Adds a notification channel to DevOps Guru. A notification channel is used to notify you 			about important DevOps Guru events, such as when an insight is generated.
///
/// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
///
/// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
#[derive(Default, serde::Serialize)]
pub struct CfnNotificationChannel {


    /// 
    /// A NotificationChannelConfig object that contains information about 			configured notification channels.
    /// 
    /// Required: Yes
    ///
    /// Type: NotificationChannelConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Config")]
    pub config: NotificationChannelConfig,

}


/// Contains the Amazon Resource Name (ARN) of an Amazon Simple Notification Service topic.
///
/// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
///
/// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
#[derive(Default, serde::Serialize)]
pub struct SnsChannelConfig {


    /// 
    /// The Amazon Resource Name (ARN) of an Amazon Simple Notification Service topic.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 36
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^arn:aws[a-z0-9-]*:sns:[a-z0-9-]+:\d{12}:[^:]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicArn")]
    pub topic_arn: Option<String>,

}


/// Information about notification channels you have configured with DevOps Guru. 			The one    	supported notification channel is Amazon Simple Notification Service (Amazon SNS).
#[derive(Default, serde::Serialize)]
pub struct NotificationChannelConfig {


    /// 
    /// Information about a notification channel configured in DevOps Guru to send notifications 			when insights are created.
    /// 
    /// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
    /// 
    /// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
    /// 
    /// Required: No
    ///
    /// Type: SnsChannelConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Sns")]
    pub sns: Option<SnsChannelConfig>,


    /// 
    /// The filter configurations for the Amazon SNS notification topic you use with DevOps Guru. 			If you do not provide filter configurations, the default configurations are to receive notifications for all message types of High or Medium severity.
    /// 
    /// Required: No
    ///
    /// Type: NotificationFilterConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Filters")]
    pub filters: Option<NotificationFilterConfig>,

}


/// The filter configurations for the Amazon SNS notification topic you use with DevOps Guru. You can choose to specify which events or message types to receive notifications for. 			You can also choose to specify which severity levels to receive notifications for.
#[derive(Default, serde::Serialize)]
pub struct NotificationFilterConfig {


    /// 
    /// The events that you want to receive notifications for. For example, you can choose to receive notifications only when the severity level is upgraded or a new insight is created.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "MessageTypes")]
    pub message_types: Option<Vec<String>>,


    /// 
    /// The severity levels that you want to receive notifications for. For example, you can choose to receive notifications only for insights with HIGH and MEDIUM severity levels. 			For more information, see Understanding insight severities.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Severities")]
    pub severities: Option<Vec<String>>,

}
