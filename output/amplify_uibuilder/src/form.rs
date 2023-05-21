

/// The AWS::AmplifyUIBuilder::Form resource specifies all of the information that is required to create a form.
#[derive(Default, serde::Serialize)]
pub struct CfnForm {


    /// 
    /// The schema version of the form.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,


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
    /// The name of the backend environment that is a part of the Amplify app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,


    /// 
    /// The name of the form.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// One or more key-value pairs to use when tagging the form data.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The FormCTA object that stores the call to action configuration for the    form.
    /// 
    /// Required: No
    ///
    /// Type: FormCTA
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cta")]
    pub cta: Option<FormCTA>,


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


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelDecorator")]
    pub label_decorator: Option<String>,


    /// 
    /// Specifies whether to perform a create or update action on the form.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormActionType")]
    pub form_action_type: String,


    /// 
    /// The unique ID of the Amplify app associated with the form.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,


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
    /// The configuration information for the form's fields.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of FieldConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fields")]
    pub fields: std::collections::HashMap<String, FieldConfig>,

}


/// The FieldInputConfig property specifies the configuration for the default input values to display for a field.
#[derive(Default, serde::Serialize)]
pub struct FieldInputConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FileUploaderFieldConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileUploaderConfig")]
    pub file_uploader_config: Option<FileUploaderFieldConfig>,


    /// 
    /// The text to display as a placeholder for the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Placeholder")]
    pub placeholder: Option<String>,


    /// 
    /// The stepping increment for a numeric value in a field.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Step")]
    pub step: Option<f64>,


    /// 
    /// Specifies whether a field has a default value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultChecked")]
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
    pub default_country_code: Option<String>,


    /// 
    /// The minimum value to display for the field.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinValue")]
    pub min_value: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsArray")]
    pub is_array: Option<bool>,


    /// 
    /// The text to display to describe the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DescriptiveText")]
    pub descriptive_text: Option<String>,


    /// 
    /// The information to use to customize the input fields with data at runtime.
    /// 
    /// Required: No
    ///
    /// Type: ValueMappings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueMappings")]
    pub value_mappings: Option<ValueMappings>,


    /// 
    /// The input type for the field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The value for the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The name of the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Specifies a read only field.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnly")]
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
    pub required: Option<bool>,


    /// 
    /// The maximum value to display for the field.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxValue")]
    pub max_value: Option<f64>,


    /// 
    /// The default value for the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}


/// The FormButton property specifies the configuration for a button UI element that is a part of a form.
#[derive(Default, serde::Serialize)]
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
    pub children: Option<String>,


    /// 
    /// The position of the button.
    /// 
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,


    /// 
    /// Specifies whether the button is visible on the form.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,

}


/// The FieldValidationConfiguration property specifies the validation configuration for a field.
#[derive(Default, serde::Serialize)]
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
    pub str_values: Option<Vec<String>>,


    /// 
    /// The validation message to display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationMessage")]
    pub validation_message: Option<String>,


    /// 
    /// The validation to perform on an object type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// The FieldConfig property specifies the configuration information for a field in a table.
#[derive(Default, serde::Serialize)]
pub struct FieldConfig {


    /// 
    /// The validations to perform on the value in the field.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldValidationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validations")]
    pub validations: Option<Vec<FieldValidationConfiguration>>,


    /// 
    /// Specifies the field position.
    /// 
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,


    /// 
    /// The label for the field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,


    /// 
    /// Describes the configuration for the default input value to display for a field.
    /// 
    /// Required: No
    ///
    /// Type: FieldInputConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputType")]
    pub input_type: Option<FieldInputConfig>,


    /// 
    /// Specifies whether to hide a field.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,

}


/// The ValueMappings property specifies the data binding configuration for a value map.
#[derive(Default, serde::Serialize)]
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


/// The FormStyle property specifies the configuration for the form's style.
#[derive(Default, serde::Serialize)]
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
    pub horizontal_gap: Option<FormStyleConfig>,


    /// 
    /// The spacing for the vertical gap.
    /// 
    /// Required: No
    ///
    /// Type: FormStyleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalGap")]
    pub vertical_gap: Option<FormStyleConfig>,


    /// 
    /// The size of the outer padding for the form.
    /// 
    /// Required: No
    ///
    /// Type: FormStyleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OuterPadding")]
    pub outer_padding: Option<FormStyleConfig>,

}


/// The SectionalElement property specifies the configuration information for a visual helper element for a form. A sectional    element can be a header, a text block, or a divider. These elements are static and not    associated with any data.
#[derive(Default, serde::Serialize)]
pub struct SectionalElement {


    /// 
    /// The type of sectional element. Valid values are Heading, Text,    and Divider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Specifies the size of the font for a Heading sectional element. Valid values    are 1 | 2 | 3 | 4 | 5 | 6.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Level")]
    pub level: Option<f64>,


    /// 
    /// Specifies the position of the text in a field for a Text sectional    element.
    /// 
    /// Required: No
    ///
    /// Type: FieldPosition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,


    /// 
    /// Specifies the orientation for a Divider sectional element. Valid values are     horizontal or vertical.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Orientation")]
    pub orientation: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,


    /// 
    /// The text for a Text sectional element.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    pub text: Option<String>,

}


/// The FormCTA property specifies the call to action button configuration for the form.
#[derive(Default, serde::Serialize)]
pub struct FormCTA {


    /// 
    /// Displays a clear button.
    /// 
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Clear")]
    pub clear: Option<FormButton>,


    /// 
    /// Displays a cancel button.
    /// 
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cancel")]
    pub cancel: Option<FormButton>,


    /// 
    /// The position of the button.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<String>,


    /// 
    /// Displays a submit button.
    /// 
    /// Required: No
    ///
    /// Type: FormButton
    ///
    /// Update requires: No interruption
    #[serde(rename = "Submit")]
    pub submit: Option<FormButton>,

}


/// The ValueMapping property specifies the association between a complex object and a display value. Use ValueMapping to store    how to represent complex objects when they are displayed.
#[derive(Default, serde::Serialize)]
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


/// The FieldPosition property specifies the field position.
#[derive(Default, serde::Serialize)]
pub struct FieldPosition {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fixed")]
    pub fixed: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightOf")]
    pub right_of: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Below")]
    pub below: Option<String>,

}


/// The FormInputValueProperty property specifies the configuration for an input field on a form. Use    FormInputValueProperty to specify the values to render or bind by    default.
#[derive(Default, serde::Serialize)]
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
    pub value: Option<String>,

}


/// The FormStyleConfig property specifies the configuration settings for the form's style properties.
#[derive(Default, serde::Serialize)]
pub struct FormStyleConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenReference")]
    pub token_reference: Option<String>,

}


/// The FileUploaderFieldConfig property type specifies Property description not available. for an AWS::AmplifyUIBuilder::Form.
#[derive(Default, serde::Serialize)]
pub struct FileUploaderFieldConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShowThumbnails")]
    pub show_thumbnails: Option<bool>,


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
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsResumable")]
    pub is_resumable: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileCount")]
    pub max_file_count: Option<f64>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLevel")]
    pub access_level: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: Option<f64>,

}


/// The FormDataTypeConfig property specifies the data type configuration for the data source associated with a form.
#[derive(Default, serde::Serialize)]
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
    pub data_source_type: String,


    /// 
    /// The unique name of the data type you are using as the data source for the form.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTypeName")]
    pub data_type_name: String,

}
