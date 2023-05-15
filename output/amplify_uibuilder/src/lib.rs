
pub mod cfn_component {

#[derive(serde::Serialize, Default)]
pub struct CfnComponent {
    /// No documentation provided by AWS
    #[serde(rename = "Events")]
    pub events: Option<ComponentEvents>,
    /// No documentation provided by AWS
    #[serde(rename = "BindingProperties")]
    pub binding_properties: ComponentBindingProperties,
    /// No documentation provided by AWS
    #[serde(rename = "SourceId")]
    pub source_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Properties")]
    pub properties: ComponentProperties,
    /// No documentation provided by AWS
    #[serde(rename = "CollectionProperties")]
    pub collection_properties: Option<ComponentCollectionProperties>,
    /// List of ComponentChild
    #[serde(rename = "Children")]
    pub children: Option<Vec<ComponentChild>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SchemaVersion")]
    pub schema_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Overrides")]
    pub overrides: ComponentOverrides,
    /// No documentation provided by AWS
    #[serde(rename = "ComponentType")]
    pub component_type: String,
    /// List of ComponentVariant
    #[serde(rename = "Variants")]
    pub variants: Vec<ComponentVariant>,
    /// No documentation provided by AWS
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct FormBindings {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentOverrides {

}

#[derive(serde::Serialize, Default)]
pub struct MutationActionSetStateParameter {
    #[serde(rename = "Set")]
    pub set: ComponentProperty,
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    #[serde(rename = "Property")]
    pub property: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentBindingPropertiesValue {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "BindingProperties")]
    pub binding_properties: Option<ComponentBindingPropertiesValueProperties>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentConditionProperty {
    #[serde(rename = "Then")]
    pub then: Option<Box<ComponentProperty>>,
    #[serde(rename = "Else")]
    pub els: Option<Box<ComponentProperty>>,
    #[serde(rename = "Operand")]
    pub operand: Option<String>,
    #[serde(rename = "Property")]
    pub property: Option<String>,
    #[serde(rename = "Operator")]
    pub operator: Option<String>,
    #[serde(rename = "OperandType")]
    pub operand_type: Option<String>,
    #[serde(rename = "Field")]
    pub field: Option<String>,

}
pub type SortDirection = String;
#[derive(serde::Serialize, Default)]
pub struct ComponentProperties {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentVariant {
    #[serde(rename = "VariantValues")]
    pub variant_values: Option<ComponentVariantValues>,
    #[serde(rename = "Overrides")]
    pub overrides: Option<ComponentOverrides>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentEvent {
    #[serde(rename = "Action")]
    pub action: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<ActionParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentPropertyBindingProperties {
    #[serde(rename = "Field")]
    pub field: Option<String>,
    #[serde(rename = "Property")]
    pub property: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentEvents {

}

#[derive(serde::Serialize, Default)]
pub struct Predicate {
    #[serde(rename = "Operand")]
    pub operand: Option<String>,
    #[serde(rename = "Or")]
    pub or: Option<Vec<Predicate>>,
    #[serde(rename = "And")]
    pub and: Option<Vec<Predicate>>,
    #[serde(rename = "Operator")]
    pub operator: Option<String>,
    #[serde(rename = "Field")]
    pub field: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ActionParameters {
    #[serde(rename = "Id")]
    pub id: Option<ComponentProperty>,
    #[serde(rename = "Url")]
    pub url: Option<ComponentProperty>,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "Anchor")]
    pub anchor: Option<ComponentProperty>,
    #[serde(rename = "Target")]
    pub target: Option<ComponentProperty>,
    #[serde(rename = "Global")]
    pub global: Option<ComponentProperty>,
    #[serde(rename = "Type")]
    pub ty: Option<ComponentProperty>,
    #[serde(rename = "Fields")]
    pub fields: Option<ComponentProperties>,
    #[serde(rename = "State")]
    pub state: Option<MutationActionSetStateParameter>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentCollectionProperties {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentProperty {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "ComponentName")]
    pub component_name: Option<String>,
    #[serde(rename = "Concat")]
    pub concat: Option<Vec<ComponentProperty>>,
    #[serde(rename = "Condition")]
    pub condition: Option<Box<ComponentConditionProperty>>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "Event")]
    pub event: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "BindingProperties")]
    pub binding_properties: Option<ComponentPropertyBindingProperties>,
    #[serde(rename = "UserAttribute")]
    pub user_attribute: Option<String>,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "Configured")]
    pub configured: Option<bool>,
    #[serde(rename = "CollectionBindingProperties")]
    pub collection_binding_properties: Option<ComponentPropertyBindingProperties>,
    #[serde(rename = "Property")]
    pub property: Option<String>,
    #[serde(rename = "Bindings")]
    pub bindings: Option<FormBindings>,
    #[serde(rename = "ImportedValue")]
    pub imported_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentDataConfiguration {
    #[serde(rename = "Identifiers")]
    pub identifiers: Option<Vec<String>>,
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Predicate")]
    pub predicate: Option<Predicate>,
    #[serde(rename = "Sort")]
    pub sort: Option<Vec<SortProperty>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentBindingProperties {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentVariantValues {

}

#[derive(serde::Serialize, Default)]
pub struct SortProperty {
    #[serde(rename = "Field")]
    pub field: String,
    #[serde(rename = "Direction")]
    pub direction: SortDirection,

}

#[derive(serde::Serialize, Default)]
pub struct FormBindingElement {
    #[serde(rename = "Property")]
    pub property: String,
    #[serde(rename = "Element")]
    pub element: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentBindingPropertiesValueProperties {
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "Field")]
    pub field: Option<String>,
    #[serde(rename = "Predicates")]
    pub predicates: Option<Vec<Predicate>>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "UserAttribute")]
    pub user_attribute: Option<String>,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentOverridesValue {

}

#[derive(serde::Serialize, Default)]
pub struct ComponentChild {
    #[serde(rename = "Events")]
    pub events: Option<ComponentEvents>,
    #[serde(rename = "ComponentType")]
    pub component_type: String,
    #[serde(rename = "Properties")]
    pub properties: ComponentProperties,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Children")]
    pub children: Option<Vec<ComponentChild>>,

}


}

pub mod cfn_form {

#[derive(serde::Serialize, Default)]
pub struct CfnForm {
    /// No documentation provided by AWS
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// No documentation provided by AWS
    #[serde(rename = "Style")]
    pub style: FormStyle,
    /// No documentation provided by AWS
    #[serde(rename = "DataType")]
    pub data_type: FormDataTypeConfig,
    /// No documentation provided by AWS
    #[serde(rename = "SectionalElements")]
    pub sectional_elements: SectionalElementMap,
    /// No documentation provided by AWS
    #[serde(rename = "Cta")]
    pub cta: Option<FormCTA>,
    /// No documentation provided by AWS
    #[serde(rename = "Fields")]
    pub fields: FieldsMap,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LabelDecorator")]
    pub label_decorator: Option<LabelDecorator>,
    /// No documentation provided by AWS
    #[serde(rename = "FormActionType")]
    pub form_action_type: FormActionType,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct FormCTA {
    #[serde(rename = "Position")]
    pub position: Option<FormButtonsPosition>,
    #[serde(rename = "Clear")]
    pub clear: Option<FormButton>,
    #[serde(rename = "Cancel")]
    pub cancel: Option<FormButton>,
    #[serde(rename = "Submit")]
    pub submit: Option<FormButton>,

}
pub type StorageAccessLevel = String;
#[derive(serde::Serialize, Default)]
pub struct FieldsMap {

}

#[derive(serde::Serialize, Default)]
pub struct FileUploaderFieldConfig {
    #[serde(rename = "MaxSize")]
    pub max_size: Option<f64>,
    #[serde(rename = "IsResumable")]
    pub is_resumable: Option<bool>,
    #[serde(rename = "AcceptedFileTypes")]
    pub accepted_file_types: Vec<String>,
    #[serde(rename = "AccessLevel")]
    pub access_level: StorageAccessLevel,
    #[serde(rename = "MaxFileCount")]
    pub max_file_count: Option<f64>,
    #[serde(rename = "ShowThumbnails")]
    pub show_thumbnails: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct FormButton {
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,
    #[serde(rename = "Children")]
    pub children: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FormStyleConfig {

}
pub type LabelDecorator = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct ValueMapping {
    #[serde(rename = "Value")]
    pub value: FormInputValueProperty,
    #[serde(rename = "DisplayValue")]
    pub display_value: Option<FormInputValueProperty>,

}
pub type FormActionType = String;
#[derive(serde::Serialize, Default)]
pub struct FormDataTypeConfig {
    #[serde(rename = "DataSourceType")]
    pub data_source_type: FormDataSourceType,
    #[serde(rename = "DataTypeName")]
    pub data_type_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ValueMappings {
    #[serde(rename = "Values")]
    pub values: Vec<ValueMapping>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldConfig {
    #[serde(rename = "InputType")]
    pub input_type: Option<FieldInputConfig>,
    #[serde(rename = "Validations")]
    pub validations: Option<Vec<FieldValidationConfiguration>>,
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,

}
pub type FormButtonsPosition = String;
#[derive(serde::Serialize, Default)]
pub struct SectionalElement {
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<String>,
    #[serde(rename = "Position")]
    pub position: Option<FieldPosition>,
    #[serde(rename = "Excluded")]
    pub excluded: Option<bool>,
    #[serde(rename = "Level")]
    pub level: Option<f64>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct FormStyle {
    #[serde(rename = "HorizontalGap")]
    pub horizontal_gap: Option<FormStyleConfig>,
    #[serde(rename = "VerticalGap")]
    pub vertical_gap: Option<FormStyleConfig>,
    #[serde(rename = "OuterPadding")]
    pub outer_padding: Option<FormStyleConfig>,

}
pub type FixedPosition = String;
#[derive(serde::Serialize, Default)]
pub struct FieldInputConfig {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Required")]
    pub required: Option<bool>,
    #[serde(rename = "MaxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "DefaultCountryCode")]
    pub default_country_code: Option<String>,
    #[serde(rename = "DescriptiveText")]
    pub descriptive_text: Option<String>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "Step")]
    pub step: Option<f64>,
    #[serde(rename = "MinValue")]
    pub min_value: Option<f64>,
    #[serde(rename = "DefaultChecked")]
    pub default_checked: Option<bool>,
    #[serde(rename = "Placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "FileUploaderConfig")]
    pub file_uploader_config: Option<FileUploaderFieldConfig>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "IsArray")]
    pub is_array: Option<bool>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "ValueMappings")]
    pub value_mappings: Option<ValueMappings>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionalElementMap {

}

#[derive(serde::Serialize, Default)]
pub struct FormInputValueProperty {
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldValidationConfiguration {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "StrValues")]
    pub str_values: Option<Vec<String>>,
    #[serde(rename = "ValidationMessage")]
    pub validation_message: Option<String>,
    #[serde(rename = "NumValues")]
    pub num_values: Option<Vec<f64>>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldPosition {

}
pub type FormDataSourceType = String;

}

pub mod cfn_theme {

#[derive(serde::Serialize, Default)]
pub struct CfnTheme {
    /// List of ThemeValues
    #[serde(rename = "Values")]
    pub values: Vec<ThemeValues>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// List of ThemeValues
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<ThemeValues>>,
    /// No documentation provided by AWS
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ThemeValues {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<ThemeValue>,

}

#[derive(serde::Serialize, Default)]
pub struct ThemeValue {
    #[serde(rename = "Children")]
    pub children: Option<Vec<ThemeValues>>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}
