

/// The AWS::SNS::Subscription resource subscribes an endpoint to an Amazon SNS topic. For a subscription to be created, the owner of the endpoint must     confirm the subscription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubscription {


    /// 
    /// The delivery policy JSON assigned to the subscription. Enables the subscriber to define     the message delivery retry strategy in the case of an HTTP/S endpoint subscribed to the     topic. For more information, see       GetSubscriptionAttributes      in the Amazon SNS API Reference and Message       delivery retries in the Amazon SNS Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryPolicy")]
    pub delivery_policy: Option<serde_json::Value>,


    /// 
    /// The subscription's endpoint. The endpoint value depends on the protocol that you     specify. For more information, see the Endpoint parameter of the       Subscribe      action in the Amazon SNS API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<String>,


    /// 
    /// The filter policy JSON assigned to the subscription. Enables the subscriber to filter     out unwanted messages. For more information, see       GetSubscriptionAttributes      in the Amazon SNS API Reference and Message       filtering in the Amazon SNS Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterPolicy")]
    pub filter_policy: Option<serde_json::Value>,


    /// 
    /// This attribute lets you choose the filtering scope by using one of the following string     value types:
    /// 
    /// MessageAttributes (default) - The filter is applied on the message        attributes.            MessageBody - The filter is applied on the message body.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterPolicyScope")]
    pub filter_policy_scope: Option<String>,


    /// 
    /// The subscription's protocol. For more information, see the Protocol     parameter of the       Subscribe      action in the Amazon SNS API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: String,


    /// 
    /// When set to true, enables raw message delivery. Raw messages don't contain     any JSON formatting and can be sent to Amazon SQS and HTTP/S endpoints. For more     information, see       GetSubscriptionAttributes      in the Amazon SNS API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RawMessageDelivery")]
    pub raw_message_delivery: Option<bool>,


    /// 
    /// When specified, sends undeliverable messages to the specified Amazon SQS     dead-letter queue. Messages that can't be delivered due to client errors (for example, when     the subscribed endpoint is unreachable) or server errors (for example, when the service     that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue     for further analysis or reprocessing.
    /// 
    /// For more information about the redrive policy and dead-letter queues, see Amazon       SQS dead-letter queues in the Amazon SQS Developer       Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedrivePolicy")]
    pub redrive_policy: Option<serde_json::Value>,


    /// 
    /// For cross-region subscriptions, the region in which the topic resides.
    /// 
    /// If no region is specified, AWS CloudFormation uses the region of the caller as the     default.
    /// 
    /// If you perform an update operation that only updates the Region property of     a AWS::SNS::Subscription resource, that operation will fail unless you are     either:
    /// 
    /// Updating the Region from NULL to the caller        region.            Updating the Region from the caller region to        NULL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: Option<String>,


    /// 
    /// This property applies only to Amazon Kinesis Data Firehose delivery stream subscriptions.     Specify the ARN of the IAM role that has the following:
    /// 
    /// Permission to write to the Amazon Kinesis Data Firehose delivery stream            Amazon SNS listed as a trusted entity
    /// 
    /// Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery     stream subscriptions. For more information, see Fanout to Amazon Kinesis Data Firehose       delivery streams in the Amazon SNS Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubscriptionRoleArn")]
    pub subscription_role_arn: Option<String>,


    /// 
    /// The ARN of the topic to subscribe to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,

}



impl cfn_resources::CfnResource for CfnSubscription {
    fn type_string() -> &'static str {
        "AWS::SNS::Subscription"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}