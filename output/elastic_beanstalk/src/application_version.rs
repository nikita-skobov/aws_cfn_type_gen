/// The AWS::ElasticBeanstalk::ApplicationVersion resource is an AWS Elastic Beanstalk    resource type that specifies an application version, an iteration of deployable code, for an    Elastic Beanstalk application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApplicationVersion {
    ///
    /// The name of the Elastic Beanstalk application that is associated with this application    version.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: cfn_resources::StrVal,

    ///
    /// A description of this application version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon S3 bucket and key that identify the location of the source bundle for this    version.
    ///
    /// NoteThe Amazon S3 bucket must be in the same region as the    environment.
    ///
    /// Required: Yes
    ///
    /// Type: SourceBundle
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceBundle")]
    pub source_bundle: SourceBundle,

    #[serde(skip_serializing)]
    pub att_id: CfnApplicationVersionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApplicationVersionid;
impl CfnApplicationVersionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnApplicationVersion {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticBeanstalk::ApplicationVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'application_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'application_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 200",
                        s.len()
                    ));
                }
            }
        }

        self.source_bundle.validate()?;

        Ok(())
    }
}

/// The SourceBundle property is an embedded property of the AWS::ElasticBeanstalk::ApplicationVersion resource. It specifies the Amazon S3     location of the source bundle for an AWS Elastic Beanstalk application version.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceBundle {
    ///
    /// The Amazon S3 bucket where the data is located.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: cfn_resources::StrVal,

    ///
    /// The Amazon S3 key where the data is located.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Key")]
    pub s3_key: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SourceBundle {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.s3_bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_bucket'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.s3_key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_key'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
