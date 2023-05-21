/// Creates an asset model from specified property and hierarchy definitions. You create    assets from asset models. With asset models, you can easily create assets of the same type    that have standardized definitions. Each asset created from a model inherits the asset model's    property and hierarchy definitions. For more information, see Defining asset models in the       AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssetModel {
    ///
    /// The composite asset models that are part of this asset model.       Composite asset models are asset models that contain specific properties. Each composite model       has a type that defines the properties that the composite model supports. You can use composite asset       models to define alarms on this asset model.
    ///
    /// Required: No
    ///
    /// Type: List of AssetModelCompositeModel
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelCompositeModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_model_composite_models: Option<Vec<AssetModelCompositeModel>>,

    ///
    /// A description for the asset model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_model_description: Option<cfn_resources::StrVal>,

    ///
    /// The hierarchy definitions of the asset model. Each hierarchy specifies an asset model    whose assets can be children of any other assets created from this asset model. For more    information, see Defining relationships between assets in the AWS IoT SiteWiseUser Guide.
    ///
    /// You can specify up to 10 hierarchies per asset model. For more       information, see Quotas in the AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of AssetModelHierarchy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelHierarchies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_model_hierarchies: Option<Vec<AssetModelHierarchy>>,

    ///
    /// A unique, friendly name for the asset model.
    ///
    /// The maximum length is 256 characters with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelName")]
    pub asset_model_name: cfn_resources::StrVal,

    ///
    /// The property definitions of the asset model. For more information, see       Defining data properties in the AWS IoT SiteWise User Guide.
    ///
    /// You can specify up to 200 properties per asset model. For more       information, see Quotas in the AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of AssetModelProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_model_properties: Option<Vec<AssetModelProperty>>,

    ///
    /// A list of key-value pairs that contain metadata for the asset. For more information, see       Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
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

impl cfn_resources::CfnResource for CfnAssetModel {
    fn type_string(&self) -> &'static str {
        "AWS::IoTSiteWise::AssetModel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Contains information about a composite model in an asset model. This object contains the       asset property definitions that you define in the composite model. You can use composite asset       models to define alarms on this asset model.
///
/// If you use the AssetModelCompositeModel property to create an alarm, you must use the following information to define three asset model properties:
///
/// At the bottom of this page, we provide a YAML example that you can modify to create an alarm.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetModelCompositeModel {
    ///
    /// The asset property definitions for this composite model.
    ///
    /// Required: No
    ///
    /// Type: List of AssetModelProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompositeModelProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_model_properties: Option<Vec<AssetModelProperty>>,

    ///
    /// The description of the composite model.
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
    /// The name of the composite model.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The type of the composite model. For alarm composite models, this type is     AWS/ALARM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AssetModelCompositeModel {
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

/// Describes an asset hierarchy that contains a hierarchy's name, LogicalID, and child asset model    ID that specifies the type of asset that can be in this hierarchy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetModelHierarchy {
    ///
    /// The Id of the asset model.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChildAssetModelId")]
    pub child_asset_model_id: cfn_resources::StrVal,

    ///
    /// The LogicalID of the asset model hierarchy. This ID is a hierarchyLogicalId.
    ///
    /// The maximum length is 256 characters, with the pattern [^\u0000-\u001F\u007F]+
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalId")]
    pub logical_id: cfn_resources::StrVal,

    ///
    /// The name of the asset model hierarchy.
    ///
    /// The maximum length is 256 characters with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AssetModelHierarchy {
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

/// Contains information about an asset model property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetModelProperty {
    ///
    /// The data type of the asset model property. The value can be STRING, INTEGER, DOUBLE,      BOOLEAN, or STRUCT.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: cfn_resources::StrVal,

    ///
    /// The data type of the structure for this property. This parameter exists on properties that    have the STRUCT data type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTypeSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type_spec: Option<cfn_resources::StrVal>,

    ///
    /// The LogicalID of the asset model property.
    ///
    /// The maximum length is 256 characters, with the pattern [^\\u0000-\\u001F\\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalId")]
    pub logical_id: cfn_resources::StrVal,

    ///
    /// The name of the asset model property.
    ///
    /// The maximum length is 256 characters with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Contains a property type, which can be one of Attribute,     Measurement, Metric, or Transform.
    ///
    /// Required: Yes
    ///
    /// Type: PropertyType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: PropertyType,

    ///
    /// The unit of the asset model property, such as Newtons or    RPM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AssetModelProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cfn_type.validate()?;

        Ok(())
    }
}

/// Contains an asset attribute property. For more information, see       Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Attribute {
    ///
    /// The default value of the asset model property attribute. All assets that you create from    the asset model contain this attribute value. You can update an attribute's value after you    create an asset. For more information, see Updating attribute values in the       AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Attribute {
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

/// Contains expression variable information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExpressionVariable {
    ///
    /// The friendly name of the variable to be used in the expression.
    ///
    /// The maximum length is 64 characters with the pattern ^[a-z][a-z0-9_]*$.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The variable that identifies an asset property from which to use values.
    ///
    /// Required: Yes
    ///
    /// Type: VariableValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: VariableValue,
}

impl cfn_resources::CfnResource for ExpressionVariable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.value.validate()?;

        Ok(())
    }
}

/// Contains an asset metric property. With metrics, you can calculate aggregate functions,    such as an average, maximum, or minimum, as specified through an expression. A metric maps    several values to a single value (such as a sum).
///
/// The maximum number of dependent/cascading variables used in any one metric calculation is    10. Therefore, a root metric can have    up to 10 cascading metrics in its computational dependency    tree. Additionally, a metric can only have a data type of DOUBLE and consume    properties with data types of INTEGER or DOUBLE.
///
/// For more information, see Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metric {
    ///
    /// The mathematical expression that defines the metric aggregation function. You can specify    up to 10 variables per expression. You can specify up to 10 functions    per expression.
    ///
    /// For more information, see Quotas in the       AWS IoT SiteWise User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: cfn_resources::StrVal,

    ///
    /// The list of variables used in the expression.
    ///
    /// Required: Yes
    ///
    /// Type: List of ExpressionVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    pub variables: Vec<ExpressionVariable>,

    ///
    /// The window (time interval) over which AWS IoT SiteWise computes the metric's aggregation expression.    AWS IoT SiteWise computes one data point per window.
    ///
    /// Required: Yes
    ///
    /// Type: MetricWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "Window")]
    pub window: MetricWindow,
}

impl cfn_resources::CfnResource for Metric {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.window.validate()?;

        Ok(())
    }
}

/// Contains a time interval window used for data aggregate computations (for example,    average, sum, count, and so on).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricWindow {
    ///
    /// The tumbling time interval window.
    ///
    /// Required: No
    ///
    /// Type: TumblingWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tumbling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling: Option<TumblingWindow>,
}

impl cfn_resources::CfnResource for MetricWindow {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.tumbling
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains a property type, which can be one of Attribute,     Measurement, Metric, or Transform.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PropertyType {
    ///
    /// Specifies an asset attribute property. An attribute generally contains static information,    such as the serial number of an industrial IoT wind turbine.
    ///
    /// This is required if the TypeName is Attribute and has a DefaultValue.
    ///
    /// Required: No
    ///
    /// Type: Attribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,

    ///
    /// Specifies an asset metric property. A metric contains a mathematical expression that uses    aggregate functions to process all input data points over a time interval and output a single    data point, such as to calculate the average hourly temperature.
    ///
    /// This is required if the TypeName is Metric.
    ///
    /// Required: No
    ///
    /// Type: Metric
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,

    ///
    /// Specifies an asset transform property. A transform contains a mathematical expression that    maps a property's data points from one form to another, such as a unit conversion from Celsius    to Fahrenheit.
    ///
    /// This is required if the TypeName is Transform.
    ///
    /// Required: No
    ///
    /// Type: Transform
    ///
    /// Update requires: No interruption
    #[serde(rename = "Transform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Transform>,

    ///
    /// The type of property type, which can be one       of Attribute, Measurement,       Metric, or Transform.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PropertyType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.attribute
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.metric.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.transform
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// Contains an asset transform property. A transform is a one-to-one mapping of a property's    data points from one form to another. For example, you can use a transform to convert a    Celsius data stream to Fahrenheit by applying the transformation expression to each data point    of the Celsius stream. Transforms can only input properties that are INTEGER, DOUBLE, or BOOLEAN type.    Booleans convert to 0 (FALSE) and 1 (TRUE)..
///
/// For more information, see Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Transform {
    ///
    /// The mathematical expression that defines the transformation function. You can specify up    to 10 variables per expression. You can specify up to 10 functions per    expression.
    ///
    /// For more information, see Quotas in the       AWS IoT SiteWise User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: cfn_resources::StrVal,

    ///
    /// The list of variables used in the expression.
    ///
    /// Required: Yes
    ///
    /// Type: List of ExpressionVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    pub variables: Vec<ExpressionVariable>,
}

impl cfn_resources::CfnResource for Transform {
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

/// Contains a tumbling window, which is a repeating fixed-sized, non-overlapping, and    contiguous time window. You can use this window in metrics to aggregate data from properties    and other assets.
///
/// You can use m, h, d, and w when you    specify an interval or offset. Note that m represents minutes, h    represents hours, d represents days, and w represents weeks. You can    also use s to represent seconds in offset.
///
/// The interval and offset parameters support the ISO 8601 format. For example,     PT5S represents 5 seconds, PT5M represents 5 minutes, and     PT5H represents 5 hours.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TumblingWindow {
    ///
    /// The time interval for the tumbling window. The interval time must be between 1 minute and    1 week.
    ///
    /// AWS IoT SiteWise computes the 1w interval the end of Sunday at midnight each week (UTC),    the 1d interval at the end of each day at midnight (UTC), the 1h    interval at the end of each hour, and so on.
    ///
    /// When AWS IoT SiteWise aggregates data points for metric computations, the start of each interval is    exclusive and the end of each interval is inclusive. AWS IoT SiteWise places the computed data point at    the end of the interval.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: cfn_resources::StrVal,

    ///
    /// The offset for the tumbling window. The offset parameter accepts the    following:
    ///
    /// The offset time.        For example, if you specify 18h for offset and       1d for interval, AWS IoT SiteWise aggregates data in one of the following      ways:                                               If you create the metric before or at 6 PM (UTC), you get the first aggregation        result at 6 PM (UTC) on the day when you create the metric.                     If you create the metric after 6 PM (UTC), you get the first aggregation result at        6 PM (UTC) the next day.                        The ISO 8601 format.        For example, if you specify PT18H for offset and       1d for interval, AWS IoT SiteWise aggregates data in one of the following      ways:                                               If you create the metric before or at 6 PM (UTC), you get the first aggregation        result at 6 PM (UTC) on the day when you create the metric.                     If you create the metric after 6 PM (UTC), you get the first aggregation result at        6 PM (UTC) the next day.                        The 24-hour clock.        For example, if you specify 00:03:00 for offset,       5m for interval, and you create the metric at 2 PM (UTC), you      get the first aggregation result at 2:03 PM (UTC). You get the second aggregation result      at 2:08 PM (UTC).               The offset time zone.        For example, if you specify 2021-07-23T18:00-08 for offset      and 1d for interval, AWS IoT SiteWise aggregates data in one of the      following ways:                                               If you create the metric before or at 6 PM (PST), you get the first aggregation        result at 6 PM (PST) on the day when you create the metric.                     If you create the metric after 6 PM (PST), you get the first aggregation result at        6 PM (PST) the next day.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TumblingWindow {
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

/// Identifies a property value used in an expression.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VariableValue {
    ///
    /// The LogicalID of the hierarchy to query for the PropertyLogicalID.
    ///
    /// You use a hierarchyLogicalID instead of a model ID because you can have several hierarchies    using the same model and therefore the same property. For example, you might    have separately grouped assets that come from the same asset model. For more information, see      Defining relationships between assets in the AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyLogicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_logical_id: Option<cfn_resources::StrVal>,

    ///
    /// The LogicalID of the property to use as the variable.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyLogicalId")]
    pub property_logical_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for VariableValue {
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
