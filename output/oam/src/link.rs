

/// Creates a link between a source account and a sink that you have created in a monitoring account.
///
/// Before you create a link, you must create a sink in the monitoring account. The sink must have a sink policy     that permits the source account to link to it. You can grant permission to source accounts by granting permission   to an entire organization, an organizational unit, or to individual accounts.
///
/// For more information, see     CreateSink and    PutSinkPolicy.
///
/// Each monitoring account can be linked to as many as 100,000 source accounts.
///
/// Each source account can be linked to as many as five monitoring accounts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLink {


    /// 
    /// The ARN of the sink in the monitoring account that you want to link to.    You can use ListSinks to    find the ARNs of sinks.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SinkIdentifier")]
    pub sink_identifier: String,


    /// 
    /// Specify a friendly human-readable name to use to identify this source account when you are viewing data from it in the monitoring    account.
    /// 
    /// You can include the following variables in your template:
    /// 
    /// $AccountName is the name of the account$AccountEmail is a globally-unique email address, which includes the     email domain, such as mariagarcia@example.com$AccountEmailNoDomain is an email address without the domain name,     such as mariagarcia
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelTemplate")]
    pub label_template: Option<String>,


    /// 
    /// An array of strings that define which types of data that the source account shares with the monitoring   account. Valid values are AWS::CloudWatch::Metric | AWS::Logs::LogGroup | AWS::XRay::Trace.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Vec<String>,


    /// 
    /// An array of key-value pairs to apply to the link.
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



impl cfn_resources::CfnResource for CfnLink {
    fn type_string() -> &'static str {
        "AWS::Oam::Link"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
