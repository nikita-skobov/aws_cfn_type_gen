/// The AWS::AppConfig::Environment resource creates an environment, which is a    logical deployment group of AWS AppConfig targets, such as applications in a     Beta or Production environment. You define one or more    environments for each AWS AppConfig application. You can also define environments for    application subcomponents such as the Web, Mobile and     Back-end components for your application. You can configure Amazon CloudWatch alarms for each environment. The system monitors alarms during a    configuration deployment. If an alarm is triggered, the system rolls back the    configuration.
///
/// AWS AppConfig requires that you create resources and deploy a configuration in the    following order:
///
/// For more information, see AWS AppConfig in the      AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEnvironment {
    ///
    /// The application ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationId")]
    pub application_id: cfn_resources::StrVal,

    ///
    /// A description of the environment.
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
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Amazon CloudWatch alarms to monitor during the deployment process.
    ///
    /// Required: No
    ///
    /// Type: List of Monitors
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Monitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitors>>,

    ///
    /// A name for the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Metadata to assign to the environment. Tags help organize and categorize your AWS AppConfig resources. Each tag consists of a key and an optional value, both of which     you define.
    ///
    /// Required: No
    ///
    /// Type: List of Tags
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tags>>,
}

impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string(&self) -> &'static str {
        "AWS::AppConfig::Environment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.monitors {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'monitors'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
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

        Ok(())
    }
}

/// Amazon CloudWatch alarms to monitor during the deployment process.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Monitors {
    ///
    /// Amazon Resource Name (ARN) of the Amazon CloudWatch alarm.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<cfn_resources::StrVal>,

    ///
    /// ARN of an AWS Identity and Access Management (IAM) role for AWS AppConfig to monitor       AlarmArn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^((arn):(aws|aws-cn|aws-iso|aws-iso-[a-z]{1}|aws-us-gov):(iam)::\d{12}:role[/].*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Monitors {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.alarm_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'alarm_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'alarm_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'alarm_role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'alarm_role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Metadata to assign to the environment. Tags help organize and categorize your AWS AppConfig resources. Each tag consists of a key and an optional value, both of which     you define.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tags {
    ///
    /// The key-value string map. The valid character set is [a-zA-Z+-=._:/]. The tag    key can be up to 128 characters and must not start with aws:.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The tag value can be up to 256 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Tags {
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
