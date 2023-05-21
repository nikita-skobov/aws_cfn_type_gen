

/// The AWS::AppStream::DirectoryConfig resource specifies the configuration information required to join Amazon AppStream 2.0 fleets    and image builders to Microsoft Active Directory domains.
#[derive(Default, serde::Serialize)]
pub struct CfnDirectoryConfig {


    /// 
    /// The distinguished names of the organizational units for computer accounts.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    pub organizational_unit_distinguished_names: Vec<String>,


    /// 
    /// The fully qualified name of the directory (for example, corp.example.com).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,


    /// 
    /// The certificate-based authentication properties used to authenticate SAML 2.0 Identity       Provider (IdP) user identities to Active Directory domain-joined streaming instances.
    /// 
    /// Required: No
    ///
    /// Type: CertificateBasedAuthProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateBasedAuthProperties")]
    pub certificate_based_auth_properties: Option<CertificateBasedAuthProperties>,


    /// 
    /// The credentials for the service account used by the streaming instance to connect to    the directory. Do not use this parameter directly. Use ServiceAccountCredentials as an input parameter with noEcho as shown in     the Parameters. For best practices information, see Do Not Embed Credentials in Your Templates.
    /// 
    /// Required: Yes
    ///
    /// Type: ServiceAccountCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccountCredentials")]
    pub service_account_credentials: ServiceAccountCredentials,

}


/// The certificate-based authentication properties used to authenticate SAML 2.0 Identity       Provider (IdP) user identities to Active Directory domain-joined streaming instances.
#[derive(Default, serde::Serialize)]
pub struct CertificateBasedAuthProperties {


    /// 
    /// The status of the certificate-based authentication properties. Fallback is turned on by default when certificate-based authentication is Enabled. Fallback allows users to log in using their AD     domain password if certificate-based authentication is unsuccessful, or to unlock a     desktop lock screen. Enabled_no_directory_login_fallback enables certificate-based     authentication, but does not allow users to log in using their AD domain password. Users     will be disconnected to re-authenticate using certificates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED | ENABLED_NO_DIRECTORY_LOGIN_FALLBACK
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// The ARN of the AWS Certificate Manager Private CA resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: Option<String>,

}


/// The credentials for the service account used by the streaming instance to connect to the directory.
#[derive(Default, serde::Serialize)]
pub struct ServiceAccountCredentials {


    /// 
    /// The password for the account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 127
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountPassword")]
    pub account_password: String,


    /// 
    /// The user name of the account. This account must have the following privileges: create computer objects,       join computers to the domain, and change/reset the password on descendant computer objects for the       organizational units specified.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountName")]
    pub account_name: String,

}