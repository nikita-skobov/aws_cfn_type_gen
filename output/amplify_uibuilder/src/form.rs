/// The AWS::AmplifyUIBuilder::Form resource specifies all of the information that is required to create a form.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnForm {
    ///
    /// The unique ID of the Amplify app associated with the form.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<cfn_resources::StrVal>,

    ///
    /// The FormCTA object that stores the call to action configuration for the    form.
    ///
    /// Required: No
    ///
    /// Type: FormCTA
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<FormCTA>,

    ///
    /// The type of data source to use to create the form.
    ///
    /// Required: Yes
    ///
    /// Type: FormDataTypeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: FormDataTypeConfig,

    ///
    /// The name of the backend environment that is a part of the Amplify app.
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
    /// The configuration information for the form's fields.
    ///
    /// Required: Yes
    ///
    /// Type: Map of FieldConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fields")]
    pub fields: std::collections::HashMap<String, FieldConfig>,

    ///
    /// Specifies whether to perform a create or update action on the form.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormActionType")]
    pub form_action_type: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelDecorator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_decorator: Option<cfn_resources::StrVal>,

    ///
    /// The name of the form.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The schema version of the form.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersion")]
    pub schema_version: cfn_resources::StrVal,

    ///
    /// The configuration information for the visual helper elements for the form. These elements    are not associated with any data.
    ///
    /// Required: Yes
    ///
    /// Type: Map of SectionalElement
    ///
    /// Update requires: No interruption
    #[serde(rename = "SectionalElements")]
    pub sectional_elements: std::collections::HashMap<String, SectionalElement>,

    ///
    /// The configuration for the form's style.
    ///
    /// Required: Yes
    ///
    /// Type: FormStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Style")]
    pub style: FormStyle,

    ///
    /// One or more key-value pairs to use when tagging the form data.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_id: CfnFormid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFormid;
impl CfnFormid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnForm {
    fn type_string(&self) -> &'static str {
        "AWS::AmplifyUIBuilder::Form"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cta.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.data_type.validate()?;

        self.style.validate()?;

        Ok(())
    }
}

/// The FieldConfig property specifies the configuration information for a field in a table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldConfig {
    ///
    /// Specifies whether to hide a field.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,

    ///
    /// Describes the configuration for the default input value to display for a field.
    ///
    /// Required: No
    ///
    /// Type: FieldInputConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<FieldInputConfig>,

    ///
    /// The label for the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the field position.
    ///
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,

    ///
    /// The validations to perform on the value in the field.
    ///
    /// Required: No
    ///
    /// Type: List of FieldValidationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<FieldValidationConfiguration>>,
}

impl cfn_resources::CfnResource for FieldConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.position
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FieldInputConfig property specifies the configuration for the default input values to display for a field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldInputConfig {
    ///
    /// Specifies whether a field has a default value.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_checked: Option<bool>,

    ///
    /// The default country code for a phone number.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_country_code: Option<cfn_resources::StrVal>,

    ///
    /// The default value for the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<cfn_resources::StrVal>,

    ///
    /// The text to display to describe the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DescriptiveText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_text: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FileUploaderFieldConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileUploaderConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_uploader_config: Option<FileUploaderFieldConfig>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsArray")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_array: Option<bool>,

    ///
    /// The maximum value to display for the field.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,

    ///
    /// The minimum value to display for the field.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,

    ///
    /// The name of the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The text to display as a placeholder for the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Placeholder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<cfn_resources::StrVal>,

    ///
    /// Specifies a read only field.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    ///
    /// Specifies a field that requires input.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    ///
    /// The stepping increment for a numeric value in a field.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,

    ///
    /// The input type for the field.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,

    ///
    /// The value for the field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,

    ///
    /// The information to use to customize the input fields with data at runtime.
    ///
    /// Required: No
    ///
    /// Type: ValueMappings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_mappings: Option<ValueMappings>,
}

impl cfn_resources::CfnResource for FieldInputConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file_uploader_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.value_mappings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FieldPosition property specifies the field position.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldPosition {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Below")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_of: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FieldPosition {
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

/// The FieldValidationConfiguration property specifies the validation configuration for a field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldValidationConfiguration {
    ///
    /// The validation to perform on a number value.
    ///
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_values: Option<Vec<f64>>,

    ///
    /// The validation to perform on a string value.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StrValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_values: Option<Vec<String>>,

    ///
    /// The validation to perform on an object type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,

    ///
    /// The validation message to display.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FieldValidationConfiguration {
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

/// The FileUploaderFieldConfig property type specifies Property description not available. for an AWS::AmplifyUIBuilder::Form.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileUploaderFieldConfig {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptedFileTypes")]
    pub accepted_file_types: Vec<String>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLevel")]
    pub access_level: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsResumable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resumable: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_count: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShowThumbnails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_thumbnails: Option<bool>,
}

impl cfn_resources::CfnResource for FileUploaderFieldConfig {
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

/// The FormButton property specifies the configuration for a button UI element that is a part of a form.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormButton {
    ///
    /// Describes the button's properties.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the button is visible on the form.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,

    ///
    /// The position of the button.
    ///
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,
}

impl cfn_resources::CfnResource for FormButton {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.position
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FormCTA property specifies the call to action button configuration for the form.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormCTA {
    ///
    /// Displays a cancel button.
    ///
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<FormButton>,

    ///
    /// Displays a clear button.
    ///
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Clear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<FormButton>,

    ///
    /// The position of the button.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<cfn_resources::StrVal>,

    ///
    /// Displays a submit button.
    ///
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Submit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<FormButton>,
}

impl cfn_resources::CfnResource for FormCTA {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cancel.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.clear.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.submit.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FormDataTypeConfig property specifies the data type configuration for the data source associated with a form.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormDataTypeConfig {
    ///
    /// The data source type, either an Amplify DataStore model or a custom data type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceType")]
    pub data_source_type: cfn_resources::StrVal,

    ///
    /// The unique name of the data type you are using as the data source for the form.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTypeName")]
    pub data_type_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for FormDataTypeConfig {
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

/// The FormInputValueProperty property specifies the configuration for an input field on a form. Use    FormInputValueProperty to specify the values to render or bind by    default.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormInputValueProperty {
    ///
    /// The value to assign to the input field.
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

impl cfn_resources::CfnResource for FormInputValueProperty {
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

/// The FormStyle property specifies the configuration for the form's style.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormStyle {
    ///
    /// The spacing for the horizontal gap.
    ///
    /// Required: No
    ///
    /// Type: FormStyleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HorizontalGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_gap: Option<FormStyleConfig>,

    ///
    /// The size of the outer padding for the form.
    ///
    /// Required: No
    ///
    /// Type: FormStyleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OuterPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_padding: Option<FormStyleConfig>,

    ///
    /// The spacing for the vertical gap.
    ///
    /// Required: No
    ///
    /// Type: FormStyleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_gap: Option<FormStyleConfig>,
}

impl cfn_resources::CfnResource for FormStyle {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.horizontal_gap
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.outer_padding
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vertical_gap
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FormStyleConfig property specifies the configuration settings for the form's style properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormStyleConfig {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_reference: Option<cfn_resources::StrVal>,

    /// Property description not available.
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

impl cfn_resources::CfnResource for FormStyleConfig {
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

/// The SectionalElement property specifies the configuration information for a visual helper element for a form. A sectional    element can be a header, a text block, or a divider. These elements are static and not    associated with any data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SectionalElement {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,

    ///
    /// Specifies the size of the font for a Heading sectional element. Valid values    are 1 | 2 | 3 | 4 | 5 | 6.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,

    ///
    /// Specifies the orientation for a Divider sectional element. Valid values are     horizontal or vertical.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Orientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the position of the text in a field for a Text sectional    element.
    ///
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,

    ///
    /// The text for a Text sectional element.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<cfn_resources::StrVal>,

    ///
    /// The type of sectional element. Valid values are Heading, Text,    and Divider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SectionalElement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.position
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The ValueMapping property specifies the association between a complex object and a display value. Use ValueMapping to store    how to represent complex objects when they are displayed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValueMapping {
    ///
    /// The value to display for the complex object.
    ///
    /// Required: No
    ///
    /// Type: FormInputValueProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<FormInputValueProperty>,

    ///
    /// The complex object.
    ///
    /// Required: Yes
    ///
    /// Type: FormInputValueProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: FormInputValueProperty,
}

impl cfn_resources::CfnResource for ValueMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.display_value
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.value.validate()?;

        Ok(())
    }
}

/// The ValueMappings property specifies the data binding configuration for a value map.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValueMappings {
    ///
    /// The value and display value pairs.
    ///
    /// Required: Yes
    ///
    /// Type: List of ValueMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<ValueMapping>,
}

impl cfn_resources::CfnResource for ValueMappings {
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
