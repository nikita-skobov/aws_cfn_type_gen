

/// Specifies a connection notification for a VPC endpoint or VPC endpoint service. A     connection notification notifies you of specific endpoint events. You must create an SNS     topic to receive notifications. For more information, see Create a Topic in the Amazon       Simple Notification Service Developer Guide.
///
/// You can create a connection notification for interface endpoints only.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPCEndpointConnectionNotification {


    /// 
    /// The endpoint events for which to receive notifications. Valid values are         Accept, Connect, Delete, and         Reject.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionEvents")]
    pub connection_events: Vec<String>,


    /// 
    /// The ARN of the SNS topic for the notifications.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionNotificationArn")]
    pub connection_notification_arn: String,


    /// 
    /// The ID of the endpoint service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceId")]
    pub service_id: Option<String>,


    /// 
    /// The ID of the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VPCEndpointId")]
    pub vpcendpoint_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnVPCEndpointConnectionNotification {
    fn type_string() -> &'static str {
        "AWS::EC2::VPCEndpointConnectionNotification"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
