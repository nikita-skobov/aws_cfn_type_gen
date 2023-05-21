

/// Creates or updates a sink in the current account, so that it can be used as a     monitoring account in CloudWatch cross-account observability. A sink is a resource that represents an      attachment point in a monitoring account, which source accounts can link to to be able to send observability data.
///
/// After you create a sink, you must create a sink policy that allows source accounts to attach to it.     For more information, see PutSinkPolicy.
///
/// An account can have one sink.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSink {


    /// 
    /// A name for the sink.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The IAM policy that grants permissions to source accounts to link to this sink. The policy can grant permission    in the following ways:
    /// 
    /// Include organization     IDs or organization paths to permit all accounts in an organizationInclude account IDs to permit the specified accounts
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: Option<serde_json::Value>,


    /// 
    /// An array of key-value pairs to apply to the sink.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}



impl cfn_resources::CfnResource for CfnSink {
    fn type_string() -> &'static str {
        "AWS::Oam::Sink"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
