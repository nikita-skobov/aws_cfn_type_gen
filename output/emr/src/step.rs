/// Use Step to specify a cluster (job flow) step, which runs only on the master node. Steps are used to submit data processing jobs to a cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnStep {
    ///
    /// This specifies what action to take when the cluster step fails. Possible values are CANCEL_AND_WAIT and CONTINUE.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: cfn_resources::StrVal,

    ///
    /// The HadoopJarStepConfig property type specifies a job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for the cluster to execute as a step on the master node, and then waits for the job to finish or fail before executing subsequent steps.
    ///
    /// Required: Yes
    ///
    /// Type: HadoopJarStepConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,

    ///
    /// A string that uniquely identifies the cluster (job flow).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: cfn_resources::StrVal,

    ///
    /// The name of the cluster step.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnStep {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::Step"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hadoop_jar_step.validate()?;

        let the_val = &self.job_flow_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'job_flow_id'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.job_flow_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'job_flow_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A job flow step consisting of a JAR file whose main function will be executed. The main     function submits a job for Hadoop to execute and waits for the job to finish or     fail.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HadoopJarStepConfig {
    ///
    /// A list of command line arguments passed to the JAR file's main function when     executed.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    ///
    /// A path to a JAR file run during the step.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Jar")]
    pub jar: cfn_resources::StrVal,

    ///
    /// The name of the main class in the specified Java file. If not specified, the JAR file     should specify a Main-Class in its manifest file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "MainClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<cfn_resources::StrVal>,

    ///
    /// A list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValue
    ///
    /// Update requires: Replacement
    #[serde(rename = "StepProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_properties: Option<Vec<KeyValue>>,
}

impl cfn_resources::CfnResource for HadoopJarStepConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.jar;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10280 as _ {
                return Err(format!(
                    "Max validation failed on field 'jar'. {} is greater than 10280",
                    s.len()
                ));
            }
        }

        let the_val = &self.jar;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'jar'. {} is less than 0",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.main_class {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'main_class'. {} is greater than 10280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.main_class {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'main_class'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// KeyValue is a subproperty of the HadoopJarStepConfig property type. KeyValue is used to pass parameters to a step.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct KeyValue {
    ///
    /// The unique identifier of a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The value part of the identified key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for KeyValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 10280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'value'. {} is greater than 10280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'value'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
