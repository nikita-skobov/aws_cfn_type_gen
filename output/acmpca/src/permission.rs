/// Grants permissions to the AWS Certificate Manager (ACM) service       principal (acm.amazonaws.com) to perform IssueCertificate, GetCertificate, and ListPermissions       actions on a CA. These actions are needed for the ACM principal to renew private PKI       certificates requested through ACM and residing in the same AWS account       as the CA.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPermission {
    ///
    /// The private CA actions that can be performed by the designated AWS       service. Supported actions are IssueCertificate,         GetCertificate, and ListPermissions.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

    ///
    /// The Amazon Resource Number (ARN) of the private CA from which the permission was 			issued.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 200
    ///
    /// Pattern: arn:[\w+=/,.@-]+:[\w+=/,.@-]+:[\w+=/,.@-]*:[0-9]*:[\w+=,.@-]+(/[\w+=,.@-]+)*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: cfn_resources::StrVal,

    ///
    /// The AWS service or entity that holds the permission. At this time, the only valid 			principal is acm.amazonaws.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[^*]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: cfn_resources::StrVal,

    ///
    /// The ID of the account that assigned the permission.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: [0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnPermission {
    fn type_string(&self) -> &'static str {
        "AWS::ACMPCA::Permission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.actions;

        if the_val.len() > 3 as _ {
            return Err(format!(
                "Max validation failed on field 'actions'. {} is greater than 3",
                the_val.len()
            ));
        }

        let the_val = &self.certificate_authority_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 200 as _ {
                return Err(format!("Max validation failed on field 'certificate_authority_arn'. {} is greater than 200", s.len()));
            }
        }

        let the_val = &self.certificate_authority_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 5 as _ {
                return Err(format!(
                    "Min validation failed on field 'certificate_authority_arn'. {} is less than 5",
                    s.len()
                ));
            }
        }

        let the_val = &self.principal;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'principal'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.principal;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'principal'. {} is less than 0",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.source_account {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 12 as _ {
                    return Err(format!(
                        "Max validation failed on field 'source_account'. {} is greater than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.source_account {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 12 as _ {
                    return Err(format!(
                        "Min validation failed on field 'source_account'. {} is less than 12",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
