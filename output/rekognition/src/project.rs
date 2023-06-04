/// The AWS::Rekognition::Project type creates an Amazon Rekognition Custom Labels     project. A project is a group of resources needed to create and manage versions of an      Amazon Rekognition Custom Labels model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProject {
    ///
    /// The name of the project to create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnProjectarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectarn;
impl CfnProjectarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::Rekognition::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.project_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'project_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.project_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'project_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
