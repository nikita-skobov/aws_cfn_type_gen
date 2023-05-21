

/// Creates an asset model from specified property and hierarchy definitions. You create    assets from asset models. With asset models, you can easily create assets of the same type    that have standardized definitions. Each asset created from a model inherits the asset model's    property and hierarchy definitions. For more information, see Defining asset models in the       AWS IoT SiteWise User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnAssetModel {


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
    pub asset_model_properties: Option<Vec<AssetModelProperty>>,


    /// 
    /// The composite asset models that are part of this asset model.       Composite asset models are asset models that contain specific properties. Each composite model       has a type that defines the properties that the composite model supports. You can use composite asset       models to define alarms on this asset model.
    /// 
    /// Required: No
    ///
    /// Type: List of AssetModelCompositeModel
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelCompositeModels")]
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
    pub asset_model_description: Option<String>,


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
    pub asset_model_name: String,


    /// 
    /// A list of key-value pairs that contain metadata for the asset. For more information, see       Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// Contains information about a composite model in an asset model. This object contains the       asset property definitions that you define in the composite model. You can use composite asset       models to define alarms on this asset model.
///
/// If you use the AssetModelCompositeModel property to create an alarm, you must use the following information to define three asset model properties:
///
/// At the bottom of this page, we provide a YAML example that you can modify to create an alarm.
#[derive(Default, serde::Serialize)]
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
    pub composite_model_properties: Option<Vec<AssetModelProperty>>,


    /// 
    /// The name of the composite model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The description of the composite model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The type of the composite model. For alarm composite models, this type is     AWS/ALARM.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// Contains a tumbling window, which is a repeating fixed-sized, non-overlapping, and    contiguous time window. You can use this window in metrics to aggregate data from properties    and other assets.
///
/// You can use m, h, d, and w when you    specify an interval or offset. Note that m represents minutes, h    represents hours, d represents days, and w represents weeks. You can    also use s to represent seconds in offset.
///
/// The interval and offset parameters support the ISO 8601 format. For example,     PT5S represents 5 seconds, PT5M represents 5 minutes, and     PT5H represents 5 hours.
#[derive(Default, serde::Serialize)]
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
    pub interval: String,


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
    pub offset: Option<String>,

}


/// Identifies a property value used in an expression.
#[derive(Default, serde::Serialize)]
pub struct VariableValue {


    /// 
    /// The LogicalID of the property to use as the variable.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyLogicalId")]
    pub property_logical_id: String,


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
    pub hierarchy_logical_id: Option<String>,

}


/// Contains expression variable information.
#[derive(Default, serde::Serialize)]
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
    pub name: String,


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


/// Contains a property type, which can be one of Attribute,     Measurement, Metric, or Transform.
#[derive(Default, serde::Serialize)]
pub struct PropertyType {


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
    pub metric: Option<Metric>,


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
    pub attribute: Option<Attribute>,


    /// 
    /// The type of property type, which can be one       of Attribute, Measurement,       Metric, or Transform.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: String,


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
    pub transform: Option<Transform>,

}


/// Describes an asset hierarchy that contains a hierarchy's name, LogicalID, and child asset model    ID that specifies the type of asset that can be in this hierarchy.
#[derive(Default, serde::Serialize)]
pub struct AssetModelHierarchy {


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
    pub logical_id: String,


    /// 
    /// The Id of the asset model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChildAssetModelId")]
    pub child_asset_model_id: String,


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
    pub name: String,

}


/// Contains an asset metric property. With metrics, you can calculate aggregate functions,    such as an average, maximum, or minimum, as specified through an expression. A metric maps    several values to a single value (such as a sum).
///
/// The maximum number of dependent/cascading variables used in any one metric calculation is    10. Therefore, a root metric can have    up to 10 cascading metrics in its computational dependency    tree. Additionally, a metric can only have a data type of DOUBLE and consume    properties with data types of INTEGER or DOUBLE.
///
/// For more information, see Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Default, serde::Serialize)]
pub struct Metric {


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
    pub expression: String,


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


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// Contains an asset transform property. A transform is a one-to-one mapping of a property's    data points from one form to another. For example, you can use a transform to convert a    Celsius data stream to Fahrenheit by applying the transformation expression to each data point    of the Celsius stream. Transforms can only input properties that are INTEGER, DOUBLE, or BOOLEAN type.    Booleans convert to 0 (FALSE) and 1 (TRUE)..
///
/// For more information, see Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Default, serde::Serialize)]
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
    pub expression: String,


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


/// Contains an asset attribute property. For more information, see       Defining data properties in the AWS IoT SiteWise User Guide.
#[derive(Default, serde::Serialize)]
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
    pub default_value: Option<String>,

}


/// Contains a time interval window used for data aggregate computations (for example,    average, sum, count, and so on).
#[derive(Default, serde::Serialize)]
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
    pub tumbling: Option<TumblingWindow>,

}


/// Contains information about an asset model property.
#[derive(Default, serde::Serialize)]
pub struct AssetModelProperty {


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
    pub name: String,


    /// 
    /// The data type of the asset model property. The value can be STRING, INTEGER, DOUBLE,      BOOLEAN, or STRUCT.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: String,


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
    pub logical_id: String,


    /// 
    /// The data type of the structure for this property. This parameter exists on properties that    have the STRUCT data type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTypeSpec")]
    pub data_type_spec: Option<String>,


    /// 
    /// The unit of the asset model property, such as Newtons or    RPM.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,


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

}