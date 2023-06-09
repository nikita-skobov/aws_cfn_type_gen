/// The AWS::MediaConvert::Preset resource is an AWS Elemental MediaConvert resource type       that you can use to specify encoding settings for a single output in a transcoding       job.
///
/// When you declare this entity in your AWS CloudFormation template, you pass in your       transcoding job settings in JSON or YAML format. This settings specification must be       formed in a particular way that conforms to AWS Elemental MediaConvert job validation. For       more information about creating an output preset model for the SettingsJson       property, see the Remarks section later in this topic.
///
/// For more information about output MediaConvert presets, see Working       with AWS Elemental MediaConvert Output Presets in the AWS Elemental MediaConvert User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPreset {
    ///
    /// The new category for the preset, if you are changing it.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<cfn_resources::StrVal>,

    ///
    /// The new description for the preset, if you are changing it.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the preset that you are       modifying.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specify, in JSON format, the transcoding job settings for this output preset. This       specification must conform to the AWS Elemental MediaConvert job validation. For       information about forming this specification, see the Remarks section later in this       topic.
    ///
    /// For more information about MediaConvert output presets, see Working       with AWS Elemental MediaConvert Output Presets in the AWS Elemental MediaConvert User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SettingsJson")]
    pub settings_json: serde_json::Value,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnPresetarn,

    #[serde(skip_serializing)]
    pub att_name: CfnPresetname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPresetarn;
impl CfnPresetarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPresetname;
impl CfnPresetname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnPreset {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConvert::Preset"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
