/// The AWS::IoTEvents::Input resource creates an input. To monitor your devices and processes,    they must have a way to get telemetry data into AWS IoT Events. This is done by sending messages    as inputs to AWS IoT Events. For more information, see       How to Use AWS IoT Events in the AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnInput {
    ///
    /// The definition of the input.
    ///
    /// Required: Yes
    ///
    /// Type: InputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputDefinition")]
    pub input_definition: InputDefinition,

    ///
    /// A brief description of the input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_name: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnInput {
    fn type_string(&self) -> &'static str {
        "AWS::IoTEvents::Input"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input_definition.validate()?;

        if let Some(the_val) = &self.input_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'input_description'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.input_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'input_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.input_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'input_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The attributes from the JSON payload that are made available by the input. Inputs are    derived from messages sent to the AWS IoT Events system using BatchPutMessage. Each such    message contains a JSON payload. Those attributes (and their paired values) specified here are    available for use in the condition expressions used by detectors.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Attribute {
    ///
    /// An expression that specifies an attribute-value pair in a JSON structure. Use this to    specify an attribute from the JSON payload that is made available by the input. Inputs are    derived from messages sent to AWS IoT Events (BatchPutMessage). Each such message contains    a JSON payload. The attribute (and its paired value) specified here are available for use in    the condition expressions used by detectors.
    ///
    /// Syntax: <field-name>.<field-name>...
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^((`[\w\- ]+`)|([\w\-]+))(\.((`[\w- ]+`)|([\w\-]+)))*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonPath")]
    pub json_path: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Attribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.json_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'json_path'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.json_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'json_path'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The definition of the input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputDefinition {
    ///
    /// The attributes from the JSON payload that are made available by the input. Inputs are    derived from messages sent to the AWS IoT Events system using BatchPutMessage. Each such    message contains a JSON payload, and those attributes (and their paired values) specified here    are available for use in the condition expressions used by detectors that monitor    this input.
    ///
    /// Required: Yes
    ///
    /// Type: List of Attribute
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,
}

impl cfn_resources::CfnResource for InputDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.attributes;

        if the_val.len() > 200 as _ {
            return Err(format!(
                "Max validation failed on field 'attributes'. {} is greater than 200",
                the_val.len()
            ));
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
