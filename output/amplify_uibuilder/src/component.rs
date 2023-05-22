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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_properties:
        Option<std::collections::HashMap<String, ComponentDataConfiguration>>,

    ///
    /// The type of the component. This can be an Amplify custom UI component or    another custom component.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentType")]
    pub component_type: cfn_resources::StrVal,

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
    /// Describes the events that can be raised on the component. Use for the workflow feature in     Amplify Studio that allows you to bind events and actions to    components.
    ///
    /// Required: No
    ///
    /// Type: Map of ComponentEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<cfn_resources::StrVal>,

    ///
    /// The unique ID of the component in its original source system, such as Figma.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<cfn_resources::StrVal>,

    ///
    /// One or more key-value pairs to use when tagging the component.
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
    /// A list of the component's variants. A variant is a unique style configuration of a main    component.
    ///
    /// Required: Yes
    ///
    /// Type: List of ComponentVariant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variants")]
    pub variants: Vec<ComponentVariant>,

    #[serde(skip_serializing)]
    pub att_id: CfnComponentid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnComponentid;
impl CfnComponentid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnComponent {
    fn type_string(&self) -> &'static str {
        "AWS::AmplifyUIBuilder::Component"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<cfn_resources::StrVal>,

    ///
    /// A key-value pair that specifies the state property name and its initial value.
    ///
    /// Required: No
    ///
    /// Type: MutationActionSetStateParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<ComponentProperty>,
}

impl cfn_resources::CfnResource for ActionParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.anchor.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.global.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.id.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.state.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.target.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.cfn_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.url.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<cfn_resources::StrVal>,

    ///
    /// The property type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ComponentBindingPropertiesValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.binding_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<cfn_resources::StrVal>,

    ///
    /// The default value to assign to the property.
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
    /// The field to bind the data to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The storage key for an Amazon S3 bucket.
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
    /// An Amplify DataStore model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<cfn_resources::StrVal>,

    ///
    /// A list of predicates for binding a component's properties to data.
    ///
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicates")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ComponentBindingPropertiesValueProperties {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub component_type: cfn_resources::StrVal,

    ///
    /// Describes the events that can be raised on the child component. Use for the workflow    feature in Amplify Studio that allows you to bind events and actions to    components.
    ///
    /// Required: No
    ///
    /// Type: Map of ComponentEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub name: cfn_resources::StrVal,

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

impl cfn_resources::CfnResource for ComponentChild {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The value of the property to evaluate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<cfn_resources::StrVal>,

    ///
    /// The type of the property to evaluate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperandType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_type: Option<cfn_resources::StrVal>,

    ///
    /// The operator to use to perform the evaluation, such as eq to represent    equals.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<cfn_resources::StrVal>,

    ///
    /// The name of the conditional property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<cfn_resources::StrVal>,

    ///
    /// The value to assign to the property if the condition is met.
    ///
    /// Required: No
    ///
    /// Type: ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Then")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<ComponentProperty>,
}

impl cfn_resources::CfnResource for ComponentConditionProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cfn_else
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.then.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub model: cfn_resources::StrVal,

    ///
    /// Represents the conditional logic to use when binding data to a component. Use this    property to retrieve only a subset of the data in a collection.
    ///
    /// Required: No
    ///
    /// Type: Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortProperty>>,
}

impl cfn_resources::CfnResource for ComponentDataConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.predicate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<cfn_resources::StrVal>,

    ///
    /// Describes information about the action.
    ///
    /// Required: No
    ///
    /// Type: ActionParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ActionParameters>,
}

impl cfn_resources::CfnResource for ComponentEvent {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of component properties to concatenate to create the value to assign to this    component property.
    ///
    /// Required: No
    ///
    /// Type: List of ComponentProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Concat")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<cfn_resources::StrVal>,

    ///
    /// An event that occurs in your app. Use this for workflow data binding.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<cfn_resources::StrVal>,

    ///
    /// The default value assigned to the property when the component is imported into an    app.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImportedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_value: Option<cfn_resources::StrVal>,

    ///
    /// The data model to use to assign a value to the component property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<cfn_resources::StrVal>,

    ///
    /// The name of the component's property that is affected by an event.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<cfn_resources::StrVal>,

    ///
    /// The component type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,

    ///
    /// An authenticated user attribute to use to assign a value to the component property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<cfn_resources::StrVal>,

    ///
    /// The value to assign to the component property.
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

impl cfn_resources::CfnResource for ComponentProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.binding_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.collection_binding_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The component property to bind to the data field.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ComponentPropertyBindingProperties {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_values: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for ComponentVariant {
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
    pub element: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for FormBindingElement {
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
    pub component_name: cfn_resources::StrVal,

    ///
    /// The name of the component property to apply the state configuration to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: cfn_resources::StrVal,

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

impl cfn_resources::CfnResource for MutationActionSetStateParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.set.validate()?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The value to use when performing the evaluation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<cfn_resources::StrVal>,

    ///
    /// The operator to use to perform the evaluation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<cfn_resources::StrVal>,

    ///
    /// A list of predicates to combine logically.
    ///
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Predicate>>,
}

impl cfn_resources::CfnResource for Predicate {
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
    pub direction: cfn_resources::StrVal,

    ///
    /// The field to perform the sort on.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SortProperty {
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
