

/// The AWS::Cognito::UserPoolDomain resource creates a new domain for a user pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolDomain {


    /// 
    /// The configuration for a custom domain that hosts the sign-up and sign-in pages for       your application. Use this object to specify an SSL certificate that is managed by       ACM.
    /// 
    /// Required: No
    ///
    /// Type: CustomDomainConfigType
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomDomainConfig")]
    pub custom_domain_config: Option<CustomDomainConfigType>,


    /// 
    /// The domain name for the domain that hosts the sign-up and sign-in pages for your    application. For example: auth.example.com. If you're using a prefix domain, this    field denotes the first part of the domain before     .auth.[region].amazoncognito.com.
    /// 
    /// This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen    for the first or last character. Use periods to separate subdomain names.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-z0-9](?:[a-z0-9\-]{0,61}[a-z0-9])?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: String,


    /// 
    /// The user pool ID for the user pool where you want to associate a user pool domain.
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

}



impl cfn_resources::CfnResource for CfnUserPoolDomain {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolDomain"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The configuration for a custom domain that hosts the sign-up and sign-in webpages for       your application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomDomainConfigType {


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Certificate Manager SSL certificate. You use       this certificate for the subdomain of your custom domain.
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
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}


