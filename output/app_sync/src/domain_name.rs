/// The AWS::AppSync::DomainName resource creates a DomainNameConfig object to     configure a custom domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDomainName {
    ///
    /// The Amazon Resource Name (ARN) of the certificate. This will be an AWS Certificate Manager     certificate.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: cfn_resources::StrVal,

    ///
    /// The decription for your domain name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The domain name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_app_sync_domain_name: CfnDomainNameappsyncdomainname,

    #[serde(skip_serializing)]
    pub att_domain_name: CfnDomainNamedomainname,

    #[serde(skip_serializing)]
    pub att_hosted_zone_id: CfnDomainNamehostedzoneid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNameappsyncdomainname;
impl CfnDomainNameappsyncdomainname {
    pub fn att_name(&self) -> &'static str {
        r#"AppSyncDomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNamedomainname;
impl CfnDomainNamedomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNamehostedzoneid;
impl CfnDomainNamehostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"HostedZoneId"#
    }
}

impl cfn_resources::CfnResource for CfnDomainName {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::DomainName"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
