

/// The AWS::Cognito::UserPool resource creates an Amazon Cognito user pool. For    more information on working with Amazon Cognito user pools, see Amazon Cognito User     Pools and CreateUserPool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPool {


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
    pub enabled_mfas: Option<Vec<String>>,


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
    pub email_verification_subject: Option<String>,


    /// 
    /// The configuration for creating a new user profile.
    /// 
    /// Required: No
    ///
    /// Type: AdminCreateUserConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminCreateUserConfig")]
    pub admin_create_user_config: Option<AdminCreateUserConfig>,


    /// 
    /// The policy associated with a user pool.
    /// 
    /// Required: No
    ///
    /// Type: Policies
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    pub policies: Option<Policies>,


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
    pub mfa_configuration: Option<String>,


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
    pub deletion_protection: Option<String>,


    /// 
    /// Use this setting to define which verified available method a user can use to recover their    password when they call ForgotPassword. It allows you to define a preferred    method when a user has more than one method available. With this setting, SMS does not qualify    for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence    of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS    is preferred over email.
    /// 
    /// Required: No
    ///
    /// Type: AccountRecoverySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountRecoverySetting")]
    pub account_recovery_setting: Option<AccountRecoverySetting>,


    /// 
    /// The settings for updates to user attributes. These settings include the property AttributesRequireVerificationBeforeUpdate, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see Verifying updates to email addresses and phone numbers.
    /// 
    /// Required: No
    ///
    /// Type: UserAttributeUpdateSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttributeUpdateSettings")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettings>,


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
    pub sms_authentication_message: Option<String>,


    /// 
    /// The email configuration of your user pool. The email configuration type sets your       preferred sending method, AWS Region, and sender for messages from your user       pool.
    /// 
    /// Required: No
    ///
    /// Type: EmailConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailConfiguration")]
    pub email_configuration: Option<EmailConfiguration>,


    /// 
    /// The attributes to be auto-verified. Possible values: email, phone_number.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoVerifiedAttributes")]
    pub auto_verified_attributes: Option<Vec<String>>,


    /// 
    /// The tag keys and values to assign to the user pool. A tag is a label that you can use       to categorize and manage user pools in different ways, such as by purpose, owner,       environment, or other criteria.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolTags")]
    pub user_pool_tags: Option<serde_json::Value>,


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
    pub device_configuration: Option<DeviceConfiguration>,


    /// 
    /// The template for the verification message that the user sees when the app requests       permission to access the user's information.
    /// 
    /// Required: No
    ///
    /// Type: VerificationMessageTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerificationMessageTemplate")]
    pub verification_message_template: Option<VerificationMessageTemplate>,


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
    pub sms_verification_message: Option<String>,


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
    pub schema: Option<Vec<SchemaAttribute>>,


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
    pub email_verification_message: Option<String>,


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
    pub user_pool_name: Option<String>,


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
    pub alias_attributes: Option<Vec<String>>,


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
    pub username_attributes: Option<Vec<String>>,


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
    pub lambda_config: Option<LambdaConfig>,


    /// 
    /// You can choose to set case sensitivity on the username input for the selected sign-in    option. For example, when this is set to False, users will be able to sign in    using either "username" or "Username". This configuration is immutable once it has been    set.
    /// 
    /// Required: No
    ///
    /// Type: UsernameConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsernameConfiguration")]
    pub username_configuration: Option<UsernameConfiguration>,


    /// 
    /// Enables advanced security risk detection. Set the key         AdvancedSecurityMode to the value "AUDIT".
    /// 
    /// Required: No
    ///
    /// Type: UserPoolAddOns
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolAddOns")]
    pub user_pool_add_ons: Option<UserPoolAddOns>,


    /// 
    /// The SMS configuration with the settings that your Amazon Cognito user pool must use to send an       SMS message from your AWS account through Amazon Simple Notification Service. To send SMS messages       with Amazon SNS in the AWS Region that you want, the Amazon Cognito user pool uses an AWS Identity and Access Management       (IAM) role in your AWS account.
    /// 
    /// Required: No
    ///
    /// Type: SmsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmsConfiguration")]
    pub sms_configuration: Option<SmsConfiguration>,

}

impl cfn_resources::CfnResource for CfnUserPool {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPool"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains information about the schema attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SchemaAttribute {


    /// 
    /// Specifies whether a user pool attribute is required. If the attribute is required and       the user doesn't provide a value, registration or sign-in will fail.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
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
    pub string_attribute_constraints: Option<StringAttributeConstraints>,


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
    pub mutable: Option<bool>,


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
    pub attribute_data_type: Option<String>,


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
    pub name: Option<String>,


    /// 
    /// Specifies the constraints for an attribute of the number type.
    /// 
    /// Required: No
    ///
    /// Type: NumberAttributeConstraints
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberAttributeConstraints")]
    pub number_attribute_constraints: Option<NumberAttributeConstraints>,

}


/// The device-remembering configuration for a user pool. A         DescribeUserPool request returns a null value for this object when the user       pool isn't configured to remember devices. When device remembering is active, you can       remember a user's device with a ConfirmDevice API request. Additionally. when the property         DeviceOnlyRememberedOnUserPrompt is true, you must follow         ConfirmDevice with an UpdateDeviceStatus API request that sets the user's device to         remembered or not_remembered.
///
/// To sign in with a remembered device, include DEVICE_KEY in the       authentication parameters in your user's         InitiateAuth request. If your app doesn't include a DEVICE_KEY       parameter, the response from Amazon Cognito includes newly-generated DEVICE_KEY and         DEVICE_GROUP_KEY values under NewDeviceMetadata. Store       these values to use in future device-authentication requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_only_remembered_on_user_prompt: Option<bool>,

}


/// The SMS configuration type that includes the settings the Cognito User Pool needs to call    for the Amazon SNS service to send an SMS message from your AWS account. The    Cognito User Pool makes the request to the Amazon SNS Service by using an IAM    role that you provide for your AWS account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SmsConfiguration {


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
    pub sns_caller_arn: Option<String>,


    /// 
    /// The external ID is a value. We recommend you use ExternalIdto add security to    your IAM role, which is used to call Amazon SNS to send SMS messages for your user pool. If    you provide an ExternalId, the Cognito User Pool uses it when attempting to    assume your IAM role. You can also set your roles trust policy to require the     ExternalID. If you use the Cognito Management Console to create a role for SMS    MFA, Cognito creates a role with the required permissions and a trust policy that uses     ExternalId.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,


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
    pub sns_region: Option<String>,

}


/// The configuration for AdminCreateUser requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdminCreateUserConfig {


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
    pub unused_account_validity_days: Option<i64>,


    /// 
    /// Set to True if only the administrator is allowed to create user profiles.       Set to False if users can sign themselves up via an app.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowAdminCreateUserOnly")]
    pub allow_admin_create_user_only: Option<bool>,

}


/// The UsernameConfiguration property type specifies case sensitivity on the    username input for the selected sign-in option.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub case_sensitive: Option<bool>,

}


/// A custom SMS sender AWS Lambda trigger.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomSMSSender {


    /// 
    /// The Lambda version represents the signature of the "request" attribute in the    "event" information Amazon Cognito passes to your custom SMS sender Lambda    function. The only supported value is V1_0.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaVersion")]
    pub lambda_version: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Lambda function that Amazon Cognito triggers to send SMS notifications to users.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: Option<String>,

}


/// The user pool add-ons type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub advanced_security_mode: Option<String>,

}


/// A map containing a priority as a key, and recovery method name as a value.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub name: Option<String>,


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
    pub priority: Option<i64>,

}


/// The StringAttributeConstraints property type defines the string attribute    constraints of an Amazon Cognito user pool. StringAttributeConstraints is a    subproperty of the SchemaAttribute property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub max_length: Option<String>,


    /// 
    /// The minimum length.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinLength")]
    pub min_length: Option<String>,

}


/// The settings for updates to user attributes. These settings include the property AttributesRequireVerificationBeforeUpdate, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see Verifying updates to email addresses and phone numbers.
#[derive(Clone, Debug, Default, serde::Serialize)]
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


/// Specifies the configuration for AWS Lambda triggers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaConfig {


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
    pub custom_message: Option<String>,


    /// 
    /// The Amazon Resource Name of a AWS Key Management Service (AWS KMS) key. Amazon Cognito uses the key to encrypt codes and temporary passwords sent to     CustomEmailSender and CustomSMSSender.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSKeyID")]
    pub kmskey_id: Option<String>,


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
    pub pre_sign_up: Option<String>,


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
    pub pre_authentication: Option<String>,


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
    pub define_auth_challenge: Option<String>,


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
    pub verify_auth_challenge_response: Option<String>,


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
    pub post_authentication: Option<String>,


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
    pub create_auth_challenge: Option<String>,


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
    pub post_confirmation: Option<String>,


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
    pub pre_token_generation: Option<String>,


    /// 
    /// A custom email sender AWS Lambda trigger.
    /// 
    /// Required: No
    ///
    /// Type: CustomEmailSender
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEmailSender")]
    pub custom_email_sender: Option<CustomEmailSender>,


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
    pub user_migration: Option<String>,


    /// 
    /// A custom SMS sender AWS Lambda trigger.
    /// 
    /// Required: No
    ///
    /// Type: CustomSMSSender
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomSMSSender")]
    pub custom_smssender: Option<CustomSMSSender>,

}


/// The policy associated with a user pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub password_policy: Option<PasswordPolicy>,

}


/// Use this setting to define which verified available method a user can use to recover their    password when they call ForgotPassword. It allows you to define a preferred    method when a user has more than one method available. With this setting, SMS does not qualify    for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence    of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS    is preferred over email.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub recovery_mechanisms: Option<Vec<RecoveryOption>>,

}


/// The password policy type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PasswordPolicy {


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
    pub temporary_password_validity_days: Option<i64>,


    /// 
    /// In the password policy that you have set, refers to whether you have required users to       use at least one symbol in their password.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireSymbols")]
    pub require_symbols: Option<bool>,


    /// 
    /// In the password policy that you have set, refers to whether you have required users to       use at least one lowercase letter in their password.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireLowercase")]
    pub require_lowercase: Option<bool>,


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
    pub minimum_length: Option<i64>,


    /// 
    /// In the password policy that you have set, refers to whether you have required users to       use at least one number in their password.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireNumbers")]
    pub require_numbers: Option<bool>,


    /// 
    /// In the password policy that you have set, refers to whether you have required users to       use at least one uppercase letter in their password.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireUppercase")]
    pub require_uppercase: Option<bool>,

}


/// A custom email sender AWS Lambda trigger.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub lambda_arn: Option<String>,


    /// 
    /// The Lambda version represents the signature of the "request" attribute in the    "event" information that Amazon Cognito passes to your custom email sender AWS Lambda function. The only supported value is V1_0.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaVersion")]
    pub lambda_version: Option<String>,

}


/// The minimum and maximum values of an attribute that is of the number data type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub max_value: Option<String>,


    /// 
    /// The minimum value of an attribute that is of the number data type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinValue")]
    pub min_value: Option<String>,

}


/// The template for verification messages.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VerificationMessageTemplate {


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
    pub email_subject_by_link: Option<String>,


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
    pub sms_message: Option<String>,


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
    pub email_message: Option<String>,


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
    pub default_email_option: Option<String>,


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
    pub email_subject: Option<String>,


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
    pub email_message_by_link: Option<String>,

}


/// The email configuration of your user pool. The email configuration type sets your       preferred sending method, AWS Region, and sender for messages from your user       pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EmailConfiguration {


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
    pub email_sending_account: Option<String>,


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
    pub source_arn: Option<String>,


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
    pub configuration_set: Option<String>,


    /// 
    /// Identifies either the sender's email address or the sender's name with their email    address. For example, testuser@example.com or Test User     <testuser@example.com>. This address appears before the body of the    email.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    pub from: Option<String>,


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
    pub reply_to_email_address: Option<String>,

}


/// The message template to be used for the welcome message to new users.
///
/// See also Customizing User Invitation Messages.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub email_message: Option<String>,


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
    pub smsmessage: Option<String>,


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
    pub email_subject: Option<String>,

}
