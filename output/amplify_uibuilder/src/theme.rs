

/// The AWS::AmplifyUIBuilder::Theme resource specifies a theme within an Amplify app. A theme    is a collection of style settings that apply globally to the components associated with the    app.
#[derive(Default, serde::Serialize)]
pub struct CfnTheme {


    /// 
    /// Describes the properties that can be overriden to customize a theme.
    /// 
    /// Required: No
    ///
    /// Type: List of ThemeValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<ThemeValues>>,


    /// 
    /// The name of the theme.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,


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


    /// 
    /// One or more key-value pairs to use when tagging the theme.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

}


/// The ThemeValues property specifies key-value pair that defines a property of a theme.
#[derive(Default, serde::Serialize)]
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
    pub key: Option<String>,


    /// 
    /// The value of the property.
    /// 
    /// Required: No
    ///
    /// Type: ThemeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<ThemeValue>,

}


/// The ThemeValue property specifies the configuration of a theme's    properties.
#[derive(Default, serde::Serialize)]
pub struct ThemeValue {


    /// 
    /// The value of a theme property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// A list of key-value pairs that define the theme's properties.
    /// 
    /// Required: No
    ///
    /// Type: List of ThemeValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "Children")]
    pub children: Option<Vec<ThemeValues>>,

}
