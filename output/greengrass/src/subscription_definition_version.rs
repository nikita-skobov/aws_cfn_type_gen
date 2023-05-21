/// The     AWS::Greengrass::SubscriptionDefinitionVersion resource represents a subscription definition version for AWS IoT Greengrass.     A subscription definition version contains a list of subscriptions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubscriptionDefinitionVersion {
    ///
    /// The ID of the subscription definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,

    ///
    /// The subscriptions in this version.
    ///
    /// Required: Yes
    ///
    /// Type: List of Subscription
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subscriptions")]
    pub subscriptions: Vec<Subscription>,
}

impl cfn_resources::CfnResource for CfnSubscriptionDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::SubscriptionDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Subscriptions define how MQTT messages can be exchanged between devices, functions, and connectors in the group, and   with AWS IoT or the local shadow service. A subscription defines a message source, message target, and a topic (or subject) that's used to route messages from the source to the target. A subscription defines the   message flow in one direction, from the source to the target. For two-way communication, you must set up two subscriptions, one for each direction.
///
/// In an AWS CloudFormation template, the Subscriptions 		 property of the AWS::Greengrass::SubscriptionDefinitionVersion resource contains a      list of Subscription property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Subscription {
    ///
    /// A descriptive or arbitrary ID for the subscription. This value must be unique within       the subscription definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,

    ///
    /// The originator of the message. The value can be a thing ARN, the ARN of a Lambda function alias (recommended) or version, a connector ARN, cloud (which represents the AWS IoT cloud), or GGShadowService.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: String,

    ///
    /// The MQTT topic used to route the message.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subject")]
    pub subject: String,

    ///
    /// The destination of the message. The value can be a thing ARN, the ARN of a Lambda function alias (recommended) or version, a connector ARN, cloud (which represents the AWS IoT cloud), or GGShadowService.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Target")]
    pub target: String,
}

impl cfn_resources::CfnResource for Subscription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
