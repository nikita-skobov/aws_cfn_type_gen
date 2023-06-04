/// Registers the specified version of the module with the CloudFormation service. Registering a module  makes it available for use in CloudFormation templates in your AWS account and  Region.
///
/// To specify a module version as the default version, use the AWS::CloudFormation::ModuleDefaultVersion resource.
///
/// For more information using modules, see Using modules to encapsulate and reuse resource   configurations and Registering extensions in the   CloudFormation User Guide. For information on developing modules, see Developing modules in the   CloudFormation CLI User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersion {
    ///
    /// The name of the module being registered.
    ///
    /// Required: Yes
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
    #[serde(rename = "ModuleName")]
    pub module_name: cfn_resources::StrVal,

    ///
    /// A URL to the S3 bucket containing the package that contains the template fragment and schema files for the  module version to register.
    ///
    /// NoteThe user registering the module version must be able to access the module package in the S3 bucket. That's, the   user needs to have GetObject   permissions for the package. For more information, see Actions, Resources, and Condition Keys for Amazon S3 in the    AWS Identity and Access Management User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModulePackage")]
    pub module_package: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnModuleVersionarn,

    #[serde(skip_serializing)]
    pub att_description: CfnModuleVersiondescription,

    #[serde(skip_serializing)]
    pub att_documentation_url: CfnModuleVersiondocumentationurl,

    #[serde(skip_serializing)]
    pub att_schema: CfnModuleVersionschema,

    #[serde(skip_serializing)]
    pub att_time_created: CfnModuleVersiontimecreated,

    #[serde(skip_serializing)]
    pub att_version_id: CfnModuleVersionversionid,

    #[serde(skip_serializing)]
    pub att_visibility: CfnModuleVersionvisibility,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersionarn;
impl CfnModuleVersionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersiondescription;
impl CfnModuleVersiondescription {
    pub fn att_name(&self) -> &'static str {
        r#"Description"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersiondocumentationurl;
impl CfnModuleVersiondocumentationurl {
    pub fn att_name(&self) -> &'static str {
        r#"DocumentationUrl"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersionschema;
impl CfnModuleVersionschema {
    pub fn att_name(&self) -> &'static str {
        r#"Schema"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersiontimecreated;
impl CfnModuleVersiontimecreated {
    pub fn att_name(&self) -> &'static str {
        r#"TimeCreated"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersionversionid;
impl CfnModuleVersionversionid {
    pub fn att_name(&self) -> &'static str {
        r#"VersionId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModuleVersionvisibility;
impl CfnModuleVersionvisibility {
    pub fn att_name(&self) -> &'static str {
        r#"Visibility"#
    }
}

impl cfn_resources::CfnResource for CfnModuleVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::ModuleVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.module_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 204 as _ {
                return Err(format!(
                    "Max validation failed on field 'module_name'. {} is greater than 204",
                    s.len()
                ));
            }
        }

        let the_val = &self.module_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'module_name'. {} is less than 10",
                    s.len()
                ));
            }
        }

        let the_val = &self.module_package;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'module_package'. {} is greater than 4096",
                    s.len()
                ));
            }
        }

        let the_val = &self.module_package;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'module_package'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
