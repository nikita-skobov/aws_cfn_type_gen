

/// The AWS::Cognito::UserPoolUser resource creates an Amazon Cognito user pool    user.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolUser {


    /// 
    /// A map of custom key-value pairs that you can provide as input for the custom workflow that    is invoked by the pre sign-up trigger.
    /// 
    /// You create custom workflows by assigning AWS Lambda functions to user pool    triggers. When you create a UserPoolUser resource and include the     ClientMetadata property, Amazon Cognito invokes the function that is assigned    to the pre sign-up trigger. When Amazon Cognito invokes this function, it    passes a JSON payload, which the function receives as input. This payload contains a     clientMetadata attribute, which provides the data that you assigned to the    ClientMetadata property. In your function code in AWS Lambda, you can process    the clientMetadata value to enhance your workflow for your specific needs.
    /// 
    /// For more information, see Customizing User Pool Workflows with Lambda Triggers in the Amazon     Cognito Developer Guide.
    /// 
    /// NoteTake the following limitations into consideration when you use the ClientMetadata     parameter:                        Amazon Cognito does not store the ClientMetadata value. This data is available only       to AWS Lambda triggers that are assigned to a user pool to support custom       workflows. If your user pool configuration does not include triggers, the ClientMetadata       parameter serves no purpose.          Amazon Cognito does not validate the ClientMetadata value.          Amazon Cognito does not encrypt the the ClientMetadata value, so don't use it to       provide sensitive information.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientMetadata")]
    pub client_metadata: Option<serde_json::Value>,


    /// 
    /// Specify "EMAIL" if email will be used to send the welcome message.       Specify "SMS" if the phone number will be used. The default value is         "SMS". You can specify more than one value.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DesiredDeliveryMediums")]
    pub desired_delivery_mediums: Option<Vec<String>>,


    /// 
    /// This parameter is used only if the phone_number_verified or         email_verified attribute is set to True. Otherwise, it is       ignored.
    /// 
    /// If this parameter is set to True and the phone number or email address       specified in the UserAttributes parameter already exists as an alias with a different       user, the API call will migrate the alias from the previous user to the newly created       user. The previous user will no longer be able to log in using that alias.
    /// 
    /// If this parameter is set to False, the API throws an         AliasExistsException error if the alias already exists. The default       value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ForceAliasCreation")]
    pub force_alias_creation: Option<bool>,


    /// 
    /// Set to RESEND to resend the invitation message to a user that already       exists and reset the expiration limit on the user's account. Set to         SUPPRESS to suppress sending the message. You can specify only one       value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: RESEND | SUPPRESS
    ///
    /// Update requires: Replacement
    #[serde(rename = "MessageAction")]
    pub message_action: Option<UserPoolUserMessageActionEnum>,


    /// 
    /// The user attributes and attribute values to be set for the user to be created. These are    name-value pairs You can create a user without specifying any attributes other than     Username. However, any attributes that you specify as required (in https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateUserPool.html or in the     Attributes tab of the console) must be supplied either by    you (in your call to AdminCreateUser) or by the user (when they sign up in    response to your welcome message).
    /// 
    /// For custom attributes, you must prepend the custom: prefix to the attribute    name.
    /// 
    /// To send a message inviting the user to sign up, you must specify the user's email address    or phone number. This can be done in your call to AdminCreateUser or in the Users tab of the Amazon Cognito console for managing your user    pools.
    /// 
    /// In your call to AdminCreateUser, you can set the email_verified    attribute to True, and you can set the phone_number_verified    attribute to True. (You can also do this by calling https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateUserAttributes.html.)
    /// 
    /// email: The email address of the user to whom the message      that contains the code and user name will be sent. Required if the       email_verified attribute is set to True, or if       "EMAIL" is specified in the DesiredDeliveryMediums      parameter.              phone_number: The phone number of the user to whom the      message that contains the code and user name will be sent. Required if the       phone_number_verified attribute is set to True, or if       "SMS" is specified in the DesiredDeliveryMediums      parameter.
    /// 
    /// Required: No
    ///
    /// Type: List of AttributeType
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Option<Vec<AttributeType>>,


    /// 
    /// The user pool ID for the user pool where the user will be created.
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
    pub user_pool_id: String,


    /// 
    /// The username for the user. Must be unique within the user pool. Must be a UTF-8 string       between 1 and 128 characters. After the user is created, the username can't be       changed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Username")]
    pub username: Option<String>,


    /// 
    /// The user's validation data. This is an array of name-value pairs that contain user       attributes and attribute values that you can use for custom validation, such as       restricting the types of user accounts that can be registered. For example, you might       choose to allow or disallow user sign-up based on the user's domain.
    /// 
    /// To configure custom validation, you must create a Pre Sign-up AWS Lambda trigger for       the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the       validation data and uses it in the validation process.
    /// 
    /// The user's validation data isn't persisted.
    /// 
    /// Required: No
    ///
    /// Type: List of AttributeType
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationData")]
    pub validation_data: Option<Vec<AttributeType>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum UserPoolUserMessageActionEnum {

    /// RESEND
    #[serde(rename = "RESEND")]
    Resend,

    /// SUPPRESS
    #[serde(rename = "SUPPRESS")]
    Suppress,

}

impl Default for UserPoolUserMessageActionEnum {
    fn default() -> Self {
        UserPoolUserMessageActionEnum::Resend
    }
}


impl cfn_resources::CfnResource for CfnUserPoolUser {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolUser"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies whether the attribute is standard or custom.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AttributeType {


    /// 
    /// The name of the attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value of the attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


