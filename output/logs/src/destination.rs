

/// The AWS::Logs::Destination resource specifies a CloudWatch Logs destination. A destination encapsulates a physical resource (such    as an Amazon Kinesis data stream) and enables you to subscribe that resource to a stream of log events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDestination {


    /// 
    /// The name of the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationName")]
    pub destination_name: String,


    /// 
    /// An IAM policy document that governs which AWS accounts can create subscription filters    against this destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPolicy")]
    pub destination_policy: Option<String>,


    /// 
    /// The ARN of an IAM role that permits CloudWatch Logs to send data to the specified AWS resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the physical target where the log events are    delivered (for example, a Kinesis stream).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    pub target_arn: String,

}



impl cfn_resources::CfnResource for CfnDestination {
    fn type_string() -> &'static str {
        "AWS::Logs::Destination"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
