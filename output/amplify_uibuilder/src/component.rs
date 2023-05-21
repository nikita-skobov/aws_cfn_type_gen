

/// The AWS::AmplifyUIBuilder::Component resource specifies a component within an Amplify app.    A component is a user interface (UI) element that you can customize. Use     ComponentChild to configure an instance of a Component. A     ComponentChild instance inherits the configuration of the main     Component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnComponent {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,


    /// 
    /// The information to connect a component's properties to data at runtime. You can't specify     tags as a valid property for bindingProperties.
    /// 
    /// 
    /// 
    /// Required: Yes
    ///
    /// Type: Map of ComponentBindingPropertiesValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "BindingProperties")]
    pub binding_properties: std::collections::HashMap<String, ComponentBindingPropertiesValue>,


    /// 
    /// A list of the component's ComponentChild instances.
    /// 
    /// Required: No
    ///
    /// Type: List of ComponentChild
    ///
    /// Update requires: No interruption
    #[serde(rename = "Children")]
    pub children: Option<Vec<ComponentChild>>,


    /// 
    /// The data binding configuration for the component's properties. Use this for a collection    component. You can't specify tags as a valid property for     collectionProperties.
    /// 
    /// Required: No
    ///
    /// Type: Map of ComponentDataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectionProperties")]
    pub collection_properties: Option<std::collections::HashMap<String, ComponentDataConfiguration>>,


    /// 
    /// The type of the component. This can be an Amplify custom UI component or    another custom component.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentType")]
    pub component_type: String,


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
    /// Describes the events that can be raised on the component. Use for the workflow feature in     Amplify Studio that allows you to bind events and actions to    components.
    /// 
    /// Required: No
    ///
    /// Type: Map of ComponentEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,


    /// 
    /// The name of the component.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Describes the component's properties that can be overriden in a customized instance of the    component. You can't specify tags as a valid property for    overrides.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: serde_json::Value,


    /// 
    /// Describes the component's properties. You can't specify tags as a valid    property for properties.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    pub properties: std::collections::HashMap<String, ComponentProperty>,


    /// 
    /// The schema version of the component when it was imported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersion")]
    pub schema_version: Option<String>,


    /// 
    /// The unique ID of the component in its original source system, such as Figma.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceId")]
    pub source_id: Option<String>,


    /// 
    /// One or more key-value pairs to use when tagging the component.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// A list of the component's variants. A variant is a unique style configuration of a main    component.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ComponentVariant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variants")]
    pub variants: Vec<ComponentVariant>,

}



impl cfn_resources::CfnResource for CfnComponent {
    fn type_string() -> &'static str {
        "AWS::AmplifyUIBuilder::Component"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Represents the event action configuration for an element of a Component or     ComponentChild. Use for the workflow feature in Amplify Studio    that allows you to bind events and actions to components. ActionParameters    defines the action that is performed when an event occurs on the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ActionParameters {


    /// 
    /// The HTML anchor link to the location to open. Specify this value for a navigation    action.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Anchor")]
    pub anchor: Option<ComponentProperty>,


    /// 
    /// A dictionary of key-value pairs mapping Amplify Studio properties to fields    in a data model. Use when the action performs an operation on an Amplify    DataStore model.
    /// 
    /// Required: No
    ///
    /// Type: Map of ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fields")]
    pub fields: Option<std::collections::HashMap<String, ComponentProperty>>,


    /// 
    /// Specifies whether the user should be signed out globally. Specify this value for an auth    sign out action.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Global")]
    pub global: Option<ComponentProperty>,


    /// 
    /// The unique ID of the component that the ActionParameters apply to.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: Option<ComponentProperty>,


    /// 
    /// The name of the data model. Use when the action performs an operation on an Amplify DataStore model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    pub model: Option<String>,


    /// 
    /// A key-value pair that specifies the state property name and its initial value.
    /// 
    /// Required: No
    ///
    /// Type: MutationActionSetStateParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<MutationActionSetStateParameter>,


    /// 
    /// The element within the same component to modify when the action occurs.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Option<ComponentProperty>,


    /// 
    /// The type of navigation action. Valid values are url and anchor.    This value is required for a navigation action.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<ComponentProperty>,


    /// 
    /// The URL to the location to open. Specify this value for a navigation action.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: Option<ComponentProperty>,

}




/// The ComponentBindingPropertiesValue property specifies the data binding configuration for a component at runtime. You can use     ComponentBindingPropertiesValue to add exposed properties to a component to    allow different values to be entered when a component is reused in different places in an    app.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentBindingPropertiesValue {


    /// 
    /// Describes the properties to customize with data at runtime.
    /// 
    /// Required: No
    ///
    /// Type: ComponentBindingPropertiesValueProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "BindingProperties")]
    pub binding_properties: Option<ComponentBindingPropertiesValueProperties>,


    /// 
    /// The default value of the property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,


    /// 
    /// The property type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}




/// The ComponentBindingPropertiesValueProperties property specifies the data    binding configuration for a specific property using data stored in AWS. For     AWS connected properties, you can bind a property to data stored in an     Amazon S3 bucket, an Amplify DataStore model or an authenticated user    attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentBindingPropertiesValueProperties {


    /// 
    /// An Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,


    /// 
    /// The default value to assign to the property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,


    /// 
    /// The field to bind the data to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<String>,


    /// 
    /// The storage key for an Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// An Amplify DataStore model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    pub model: Option<String>,


    /// 
    /// A list of predicates for binding a component's properties to data.
    /// 
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicates")]
    pub predicates: Option<Vec<Predicate>>,


    /// 
    /// An authenticated user attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttribute")]
    pub user_attribute: Option<String>,

}




/// The ComponentChild property specifies a nested UI configuration within a    parent Component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentChild {


    /// 
    /// The list of ComponentChild instances for this component.
    /// 
    /// Required: No
    ///
    /// Type: List of ComponentChild
    ///
    /// Update requires: No interruption
    #[serde(rename = "Children")]
    pub children: Option<Vec<ComponentChild>>,


    /// 
    /// The type of the child component.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentType")]
    pub component_type: String,


    /// 
    /// Describes the events that can be raised on the child component. Use for the workflow    feature in Amplify Studio that allows you to bind events and actions to    components.
    /// 
    /// Required: No
    ///
    /// Type: Map of ComponentEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,


    /// 
    /// The name of the child component.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Describes the properties of the child component. You can't specify tags as a    valid property for properties.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    pub properties: std::collections::HashMap<String, ComponentProperty>,

}




/// The ComponentConditionProperty property specifies a conditional expression    for setting a component property. Use ComponentConditionProperty to set a    property to different values conditionally, based on the value of another property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentConditionProperty {


    /// 
    /// The value to assign to the property if the condition is not met.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Else")]
    pub cfn_else: Option<ComponentProperty>,


    /// 
    /// The name of a field. Specify this when the property is a data model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<String>,


    /// 
    /// The value of the property to evaluate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operand")]
    pub operand: Option<String>,


    /// 
    /// The type of the property to evaluate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperandType")]
    pub operand_type: Option<String>,


    /// 
    /// The operator to use to perform the evaluation, such as eq to represent    equals.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    pub operator: Option<String>,


    /// 
    /// The name of the conditional property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: Option<String>,


    /// 
    /// The value to assign to the property if the condition is met.
    /// 
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Then")]
    pub then: Option<ComponentProperty>,

}




/// The ComponentDataConfiguration property specifies the configuration for    binding a component's properties to data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentDataConfiguration {


    /// 
    /// A list of IDs to use to bind data to a component. Use this property to bind specifically    chosen data, rather than data retrieved from a query.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifiers")]
    pub identifiers: Option<Vec<String>>,


    /// 
    /// The name of the data model to use to bind data to a component.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    pub model: String,


    /// 
    /// Represents the conditional logic to use when binding data to a component. Use this    property to retrieve only a subset of the data in a collection.
    /// 
    /// Required: No
    ///
    /// Type: Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicate")]
    pub predicate: Option<Predicate>,


    /// 
    /// Describes how to sort the component's properties.
    /// 
    /// Required: No
    ///
    /// Type: List of SortProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sort")]
    pub sort: Option<Vec<SortProperty>>,

}




/// The ComponentEvent property specifies the configuration of an event. You can bind an event and a corresponding    action to a Component or a ComponentChild. A button click    is an example of an event.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentEvent {


    /// 
    /// The action to perform when a specific event is raised.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<String>,


    /// 
    /// Describes information about the action.
    /// 
    /// Required: No
    ///
    /// Type: ActionParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<ActionParameters>,

}




/// The ComponentProperty property specifies the configuration for all of a    component's properties. Use ComponentProperty to specify the values to render or    bind by default.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentProperty {


    /// 
    /// The information to bind the component property to data at runtime.
    /// 
    /// Required: No
    ///
    /// Type: ComponentPropertyBindingProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "BindingProperties")]
    pub binding_properties: Option<ComponentPropertyBindingProperties>,


    /// 
    /// The information to bind the component property to form data.
    /// 
    /// Required: No
    ///
    /// Type: Map of FormBindingElement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bindings")]
    pub bindings: Option<std::collections::HashMap<String, FormBindingElement>>,


    /// 
    /// The information to bind the component property to data at runtime. Use this for collection    components.
    /// 
    /// Required: No
    ///
    /// Type: ComponentPropertyBindingProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectionBindingProperties")]
    pub collection_binding_properties: Option<ComponentPropertyBindingProperties>,


    /// 
    /// The name of the component that is affected by an event.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentName")]
    pub component_name: Option<String>,


    /// 
    /// A list of component properties to concatenate to create the value to assign to this    component property.
    /// 
    /// Required: No
    ///
    /// Type: List of ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Concat")]
    pub concat: Option<Vec<ComponentProperty>>,


    /// 
    /// The conditional expression to use to assign a value to the component property.
    /// 
    /// Required: No
    ///
    /// Type: ComponentConditionProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: Option<Box<ComponentConditionProperty>>,


    /// 
    /// Specifies whether the user configured the property in Amplify Studio after    importing it.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configured")]
    pub configured: Option<bool>,


    /// 
    /// The default value to assign to the component property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,


    /// 
    /// An event that occurs in your app. Use this for workflow data binding.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Event")]
    pub event: Option<String>,


    /// 
    /// The default value assigned to the property when the component is imported into an    app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImportedValue")]
    pub imported_value: Option<String>,


    /// 
    /// The data model to use to assign a value to the component property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    pub model: Option<String>,


    /// 
    /// The name of the component's property that is affected by an event.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: Option<String>,


    /// 
    /// The component type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// An authenticated user attribute to use to assign a value to the component property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttribute")]
    pub user_attribute: Option<String>,


    /// 
    /// The value to assign to the component property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}




/// The ComponentPropertyBindingProperties property specifies a component    property to associate with a binding property. This enables exposed properties on the top    level component to propagate data to the component's property values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentPropertyBindingProperties {


    /// 
    /// The data field to bind the property to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<String>,


    /// 
    /// The component property to bind to the data field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: String,

}




/// The ComponentVariant property specifies the style configuration of a unique    variation of a main component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentVariant {


    /// 
    /// The properties of the component variant that can be overriden when customizing an instance    of the component. You can't specify tags as a valid property for     overrides.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: Option<serde_json::Value>,


    /// 
    /// The combination of variants that comprise this variant.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariantValues")]
    pub variant_values: Option<std::collections::HashMap<String, String>>,

}




/// The FormBindingElement property type specifies Property description not available. for an AWS::AmplifyUIBuilder::Component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormBindingElement {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Element")]
    pub element: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: String,

}




/// Represents the state configuration when an action modifies a property of another element    within the same component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MutationActionSetStateParameter {


    /// 
    /// The name of the component that is being modified.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentName")]
    pub component_name: String,


    /// 
    /// The name of the component property to apply the state configuration to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: String,


    /// 
    /// The state configuration to assign to the property.
    /// 
    /// Required: Yes
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Set")]
    pub set: ComponentProperty,

}




/// The Predicate property specifies information for generating Amplify DataStore queries. Use Predicate to retrieve a subset of the    data in a collection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Predicate {


    /// 
    /// A list of predicates to combine logically.
    /// 
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "And")]
    pub and: Option<Vec<Predicate>>,


    /// 
    /// The field to query.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<String>,


    /// 
    /// The value to use when performing the evaluation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operand")]
    pub operand: Option<String>,


    /// 
    /// The operator to use to perform the evaluation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    pub operator: Option<String>,


    /// 
    /// A list of predicates to combine logically.
    /// 
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Or")]
    pub or: Option<Vec<Predicate>>,

}




/// The SortProperty property specifies how to sort the data that you bind to a    component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SortProperty {


    /// 
    /// The direction of the sort, either ascending or descending.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: String,


    /// 
    /// The field to perform the sort on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: String,

}


