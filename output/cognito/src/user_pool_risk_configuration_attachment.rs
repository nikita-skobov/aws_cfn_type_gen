/// The AWS::Cognito::UserPoolRiskConfigurationAttachment resource sets the risk    configuration that is used for Amazon Cognito advanced security features.
///
/// You can specify risk configuration for a single client (with a specific     clientId) or for all clients (by setting the clientId to     ALL). If you specify ALL, the default configuration is used for    every client that has had no risk configuration set previously. If you specify risk    configuration for a particular client, it no longer falls back to the ALL    configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserPoolRiskConfigurationAttachment {
    ///
    /// The account takeover risk configuration object, including the         NotifyConfiguration object and Actions to take if there is       an account takeover.
    ///
    /// Required: No
    ///
    /// Type: AccountTakeoverRiskConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,

    ///
    /// The app client ID. You can specify the risk configuration for a single client (with a    specific ClientId) or for all clients (by setting the ClientId to ALL).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientId")]
    pub client_id: cfn_resources::StrVal,

    ///
    /// The compromised credentials risk configuration object, including the         EventFilter and the EventAction.
    ///
    /// Required: No
    ///
    /// Type: CompromisedCredentialsRiskConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_risk_configuration:
        Option<CompromisedCredentialsRiskConfigurationType>,

    ///
    /// The configuration to override the risk decision.
    ///
    /// Required: No
    ///
    /// Type: RiskExceptionConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "RiskExceptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,

    ///
    /// The user pool ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnUserPoolRiskConfigurationAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolRiskConfigurationAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.account_takeover_risk_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.client_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_id'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'client_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.compromised_credentials_risk_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.risk_exception_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 55 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_pool_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Account takeover action type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccountTakeoverActionType {
    ///
    /// The action to take in response to the account takeover action. Valid values are as       follows:
    ///
    /// BLOCK Choosing this action will block the request.                        MFA_IF_CONFIGURED Present an MFA challenge if user has configured           it, else allow the request.                        MFA_REQUIRED Present an MFA challenge if user has configured it,           else block the request.                        NO_ACTION Allow the user to sign in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BLOCK | MFA_IF_CONFIGURED | MFA_REQUIRED | NO_ACTION
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventAction")]
    pub event_action: AccountTakeoverActionTypeEventActionEnum,

    ///
    /// Flag specifying whether to send a notification.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notify")]
    pub notify: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AccountTakeoverActionTypeEventActionEnum {
    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,

    /// MFA_IF_CONFIGURED
    #[serde(rename = "MFA_IF_CONFIGURED")]
    Mfaifconfigured,

    /// MFA_REQUIRED
    #[serde(rename = "MFA_REQUIRED")]
    Mfarequired,

    /// NO_ACTION
    #[serde(rename = "NO_ACTION")]
    Noaction,
}

impl Default for AccountTakeoverActionTypeEventActionEnum {
    fn default() -> Self {
        AccountTakeoverActionTypeEventActionEnum::Block
    }
}

impl cfn_resources::CfnResource for AccountTakeoverActionType {
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

/// Account takeover actions type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccountTakeoverActionsType {
    ///
    /// Action to take for a high risk.
    ///
    /// Required: No
    ///
    /// Type: AccountTakeoverActionType
    ///
    /// Update requires: No interruption
    #[serde(rename = "HighAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_action: Option<AccountTakeoverActionType>,

    ///
    /// Action to take for a low risk.
    ///
    /// Required: No
    ///
    /// Type: AccountTakeoverActionType
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_action: Option<AccountTakeoverActionType>,

    ///
    /// Action to take for a medium risk.
    ///
    /// Required: No
    ///
    /// Type: AccountTakeoverActionType
    ///
    /// Update requires: No interruption
    #[serde(rename = "MediumAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_action: Option<AccountTakeoverActionType>,
}

impl cfn_resources::CfnResource for AccountTakeoverActionsType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.high_action
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.low_action
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.medium_action
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration for mitigation actions and notification for different levels of risk       detected for a potential account takeover.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccountTakeoverRiskConfigurationType {
    ///
    /// Account takeover risk configuration actions.
    ///
    /// Required: Yes
    ///
    /// Type: AccountTakeoverActionsType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: AccountTakeoverActionsType,

    ///
    /// The notify configuration used to construct email notifications.
    ///
    /// Required: No
    ///
    /// Type: NotifyConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_configuration: Option<NotifyConfigurationType>,
}

impl cfn_resources::CfnResource for AccountTakeoverRiskConfigurationType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.actions.validate()?;

        self.notify_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The compromised credentials actions type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompromisedCredentialsActionsType {
    ///
    /// The event action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BLOCK | NO_ACTION
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventAction")]
    pub event_action: CompromisedCredentialsActionsTypeEventActionEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CompromisedCredentialsActionsTypeEventActionEnum {
    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,

    /// NO_ACTION
    #[serde(rename = "NO_ACTION")]
    Noaction,
}

impl Default for CompromisedCredentialsActionsTypeEventActionEnum {
    fn default() -> Self {
        CompromisedCredentialsActionsTypeEventActionEnum::Block
    }
}

impl cfn_resources::CfnResource for CompromisedCredentialsActionsType {
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

/// The compromised credentials risk configuration type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompromisedCredentialsRiskConfigurationType {
    ///
    /// The compromised credentials risk configuration actions.
    ///
    /// Required: Yes
    ///
    /// Type: CompromisedCredentialsActionsType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: CompromisedCredentialsActionsType,

    ///
    /// Perform the action for these events. The default is to perform all events if no event       filter is specified.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CompromisedCredentialsRiskConfigurationType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.actions.validate()?;

        Ok(())
    }
}

/// The notify configuration type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotifyConfigurationType {
    ///
    /// Email template used when a detected risk event is blocked.
    ///
    /// Required: No
    ///
    /// Type: NotifyEmailType
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_email: Option<NotifyEmailType>,

    ///
    /// The email address that is sending the email. The address must be either individually       verified with Amazon Simple Email Service, or from a domain that has been verified with Amazon SES.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<cfn_resources::StrVal>,

    ///
    /// The multi-factor authentication (MFA) email template used when MFA is challenged as       part of a detected risk.
    ///
    /// Required: No
    ///
    /// Type: NotifyEmailType
    ///
    /// Update requires: No interruption
    #[serde(rename = "MfaEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_email: Option<NotifyEmailType>,

    ///
    /// The email template used when a detected risk event is allowed.
    ///
    /// Required: No
    ///
    /// Type: NotifyEmailType
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoActionEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_action_email: Option<NotifyEmailType>,

    ///
    /// The destination to which the receiver of an email should reply to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the identity that is associated with the sending       authorization policy. This identity permits Amazon Cognito to send for the email address       specified in the From parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:[\w+=/,.@-]+:[\w+=/,.@-]+:([\w+=/,.@-]*)?:[0-9]+:[\w+=/,.@-]+(:[\w+=/,.@-]+)?(:[\w+=/,.@-]+)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceArn")]
    pub source_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NotifyConfigurationType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.block_email
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mfa_email
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.no_action_email
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.source_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'source_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.source_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'source_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The notify email type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotifyEmailType {
    ///
    /// The email HTML body.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "HtmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<cfn_resources::StrVal>,

    ///
    /// The email subject.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subject")]
    pub subject: cfn_resources::StrVal,

    ///
    /// The email text body.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NotifyEmailType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.html_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'html_body'. {} is greater than 20000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.html_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'html_body'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.subject;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 140 as _ {
                return Err(format!(
                    "Max validation failed on field 'subject'. {} is greater than 140",
                    s.len()
                ));
            }
        }

        let the_val = &self.subject;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'subject'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.text_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'text_body'. {} is greater than 20000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.text_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'text_body'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The type of the configuration to override the risk decision.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RiskExceptionConfigurationType {
    ///
    /// Overrides the risk decision to always block the pre-authentication requests. The IP       range is in CIDR notation, a compact representation of an IP address and its routing       prefix.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockedIPRangeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_iprange_list: Option<Vec<String>>,

    ///
    /// Risk detection isn't performed on the IP addresses in this range list. The IP range is       in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkippedIPRangeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_iprange_list: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for RiskExceptionConfigurationType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.blocked_iprange_list {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'blocked_iprange_list'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.skipped_iprange_list {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'skipped_iprange_list'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
