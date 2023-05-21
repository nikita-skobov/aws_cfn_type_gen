/// The AWS::AmplifyUIBuilder::Theme resource specifies a theme within an Amplify app. A theme    is a collection of style settings that apply globally to the components associated with the    app.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTheme {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the theme.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Describes the properties that can be overriden to customize a theme.
    ///
    /// Required: No
    ///
    /// Type: List of ThemeValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ThemeValues>>,

    ///
    /// One or more key-value pairs to use when tagging the theme.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// A list of key-value pairs that defines the properties of the theme.
    ///
    /// Required: Yes
    ///
    /// Type: List of ThemeValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<ThemeValues>,
}

impl cfn_resources::CfnResource for CfnTheme {
    fn type_string(&self) -> &'static str {
        "AWS::AmplifyUIBuilder::Theme"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The ThemeValue property specifies the configuration of a theme's    properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ThemeValue {
    ///
    /// A list of key-value pairs that define the theme's properties.
    ///
    /// Required: No
    ///
    /// Type: List of ThemeValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ThemeValues>>,

    ///
    /// The value of a theme property.
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

impl cfn_resources::CfnResource for ThemeValue {
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

/// The ThemeValues property specifies key-value pair that defines a property of a theme.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ThemeValues {
    ///
    /// The name of the property.
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
    /// The value of the property.
    ///
    /// Required: No
    ///
    /// Type: ThemeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ThemeValue>,
}

impl cfn_resources::CfnResource for ThemeValues {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.value.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
