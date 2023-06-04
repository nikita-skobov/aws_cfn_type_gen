/// The AWS::SNS::TopicPolicy resource associates Amazon SNS topics     with a policy. For an example snippet, see Declaring       an Amazon SNS policy in the AWS CloudFormation User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTopicPolicy {
    ///
    /// A policy document that contains permissions to add to the specified SNS topics.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

    ///
    /// The Amazon Resource Names (ARN) of the topics to which you want to add the policy. You     can use the       Ref      function to specify an       AWS::SNS::Topic      resource.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topics")]
    pub topics: Vec<String>,
}

impl cfn_resources::CfnResource for CfnTopicPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::SNS::TopicPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
