
pub mod cfn_identity_pool {

#[derive(serde::Serialize, Default)]
pub struct CfnIdentityPool {
    /// No documentation provided by AWS
    #[serde(rename = "OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SamlProviderARNs")]
    pub saml_provider_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CognitoEvents")]
    pub cognito_events: Option<()>,
    /// List of CognitoIdentityProvider
    #[serde(rename = "CognitoIdentityProviders")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeveloperProviderName")]
    pub developer_provider_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowClassicFlow")]
    pub allow_classic_flow: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SupportedLoginProviders")]
    pub supported_login_providers: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "PushSync")]
    pub push_sync: Option<PushSync>,
    /// No documentation provided by AWS
    #[serde(rename = "IdentityPoolName")]
    pub identity_pool_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    /// No documentation provided by AWS
    #[serde(rename = "CognitoStreams")]
    pub cognito_streams: Option<CognitoStreams>,

}


#[derive(serde::Serialize, Default)]
pub struct CognitoIdentityProvider {
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,
    #[serde(rename = "ServerSideTokenCheck")]
    pub server_side_token_check: Option<bool>,
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CognitoStreams {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamingStatus")]
    pub streaming_status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PushSync {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "ApplicationArns")]
    pub application_arns: Option<Vec<String>>,

}


}

pub mod cfn_identity_pool_role_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnIdentityPoolRoleAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Roles")]
    pub roles: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleMappings")]
    pub role_mappings: Option<()>,

}



}

pub mod cfn_user_pool {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPool {
    /// No documentation provided by AWS
    #[serde(rename = "EmailConfiguration")]
    pub email_configuration: Option<EmailConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "UsernameAttributes")]
    pub username_attributes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SmsVerificationMessage")]
    pub sms_verification_message: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MfaConfiguration")]
    pub mfa_configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoVerifiedAttributes")]
    pub auto_verified_attributes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "EmailVerificationMessage")]
    pub email_verification_message: Option<String>,
    /// List of SchemaAttribute
    #[serde(rename = "Schema")]
    pub schema: Option<Vec<SchemaAttribute>>,
    /// No documentation provided by AWS
    #[serde(rename = "AdminCreateUserConfig")]
    pub admin_create_user_config: Option<AdminCreateUserConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "VerificationMessageTemplate")]
    pub verification_message_template: Option<VerificationMessageTemplate>,
    /// No documentation provided by AWS
    #[serde(rename = "AliasAttributes")]
    pub alias_attributes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeviceConfiguration")]
    pub device_configuration: Option<DeviceConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "UsernameConfiguration")]
    pub username_configuration: Option<UsernameConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "SmsConfiguration")]
    pub sms_configuration: Option<SmsConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "UserAttributeUpdateSettings")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "AccountRecoverySetting")]
    pub account_recovery_setting: Option<AccountRecoverySetting>,
    /// No documentation provided by AWS
    #[serde(rename = "Policies")]
    pub policies: Option<Policies>,
    /// No documentation provided by AWS
    #[serde(rename = "SmsAuthenticationMessage")]
    pub sms_authentication_message: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolName")]
    pub user_pool_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolAddOns")]
    pub user_pool_add_ons: Option<UserPoolAddOns>,
    /// No documentation provided by AWS
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolTags")]
    pub user_pool_tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EmailVerificationSubject")]
    pub email_verification_subject: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnabledMfas")]
    pub enabled_mfas: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "LambdaConfig")]
    pub lambda_config: Option<LambdaConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct InviteMessageTemplate {
    #[serde(rename = "EmailSubject")]
    pub email_subject: Option<String>,
    #[serde(rename = "EmailMessage")]
    pub email_message: Option<String>,
    #[serde(rename = "SMSMessage")]
    pub smsmessage: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Policies {
    #[serde(rename = "PasswordPolicy")]
    pub password_policy: Option<PasswordPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct UserPoolAddOns {
    #[serde(rename = "AdvancedSecurityMode")]
    pub advanced_security_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AccountRecoverySetting {
    #[serde(rename = "RecoveryMechanisms")]
    pub recovery_mechanisms: Option<Vec<RecoveryOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct EmailConfiguration {
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,
    #[serde(rename = "ConfigurationSet")]
    pub configuration_set: Option<String>,
    #[serde(rename = "EmailSendingAccount")]
    pub email_sending_account: Option<String>,
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "ReplyToEmailAddress")]
    pub reply_to_email_address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberAttributeConstraints {
    #[serde(rename = "MaxValue")]
    pub max_value: Option<String>,
    #[serde(rename = "MinValue")]
    pub min_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DeviceConfiguration {
    #[serde(rename = "DeviceOnlyRememberedOnUserPrompt")]
    pub device_only_remembered_on_user_prompt: Option<bool>,
    #[serde(rename = "ChallengeRequiredOnNewDevice")]
    pub challenge_required_on_new_device: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomSMSSender {
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: Option<String>,
    #[serde(rename = "LambdaVersion")]
    pub lambda_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VerificationMessageTemplate {
    #[serde(rename = "EmailMessage")]
    pub email_message: Option<String>,
    #[serde(rename = "SmsMessage")]
    pub sms_message: Option<String>,
    #[serde(rename = "EmailSubject")]
    pub email_subject: Option<String>,
    #[serde(rename = "EmailMessageByLink")]
    pub email_message_by_link: Option<String>,
    #[serde(rename = "DefaultEmailOption")]
    pub default_email_option: Option<String>,
    #[serde(rename = "EmailSubjectByLink")]
    pub email_subject_by_link: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct UsernameConfiguration {
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConfig {
    #[serde(rename = "PreSignUp")]
    pub pre_sign_up: Option<String>,
    #[serde(rename = "PostAuthentication")]
    pub post_authentication: Option<String>,
    #[serde(rename = "CustomEmailSender")]
    pub custom_email_sender: Option<CustomEmailSender>,
    #[serde(rename = "CreateAuthChallenge")]
    pub create_auth_challenge: Option<String>,
    #[serde(rename = "PreAuthentication")]
    pub pre_authentication: Option<String>,
    #[serde(rename = "PostConfirmation")]
    pub post_confirmation: Option<String>,
    #[serde(rename = "CustomMessage")]
    pub custom_message: Option<String>,
    #[serde(rename = "DefineAuthChallenge")]
    pub define_auth_challenge: Option<String>,
    #[serde(rename = "UserMigration")]
    pub user_migration: Option<String>,
    #[serde(rename = "PreTokenGeneration")]
    pub pre_token_generation: Option<String>,
    #[serde(rename = "CustomSMSSender")]
    pub custom_smssender: Option<CustomSMSSender>,
    #[serde(rename = "KMSKeyID")]
    pub kmskey_id: Option<String>,
    #[serde(rename = "VerifyAuthChallengeResponse")]
    pub verify_auth_challenge_response: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PasswordPolicy {
    #[serde(rename = "MinimumLength")]
    pub minimum_length: Option<usize>,
    #[serde(rename = "RequireUppercase")]
    pub require_uppercase: Option<bool>,
    #[serde(rename = "RequireSymbols")]
    pub require_symbols: Option<bool>,
    #[serde(rename = "RequireLowercase")]
    pub require_lowercase: Option<bool>,
    #[serde(rename = "RequireNumbers")]
    pub require_numbers: Option<bool>,
    #[serde(rename = "TemporaryPasswordValidityDays")]
    pub temporary_password_validity_days: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct StringAttributeConstraints {
    #[serde(rename = "MinLength")]
    pub min_length: Option<String>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AdminCreateUserConfig {
    #[serde(rename = "AllowAdminCreateUserOnly")]
    pub allow_admin_create_user_only: Option<bool>,
    #[serde(rename = "UnusedAccountValidityDays")]
    pub unused_account_validity_days: Option<usize>,
    #[serde(rename = "InviteMessageTemplate")]
    pub invite_message_template: Option<InviteMessageTemplate>,

}

#[derive(serde::Serialize, Default)]
pub struct UserAttributeUpdateSettings {
    #[serde(rename = "AttributesRequireVerificationBeforeUpdate")]
    pub attributes_require_verification_before_update: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaAttribute {
    #[serde(rename = "Required")]
    pub required: Option<bool>,
    #[serde(rename = "Mutable")]
    pub mutable: Option<bool>,
    #[serde(rename = "AttributeDataType")]
    pub attribute_data_type: Option<String>,
    #[serde(rename = "StringAttributeConstraints")]
    pub string_attribute_constraints: Option<StringAttributeConstraints>,
    #[serde(rename = "DeveloperOnlyAttribute")]
    pub developer_only_attribute: Option<bool>,
    #[serde(rename = "NumberAttributeConstraints")]
    pub number_attribute_constraints: Option<NumberAttributeConstraints>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RecoveryOption {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SmsConfiguration {
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,
    #[serde(rename = "SnsCallerArn")]
    pub sns_caller_arn: Option<String>,
    #[serde(rename = "SnsRegion")]
    pub sns_region: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomEmailSender {
    #[serde(rename = "LambdaVersion")]
    pub lambda_version: Option<String>,
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: Option<String>,

}


}

pub mod cfn_user_pool_client {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolClient {
    /// No documentation provided by AWS
    #[serde(rename = "TokenValidityUnits")]
    pub token_validity_units: Option<TokenValidityUnits>,
    /// No documentation provided by AWS
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    pub allowed_oauth_flows_user_pool_client: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "CallbackURLs")]
    pub callback_urls: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthSessionValidity")]
    pub auth_session_validity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowedOAuthScopes")]
    pub allowed_oauth_scopes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "RefreshTokenValidity")]
    pub refresh_token_validity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "GenerateSecret")]
    pub generate_secret: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "WriteAttributes")]
    pub write_attributes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessTokenValidity")]
    pub access_token_validity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "SupportedIdentityProviders")]
    pub supported_identity_providers: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "IdTokenValidity")]
    pub id_token_validity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowedOAuthFlows")]
    pub allowed_oauth_flows: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    pub enable_propagate_additional_user_context_data: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableTokenRevocation")]
    pub enable_token_revocation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "LogoutURLs")]
    pub logout_urls: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PreventUserExistenceErrors")]
    pub prevent_user_existence_errors: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadAttributes")]
    pub read_attributes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRedirectURI")]
    pub default_redirect_uri: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientName")]
    pub client_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ExplicitAuthFlows")]
    pub explicit_auth_flows: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct AnalyticsConfiguration {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "ApplicationArn")]
    pub application_arn: Option<String>,
    #[serde(rename = "UserDataShared")]
    pub user_data_shared: Option<bool>,
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,
    #[serde(rename = "ApplicationId")]
    pub application_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TokenValidityUnits {
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,
    #[serde(rename = "IdToken")]
    pub id_token: Option<String>,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,

}


}

pub mod cfn_user_pool_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolDomain {
    /// No documentation provided by AWS
    #[serde(rename = "Domain")]
    pub domain: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "CustomDomainConfig")]
    pub custom_domain_config: Option<CustomDomainConfigType>,

}


#[derive(serde::Serialize, Default)]
pub struct CustomDomainConfigType {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}


}

pub mod cfn_user_pool_group {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Precedence")]
    pub precedence: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,

}



}

pub mod cfn_user_pool_identity_provider {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolIdentityProvider {
    /// No documentation provided by AWS
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "IdpIdentifiers")]
    pub idp_identifiers: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AttributeMapping")]
    pub attribute_mapping: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProviderType")]
    pub provider_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProviderDetails")]
    pub provider_details: Option<()>,

}



}

pub mod cfn_user_pool_resource_server {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolResourceServer {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// List of ResourceServerScopeType
    #[serde(rename = "Scopes")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,

}


#[derive(serde::Serialize, Default)]
pub struct ResourceServerScopeType {
    #[serde(rename = "ScopeName")]
    pub scope_name: String,
    #[serde(rename = "ScopeDescription")]
    pub scope_description: String,

}


}

pub mod cfn_user_pool_risk_configuration_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolRiskConfigurationAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    pub compromised_credentials_risk_configuration: Option<CompromisedCredentialsRiskConfigurationType>,
    /// No documentation provided by AWS
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,
    /// No documentation provided by AWS
    #[serde(rename = "RiskExceptionConfiguration")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct NotifyEmailType {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "HtmlBody")]
    pub html_body: Option<String>,
    #[serde(rename = "TextBody")]
    pub text_body: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AccountTakeoverActionsType {
    #[serde(rename = "MediumAction")]
    pub medium_action: Option<AccountTakeoverActionType>,
    #[serde(rename = "LowAction")]
    pub low_action: Option<AccountTakeoverActionType>,
    #[serde(rename = "HighAction")]
    pub high_action: Option<AccountTakeoverActionType>,

}

#[derive(serde::Serialize, Default)]
pub struct CompromisedCredentialsRiskConfigurationType {
    #[serde(rename = "Actions")]
    pub actions: CompromisedCredentialsActionsType,
    #[serde(rename = "EventFilter")]
    pub event_filter: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct RiskExceptionConfigurationType {
    #[serde(rename = "SkippedIPRangeList")]
    pub skipped_iprange_list: Option<Vec<String>>,
    #[serde(rename = "BlockedIPRangeList")]
    pub blocked_iprange_list: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AccountTakeoverActionType {
    #[serde(rename = "Notify")]
    pub notify: bool,
    #[serde(rename = "EventAction")]
    pub event_action: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccountTakeoverRiskConfigurationType {
    #[serde(rename = "Actions")]
    pub actions: AccountTakeoverActionsType,
    #[serde(rename = "NotifyConfiguration")]
    pub notify_configuration: Option<NotifyConfigurationType>,

}

#[derive(serde::Serialize, Default)]
pub struct CompromisedCredentialsActionsType {
    #[serde(rename = "EventAction")]
    pub event_action: String,

}

#[derive(serde::Serialize, Default)]
pub struct NotifyConfigurationType {
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    #[serde(rename = "ReplyTo")]
    pub reply_to: Option<String>,
    #[serde(rename = "MfaEmail")]
    pub mfa_email: Option<NotifyEmailType>,
    #[serde(rename = "BlockEmail")]
    pub block_email: Option<NotifyEmailType>,
    #[serde(rename = "NoActionEmail")]
    pub no_action_email: Option<NotifyEmailType>,

}


}

pub mod cfn_user_pool_uicustomization_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolUICustomizationAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "CSS")]
    pub css: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ClientId")]
    pub client_id: String,

}



}

pub mod cfn_user_pool_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolUser {
    /// List of AttributeType
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Option<Vec<AttributeType>>,
    /// No documentation provided by AWS
    #[serde(rename = "Username")]
    pub username: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientMetadata")]
    pub client_metadata: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DesiredDeliveryMediums")]
    pub desired_delivery_mediums: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ForceAliasCreation")]
    pub force_alias_creation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "MessageAction")]
    pub message_action: Option<String>,
    /// List of AttributeType
    #[serde(rename = "ValidationData")]
    pub validation_data: Option<Vec<AttributeType>>,

}


#[derive(serde::Serialize, Default)]
pub struct AttributeType {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}

pub mod cfn_user_pool_user_to_group_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnUserPoolUserToGroupAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "Username")]
    pub username: String,
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,

}



}
