/// The AWS::RoboMaker::SimulationApplicationVersion resource creates a version     of an AWS RoboMaker simulation application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSimulationApplicationVersion {
    ///
    /// The application information for the simulation application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: cfn_resources::StrVal,

    ///
    /// The current revision id for the simulation application. If you provide a value and it     matches the latest revision ID, a new version will be created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40
    ///
    /// Pattern: [a-zA-Z0-9_.\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CurrentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_application_version: CfnSimulationApplicationVersionapplicationversion,

    #[serde(skip_serializing)]
    pub att_arn: CfnSimulationApplicationVersionarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimulationApplicationVersionapplicationversion;
impl CfnSimulationApplicationVersionapplicationversion {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationVersion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimulationApplicationVersionarn;
impl CfnSimulationApplicationVersionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnSimulationApplicationVersion {
    fn type_string(&self) -> &'static str {
        "AWS::RoboMaker::SimulationApplicationVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.application;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1224 as _ {
                return Err(format!(
                    "Max validation failed on field 'application'. {} is greater than 1224",
                    s.len()
                ));
            }
        }

        let the_val = &self.application;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'application'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.current_revision_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 40 as _ {
                    return Err(format!("Max validation failed on field 'current_revision_id'. {} is greater than 40", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.current_revision_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'current_revision_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
