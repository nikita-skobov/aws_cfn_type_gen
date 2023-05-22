/// The AWS::FinSpace::Environment resource represents an Amazon FinSpace     environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironment {
    ///
    /// The description of the FinSpace environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^[a-zA-Z0-9. ]{1,1000}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The authentication mode for the environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FEDERATED | LOCAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "FederationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_mode: Option<EnvironmentFederationModeEnum>,

    ///
    /// Configuration information when authentication mode is FEDERATED.
    ///
    /// Required: No
    ///
    /// Type: FederationParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_parameters: Option<FederationParameters>,

    ///
    /// The KMS key id used to encrypt in the FinSpace environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^[a-zA-Z-0-9-:\/]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the FinSpace environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9]+[a-zA-Z0-9-]*[a-zA-Z0-9]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Configuration information for the superuser.
    ///
    /// Required: No
    ///
    /// Type: SuperuserParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "SuperuserParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superuser_parameters: Option<SuperuserParameters>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_aws_account_id: CfnEnvironmentawsaccountid,

    #[serde(skip_serializing)]
    pub att_dedicated_service_account_id: CfnEnvironmentdedicatedserviceaccountid,

    #[serde(skip_serializing)]
    pub att_environment_arn: CfnEnvironmentenvironmentarn,

    #[serde(skip_serializing)]
    pub att_environment_id: CfnEnvironmentenvironmentid,

    #[serde(skip_serializing)]
    pub att_environment_url: CfnEnvironmentenvironmenturl,

    #[serde(skip_serializing)]
    pub att_sage_maker_studio_domain_url: CfnEnvironmentsagemakerstudiodomainurl,

    #[serde(skip_serializing)]
    pub att_status: CfnEnvironmentstatus,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EnvironmentFederationModeEnum {
    /// FEDERATED
    #[serde(rename = "FEDERATED")]
    Federated,

    /// LOCAL
    #[serde(rename = "LOCAL")]
    Local,
}

impl Default for EnvironmentFederationModeEnum {
    fn default() -> Self {
        EnvironmentFederationModeEnum::Federated
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentawsaccountid;
impl CfnEnvironmentawsaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"AwsAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentdedicatedserviceaccountid;
impl CfnEnvironmentdedicatedserviceaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"DedicatedServiceAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentenvironmentarn;
impl CfnEnvironmentenvironmentarn {
    pub fn att_name(&self) -> &'static str {
        r#"EnvironmentArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentenvironmentid;
impl CfnEnvironmentenvironmentid {
    pub fn att_name(&self) -> &'static str {
        r#"EnvironmentId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentenvironmenturl;
impl CfnEnvironmentenvironmenturl {
    pub fn att_name(&self) -> &'static str {
        r#"EnvironmentUrl"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentsagemakerstudiodomainurl;
impl CfnEnvironmentsagemakerstudiodomainurl {
    pub fn att_name(&self) -> &'static str {
        r#"SageMakerStudioDomainUrl"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentstatus;
impl CfnEnvironmentstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string(&self) -> &'static str {
        "AWS::FinSpace::Environment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.federation_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.superuser_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AttributeMapItems property type specifies Property description not available. for an AWS::FinSpace::Environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AttributeMapItems {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AttributeMapItems {
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

/// Configuration information when authentication mode is FEDERATED.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FederationParameters {
    ///
    /// The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration    (IdP).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^https?://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationCallBackURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_call_back_url: Option<cfn_resources::StrVal>,

    ///
    /// SAML attribute name and value. The name must always be Email and the value should be set to     the attribute definition in which user email is set. For example, name would be Email and     value http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress.     Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.
    ///
    /// Required: No
    ///
    /// Type: List of AttributeMapItems
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttributeMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_map: Option<Vec<AttributeMapItems>>,

    ///
    /// Name of the identity provider (IdP).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^_\p{Z}][\p{L}\p{M}\p{S}\p{N}\p{P}][^_\p{Z}]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_provider_name: Option<cfn_resources::StrVal>,

    ///
    /// The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[A-Za-z0-9._\-:\/#\+]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationURN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_urn: Option<cfn_resources::StrVal>,

    ///
    /// SAML 2.0 Metadata document from identity provider (IdP).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1000
    ///
    /// Maximum: 10000000
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "SamlMetadataDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_metadata_document: Option<cfn_resources::StrVal>,

    ///
    /// Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^https?://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: Replacement
    #[serde(rename = "SamlMetadataURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FederationParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.application_call_back_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!("Max validation failed on field 'application_call_back_url'. {} is greater than 1000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.application_call_back_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'application_call_back_url'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.federation_provider_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!("Max validation failed on field 'federation_provider_name'. {} is greater than 32", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.federation_provider_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'federation_provider_name'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.federation_urn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'federation_urn'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.federation_urn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'federation_urn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.saml_metadata_document {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000000 as _ {
                    return Err(format!("Max validation failed on field 'saml_metadata_document'. {} is greater than 10000000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.saml_metadata_document {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1000 as _ {
                    return Err(format!("Min validation failed on field 'saml_metadata_document'. {} is less than 1000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.saml_metadata_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!("Max validation failed on field 'saml_metadata_url'. {} is greater than 1000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.saml_metadata_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'saml_metadata_url'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Configuration information for the superuser.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SuperuserParameters {
    ///
    /// The email address of the superuser.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+[.]+[A-Za-z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<cfn_resources::StrVal>,

    ///
    /// The first name of the superuser.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[a-zA-Z0-9]{1,50}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<cfn_resources::StrVal>,

    ///
    /// The last name of the superuser.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[a-zA-Z0-9]{1,50}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SuperuserParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.email_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_address'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.first_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!(
                        "Max validation failed on field 'first_name'. {} is greater than 50",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.first_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'first_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.last_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!(
                        "Max validation failed on field 'last_name'. {} is greater than 50",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.last_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'last_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
