

/// Specifies an identity for using within SES. An identity is an email address or domain       that you use when you send email. Before you can use an identity to send email, you       first have to verify it. By verifying an identity, you demonstrate that you're the owner       of the identity, and that you've given Amazon SES API v2 permission to send email from       the identity.
///
/// When you verify an email address, SES sends an email to the address. Your email       address is verified as soon as you follow the link in the verification email. When you       verify a domain without specifying the DkimSigningAttributes properties, OR only the       NextSigningKeyLength property of DkimSigningAttributes, this resource provides a set of       CNAME token names and values (DkimDNSTokenName1, DkimDNSTokenValue1, DkimDNSTokenName2,       DkimDNSTokenValue2, DkimDNSTokenName3, DkimDNSTokenValue3) as outputs. You can then add       these to the DNS configuration for your domain. Your domain is verified when Amazon SES       detects these records in the DNS configuration for your domain. This verification method       is known as Easy DKIM.
///
/// Alternatively, you can perform the verification process by providing your own       public-private key pair. This verification method is known as Bring Your Own DKIM       (BYODKIM). To use BYODKIM, your resource must include DkimSigningAttributes properties       DomainSigningSelector and DomainSigningPrivateKey. When you specify this object, you       provide a selector (DomainSigningSelector) (a component of the DNS record name that       identifies the public key to use for DKIM authentication) and a private key       (DomainSigningPrivateKey).
///
/// Additionally, you can associate an existing configuration set with the email identity       that you're verifying.
#[derive(Default, serde::Serialize)]
pub struct CfnEmailIdentity {


    /// 
    /// Used to enable or disable the custom Mail-From domain configuration for an email       identity.
    /// 
    /// Required: No
    ///
    /// Type: MailFromAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "MailFromAttributes")]
    pub mail_from_attributes: Option<MailFromAttributes>,


    /// 
    /// Used to associate a configuration set with an email identity.
    /// 
    /// Required: No
    ///
    /// Type: ConfigurationSetAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationSetAttributes")]
    pub configuration_set_attributes: Option<ConfigurationSetAttributes>,


    /// 
    /// The email address or domain to verify.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,


    /// 
    /// If your request includes this object, Amazon SES configures the identity to use Bring       Your Own DKIM (BYODKIM) for DKIM authentication purposes, or, configures the key length       to be used for Easy       DKIM.
    /// 
    /// Required: No
    ///
    /// Type: DkimSigningAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "DkimSigningAttributes")]
    pub dkim_signing_attributes: Option<DkimSigningAttributes>,


    /// 
    /// An object that contains information about the DKIM attributes for the identity.
    /// 
    /// Required: No
    ///
    /// Type: DkimAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "DkimAttributes")]
    pub dkim_attributes: Option<DkimAttributes>,


    /// 
    /// Used to enable or disable feedback forwarding for an identity.
    /// 
    /// Required: No
    ///
    /// Type: FeedbackAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeedbackAttributes")]
    pub feedback_attributes: Option<FeedbackAttributes>,

}


/// Used to enable or disable the custom Mail-From domain configuration for an email       identity.
#[derive(Default, serde::Serialize)]
pub struct MailFromAttributes {


    /// 
    /// The action to take if the required MX record isn't found when you send an email. When       you set this value to USE_DEFAULT_VALUE, the mail is sent using         amazonses.com as the MAIL FROM domain. When you set this value       to REJECT_MESSAGE, the Amazon SES API v2 returns a         MailFromDomainNotVerified error, and doesn't attempt to deliver the       email.
    /// 
    /// These behaviors are taken when the custom MAIL FROM domain configuration is in the         Pending, Failed, and TemporaryFailure       states.
    /// 
    /// Valid Values: USE_DEFAULT_VALUE | REJECT_MESSAGE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BehaviorOnMxFailure")]
    pub behavior_on_mx_failure: Option<String>,


    /// 
    /// The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM       domain must meet the following criteria:
    /// 
    /// It has to be a subdomain of the verified identity.               It can't be used to receive email.               It can't be used in a "From" address if the MAIL FROM domain is a destination           for feedback forwarding emails.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MailFromDomain")]
    pub mail_from_domain: Option<String>,

}


/// Used to enable or disable DKIM authentication for an email identity.
#[derive(Default, serde::Serialize)]
pub struct DkimAttributes {


    /// 
    /// Sets the DKIM signing configuration for the identity.
    /// 
    /// When you set this value true, then the messages that are sent from the       identity are signed using DKIM. If you set this value to false, your       messages are sent without DKIM signing.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningEnabled")]
    pub signing_enabled: Option<bool>,

}


/// Used to enable or disable feedback forwarding for an identity. This setting determines       what happens when an identity is used to send an email that results in a bounce or       complaint event.
#[derive(Default, serde::Serialize)]
pub struct FeedbackAttributes {


    /// 
    /// Sets the feedback forwarding configuration for the identity.
    /// 
    /// If the value is true, you receive email notifications when bounce or       complaint events occur. These notifications are sent to the address that you specified       in the Return-Path header of the original email.
    /// 
    /// You're required to have a method of tracking bounces and complaints. If you haven't       set up another mechanism for receiving bounce or complaint notifications (for example,       by setting up an event destination), you receive an email notification when these events       occur (even if this setting is disabled).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmailForwardingEnabled")]
    pub email_forwarding_enabled: Option<bool>,

}


/// Used to configure or change the DKIM authentication settings for an email domain       identity. You can use this operation to do any of the following:
#[derive(Default, serde::Serialize)]
pub struct DkimSigningAttributes {


    /// 
    /// [Easy DKIM] The key length of the future DKIM key pair to be generated. This can be       changed at most once per day.
    /// 
    /// Valid Values: RSA_1024_BIT | RSA_2048_BIT
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NextSigningKeyLength")]
    pub next_signing_key_length: Option<String>,


    /// 
    /// [Bring Your Own DKIM] A string that's used to identify a public key in the DNS       configuration for a domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainSigningSelector")]
    pub domain_signing_selector: Option<String>,


    /// 
    /// [Bring Your Own DKIM] A private key that's used to generate a DKIM signature.
    /// 
    /// The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using       base64 encoding.
    /// 
    /// ImportantRather than embedding sensitive information directly in your CFN templates, we         recommend you use dynamic parameters in the stack template to reference sensitive         information that is stored and managed outside of CFN, such as in the AWS Systems Manager Parameter Store or AWS Secrets         Manager.For more information, see the Do not embed           credentials in your templates best practice.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainSigningPrivateKey")]
    pub domain_signing_private_key: Option<String>,

}


/// Used to associate a configuration set with an email identity.
#[derive(Default, serde::Serialize)]
pub struct ConfigurationSetAttributes {


    /// 
    /// The configuration set to associate with an email identity.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: Option<String>,

}