/// Tests and publishes a registered extension as a public, third-party extension.
///
/// CloudFormation first tests the extension to make sure it meets all necessary requirements for being  published in the CloudFormation registry. If it does, CloudFormation then publishes it to the  registry as a public third-party extension in this Region. Public extensions are available for use by all CloudFormation users.
///
/// For more information, see Testing your public   extension prior to publishing in the CloudFormation CLI User Guide.
///
/// If you don't specify a version, CloudFormation uses the default version of the extension in your  account and Region for testing.
///
/// To perform testing, CloudFormation assumes the execution role specified when the type was  registered.
///
/// An extension must have a test status of PASSED before it can be published. For more information,  see Publishing   extensions to make them available for public use in the CloudFormation CLI User   Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPublicTypeVersion {
    ///
    /// The Amazon Resource Number (ARN) of the extension.
    ///
    /// Conditional: You must specify Arn, or TypeName and Type.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}:([0-9]{12})?:type/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,

    ///
    /// The S3 bucket to which CloudFormation delivers the contract test execution logs.
    ///
    /// CloudFormation delivers the logs by the time contract testing has completed and the extension has been  assigned a test type status of PASSED or FAILED.
    ///
    /// The user initiating the stack operation must be able to access items in the specified S3 bucket. Specifically,  the user needs the following permissions:
    ///
    /// GetObject     PutObject
    ///
    /// For more information, see Actions, Resources, and Condition Keys for Amazon S3 in the AWS Identity and Access Management User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDeliveryBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_bucket: Option<cfn_resources::StrVal>,

    ///
    /// The version number to assign to this version of the extension.
    ///
    /// Use the following format, and adhere to semantic versioning when assigning a version number to your  extension:
    ///
    /// MAJOR.MINOR.PATCH
    ///
    /// For more information, see Semantic Versioning 2.0.0.
    ///
    /// If you don't specify a version number, CloudFormation increments the version number by one minor  version release.
    ///
    /// You cannot specify a version number the first time you publish a type. AWS CloudFormation automatically sets the first  version number to be 1.0.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublicVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<cfn_resources::StrVal>,

    ///
    /// The type of the extension to test.
    ///
    /// Conditional: You must specify Arn, or TypeName and Type.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: HOOK | MODULE | RESOURCE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<PublicTypeVersionTypeEnum>,

    ///
    /// The name of the extension to test.
    ///
    /// Conditional: You must specify Arn, or TypeName and Type.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 204
    ///
    /// Pattern: [A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}(::MODULE){0,1}
    ///
    /// Update requires: Replacement
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_public_type_arn: CfnPublicTypeVersionpublictypearn,

    #[serde(skip_serializing)]
    pub att_publisher_id: CfnPublicTypeVersionpublisherid,

    #[serde(skip_serializing)]
    pub att_type_version_arn: CfnPublicTypeVersiontypeversionarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PublicTypeVersionTypeEnum {
    /// HOOK
    #[serde(rename = "HOOK")]
    Hook,

    /// MODULE
    #[serde(rename = "MODULE")]
    Module,

    /// RESOURCE
    #[serde(rename = "RESOURCE")]
    Resource,
}

impl Default for PublicTypeVersionTypeEnum {
    fn default() -> Self {
        PublicTypeVersionTypeEnum::Hook
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublicTypeVersionpublictypearn;
impl CfnPublicTypeVersionpublictypearn {
    pub fn att_name(&self) -> &'static str {
        r#"PublicTypeArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublicTypeVersionpublisherid;
impl CfnPublicTypeVersionpublisherid {
    pub fn att_name(&self) -> &'static str {
        r#"PublisherId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublicTypeVersiontypeversionarn;
impl CfnPublicTypeVersiontypeversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"TypeVersionArn"#
    }
}

impl cfn_resources::CfnResource for CfnPublicTypeVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::PublicTypeVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'arn'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.log_delivery_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'log_delivery_bucket'. {} is greater than 63", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.log_delivery_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 3 as _ {
                    return Err(format!(
                        "Min validation failed on field 'log_delivery_bucket'. {} is less than 3",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.public_version_number {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 5 as _ {
                    return Err(format!(
                        "Min validation failed on field 'public_version_number'. {} is less than 5",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.type_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 204 as _ {
                    return Err(format!(
                        "Max validation failed on field 'type_name'. {} is greater than 204",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.type_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 10 as _ {
                    return Err(format!(
                        "Min validation failed on field 'type_name'. {} is less than 10",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
