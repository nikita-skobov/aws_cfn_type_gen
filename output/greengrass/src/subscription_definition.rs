/// The     AWS::Greengrass::SubscriptionDefinition resource represents a subscription definition for AWS IoT Greengrass.   Subscription definitions are used to organize your subscription definition versions.
///
/// Subscription definitions can reference multiple subscription definition versions. All subscription definition versions      must be associated with a subscription definition. Each subscription definition version can contain one or more subscriptions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubscriptionDefinition {
    ///
    /// The subscription definition version to include when the subscription definition is created.          A subscription definition version contains a list of          subscription property types.
    ///
    /// NoteTo associate a subscription definition version after the subscription definition is created, 				   create an AWS::Greengrass::SubscriptionDefinitionVersion 				   resource and specify the ID of this subscription definition.
    ///
    /// Required: No
    ///
    /// Type: SubscriptionDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,

    ///
    /// The name of the subscription definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Application-specific metadata to attach to the subscription definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    ///
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for CfnSubscriptionDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::SubscriptionDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.initial_version
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Subscriptions define how MQTT messages can be exchanged between devices, functions, and connectors in the group, and with AWS IoT   or the local shadow service. A subscription defines a message source, message target, and a topic (or subject) that's used to route messages from the source to the target. A subscription defines the message flow in   one direction, from the source to the target. For two-way communication, you must set up two subscriptions, one for each direction.
///
/// In an AWS CloudFormation template, the Subscriptions 		 property of the SubscriptionDefinitionVersion property type contains a list       of Subscription property types.
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

/// A subscription definition version contains a list of subscriptions.
///
/// In an AWS CloudFormation template, SubscriptionDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::SubscriptionDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SubscriptionDefinitionVersion {
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

impl cfn_resources::CfnResource for SubscriptionDefinitionVersion {
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
