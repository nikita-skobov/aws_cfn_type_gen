

/// The AWS::SQS::QueuePolicy type applies a policy to Amazon SQS queues.    For an example snippet, see Declaring an      Amazon SQS policy in the AWS CloudFormation User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnQueuePolicy {


    /// 
    /// A policy document that contains the permissions for the specified Amazon SQS    queues. For more information about Amazon SQS policies, see Using     custom policies with the Amazon SQS access policy language in the      Amazon SQS Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,


    /// 
    /// The URLs of the queues to which you want to add the policy. You can use the Ref function to specify an AWS::SQS::Queue resource.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Queues")]
    pub queues: Vec<String>,

}



impl cfn_resources::CfnResource for CfnQueuePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::SQS::QueuePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}