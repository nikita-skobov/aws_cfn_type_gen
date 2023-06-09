/// The AWS::Cognito::UserPoolDomain resource creates a new domain for a user pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub domain: cfn_resources::StrVal,

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
    pub user_pool_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_cloud_front_distribution: CfnUserPoolDomaincloudfrontdistribution,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserPoolDomaincloudfrontdistribution;
impl CfnUserPoolDomaincloudfrontdistribution {
    pub fn att_name(&self) -> &'static str {
        r#"CloudFrontDistribution"#
    }
}

impl cfn_resources::CfnResource for CfnUserPoolDomain {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolDomain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_domain_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.domain;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.domain;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'domain'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 55 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_pool_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The configuration for a custom domain that hosts the sign-up and sign-in webpages for       your application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CustomDomainConfigType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.certificate_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'certificate_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.certificate_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'certificate_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
