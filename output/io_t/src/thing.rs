/// Use the AWS::IoT::Thing resource to declare an AWS IoT thing.
///
/// For information about working with things, see How AWS IoT Works and       Device       Registry for AWS IoT in the AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnThing {
    ///
    /// A string that contains up to three key value pairs. Maximum length of 800. Duplicates     not allowed.
    ///
    /// Required: No
    ///
    /// Type: AttributePayload
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,

    ///
    /// The name of the thing to update.
    ///
    /// You can't change a thing's name. To change a thing's name, you must create a 			new thing, give it the new name, and then delete the old thing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnThingarn,

    #[serde(skip_serializing)]
    pub att_id: CfnThingid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingarn;
impl CfnThingarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingid;
impl CfnThingid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnThing {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::Thing"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.attribute_payload
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AttributePayload property specifies up to three attributes for an AWS IoT as     key-value pairs. AttributePayload is a property of the AWS::IoT::Thing resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AttributePayload {
    ///
    /// A JSON string containing up to three key-value pair in JSON format. For example:
    ///
    /// {\"attributes\":{\"string1\":\"string2\"}}
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for AttributePayload {
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
