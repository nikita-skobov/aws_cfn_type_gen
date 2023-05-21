

/// Specifies a configuration set event destination. An event destination is an AWS service that Amazon SES publishes email sending events to. When you       specify an event destination, you provide one, and only one, destination. You can send       event data to Amazon CloudWatch, Amazon Kinesis Data Firehose, or Amazon Simple       Notification Service (Amazon SNS).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConfigurationSetEventDestination {


    /// 
    /// The name of the configuration set that contains the event destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,


    /// 
    /// The event destination object.
    /// 
    /// Required: Yes
    ///
    /// Type: EventDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventDestination")]
    pub event_destination: EventDestination,

}



impl cfn_resources::CfnResource for CfnConfigurationSetEventDestination {
    fn type_string() -> &'static str {
        "AWS::SES::ConfigurationSetEventDestination"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains the delivery stream ARN and the IAM role ARN associated with an Amazon Kinesis Firehose event       destination.
///
/// Event destinations, such as Amazon Kinesis Firehose, are associated with configuration sets, which enable       you to publish email sending events. For information about using configuration sets, see       the Amazon SES         Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseDestination {


    /// 
    /// The ARN of the Amazon Kinesis Firehose stream that email sending events should be published to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,


    /// 
    /// The ARN of the IAM role under which Amazon SES publishes email sending events to the Amazon Kinesis Firehose       stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IAMRoleARN")]
    pub iamrole_arn: String,

}




/// Contains the topic ARN associated with an Amazon Simple Notification Service (Amazon SNS) event destination.
///
/// Event destinations, such as Amazon SNS, are associated with configuration sets, which       enable you to publish email sending events. For information about using configuration       sets, see the Amazon SES Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnsDestination {


    /// 
    /// The ARN of the Amazon SNS topic for email sending events. You can find the ARN of a topic       by using the ListTopics Amazon SNS operation.
    /// 
    /// For more information about Amazon SNS topics, see the Amazon SNS Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicARN")]
    pub topic_arn: String,

}




/// Contains information about an event destination.
///
/// Event destinations are associated with configuration sets, which enable you to publish       email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about       using configuration sets, see the Amazon SES Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventDestination {


    /// 
    /// An object that contains the topic ARN associated with an Amazon Simple Notification Service (Amazon SNS) event       destination.
    /// 
    /// Required: No
    ///
    /// Type: SnsDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsDestination")]
    pub sns_destination: Option<SnsDestination>,


    /// 
    /// Sets whether Amazon SES publishes events to this destination when you send an email with       the associated configuration set. Set to true to enable publishing to this       destination; set to false to prevent publishing to this destination. The       default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// An object that contains the delivery stream ARN and the IAM role ARN associated with       an Amazon Kinesis Firehose event destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisFirehoseDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseDestination")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,


    /// 
    /// The type of email sending events to publish to the event destination.
    /// 
    /// send - The send request was successful and SES will attempt to           deliver the message to the recipient’s mail server. (If account-level or global           suppression is being used, SES will still count it as a send, but delivery is           suppressed.)               reject - SES accepted the email, but determined that it contained           a virus and didn’t attempt to deliver it to the recipient’s mail server.               bounce - (Hard bounce) The recipient's mail           server permanently rejected the email. (Soft bounces are           only included when SES fails to deliver the email after retrying for a period of           time.)               complaint - The email was successfully delivered to the           recipient’s mail server, but the recipient marked it as spam.               delivery - SES successfully delivered the email to the           recipient's mail server.               open - The recipient received the message and opened it in their           email client.               click - The recipient clicked one or more links in the           email.               renderingFailure - The email wasn't sent because of a template           rendering issue. This event type can occur when template data is missing, or           when there is a mismatch between template parameters and data. (This event type           only occurs when you send email using the SendTemplatedEmail or SendBulkTemplatedEmail API operations.)                deliveryDelay - The email couldn't be delivered to the           recipient’s mail server because a temporary issue occurred. Delivery delays can           occur, for example, when the recipient's inbox is full, or when the receiving           email server experiences a transient issue.               subscription - The email was successfully delivered, but the           recipient updated their subscription preferences by clicking on an             unsubscribe link as part of your subscription             management.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchingEventTypes")]
    pub matching_event_types: Vec<String>,


    /// 
    /// The name of the event destination. The name must meet the following       requirements:
    /// 
    /// Contain only ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or           dashes (-).               Contain 64 characters or fewer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// An object that contains the names, default values, and sources of the dimensions       associated with an Amazon CloudWatch event destination.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchDestination")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,

}




/// Contains information associated with an Amazon CloudWatch event destination to which email       sending events are published.
///
/// Event destinations, such as Amazon CloudWatch, are associated with configuration sets, which       enable you to publish email sending events. For information about using configuration       sets, see the Amazon SES Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchDestination {


    /// 
    /// A list of dimensions upon which to categorize your emails when you publish email       sending events to Amazon CloudWatch.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionConfigurations")]
    pub dimension_configurations: Option<Vec<DimensionConfiguration>>,

}




/// Contains the dimension configuration to use when you publish email sending events to       Amazon CloudWatch.
///
/// For information about publishing email sending events to Amazon CloudWatch, see the Amazon SES         Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DimensionConfiguration {


    /// 
    /// The name of an Amazon CloudWatch dimension associated with an email sending metric. The name       must meet the following requirements:
    /// 
    /// Contain only ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), dashes           (-), or colons (:).               Contain 256 characters or fewer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,


    /// 
    /// The default value of the dimension that is published to Amazon CloudWatch if you do not provide       the value of the dimension when you send an email. The default value must meet the       following requirements:
    /// 
    /// Contain only ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), dashes           (-), at signs (@), or periods (.).               Contain 256 characters or fewer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultDimensionValue")]
    pub default_dimension_value: String,


    /// 
    /// The place where Amazon SES finds the value of a dimension to publish to Amazon CloudWatch. To use       the message tags that you specify using an X-SES-MESSAGE-TAGS header or a       parameter to the SendEmail/SendRawEmail API, specify         messageTag. To use your own email headers, specify         emailHeader. To put a custom tag on any link included in your email,       specify linkTag.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: emailHeader | linkTag | messageTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionValueSource")]
    pub dimension_value_source: DimensionConfigurationDimensionValueSourceEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DimensionConfigurationDimensionValueSourceEnum {

    /// emailHeader
    #[serde(rename = "emailHeader")]
    Emailheader,

    /// linkTag
    #[serde(rename = "linkTag")]
    Linktag,

    /// messageTag
    #[serde(rename = "messageTag")]
    Messagetag,

}

impl Default for DimensionConfigurationDimensionValueSourceEnum {
    fn default() -> Self {
        DimensionConfigurationDimensionValueSourceEnum::Emailheader
    }
}

