/// OrganizationConformancePack deploys conformance packs across member accounts in an AWS Organizations.       OrganizationConformancePack enables organization service access for config-multiaccountsetup.amazonaws.com through the EnableAWSServiceAccess action and       creates a service linked role in the master account of your organization.       The service linked role is created only when the role does not exist in the master account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOrganizationConformancePack {
    ///
    /// A list of ConformancePackInputParameter objects.
    ///
    /// Required: No
    ///
    /// Type: List of ConformancePackInputParameter
    ///
    /// Maximum: 60
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,

    ///
    /// The name of the Amazon S3 bucket where AWS Config stores conformance pack templates.
    ///
    /// NoteThis field is optional.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryS3Bucket")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub delivery_s3_bucket: Option<cfn_resources::StrVal>,

    ///
    /// Any folder structure you want to add to an Amazon S3 bucket.
    ///
    /// NoteThis field is optional.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub delivery_s3_key_prefix: Option<cfn_resources::StrVal>,

    ///
    /// A comma-separated list of accounts excluded from organization conformance pack.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedAccounts")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub excluded_accounts: Option<Vec<String>>,

    ///
    /// The name you assign to an organization conformance pack.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z][-a-zA-Z0-9]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: cfn_resources::StrVal,

    ///
    /// A string containing full conformance pack template body. Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub template_body: Option<cfn_resources::StrVal>,

    ///
    /// Location of file containing the template body. The uri must point to the conformance pack template (max size: 300 KB).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateS3Uri")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub template_s3_uri: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnOrganizationConformancePack {
    fn type_string(&self) -> &'static str {
        "AWS::Config::OrganizationConformancePack"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.conformance_pack_input_parameters {
            if the_val.len() > 60 as _ {
                return Err(format!("Max validation failed on field 'conformance_pack_input_parameters'. {} is greater than 60", the_val.len()));
            }
        }

        if let Some(the_val) = &self.delivery_s3_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'delivery_s3_bucket'. {} is greater than 63", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.delivery_s3_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'delivery_s3_bucket'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.delivery_s3_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'delivery_s3_key_prefix'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.delivery_s3_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!("Min validation failed on field 'delivery_s3_key_prefix'. {} is less than 0", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.excluded_accounts {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'excluded_accounts'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.organization_conformance_pack_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!("Max validation failed on field 'organization_conformance_pack_name'. {} is greater than 128", s.len()));
            }
        }

        let the_val = &self.organization_conformance_pack_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!("Min validation failed on field 'organization_conformance_pack_name'. {} is less than 1", s.len()));
            }
        }

        Ok(())
    }
}

/// Input parameters in the form of key-value pairs for the conformance pack, both of which you define. 			Keys can have a maximum character length of 255 characters, and values can have a maximum length of 4096 characters.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConformancePackInputParameter {
    ///
    /// One part of a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: cfn_resources::StrVal,

    ///
    /// One part of a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ConformancePackInputParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.parameter_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'parameter_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.parameter_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'parameter_name'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.parameter_value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'parameter_value'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.parameter_value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'parameter_value'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
