/// The AWS::Cognito::UserPool resource creates an Amazon Cognito user pool. For    more information on working with Amazon Cognito user pools, see Amazon Cognito User     Pools and CreateUserPool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserPool {
    ///
    /// Use this setting to define which verified available method a user can use to recover their    password when they call ForgotPassword. It allows you to define a preferred    method when a user has more than one method available. With this setting, SMS does not qualify    for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence    of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS    is preferred over email.
    ///
    /// Required: No
    ///
    /// Type: AccountRecoverySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub account_recovery_setting: Option<AccountRecoverySetting>,

    ///
    /// The configuration for creating a new user profile.
    ///
    /// Required: No
    ///
    /// Type: AdminCreateUserConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub admin_create_user_config: Option<AdminCreateUserConfig>,

    ///
    /// Attributes supported as an alias for this user pool. Possible values: phone_number, email, or preferred_username.
    ///
    /// NoteThis user pool property cannot be updated.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AliasAttributes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub alias_attributes: Option<Vec<String>>,

    ///
    /// The attributes to be auto-verified. Possible values: email, phone_number.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub auto_verified_attributes: Option<Vec<String>>,

    ///
    /// When active, DeletionProtection prevents accidental deletion of your user     pool. Before you can delete a user pool that you have protected against deletion, you     must deactivate this feature.
    ///
    /// When you try to delete a protected user pool in a DeleteUserPool API request,     Amazon Cognito returns an InvalidParameterException error. To delete a protected user pool,     send a new DeleteUserPool request after you deactivate deletion protection in an     UpdateUserPool API request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTIVE | INACTIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub deletion_protection: Option<UserPoolDeletionProtectionEnum>,

    ///
    /// The device-remembering configuration for a user pool. A null value indicates that you       have deactivated device remembering in your user pool.
    ///
    /// NoteWhen you provide a value for any DeviceConfiguration field, you         activate the Amazon Cognito device-remembering feature.
    ///
    /// Required: No
    ///
    /// Type: DeviceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_configuration: Option<DeviceConfiguration>,

    ///
    /// The email configuration of your user pool. The email configuration type sets your       preferred sending method, AWS Region, and sender for messages from your user       pool.
    ///
    /// Required: No
    ///
    /// Type: EmailConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_configuration: Option<EmailConfiguration>,

    ///
    /// This parameter is no longer used. See VerificationMessageTemplateType.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_verification_message: Option<cfn_resources::StrVal>,

    ///
    /// This parameter is no longer used. See VerificationMessageTemplateType.
    ///
    /// Required: No
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
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_verification_subject: Option<cfn_resources::StrVal>,

    ///
    /// Enables MFA on a specified user pool. To disable all MFAs after it has been enabled, set    MfaConfiguration to “OFF” and remove EnabledMfas. MFAs can only be all disabled if    MfaConfiguration is OFF. Once SMS_MFA is enabled, SMS_MFA can only be disabled by setting    MfaConfiguration to “OFF”. Can be one of the following values:
    ///
    /// SMS_MFA - Enables SMS MFA for the user pool. SMS_MFA can only be enabled      if SMS configuration is provided.        SOFTWARE_TOKEN_MFA - Enables software token MFA for the user pool.
    ///
    /// Allowed values: SMS_MFA | SOFTWARE_TOKEN_MFA
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnabledMfas")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enabled_mfas: Option<Vec<String>>,

    ///
    /// The Lambda trigger configuration information for the new user pool.
    ///
    /// NoteIn a push model, event sources (such as Amazon S3 and custom applications) need         permission to invoke a function. So you must make an extra call to add permission         for these event sources to invoke your Lambda function.For more information on using the Lambda API to add permission, see           AddPermission . For adding permission using the AWS CLI, see add-permission         .
    ///
    /// Required: No
    ///
    /// Type: LambdaConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_config: Option<LambdaConfig>,

    ///
    /// The multi-factor authentication (MFA) configuration. Valid values include:
    ///
    /// OFF MFA won't be used for any users.                        ON MFA is required for all users to sign in.                        OPTIONAL MFA will be required only for individual users who have           an MFA factor activated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: OFF | ON | OPTIONAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mfa_configuration: Option<UserPoolMfaConfigurationEnum>,

    ///
    /// The policy associated with a user pool.
    ///
    /// Required: No
    ///
    /// Type: Policies
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub policies: Option<Policies>,

    ///
    /// The schema attributes for the new user pool. These attributes can be standard or custom    attributes.
    ///
    /// Note During a user pool update, you can add new schema attributes but you cannot modify or     delete an existing schema attribute.
    ///
    /// Required: No
    ///
    /// Type: List of SchemaAttribute
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub schema: Option<Vec<SchemaAttribute>>,

    ///
    /// A string representing the SMS authentication message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 140
    ///
    /// Pattern: .*\{####\}.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sms_authentication_message: Option<cfn_resources::StrVal>,

    ///
    /// The SMS configuration with the settings that your Amazon Cognito user pool must use to send an       SMS message from your AWS account through Amazon Simple Notification Service. To send SMS messages       with Amazon SNS in the AWS Region that you want, the Amazon Cognito user pool uses an AWS Identity and Access Management       (IAM) role in your AWS account.
    ///
    /// Required: No
    ///
    /// Type: SmsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmsConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sms_configuration: Option<SmsConfiguration>,

    ///
    /// This parameter is no longer used. See VerificationMessageTemplateType.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 140
    ///
    /// Pattern: .*\{####\}.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sms_verification_message: Option<cfn_resources::StrVal>,

    ///
    /// The settings for updates to user attributes. These settings include the property AttributesRequireVerificationBeforeUpdate, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see Verifying updates to email addresses and phone numbers.
    ///
    /// Required: No
    ///
    /// Type: UserAttributeUpdateSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttributeUpdateSettings")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettings>,

    ///
    /// Enables advanced security risk detection. Set the key         AdvancedSecurityMode to the value "AUDIT".
    ///
    /// Required: No
    ///
    /// Type: UserPoolAddOns
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolAddOns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_pool_add_ons: Option<UserPoolAddOns>,

    ///
    /// A string used to name the user pool.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w\s+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_pool_name: Option<cfn_resources::StrVal>,

    ///
    /// The tag keys and values to assign to the user pool. A tag is a label that you can use       to categorize and manage user pools in different ways, such as by purpose, owner,       environment, or other criteria.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolTags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_pool_tags: Option<serde_json::Value>,

    ///
    /// Determines whether email addresses or phone numbers can be specified as user names when a    user signs up. Possible values: phone_number or email.
    ///
    /// This user pool property cannot be updated.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsernameAttributes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub username_attributes: Option<Vec<String>>,

    ///
    /// You can choose to set case sensitivity on the username input for the selected sign-in    option. For example, when this is set to False, users will be able to sign in    using either "username" or "Username". This configuration is immutable once it has been    set.
    ///
    /// Required: No
    ///
    /// Type: UsernameConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsernameConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub username_configuration: Option<UsernameConfiguration>,

    ///
    /// The template for the verification message that the user sees when the app requests       permission to access the user's information.
    ///
    /// Required: No
    ///
    /// Type: VerificationMessageTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub verification_message_template: Option<VerificationMessageTemplate>,

    #[serde(skip_serializing)]
    pub att_arn: CfnUserPoolarn,

    #[serde(skip_serializing)]
    pub att_provider_name: CfnUserPoolprovidername,

    #[serde(skip_serializing)]
    pub att_provider_url: CfnUserPoolproviderurl,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserPoolDeletionProtectionEnum {
    /// ACTIVE
    #[serde(rename = "ACTIVE")]
    Active,

    /// INACTIVE
    #[serde(rename = "INACTIVE")]
    Inactive,
}

impl Default for UserPoolDeletionProtectionEnum {
    fn default() -> Self {
        UserPoolDeletionProtectionEnum::Active
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserPoolMfaConfigurationEnum {
    /// OFF
    #[serde(rename = "OFF")]
    Off,

    /// ON
    #[serde(rename = "ON")]
    On,

    /// OPTIONAL
    #[serde(rename = "OPTIONAL")]
    Optional,
}

impl Default for UserPoolMfaConfigurationEnum {
    fn default() -> Self {
        UserPoolMfaConfigurationEnum::Off
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserPoolarn;
impl CfnUserPoolarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserPoolprovidername;
impl CfnUserPoolprovidername {
    pub fn att_name(&self) -> &'static str {
        r#"ProviderName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserPoolproviderurl;
impl CfnUserPoolproviderurl {
    pub fn att_name(&self) -> &'static str {
        r#"ProviderURL"#
    }
}

impl cfn_resources::CfnResource for CfnUserPool {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPool"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.account_recovery_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.admin_create_user_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.device_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.email_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.email_verification_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!("Max validation failed on field 'email_verification_message'. {} is greater than 20000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.email_verification_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!("Min validation failed on field 'email_verification_message'. {} is less than 6", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.email_verification_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!("Max validation failed on field 'email_verification_subject'. {} is greater than 140", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.email_verification_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'email_verification_subject'. {} is less than 1", s.len()));
                }
            }
        }

        self.lambda_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.policies
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.schema {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'schema'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sms_authentication_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!("Max validation failed on field 'sms_authentication_message'. {} is greater than 140", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.sms_authentication_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!("Min validation failed on field 'sms_authentication_message'. {} is less than 6", s.len()));
                }
            }
        }

        self.sms_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.sms_verification_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!("Max validation failed on field 'sms_verification_message'. {} is greater than 140", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.sms_verification_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!("Min validation failed on field 'sms_verification_message'. {} is less than 6", s.len()));
                }
            }
        }

        self.user_attribute_update_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.user_pool_add_ons
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.user_pool_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'user_pool_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_pool_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'user_pool_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.username_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.verification_message_template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use this setting to define which verified available method a user can use to recover their    password when they call ForgotPassword. It allows you to define a preferred    method when a user has more than one method available. With this setting, SMS does not qualify    for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence    of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS    is preferred over email.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccountRecoverySetting {
    ///
    /// The list of RecoveryOptionTypes.
    ///
    /// Required: No
    ///
    /// Type: List of RecoveryOption
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecoveryMechanisms")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub recovery_mechanisms: Option<Vec<RecoveryOption>>,
}

impl cfn_resources::CfnResource for AccountRecoverySetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.recovery_mechanisms {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'recovery_mechanisms'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The configuration for AdminCreateUser requests.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdminCreateUserConfig {
    ///
    /// Set to True if only the administrator is allowed to create user profiles.       Set to False if users can sign themselves up via an app.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowAdminCreateUserOnly")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub allow_admin_create_user_only: Option<bool>,

    ///
    /// The message template to be used for the welcome message to new users.
    ///
    /// See also Customizing User Invitation Messages.
    ///
    /// Required: No
    ///
    /// Type: InviteMessageTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "InviteMessageTemplate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub invite_message_template: Option<InviteMessageTemplate>,

    ///
    /// The user account expiration limit, in days, after which a new account that hasn't       signed in is no longer usable. To reset the account after that time limit, you must call         AdminCreateUser again, specifying "RESEND" for the         MessageAction parameter. The default value for this parameter is 7.
    ///
    /// NoteIf you set a value for TemporaryPasswordValidityDays in           PasswordPolicy, that value will be used, and           UnusedAccountValidityDays will be no longer be an available         parameter for that user pool.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 365
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnusedAccountValidityDays")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub unused_account_validity_days: Option<i64>,
}

impl cfn_resources::CfnResource for AdminCreateUserConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.invite_message_template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.unused_account_validity_days {
            if *the_val > 365 as _ {
                return Err(format!("Max validation failed on field 'unused_account_validity_days'. {} is greater than 365", the_val));
            }
        }

        if let Some(the_val) = &self.unused_account_validity_days {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'unused_account_validity_days'. {} is less than 0", the_val));
            }
        }

        Ok(())
    }
}

/// A custom email sender AWS Lambda trigger.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomEmailSender {
    ///
    /// The Amazon Resource Name (ARN) of the AWS Lambda function that Amazon Cognito triggers to send email notifications to users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Lambda version represents the signature of the "request" attribute in the    "event" information that Amazon Cognito passes to your custom email sender AWS Lambda function. The only supported value is V1_0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CustomEmailSender {
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

/// A custom SMS sender AWS Lambda trigger.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomSMSSender {
    ///
    /// The Amazon Resource Name (ARN) of the AWS Lambda function that Amazon Cognito triggers to send SMS notifications to users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Lambda version represents the signature of the "request" attribute in the    "event" information Amazon Cognito passes to your custom SMS sender Lambda    function. The only supported value is V1_0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CustomSMSSender {
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

/// The device-remembering configuration for a user pool. A         DescribeUserPool request returns a null value for this object when the user       pool isn't configured to remember devices. When device remembering is active, you can       remember a user's device with a ConfirmDevice API request. Additionally. when the property         DeviceOnlyRememberedOnUserPrompt is true, you must follow         ConfirmDevice with an UpdateDeviceStatus API request that sets the user's device to         remembered or not_remembered.
///
/// To sign in with a remembered device, include DEVICE_KEY in the       authentication parameters in your user's         InitiateAuth request. If your app doesn't include a DEVICE_KEY       parameter, the response from Amazon Cognito includes newly-generated DEVICE_KEY and         DEVICE_GROUP_KEY values under NewDeviceMetadata. Store       these values to use in future device-authentication requests.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeviceConfiguration {
    ///
    /// When true, a remembered device can sign in with device authentication instead of SMS       and time-based one-time password (TOTP) factors for multi-factor authentication       (MFA).
    ///
    /// NoteWhether or not ChallengeRequiredOnNewDevice is true, users who sign         in with devices that have not been confirmed or remembered must still provide a         second factor in a user pool that requires MFA.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChallengeRequiredOnNewDevice")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub challenge_required_on_new_device: Option<bool>,

    ///
    /// When true, Amazon Cognito doesn't automatically remember a user's device when your app sends a                 ConfirmDevice API request. In your app, create a prompt for your user to       choose whether they want to remember their device. Return the user's choice in an         UpdateDeviceStatus API request.
    ///
    /// When DeviceOnlyRememberedOnUserPrompt is false, Amazon       Cognito immediately remembers devices that you register in a ConfirmDevice       API request.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceOnlyRememberedOnUserPrompt")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_only_remembered_on_user_prompt: Option<bool>,
}

impl cfn_resources::CfnResource for DeviceConfiguration {
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

/// The email configuration of your user pool. The email configuration type sets your       preferred sending method, AWS Region, and sender for messages from your user       pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailConfiguration {
    ///
    /// The set of configuration rules that can be applied to emails sent using Amazon SES. A    configuration set is applied to an email by including a reference to the configuration set in    the headers of the email. Once applied, all of the rules in that configuration set are applied    to the email. Configuration sets can be used to apply the following types of rules to emails:
    ///
    /// Event publishing – Amazon SES can track the number of send, delivery, open, click,      bounce, and complaint events for each email sent. Use event publishing to send information      about these events to other AWS services such as SNS and      CloudWatch.        IP pool management – When leasing dedicated IP addresses with Amazon SES, you can      create groups of IP addresses, called dedicated IP pools. You can then associate the      dedicated IP pools with configuration sets.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9_-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub configuration_set: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether Amazon Cognito uses its built-in functionality to send your users email       messages, or uses your Amazon Simple Email Service email configuration. Specify one of the following       values:
    ///
    /// COGNITO_DEFAULT                  When Amazon Cognito emails your users, it uses its built-in email functionality.             When you use the default option, Amazon Cognito allows only a limited number of             emails each day for your user pool. For typical production environments, the             default email limit is less than the required delivery volume. To achieve a             higher delivery volume, specify DEVELOPER to use your Amazon SES email             configuration.          To look up the email delivery limit for the default option, see Limits in the Amazon Cognito Developer             Guide.          The default FROM address is no-reply@verificationemail.com.             To customize the FROM address, provide the Amazon Resource Name (ARN) of an             Amazon SES verified email address for the SourceArn             parameter.                       DEVELOPER                  When Amazon Cognito emails your users, it uses your Amazon SES configuration. Amazon Cognito             calls Amazon SES on your behalf to send email from your verified email address.             When you use this option, the email delivery limits are the same limits that             apply to your Amazon SES verified email address in your AWS account.          If you use this option, provide the ARN of an Amazon SES verified email address             for the SourceArn parameter.          Before Amazon Cognito can email your users, it requires additional permissions to             call Amazon SES on your behalf. When you update your user pool with this option,             Amazon Cognito creates a service-linked role, which is a type of             role in your AWS account. This role contains the permissions             that allow you to access Amazon SES and send email messages from your email             address. For more information about the service-linked role that Amazon Cognito             creates, see Using Service-Linked Roles for Amazon Cognito in the               Amazon Cognito Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COGNITO_DEFAULT | DEVELOPER
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailSendingAccount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_sending_account: Option<EmailConfigurationEmailSendingAccountEnum>,

    ///
    /// Identifies either the sender's email address or the sender's name with their email    address. For example, testuser@example.com or Test User     <testuser@example.com>. This address appears before the body of the    email.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub from: Option<cfn_resources::StrVal>,

    ///
    /// The destination to which the receiver of the email should reply.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+@[\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplyToEmailAddress")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub reply_to_email_address: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of a verified email address in Amazon SES. Amazon Cognito uses this email address in one of       the following ways, depending on the value that you specify for the         EmailSendingAccount parameter:
    ///
    /// If you specify COGNITO_DEFAULT, Amazon Cognito uses this address as the           custom FROM address when it emails your users using its built-in email           account.               If you specify DEVELOPER, Amazon Cognito emails your users with this           address by calling Amazon SES on your behalf.
    ///
    /// The Region value of the SourceArn parameter must indicate a supported       AWS Region of your user pool. Typically, the Region in the SourceArn and       the user pool Region are the same. For more information, see Amazon SES email configuration regions in the Amazon Cognito Developer         Guide.
    ///
    /// Required: No
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub source_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EmailConfigurationEmailSendingAccountEnum {
    /// COGNITO_DEFAULT
    #[serde(rename = "COGNITO_DEFAULT")]
    Cognitodefault,

    /// DEVELOPER
    #[serde(rename = "DEVELOPER")]
    Developer,
}

impl Default for EmailConfigurationEmailSendingAccountEnum {
    fn default() -> Self {
        EmailConfigurationEmailSendingAccountEnum::Cognitodefault
    }
}

impl cfn_resources::CfnResource for EmailConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.configuration_set {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'configuration_set'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.configuration_set {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'configuration_set'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.source_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'source_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.source_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'source_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The message template to be used for the welcome message to new users.
///
/// See also Customizing User Invitation Messages.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InviteMessageTemplate {
    ///
    /// The message template for email messages. EmailMessage is allowed only if EmailSendingAccount is DEVELOPER.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_message: Option<cfn_resources::StrVal>,

    ///
    /// The subject line for email messages. EmailSubject is allowed only if EmailSendingAccount is DEVELOPER.
    ///
    /// Required: No
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
    #[serde(rename = "EmailSubject")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_subject: Option<cfn_resources::StrVal>,

    ///
    /// The message template for SMS messages.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 140
    ///
    /// Pattern: .*\{####\}.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub smsmessage: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InviteMessageTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.email_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_message'. {} is greater than 20000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_message'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_subject'. {} is greater than 140",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_subject'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.smsmessage {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!(
                        "Max validation failed on field 'smsmessage'. {} is greater than 140",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.smsmessage {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'smsmessage'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies the configuration for AWS Lambda triggers.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaConfig {
    ///
    /// Creates an authentication challenge.
    ///
    /// Required: No
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
    #[serde(rename = "CreateAuthChallenge")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub create_auth_challenge: Option<cfn_resources::StrVal>,

    ///
    /// A custom email sender AWS Lambda trigger.
    ///
    /// Required: No
    ///
    /// Type: CustomEmailSender
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEmailSender")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_email_sender: Option<CustomEmailSender>,

    ///
    /// A custom Message AWS Lambda trigger.
    ///
    /// Required: No
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
    #[serde(rename = "CustomMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_message: Option<cfn_resources::StrVal>,

    ///
    /// A custom SMS sender AWS Lambda trigger.
    ///
    /// Required: No
    ///
    /// Type: CustomSMSSender
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomSMSSender")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_smssender: Option<CustomSMSSender>,

    ///
    /// Defines the authentication challenge.
    ///
    /// Required: No
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
    #[serde(rename = "DefineAuthChallenge")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub define_auth_challenge: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name of a AWS Key Management Service (AWS KMS) key. Amazon Cognito uses the key to encrypt codes and temporary passwords sent to     CustomEmailSender and CustomSMSSender.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSKeyID")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kmskey_id: Option<cfn_resources::StrVal>,

    ///
    /// A post-authentication AWS Lambda trigger.
    ///
    /// Required: No
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
    #[serde(rename = "PostAuthentication")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub post_authentication: Option<cfn_resources::StrVal>,

    ///
    /// A post-confirmation AWS Lambda trigger.
    ///
    /// Required: No
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
    #[serde(rename = "PostConfirmation")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub post_confirmation: Option<cfn_resources::StrVal>,

    ///
    /// A pre-authentication AWS Lambda trigger.
    ///
    /// Required: No
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
    #[serde(rename = "PreAuthentication")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub pre_authentication: Option<cfn_resources::StrVal>,

    ///
    /// A pre-registration AWS Lambda trigger.
    ///
    /// Required: No
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
    #[serde(rename = "PreSignUp")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub pre_sign_up: Option<cfn_resources::StrVal>,

    ///
    /// A Lambda trigger that is invoked before token generation.
    ///
    /// Required: No
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
    #[serde(rename = "PreTokenGeneration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub pre_token_generation: Option<cfn_resources::StrVal>,

    ///
    /// The user migration Lambda config type.
    ///
    /// Required: No
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
    #[serde(rename = "UserMigration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_migration: Option<cfn_resources::StrVal>,

    ///
    /// Verifies the authentication challenge response.
    ///
    /// Required: No
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
    #[serde(rename = "VerifyAuthChallengeResponse")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub verify_auth_challenge_response: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.create_auth_challenge {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'create_auth_challenge'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.create_auth_challenge {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!("Min validation failed on field 'create_auth_challenge'. {} is less than 20", s.len()));
                }
            }
        }

        self.custom_email_sender
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.custom_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'custom_message'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_message'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        self.custom_smssender
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.define_auth_challenge {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'define_auth_challenge'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.define_auth_challenge {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!("Min validation failed on field 'define_auth_challenge'. {} is less than 20", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.post_authentication {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'post_authentication'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.post_authentication {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'post_authentication'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.post_confirmation {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'post_confirmation'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.post_confirmation {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'post_confirmation'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.pre_authentication {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'pre_authentication'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.pre_authentication {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'pre_authentication'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.pre_sign_up {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'pre_sign_up'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.pre_sign_up {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'pre_sign_up'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.pre_token_generation {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'pre_token_generation'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.pre_token_generation {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'pre_token_generation'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_migration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'user_migration'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_migration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'user_migration'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.verify_auth_challenge_response {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'verify_auth_challenge_response'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.verify_auth_challenge_response {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!("Min validation failed on field 'verify_auth_challenge_response'. {} is less than 20", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// The minimum and maximum values of an attribute that is of the number data type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NumberAttributeConstraints {
    ///
    /// The maximum value of an attribute that is of the number data type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_value: Option<cfn_resources::StrVal>,

    ///
    /// The minimum value of an attribute that is of the number data type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min_value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NumberAttributeConstraints {
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

/// The password policy type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PasswordPolicy {
    ///
    /// The minimum length of the password in the policy that you have set. This value can't       be less than 6.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 6
    ///
    /// Maximum: 99
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumLength")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub minimum_length: Option<i64>,

    ///
    /// In the password policy that you have set, refers to whether you have required users to       use at least one lowercase letter in their password.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireLowercase")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub require_lowercase: Option<bool>,

    ///
    /// In the password policy that you have set, refers to whether you have required users to       use at least one number in their password.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireNumbers")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub require_numbers: Option<bool>,

    ///
    /// In the password policy that you have set, refers to whether you have required users to       use at least one symbol in their password.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireSymbols")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub require_symbols: Option<bool>,

    ///
    /// In the password policy that you have set, refers to whether you have required users to       use at least one uppercase letter in their password.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireUppercase")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub require_uppercase: Option<bool>,

    ///
    /// The number of days a temporary password is valid in the password policy. If the user       doesn't sign in during this time, an administrator must reset their password.
    ///
    /// NoteWhen you set TemporaryPasswordValidityDays for a user pool, you can         no longer set a value for the legacy UnusedAccountValidityDays         parameter in that user pool.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 365
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemporaryPasswordValidityDays")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub temporary_password_validity_days: Option<i64>,
}

impl cfn_resources::CfnResource for PasswordPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.minimum_length {
            if *the_val > 99 as _ {
                return Err(format!(
                    "Max validation failed on field 'minimum_length'. {} is greater than 99",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.minimum_length {
            if *the_val < 6 as _ {
                return Err(format!(
                    "Min validation failed on field 'minimum_length'. {} is less than 6",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.temporary_password_validity_days {
            if *the_val > 365 as _ {
                return Err(format!("Max validation failed on field 'temporary_password_validity_days'. {} is greater than 365", the_val));
            }
        }

        if let Some(the_val) = &self.temporary_password_validity_days {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'temporary_password_validity_days'. {} is less than 0", the_val));
            }
        }

        Ok(())
    }
}

/// The policy associated with a user pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Policies {
    ///
    /// The password policy.
    ///
    /// Required: No
    ///
    /// Type: PasswordPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordPolicy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub password_policy: Option<PasswordPolicy>,
}

impl cfn_resources::CfnResource for Policies {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.password_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A map containing a priority as a key, and recovery method name as a value.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RecoveryOption {
    ///
    /// Specifies the recovery method for a user.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: admin_only | verified_email | verified_phone_number
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<RecoveryOptionNameEnum>,

    ///
    /// A positive integer specifying priority of a method with 1 being the highest    priority.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub priority: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RecoveryOptionNameEnum {
    /// admin_only
    #[serde(rename = "admin_only")]
    Adminonly,

    /// verified_email
    #[serde(rename = "verified_email")]
    Verifiedemail,

    /// verified_phone_number
    #[serde(rename = "verified_phone_number")]
    Verifiedphonenumber,
}

impl Default for RecoveryOptionNameEnum {
    fn default() -> Self {
        RecoveryOptionNameEnum::Adminonly
    }
}

impl cfn_resources::CfnResource for RecoveryOption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.priority {
            if *the_val > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'priority'. {} is greater than 2",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.priority {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'priority'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Contains information about the schema attribute.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SchemaAttribute {
    ///
    /// The attribute data type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Boolean | DateTime | Number | String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeDataType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub attribute_data_type: Option<SchemaAttributeAttributeDataTypeEnum>,

    ///
    /// NoteWe recommend that you use WriteAttributes in the user pool client to control how attributes can be mutated     for new use cases instead of using DeveloperOnlyAttribute.
    ///
    /// Specifies whether the attribute type is developer only. This attribute can only be    modified by an administrator. Users will not be able to modify this attribute using their    access token.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeveloperOnlyAttribute")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub developer_only_attribute: Option<bool>,

    ///
    /// Specifies whether the value of the attribute can be changed.
    ///
    /// For any user pool attribute that is mapped to an IdP attribute, you must set this       parameter to true. Amazon Cognito updates mapped attributes when users sign in to       your application through an IdP. If an attribute is immutable, Amazon Cognito throws an error       when it attempts to update the attribute. For more information, see Specifying Identity Provider Attribute Mappings for Your User         Pool.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mutable")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mutable: Option<bool>,

    ///
    /// A schema attribute of the name type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the constraints for an attribute of the number type.
    ///
    /// Required: No
    ///
    /// Type: NumberAttributeConstraints
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberAttributeConstraints")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub number_attribute_constraints: Option<NumberAttributeConstraints>,

    ///
    /// Specifies whether a user pool attribute is required. If the attribute is required and       the user doesn't provide a value, registration or sign-in will fail.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub required: Option<bool>,

    ///
    /// Specifies the constraints for an attribute of the string type.
    ///
    /// Required: No
    ///
    /// Type: StringAttributeConstraints
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringAttributeConstraints")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub string_attribute_constraints: Option<StringAttributeConstraints>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SchemaAttributeAttributeDataTypeEnum {
    /// Boolean
    #[serde(rename = "Boolean")]
    Boolean,

    /// DateTime
    #[serde(rename = "DateTime")]
    Datetime,

    /// Number
    #[serde(rename = "Number")]
    Number,

    /// String
    #[serde(rename = "String")]
    String,
}

impl Default for SchemaAttributeAttributeDataTypeEnum {
    fn default() -> Self {
        SchemaAttributeAttributeDataTypeEnum::Boolean
    }
}

impl cfn_resources::CfnResource for SchemaAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.number_attribute_constraints
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.string_attribute_constraints
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The SMS configuration type that includes the settings the Cognito User Pool needs to call    for the Amazon SNS service to send an SMS message from your AWS account. The    Cognito User Pool makes the request to the Amazon SNS Service by using an IAM    role that you provide for your AWS account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmsConfiguration {
    ///
    /// The external ID is a value. We recommend you use ExternalIdto add security to    your IAM role, which is used to call Amazon SNS to send SMS messages for your user pool. If    you provide an ExternalId, the Cognito User Pool uses it when attempting to    assume your IAM role. You can also set your roles trust policy to require the     ExternalID. If you use the Cognito Management Console to create a role for SMS    MFA, Cognito creates a role with the required permissions and a trust policy that uses     ExternalId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub external_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SNS caller. This is the ARN of the IAM role       in your AWS account that Amazon Cognito will use to send SMS messages. SMS       messages are subject to a spending limit.
    ///
    /// Required: No
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
    #[serde(rename = "SnsCallerArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sns_caller_arn: Option<cfn_resources::StrVal>,

    ///
    /// The AWS Region to use with Amazon SNS integration. You can choose the same Region as your       user pool, or a supported Legacy Amazon SNS alternate       Region.
    ///
    /// Amazon Cognito resources in the Asia Pacific (Seoul) AWS Region must use your Amazon SNS       configuration in the Asia Pacific (Tokyo) Region. For more information, see SMS message settings for Amazon Cognito user pools.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsRegion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sns_region: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SmsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sns_caller_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'sns_caller_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sns_caller_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sns_caller_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sns_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!(
                        "Max validation failed on field 'sns_region'. {} is greater than 32",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sns_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 5 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sns_region'. {} is less than 5",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The StringAttributeConstraints property type defines the string attribute    constraints of an Amazon Cognito user pool. StringAttributeConstraints is a    subproperty of the SchemaAttribute property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StringAttributeConstraints {
    ///
    /// The maximum length.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_length: Option<cfn_resources::StrVal>,

    ///
    /// The minimum length.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min_length: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StringAttributeConstraints {
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

/// The settings for updates to user attributes. These settings include the property AttributesRequireVerificationBeforeUpdate, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see Verifying updates to email addresses and phone numbers.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserAttributeUpdateSettings {
    ///
    /// Requires that your user verifies their email address, phone number, or both before       Amazon Cognito updates the value of that attribute. When you update a user attribute that has       this option activated, Amazon Cognito sends a verification message to the new phone number or       email address. Amazon Cognito doesn’t change the value of the attribute until your user responds       to the verification message and confirms the new value.
    ///
    /// You can verify an updated email address or phone number with a VerifyUserAttribute API request. You can also call the AdminUpdateUserAttributes API and set email_verified or         phone_number_verified to true.
    ///
    /// When AttributesRequireVerificationBeforeUpdate is false, your user pool       doesn't require that your users verify attribute changes before Amazon Cognito updates them. In a       user pool where AttributesRequireVerificationBeforeUpdate is false, API       operations that change attribute values can immediately update a user’s         email or phone_number attribute.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributesRequireVerificationBeforeUpdate")]
    pub attributes_require_verification_before_update: Vec<String>,
}

impl cfn_resources::CfnResource for UserAttributeUpdateSettings {
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

/// The user pool add-ons type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserPoolAddOns {
    ///
    /// The advanced security mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUDIT | ENFORCED | OFF
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedSecurityMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub advanced_security_mode: Option<UserPoolAddOnsAdvancedSecurityModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserPoolAddOnsAdvancedSecurityModeEnum {
    /// AUDIT
    #[serde(rename = "AUDIT")]
    Audit,

    /// ENFORCED
    #[serde(rename = "ENFORCED")]
    Enforced,

    /// OFF
    #[serde(rename = "OFF")]
    Off,
}

impl Default for UserPoolAddOnsAdvancedSecurityModeEnum {
    fn default() -> Self {
        UserPoolAddOnsAdvancedSecurityModeEnum::Audit
    }
}

impl cfn_resources::CfnResource for UserPoolAddOns {
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

/// The UsernameConfiguration property type specifies case sensitivity on the    username input for the selected sign-in option.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsernameConfiguration {
    ///
    /// Specifies whether user name case sensitivity will be applied for all users in the user       pool through Amazon Cognito APIs. For most use cases, set case sensitivity to False       (case insensitive) as a best practice. When usernames and email addresses are case       insensitive, users can sign in as the same user when they enter a different       capitalization of their user name.
    ///
    /// Valid values include:
    ///
    /// True                  Enables case sensitivity for all username input. When this option is set             to True, users must sign in using the exact capitalization of             their given username, such as “UserName”. This is the default value.                       False                  Enables case insensitivity for all username input. For example, when this             option is set to False, users can sign in using               username, USERNAME, or UserName.             This option also enables both preferred_username and               email alias to be case insensitive, in addition to the               username attribute.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseSensitive")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub case_sensitive: Option<bool>,
}

impl cfn_resources::CfnResource for UsernameConfiguration {
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

/// The template for verification messages.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VerificationMessageTemplate {
    ///
    /// The default email option.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONFIRM_WITH_CODE | CONFIRM_WITH_LINK
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultEmailOption")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub default_email_option: Option<VerificationMessageTemplateDefaultEmailOptionEnum>,

    ///
    /// The template for email messages that Amazon Cognito sends to your users. You can set an         EmailMessage template only if the value of EmailSendingAccount is DEVELOPER. When your EmailSendingAccount is DEVELOPER, your user pool sends email       messages with your own Amazon SES configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_message: Option<cfn_resources::StrVal>,

    ///
    /// The email message template for sending a confirmation link to the user. You can set an         EmailMessageByLink template only if the value of EmailSendingAccount is DEVELOPER. When your EmailSendingAccount is DEVELOPER, your user pool sends email       messages with your own Amazon SES configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{##[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*##\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailMessageByLink")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_message_by_link: Option<cfn_resources::StrVal>,

    ///
    /// The subject line for the email message template. You can set an         EmailSubject template only if the value of EmailSendingAccount is DEVELOPER. When your EmailSendingAccount is DEVELOPER, your user pool sends email       messages with your own Amazon SES configuration.
    ///
    /// Required: No
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
    #[serde(rename = "EmailSubject")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_subject: Option<cfn_resources::StrVal>,

    ///
    /// The subject line for the email message template for sending a confirmation link to the       user. You can set an EmailSubjectByLink template only if the value of         EmailSendingAccount is DEVELOPER. When your EmailSendingAccount is DEVELOPER, your user pool sends email       messages with your own Amazon SES configuration.
    ///
    /// Required: No
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
    #[serde(rename = "EmailSubjectByLink")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub email_subject_by_link: Option<cfn_resources::StrVal>,

    ///
    /// The template for SMS messages that Amazon Cognito sends to your users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 140
    ///
    /// Pattern: .*\{####\}.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmsMessage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sms_message: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerificationMessageTemplateDefaultEmailOptionEnum {
    /// CONFIRM_WITH_CODE
    #[serde(rename = "CONFIRM_WITH_CODE")]
    Confirmwithcode,

    /// CONFIRM_WITH_LINK
    #[serde(rename = "CONFIRM_WITH_LINK")]
    Confirmwithlink,
}

impl Default for VerificationMessageTemplateDefaultEmailOptionEnum {
    fn default() -> Self {
        VerificationMessageTemplateDefaultEmailOptionEnum::Confirmwithcode
    }
}

impl cfn_resources::CfnResource for VerificationMessageTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.email_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_message'. {} is greater than 20000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_message'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_message_by_link {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 20000 as _ {
                    return Err(format!("Max validation failed on field 'email_message_by_link'. {} is greater than 20000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.email_message_by_link {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_message_by_link'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_subject'. {} is greater than 140",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_subject'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_subject_by_link {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!("Max validation failed on field 'email_subject_by_link'. {} is greater than 140", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.email_subject_by_link {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_subject_by_link'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sms_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 140 as _ {
                    return Err(format!(
                        "Max validation failed on field 'sms_message'. {} is greater than 140",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sms_message {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 6 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sms_message'. {} is less than 6",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
