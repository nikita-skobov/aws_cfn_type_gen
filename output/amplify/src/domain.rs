

/// The AWS::Amplify::Domain resource allows you to connect a custom domain to your app.
#[derive(Default, serde::Serialize)]
pub struct CfnDomain {


    /// 
    /// The domain name for the domain association.
    /// 
    /// Length Constraints: Maximum length of 255.
    /// 
    /// Pattern:    ^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])(\.)?$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// The unique ID for an Amplify app.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of 20.
    /// 
    /// Pattern: d[a-z0-9]+
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AppId")]
    pub app_id: String,


    /// 
    /// Sets the branch patterns for automatic subdomain creation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSubDomainCreationPatterns")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,


    /// 
    /// The required AWS Identity and Access Management (IAM) service role for the Amazon Resource Name    (ARN) for automatically creating subdomains.
    /// 
    /// Length Constraints: Maximum length of 1000.
    /// 
    /// Pattern: ^$|^arn:aws:iam::\d{12}:role.+
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSubDomainIAMRole")]
    pub auto_sub_domain_iamrole: Option<String>,


    /// 
    /// Enables the automated creation of subdomains for branches.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoSubDomain")]
    pub enable_auto_sub_domain: Option<bool>,


    /// 
    /// The setting for the subdomain.
    /// 
    /// Required: Yes
    ///
    /// Type: List of SubDomainSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,

}


/// The SubDomainSetting property type enables you to connect a subdomain (for example,    example.exampledomain.com) to a specific branch.
#[derive(Default, serde::Serialize)]
pub struct SubDomainSetting {


    /// 
    /// The branch name setting for the subdomain.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    /// 
    /// Pattern: (?s).+
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BranchName")]
    pub branch_name: String,


    /// 
    /// The prefix setting for the subdomain.
    /// 
    /// Length Constraints: Maximum length of 255.
    /// 
    /// Pattern: (?s).*
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: String,

}
