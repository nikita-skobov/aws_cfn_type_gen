/// The AWS::Amplify::Domain resource allows you to connect a custom domain to your app.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomain {
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
    pub app_id: cfn_resources::StrVal,

    ///
    /// Sets the branch patterns for automatic subdomain creation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSubDomainCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_iamrole: Option<cfn_resources::StrVal>,

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
    pub domain_name: cfn_resources::StrVal,

    ///
    /// Enables the automated creation of subdomains for branches.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoSubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    #[serde(skip_serializing)]
    pub att_arn: CfnDomainarn,

    #[serde(skip_serializing)]
    pub att_auto_sub_domain_iamrole: CfnDomainautosubdomainiamrole,

    #[serde(skip_serializing)]
    pub att_certificate_record: CfnDomaincertificaterecord,

    #[serde(skip_serializing)]
    pub att_domain_name: CfnDomaindomainname,

    #[serde(skip_serializing)]
    pub att_domain_status: CfnDomaindomainstatus,

    #[serde(skip_serializing)]
    pub att_status_reason: CfnDomainstatusreason,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainarn;
impl CfnDomainarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainautosubdomainiamrole;
impl CfnDomainautosubdomainiamrole {
    pub fn att_name(&self) -> &'static str {
        r#"AutoSubDomainIAMRole"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomaincertificaterecord;
impl CfnDomaincertificaterecord {
    pub fn att_name(&self) -> &'static str {
        r#"CertificateRecord"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomaindomainname;
impl CfnDomaindomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomaindomainstatus;
impl CfnDomaindomainstatus {
    pub fn att_name(&self) -> &'static str {
        r#"DomainStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainstatusreason;
impl CfnDomainstatusreason {
    pub fn att_name(&self) -> &'static str {
        r#"StatusReason"#
    }
}

impl cfn_resources::CfnResource for CfnDomain {
    fn type_string(&self) -> &'static str {
        "AWS::Amplify::Domain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The SubDomainSetting property type enables you to connect a subdomain (for example,    example.exampledomain.com) to a specific branch.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub branch_name: cfn_resources::StrVal,

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
    pub prefix: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SubDomainSetting {
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
