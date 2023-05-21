

/// The AWS::CertificateManager::Account resource defines the expiry event     configuration that determines the number of days prior to expiry when ACM starts generating     EventBridge events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccount {


    /// 
    /// Object containing expiration events options associated with an AWS account. For more information, see ExpiryEventsConfiguration in the API reference.
    /// 
    /// Required: Yes
    ///
    /// Type: ExpiryEventsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpiryEventsConfiguration")]
    pub expiry_events_configuration: ExpiryEventsConfiguration,

}

impl cfn_resources::CfnResource for CfnAccount {
    fn type_string() -> &'static str {
        "AWS::CertificateManager::Account"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Object containing expiration events options associated with an AWS account. For more information, see ExpiryEventsConfiguration in the API reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExpiryEventsConfiguration {


    /// 
    /// This option specifies the number of days prior to certificate expiration when ACM starts     generating EventBridge events. ACM sends one event per day per certificate     until the certificate expires. By default, accounts receive events starting 45 days before     certificate expiration.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DaysBeforeExpiry")]
    pub days_before_expiry: Option<i64>,

}
