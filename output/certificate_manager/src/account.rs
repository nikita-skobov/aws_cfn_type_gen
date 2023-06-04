/// The AWS::CertificateManager::Account resource defines the expiry event     configuration that determines the number of days prior to expiry when ACM starts generating     EventBridge events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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

    #[serde(skip_serializing)]
    pub att_account_id: CfnAccountaccountid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccountaccountid;
impl CfnAccountaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"AccountId"#
    }
}

impl cfn_resources::CfnResource for CfnAccount {
    fn type_string(&self) -> &'static str {
        "AWS::CertificateManager::Account"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.expiry_events_configuration.validate()?;

        Ok(())
    }
}

/// Object containing expiration events options associated with an AWS account. For more information, see ExpiryEventsConfiguration in the API reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_before_expiry: Option<i64>,
}

impl cfn_resources::CfnResource for ExpiryEventsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
