

/// Creates a dashboard from a template. To first create a template, see the CreateTemplate API  operation.
///
/// A dashboard is an entity in Amazon QuickSight that identifies Amazon QuickSight reports, created from  analyses. You can share Amazon QuickSight dashboards. With the right permissions, you can create scheduled email  reports from them. If you have the correct permissions, you can create a dashboard from a template that exists in a  different AWS account.
#[derive(Default, serde::Serialize)]
pub struct CfnDashboard {


    /// 
    /// The ID for the dashboard, also added to the IAM policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,


    /// 
    /// The entity that you are using as a source when you create the dashboard. In SourceEntity, you  specify the type of object that you want to use. You can only create a dashboard from a template, so you use a  SourceTemplate entity. If you need to create a dashboard from an analysis, first convert the analysis to  a template by using the CreateTemplate API operation. For SourceTemplate, specify the  Amazon Resource Name (ARN) of the source template. The SourceTemplateARN can contain any AWS account; and any QuickSight-supported AWS Region.
    /// 
    /// Use the DataSetReferences entity within SourceTemplate to list the replacement  datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder.
    /// 
    /// Required: No
    ///
    /// Type: DashboardSourceEntity
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceEntity")]
    pub source_entity: Option<DashboardSourceEntity>,


    /// 
    /// A structure that contains the permissions of the dashboard. You can use this structure       for granting permissions by providing a list of IAM action information for each       principal ARN.
    /// 
    /// To specify no permissions, omit the permissions list.
    /// 
    /// Required: No
    ///
    /// Type: List of ResourcePermission
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DashboardVersionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<DashboardVersionDefinition>,


    /// 
    /// A description for the first version of the dashboard being created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,


    /// 
    /// Options for publishing the dashboard when you create it:
    /// 
    /// AvailabilityStatus for AdHocFilteringOption - This           status can be either ENABLED or DISABLED. When this is           set to DISABLED, Amazon QuickSight disables the left filter pane on the           published dashboard, which can be used for ad hoc (one-time) filtering. This           option is ENABLED by default.                         AvailabilityStatus for ExportToCSVOption - This           status can be either ENABLED or DISABLED. The visual           option to export data to .CSV format isn't enabled when this is set to           DISABLED. This option is ENABLED by default.                         VisibilityState for SheetControlsOption - This           visibility state can be either COLLAPSED or EXPANDED.           This option is COLLAPSED by default.
    /// 
    /// Required: No
    ///
    /// Type: DashboardPublishOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardPublishOptions")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,


    /// 
    /// The parameters for the creation of the dashboard, which you want to use to override       the default settings. A dashboard can have any type of parameters, and some parameters       might accept multiple values.
    /// 
    /// Required: No
    ///
    /// Type: Parameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,


    /// 
    /// The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If       you add a value for this field, it overrides the value that is used in the source       entity. The theme ARN must exist in the same AWS account where you create the       dashboard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,


    /// 
    /// The ID of the AWS account where you want to create the dashboard.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,


    /// 
    /// Contains a map of the key-value pairs for the resource tag or tags assigned to the       dashboard.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The display name of the dashboard.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// The field well configuration of a heat map.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct HeatMapFieldWells {


    /// 
    /// The aggregated field wells of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: HeatMapAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapAggregatedFieldWells")]
    pub heat_map_aggregated_field_wells: Option<HeatMapAggregatedFieldWells>,

}


/// The field series item configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct FieldSeriesItem {


    /// 
    /// The options that determine the presentation of line series associated to the field.
    /// 
    /// Required: No
    ///
    /// Type: LineChartSeriesSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,


    /// 
    /// The axis that you are binding the field to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PRIMARY_YAXIS | SECONDARY_YAXIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisBinding")]
    pub axis_binding: String,


    /// 
    /// The field ID of the field for which you are setting the axis binding.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// The forecast scenario of a forecast in the line chart.
#[derive(Default, serde::Serialize)]
pub struct ForecastScenario {


    /// 
    /// The what-if analysis forecast setup with the date range.
    /// 
    /// Required: No
    ///
    /// Type: WhatIfRangeScenario
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhatIfRangeScenario")]
    pub what_if_range_scenario: Option<WhatIfRangeScenario>,


    /// 
    /// The what-if analysis forecast setup with the target date.
    /// 
    /// Required: No
    ///
    /// Type: WhatIfPointScenario
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhatIfPointScenario")]
    pub what_if_point_scenario: Option<WhatIfPointScenario>,

}


/// Determines the border options for a table visual.
#[derive(Default, serde::Serialize)]
pub struct GlobalTableBorderOptions {


    /// 
    /// Determines the options for side specific border.
    /// 
    /// Required: No
    ///
    /// Type: TableSideBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SideSpecificBorder")]
    pub side_specific_border: Option<TableSideBorderOptions>,


    /// 
    /// Determines the options for uniform border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "UniformBorder")]
    pub uniform_border: Option<TableBorderOptions>,

}


/// The aggregated field wells of a heat map.
#[derive(Default, serde::Serialize)]
pub struct HeatMapAggregatedFieldWells {


    /// 
    /// The values field well of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The columns field well of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,


    /// 
    /// The rows field well of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,

}


/// A control from a date filter that is used to specify the relative date.
#[derive(Default, serde::Serialize)]
pub struct FilterRelativeDateTimeControl {


    /// 
    /// The title of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The source filter ID of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: RelativeDateTimeControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,


    /// 
    /// The ID of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}


/// The percent range in the visible range.
#[derive(Default, serde::Serialize)]
pub struct PercentVisibleRange {


    /// 
    /// The lower bound of the range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    pub from: Option<f64>,


    /// 
    /// The top bound of the range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "To")]
    pub to: Option<f64>,

}


/// A filled map.
///
/// For more information, see Creating filled maps in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct FilledMapVisual {


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FilledMapConfiguration>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The conditional formatting of a FilledMapVisual.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<FilledMapConditionalFormatting>,

}


/// The options that determine the percentage display format configuration.
#[derive(Default, serde::Serialize)]
pub struct PercentageDisplayFormatConfiguration {


    /// 
    /// The options that determine the null value format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NullValueFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,


    /// 
    /// Determines the prefix value of the percentage format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The option that determines the decimal places configuration.
    /// 
    /// Required: No
    ///
    /// Type: DecimalPlacesConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,


    /// 
    /// The options that determine the numeric separator configuration.
    /// 
    /// Required: No
    ///
    /// Type: NumericSeparatorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,


    /// 
    /// Determines the suffix value of the percentage format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// The options that determine the negative value configuration.
    /// 
    /// Required: No
    ///
    /// Type: NegativeValueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,

}


/// The configuration of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIConfiguration {


    /// 
    /// The options that determine the presentation of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "KPIOptions")]
    pub kpioptions: Option<KPIOptions>,


    /// 
    /// The sort configuration of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPISortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<KPISortConfiguration>,


    /// 
    /// The field well configuration of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<KPIFieldWells>,

}


/// A column of a data set.
#[derive(Default, serde::Serialize)]
pub struct ColumnIdentifier {


    /// 
    /// The name of the column.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,


    /// 
    /// The data set that the column belongs to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}


/// The field wells of a BarChartVisual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct BarChartFieldWells {


    /// 
    /// The aggregated field wells of a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: BarChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarChartAggregatedFieldWells")]
    pub bar_chart_aggregated_field_wells: Option<BarChartAggregatedFieldWells>,

}


/// The field wells for a table visual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct TableFieldWells {


    /// 
    /// The aggregated field well for the table.
    /// 
    /// Required: No
    ///
    /// Type: TableAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableAggregatedFieldWells")]
    pub table_aggregated_field_wells: Option<TableAggregatedFieldWells>,


    /// 
    /// The unaggregated field well for the table.
    /// 
    /// Required: No
    ///
    /// Type: TableUnaggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableUnaggregatedFieldWells")]
    pub table_unaggregated_field_wells: Option<TableUnaggregatedFieldWells>,

}


/// The dynamic configuration of the reference line data configuration.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineDynamicDataConfiguration {


    /// 
    /// The column that the dynamic data targets.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The aggregation function that is used in the dynamic data.
    /// 
    /// Required: Yes
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureAggregationFunction")]
    pub measure_aggregation_function: AggregationFunction,


    /// 
    /// The calculation that is used in the dynamic data.
    /// 
    /// Required: Yes
    ///
    /// Type: NumericalAggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Calculation")]
    pub calculation: NumericalAggregationFunction,

}


/// Aggregated field wells of a tree map.
#[derive(Default, serde::Serialize)]
pub struct TreeMapAggregatedFieldWells {


    /// 
    /// The group by field well of a tree map. Values are grouped based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<DimensionField>>,


    /// 
    /// The color field well of a tree map. Values are grouped by aggregations based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<MeasureField>>,


    /// 
    /// The size field well of a tree map. Values are aggregated based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sizes")]
    pub sizes: Option<Vec<MeasureField>>,

}


/// The numeric equality type drill down filter.
#[derive(Default, serde::Serialize)]
pub struct CategoryDrillDownFilter {


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// A list of the string inputs that are the values of the category drill down filter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryValues")]
    pub category_values: Vec<String>,

}


/// The data point tooltip options.
#[derive(Default, serde::Serialize)]
pub struct DataPointTooltipOption {


    /// 
    /// The status of the data point tool tip options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The conditional formatting of a table row.
#[derive(Default, serde::Serialize)]
pub struct TableRowConditionalFormatting {


    /// 
    /// The conditional formatting color (solid, gradient) of the background for a table row.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,


    /// 
    /// The conditional formatting color (solid, gradient) of the text for a table row.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,

}


/// A control from a date parameter that specifies date and time.
#[derive(Default, serde::Serialize)]
pub struct ParameterDateTimePickerControl {


    /// 
    /// The ID of the ParameterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: DateTimePickerControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,


    /// 
    /// The name of the ParameterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,


    /// 
    /// The title of the ParameterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,

}


/// The formatting configuration for the color.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingColor {


    /// 
    /// Formatting configuration for solid color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingSolidColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "Solid")]
    pub solid: Option<ConditionalFormattingSolidColor>,


    /// 
    /// Formatting configuration for gradient color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingGradientColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gradient")]
    pub gradient: Option<ConditionalFormattingGradientColor>,

}


/// Conditional formatting options for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableConditionalFormattingOption {


    /// 
    /// The cell conditional formatting option for a pivot table.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableCellConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cell")]
    pub cell: Option<PivotTableCellConditionalFormatting>,

}


/// The field well configuration of a pie chart.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct PieChartFieldWells {


    /// 
    /// The field well configuration of a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: PieChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "PieChartAggregatedFieldWells")]
    pub pie_chart_aggregated_field_wells: Option<PieChartAggregatedFieldWells>,

}


/// The configuration that determines what the type of layout will be used on a sheet.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct LayoutConfiguration {


    /// 
    /// A section based layout organizes visuals into multiple sections and has customized header, footer and page break.
    /// 
    /// Required: No
    ///
    /// Type: SectionBasedLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SectionBasedLayout")]
    pub section_based_layout: Option<SectionBasedLayoutConfiguration>,


    /// 
    /// A type of layout that can be used on a sheet. In a grid layout, visuals snap to a grid with standard spacing and alignment. Dashboards are displayed as designed, with options to fit to screen or view at actual size. A grid layout can be configured to behave in one of two ways when the viewport is resized: FIXED or RESPONSIVE.
    /// 
    /// Required: No
    ///
    /// Type: GridLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,


    /// 
    /// A free-form is optimized for a fixed width and has more control over the exact placement of layout elements.
    /// 
    /// Required: No
    ///
    /// Type: FreeFormLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: Option<FreeFormLayoutConfiguration>,

}


/// The aggregated field wells of a word cloud.
#[derive(Default, serde::Serialize)]
pub struct WordCloudAggregatedFieldWells {


    /// 
    /// The size field well of a word cloud. Values are aggregated based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,


    /// 
    /// The group by field well of a word cloud. Values are grouped by group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,

}


/// The bound       options (north, south, west, east) of the geospatial window options.
#[derive(Default, serde::Serialize)]
pub struct GeospatialCoordinateBounds {


    /// 
    /// The latitude of the north bound of the geospatial coordinate bounds.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "North")]
    pub north: f64,


    /// 
    /// The longitude of the west bound of the geospatial coordinate bounds.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "West")]
    pub west: f64,


    /// 
    /// The longitude of the east bound of the geospatial coordinate bounds.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "East")]
    pub east: f64,


    /// 
    /// The latitude of the south bound of the geospatial coordinate bounds.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "South")]
    pub south: f64,

}


/// The range setup of a numeric axis display range.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct AxisDisplayRange {


    /// 
    /// The minimum and maximum setup of an axis display range.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayMinMaxRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinMax")]
    pub min_max: Option<AxisDisplayMinMaxRange>,


    /// 
    /// The data-driven setup of an axis display range.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataDriven")]
    pub data_driven: Option<serde_json::Value>,

}


/// The dimension type field.
#[derive(Default, serde::Serialize)]
pub struct DimensionField {


    /// 
    /// The dimension type field with date type columns.
    /// 
    /// Required: No
    ///
    /// Type: DateDimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateDimensionField")]
    pub date_dimension_field: Option<DateDimensionField>,


    /// 
    /// The dimension type field with categorical type columns.
    /// 
    /// Required: No
    ///
    /// Type: CategoricalDimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoricalDimensionField")]
    pub categorical_dimension_field: Option<CategoricalDimensionField>,


    /// 
    /// The dimension type field with numerical type columns.
    /// 
    /// Required: No
    ///
    /// Type: NumericalDimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericalDimensionField")]
    pub numerical_dimension_field: Option<NumericalDimensionField>,

}


/// The aggregated field well for a box plot.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotAggregatedFieldWells {


    /// 
    /// The group by field well of a box plot chart. Values are grouped based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,


    /// 
    /// The value field well of a box plot chart. Values are aggregated based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The option that specifies individual data values for labels.
#[derive(Default, serde::Serialize)]
pub struct DataPathLabelType {


    /// 
    /// The actual value of the field that is labeled.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,


    /// 
    /// The field ID of the field that the data label needs to be applied to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,


    /// 
    /// The visibility of the data label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The paginated report options for a table visual.
#[derive(Default, serde::Serialize)]
pub struct TablePaginatedReportOptions {


    /// 
    /// The visibility of printing table overflow across pages.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<String>,


    /// 
    /// The visibility of repeating header rows on each page.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<String>,

}


/// The side border options for a table.
#[derive(Default, serde::Serialize)]
pub struct TableSideBorderOptions {


    /// 
    /// The table border options of the top border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Top")]
    pub top: Option<TableBorderOptions>,


    /// 
    /// The table border options of the left border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Left")]
    pub left: Option<TableBorderOptions>,


    /// 
    /// The table border options of the right border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Right")]
    pub right: Option<TableBorderOptions>,


    /// 
    /// The table border options of the inner horizontal border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "InnerHorizontal")]
    pub inner_horizontal: Option<TableBorderOptions>,


    /// 
    /// The table border options of the bottom border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bottom")]
    pub bottom: Option<TableBorderOptions>,


    /// 
    /// The table border options of the inner vertical border.
    /// 
    /// Required: No
    ///
    /// Type: TableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "InnerVertical")]
    pub inner_vertical: Option<TableBorderOptions>,

}


/// The field well configuration of a scatter plot.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ScatterPlotFieldWells {


    /// 
    /// The aggregated field wells of a scatter plot. The x and y-axes of scatter plots with aggregated field wells are aggregated by category, label, or both.
    /// 
    /// Required: No
    ///
    /// Type: ScatterPlotCategoricallyAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScatterPlotCategoricallyAggregatedFieldWells")]
    pub scatter_plot_categorically_aggregated_field_wells: Option<ScatterPlotCategoricallyAggregatedFieldWells>,


    /// 
    /// The unaggregated field wells of a scatter plot. The x and y-axes of these scatter plots are       unaggregated.
    /// 
    /// Required: No
    ///
    /// Type: ScatterPlotUnaggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScatterPlotUnaggregatedFieldWells")]
    pub scatter_plot_unaggregated_field_wells: Option<ScatterPlotUnaggregatedFieldWells>,

}


/// The default values of the IntegerParameterDeclaration.
#[derive(Default, serde::Serialize)]
pub struct IntegerDefaultValues {


    /// 
    /// The static values of the IntegerDefaultValues.
    /// 
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,


    /// 
    /// The dynamic value of the IntegerDefaultValues. Different defaults are displayed according to users, groups, and values mapping.
    /// 
    /// Required: No
    ///
    /// Type: DynamicDefaultValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}


/// Configures the display properties of the given text.
#[derive(Default, serde::Serialize)]
pub struct FontConfiguration {


    /// 
    /// The option that determines the text display size.
    /// 
    /// Required: No
    ///
    /// Type: FontSize
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontSize")]
    pub font_size: Option<FontSize>,


    /// 
    /// Determines the appearance of decorative lines on the text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | UNDERLINE
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontDecoration")]
    pub font_decoration: Option<String>,


    /// 
    /// Determines the color of the text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,


    /// 
    /// The option that determines the text display weight, or boldness.
    /// 
    /// Required: No
    ///
    /// Type: FontWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontWeight")]
    pub font_weight: Option<FontWeight>,


    /// 
    /// Determines the text display face that is inherited by the given font family.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ITALIC | NORMAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontStyle")]
    pub font_style: Option<String>,

}


/// Formatting configuration for icon set.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingIconSet {


    /// 
    /// Determines the icon set type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BARS | CARET_UP_MINUS_DOWN | CHECK_X | FLAGS | FOUR_COLOR_ARROW | FOUR_GRAY_ARROW | PLUS_MINUS | THREE_CIRCLE | THREE_COLOR_ARROW | THREE_GRAY_ARROW | THREE_SHAPE
    ///
    /// Update requires: No interruption
    #[serde(rename = "IconSetType")]
    pub icon_set_type: Option<String>,


    /// 
    /// The expression that determines the formatting configuration for the icon set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

}


/// The arc configuration of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ArcConfiguration {


    /// 
    /// The options that determine the arc thickness of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LARGE | MEDIUM | SMALL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<String>,


    /// 
    /// The option that determines the arc angle of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArcAngle")]
    pub arc_angle: Option<f64>,

}


/// The map style options of the geospatial map.
#[derive(Default, serde::Serialize)]
pub struct GeospatialMapStyleOptions {


    /// 
    /// The base map style of the geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DARK_GRAY | IMAGERY | LIGHT_GRAY | STREET
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseMapStyle")]
    pub base_map_style: Option<String>,

}


/// The configuration of a word cloud visual.
#[derive(Default, serde::Serialize)]
pub struct WordCloudChartConfiguration {


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) for the word cloud category.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a word cloud visual.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WordCloudSortConfiguration>,


    /// 
    /// The options for a word cloud visual.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordCloudOptions")]
    pub word_cloud_options: Option<WordCloudOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WordCloudFieldWells>,

}


/// A text box.
#[derive(Default, serde::Serialize)]
pub struct SheetTextBox {


    /// 
    /// The content that is displayed in the text box.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 150000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: Option<String>,


    /// 
    /// The unique identifier for a text box. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have text boxes that share identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetTextBoxId")]
    pub sheet_text_box_id: String,

}


/// The metric comparison computation configuration.
#[derive(Default, serde::Serialize)]
pub struct MetricComparisonComputation {


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The field that is used in a metric comparison from value setup.
    /// 
    /// Required: Yes
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromValue")]
    pub from_value: MeasureField,


    /// 
    /// The field that is used in a metric comparison to value setup.
    /// 
    /// Required: Yes
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: MeasureField,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The options that determine the numeric separator configuration.
#[derive(Default, serde::Serialize)]
pub struct NumericSeparatorConfiguration {


    /// 
    /// The options that determine the thousands separator configuration.
    /// 
    /// Required: No
    ///
    /// Type: ThousandSeparatorOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThousandsSeparator")]
    pub thousands_separator: Option<ThousandSeparatorOptions>,


    /// 
    /// Determines the decimal separator.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMMA | DOT | SPACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalSeparator")]
    pub decimal_separator: Option<String>,

}


/// The field label type.
#[derive(Default, serde::Serialize)]
pub struct FieldLabelType {


    /// 
    /// The visibility of the field label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// Indicates the field that is targeted by the field       label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}


/// A tree map.
///
/// For more information, see Using tree maps in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct TreeMapVisual {


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TreeMapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TreeMapConfiguration>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// The table options for a pivot table visual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableOptions {


    /// 
    /// The table cell style of cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,


    /// 
    /// The visibility of the column names.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnNamesVisibility")]
    pub column_names_visibility: Option<String>,


    /// 
    /// The visibility of the single metric options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleMetricVisibility")]
    pub single_metric_visibility: Option<String>,


    /// 
    /// The table cell style of row field names.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowFieldNamesStyle")]
    pub row_field_names_style: Option<TableCellStyle>,


    /// 
    /// The table cell style of the column header.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHeaderStyle")]
    pub column_header_style: Option<TableCellStyle>,


    /// 
    /// The table cell style of the row headers.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowHeaderStyle")]
    pub row_header_style: Option<TableCellStyle>,


    /// 
    /// The metric placement (row, column) options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COLUMN | ROW
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricPlacement")]
    pub metric_placement: Option<String>,


    /// 
    /// The row alternate color options (widget status, row alternate colors).
    /// 
    /// Required: No
    ///
    /// Type: RowAlternateColorOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,


    /// 
    /// Determines the visibility of the pivot table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToggleButtonsVisibility")]
    pub toggle_buttons_visibility: Option<String>,

}


/// The configuration for default analysis settings.
#[derive(Default, serde::Serialize)]
pub struct AnalysisDefaults {


    /// 
    /// The configuration for default new sheet settings.
    /// 
    /// Required: Yes
    ///
    /// Type: DefaultNewSheetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultNewSheetConfiguration")]
    pub default_new_sheet_configuration: DefaultNewSheetConfiguration,

}


/// The table calculation measure field for pivot tables.
#[derive(Default, serde::Serialize)]
pub struct CalculatedMeasureField {


    /// 
    /// The expression in the table calculation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// A dataset parameter that is mapped to an analysis parameter.
#[derive(Default, serde::Serialize)]
pub struct MappedDataSetParameter {


    /// 
    /// The name of the dataset parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetParameterName")]
    pub data_set_parameter_name: String,


    /// 
    /// A unique name that identifies a dataset within the analysis or dashboard.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}


/// A grouping of individual filters. Filter groups are applied to the same group of visuals.
///
/// For more information, see Adding filter conditions (group filters) with AND and OR operators in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct FilterGroup {


    /// 
    /// The value that uniquely identifies a FilterGroup within a dashboard, template, or analysis.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterGroupId")]
    pub filter_group_id: String,


    /// 
    /// The filter new feature which can apply filter group to all data sets. Choose one of the following options:
    /// 
    /// ALL_DATASETS                                SINGLE_DATASET
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_DATASETS | SINGLE_DATASET
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossDataset")]
    pub cross_dataset: String,


    /// 
    /// The configuration that specifies what scope to apply to a FilterGroup.
    /// 
    /// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
    /// 
    /// Required: Yes
    ///
    /// Type: FilterScopeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScopeConfiguration")]
    pub scope_configuration: FilterScopeConfiguration,


    /// 
    /// The list of filters that are present in a FilterGroup.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Filter
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,


    /// 
    /// The status of the FilterGroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}


/// The options that determine the presentation of the data labels.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartDataLabelOptions {


    /// 
    /// The visibility option that determines if data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// Determines the style of the metric labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PERCENTAGE_BY_FIRST_STAGE | PERCENTAGE_BY_PREVIOUS_STAGE | VALUE_AND_PERCENTAGE_BY_FIRST_STAGE | VALUE_AND_PERCENTAGE_BY_PREVIOUS_STAGE | VALUE_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureDataLabelStyle")]
    pub measure_data_label_style: Option<String>,


    /// 
    /// The color of the data label text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,


    /// 
    /// The visibility of the measure labels within the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<String>,


    /// 
    /// Determines the positioning of the data label relative to a section of the funnel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOTTOM | INSIDE | LEFT | OUTSIDE | RIGHT | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<String>,


    /// 
    /// The font configuration for the data labels.
    /// 
    /// Only the FontSize attribute of the font configuration is used for data labels.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,


    /// 
    /// The visibility of the category labels within the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<String>,

}


/// The field wells of a word cloud visual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct WordCloudFieldWells {


    /// 
    /// The aggregated field wells of a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordCloudAggregatedFieldWells")]
    pub word_cloud_aggregated_field_wells: Option<WordCloudAggregatedFieldWells>,

}


/// The conditional formatting for the primary value of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIPrimaryValueConditionalFormatting {


    /// 
    /// The conditional formatting of the primary value's icon.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingIcon
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,


    /// 
    /// The conditional formatting of the primary value's text color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,

}


/// The window options of the geospatial map visual.
#[derive(Default, serde::Serialize)]
pub struct GeospatialWindowOptions {


    /// 
    /// The bounds options (north, south, west, east) of the geospatial window options.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialCoordinateBounds
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bounds")]
    pub bounds: Option<GeospatialCoordinateBounds>,


    /// 
    /// The map zoom modes (manual, auto) of the geospatial window options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | MANUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapZoomMode")]
    pub map_zoom_mode: Option<String>,

}


/// The options that determine the arc thickness of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ArcOptions {


    /// 
    /// The arc thickness of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LARGE | MEDIUM | SMALL | WHOLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<String>,

}


/// An aggregation function aggregates values from a dimension or measure.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct AggregationFunction {


    /// 
    /// Aggregation for categorical values.
    /// 
    /// COUNT: Aggregate by the total number of values, including duplicates.                        DISTINCT_COUNT: Aggregate by the total number of distinct values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COUNT | DISTINCT_COUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoricalAggregationFunction")]
    pub categorical_aggregation_function: Option<String>,


    /// 
    /// Aggregation for date values.
    /// 
    /// COUNT: Aggregate by the total number of values, including duplicates.                        DISTINCT_COUNT: Aggregate by the total number of distinct values.                        MIN: Select the smallest date value.                        MAX: Select the largest date value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COUNT | DISTINCT_COUNT | MAX | MIN
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateAggregationFunction")]
    pub date_aggregation_function: Option<String>,


    /// 
    /// Aggregation for numerical values.
    /// 
    /// Required: No
    ///
    /// Type: NumericalAggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericalAggregationFunction")]
    pub numerical_aggregation_function: Option<NumericalAggregationFunction>,

}


/// Formatting configuration for number fields.
#[derive(Default, serde::Serialize)]
pub struct NumberFormatConfiguration {


    /// 
    /// The options that determine the numeric format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NumericFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}


/// The options that determine the presentation of the data labels.
#[derive(Default, serde::Serialize)]
pub struct DataLabelOptions {


    /// 
    /// Determines the visibility of the measure field labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<String>,


    /// 
    /// Determines whether overlap is enabled or disabled for the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLE_OVERLAP | ENABLE_OVERLAP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overlap")]
    pub overlap: Option<String>,


    /// 
    /// Determines the color of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,


    /// 
    /// Determines the content of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PERCENT | VALUE | VALUE_AND_PERCENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelContent")]
    pub label_content: Option<String>,


    /// 
    /// The option that determines the data label type.
    /// 
    /// Required: No
    ///
    /// Type: List of DataLabelType
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabelTypes")]
    pub data_label_types: Option<Vec<DataLabelType>>,


    /// 
    /// Determines the visibility of the category field labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<String>,


    /// 
    /// Determines the position of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOTTOM | INSIDE | LEFT | OUTSIDE | RIGHT | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<String>,


    /// 
    /// Determines the font configuration of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,


    /// 
    /// Determines the visibility of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// A pie or donut chart.
///
/// The PieChartVisual structure describes a visual that is a member of the pie chart family.
///
/// The following charts can be described by using this structure:
///
/// For more information, see Using pie charts in the Amazon QuickSight User Guide.
///
/// For more information, see Using donut charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct PieChartVisual {


    /// 
    /// The configuration of a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: PieChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PieChartConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// The label configuration of a reference line.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineLabelConfiguration {


    /// 
    /// The font configuration of the label in a reference line.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,


    /// 
    /// The vertical position configuration of the label in a reference line. Choose one of the following options:
    /// 
    /// ABOVE                                BELOW
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ABOVE | BELOW
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalPosition")]
    pub vertical_position: Option<String>,


    /// 
    /// The horizontal position configuration of the label in a reference line. Choose one of       the following options:
    /// 
    /// LEFT                                CENTER                                RIGHT
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CENTER | LEFT | RIGHT
    ///
    /// Update requires: No interruption
    #[serde(rename = "HorizontalPosition")]
    pub horizontal_position: Option<String>,


    /// 
    /// The custom label configuration of the label in a reference line.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineCustomLabelConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabelConfiguration")]
    pub custom_label_configuration: Option<ReferenceLineCustomLabelConfiguration>,


    /// 
    /// The font color configuration of the label in a reference line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,


    /// 
    /// The value label configuration of the label in a reference line.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineValueLabelConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueLabelConfiguration")]
    pub value_label_configuration: Option<ReferenceLineValueLabelConfiguration>,

}


/// Determines whether or not hidden fields are visible on exported dashbaords.
#[derive(Default, serde::Serialize)]
pub struct ExportWithHiddenFieldsOption {


    /// 
    /// The status of the export with hidden fields options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The options that determine the default settings for interactive layout configuration.
#[derive(Default, serde::Serialize)]
pub struct DefaultInteractiveLayoutConfiguration {


    /// 
    /// The options that determine the default settings for a grid layout configuration.
    /// 
    /// Required: No
    ///
    /// Type: DefaultGridLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Grid")]
    pub grid: Option<DefaultGridLayoutConfiguration>,


    /// 
    /// The options that determine the default settings of a free-form layout configuration.
    /// 
    /// Required: No
    ///
    /// Type: DefaultFreeFormLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FreeForm")]
    pub free_form: Option<DefaultFreeFormLayoutConfiguration>,

}


/// Determines if hidden fields are included in an exported dashboard.
#[derive(Default, serde::Serialize)]
pub struct ExportHiddenFieldsOption {


    /// 
    /// The status of the export hidden fields options of a dashbaord.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The options for the canvas of a section-based layout.
#[derive(Default, serde::Serialize)]
pub struct SectionBasedLayoutCanvasSizeOptions {


    /// 
    /// The options for a paper canvas of a section-based layout.
    /// 
    /// Required: No
    ///
    /// Type: SectionBasedLayoutPaperCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaperCanvasSizeOptions")]
    pub paper_canvas_size_options: Option<SectionBasedLayoutPaperCanvasSizeOptions>,

}


/// A control to display a horizontal toggle bar. This is used to change a value by sliding the toggle.
#[derive(Default, serde::Serialize)]
pub struct FilterSliderControl {


    /// 
    /// The larger value that is displayed at the right of the slider.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,


    /// 
    /// The title of the FilterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The type of FilterSliderControl. Choose one of the following options:
    /// 
    /// SINGLE_POINT: Filter against(equals) a single data point.                        RANGE: Filter data that is in a specified range.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: RANGE | SINGLE_POINT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The source filter ID of the FilterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,


    /// 
    /// The number of increments that the slider bar is divided into.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepSize")]
    pub step_size: f64,


    /// 
    /// The smaller value that is displayed at the left of the slider.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: SliderControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,


    /// 
    /// The ID of the FilterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}


/// The option that determines the text display size.
#[derive(Default, serde::Serialize)]
pub struct FontSize {


    /// 
    /// The lexical name for the text size, proportional to its surrounding context.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EXTRA_LARGE | EXTRA_SMALL | LARGE | MEDIUM | SMALL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Relative")]
    pub relative: Option<String>,

}


/// The subtotal options.
#[derive(Default, serde::Serialize)]
pub struct SubtotalOptions {


    /// 
    /// The cell styling options for the subtotal cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,


    /// 
    /// The cell styling options for the subtotals of value cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,


    /// 
    /// The cell styling options for the subtotals of header cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,


    /// 
    /// The optional configuration of subtotal cells.
    /// 
    /// Required: No
    ///
    /// Type: List of PivotTableFieldSubtotalOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLevelOptions")]
    pub field_level_options: Option<Vec<PivotTableFieldSubtotalOptions>>,


    /// 
    /// The visibility configuration for the subtotal cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<String>,


    /// 
    /// The field level (all, custom, last) for the subtotal cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | CUSTOM | LAST
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLevel")]
    pub field_level: Option<String>,


    /// 
    /// The custom label string for the subtotal cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,

}


/// The arc axis range of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ArcAxisDisplayRange {


    /// 
    /// The maximum value of the arc axis range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    pub max: Option<f64>,


    /// 
    /// The minimum value of the arc axis range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}


/// A NumericRangeFilter filters values that are within the value range.
#[derive(Default, serde::Serialize)]
pub struct NumericRangeFilter {


    /// 
    /// The minimum value for the filter value range.
    /// 
    /// Required: No
    ///
    /// Type: NumericRangeFilterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: Option<NumericRangeFilterValue>,


    /// 
    /// Determines whether the maximum value in the filter value range should be included in the filtered results.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,


    /// 
    /// Select all of the values. Null is not the assigned value of select all.
    /// 
    /// FILTER_ALL_VALUES
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<String>,


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// The maximum value for the filter value range.
    /// 
    /// Required: No
    ///
    /// Type: NumericRangeFilterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: Option<NumericRangeFilterValue>,


    /// 
    /// Determines whether the minimum value in the filter value range should be included in the filtered results.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,


    /// 
    /// The aggregation function of the filter.
    /// 
    /// Required: No
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}


/// The options for configuring a donut chart or pie chart.
#[derive(Default, serde::Serialize)]
pub struct DonutOptions {


    /// 
    /// The label options of the label that is displayed in the center of a donut chart. This option isn't available for pie charts.
    /// 
    /// Required: No
    ///
    /// Type: DonutCenterOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DonutCenterOptions")]
    pub donut_center_options: Option<DonutCenterOptions>,


    /// 
    /// The option for define the arc of the chart shape. Valid values are as follows:
    /// 
    /// WHOLE - A pie chart                        SMALL- A small-sized donut chart                        MEDIUM- A medium-sized donut chart                        LARGE- A large-sized donut chart
    /// 
    /// Required: No
    ///
    /// Type: ArcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArcOptions")]
    pub arc_options: Option<ArcOptions>,

}


/// The logarithmic axis scale setup.
#[derive(Default, serde::Serialize)]
pub struct AxisLogarithmicScale {


    /// 
    /// The base setup of a logarithmic axis scale.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    pub base: Option<f64>,

}


/// The configuration of a CustomContentVisual.
#[derive(Default, serde::Serialize)]
pub struct CustomContentConfiguration {


    /// 
    /// The input URL that links to the custom content that you want in the custom visual.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentUrl")]
    pub content_url: Option<String>,


    /// 
    /// The sizing options for the size of the custom content visual. This structure is required when the ContentType of the visual is 'IMAGE'.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DO_NOT_SCALE | FIT_TO_HEIGHT | FIT_TO_WIDTH | SCALE_TO_VISUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageScaling")]
    pub image_scaling: Option<String>,


    /// 
    /// The content type of the custom content visual. You can use this to have the visual render as an image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IMAGE | OTHER_EMBEDDED_CONTENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

}


/// The configuration of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartConfiguration {


    /// 
    /// The field well configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GaugeChartFieldWells>,


    /// 
    /// The data label configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The options that determine the presentation of the GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "GaugeChartOptions")]
    pub gauge_chart_options: Option<GaugeChartOptions>,


    /// 
    /// The visual palette configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The tooltip configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TooltipOptions")]
    pub tooltip_options: Option<TooltipOptions>,

}


/// The options that determine the default settings of a free-form layout configuration.
#[derive(Default, serde::Serialize)]
pub struct DefaultFreeFormLayoutConfiguration {


    /// 
    /// Determines the screen canvas size options for a free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: FreeFormLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: FreeFormLayoutCanvasSizeOptions,

}


/// Export to .csv option.
#[derive(Default, serde::Serialize)]
pub struct ExportToCSVOption {


    /// 
    /// Availability status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The configuration that defines the default value of a DateTime parameter when a value has not been set.
#[derive(Default, serde::Serialize)]
pub struct DateTimeValueWhenUnsetConfiguration {


    /// 
    /// The built-in options for default values. The value can be one of the following:
    /// 
    /// RECOMMENDED: The recommended value.                        NULL: The NULL value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NULL | RECOMMENDED_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<String>,


    /// 
    /// A custom value that's used when the value of a parameter isn't set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}


/// A TimeEqualityFilter filters values that are equal to a given value.
#[derive(Default, serde::Serialize)]
pub struct TimeEqualityFilter {


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<String>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The parameter whose value should be used for the filter value.
    /// 
    /// This field is mutually exclusive to Value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,


    /// 
    /// The value of a TimeEquality filter.
    /// 
    /// This field is mutually exclusive to ParameterName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


/// The selected field options for the pivot table field options.
#[derive(Default, serde::Serialize)]
pub struct PivotTableFieldOption {


    /// 
    /// The field ID of the pivot table field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The custom label of the pivot table field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The visibility of the pivot table field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// A NumericEqualityFilter filters values that are equal to the specified value.
#[derive(Default, serde::Serialize)]
pub struct NumericEqualityFilter {


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// The parameter whose value should be used for the filter value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// Select all of the values. Null is not the assigned value of select all.
    /// 
    /// FILTER_ALL_VALUES
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<String>,


    /// 
    /// The input value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The match operator that is used to determine if a filter should be applied.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DOES_NOT_EQUAL | EQUALS
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchOperator")]
    pub match_operator: String,


    /// 
    /// The aggregation function of the filter.
    /// 
    /// Required: No
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,

}


/// The conditional formatting for the primary value of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartPrimaryValueConditionalFormatting {


    /// 
    /// The conditional formatting of the primary value text color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,


    /// 
    /// The conditional formatting of the primary value icon.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingIcon
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}


/// The sort configuration of a tree map.
#[derive(Default, serde::Serialize)]
pub struct TreeMapSortConfiguration {


    /// 
    /// The sort configuration of group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreeMapSort")]
    pub tree_map_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of groups that are displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreeMapGroupItemsLimitConfiguration")]
    pub tree_map_group_items_limit_configuration: Option<ItemsLimitConfiguration>,

}


/// The period over period computation configuration.
#[derive(Default, serde::Serialize)]
pub struct PeriodOverPeriodComputation {


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,

}


/// The configuration of a RadarChartVisual.
#[derive(Default, serde::Serialize)]
pub struct RadarChartConfiguration {


    /// 
    /// The shape of the radar chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CIRCLE | POLYGON
    ///
    /// Update requires: No interruption
    #[serde(rename = "Shape")]
    pub shape: Option<String>,


    /// 
    /// The sort configuration of a RadarChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<RadarChartSortConfiguration>,


    /// 
    /// The field well configuration of a RadarChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<RadarChartFieldWells>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The category axis of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The category label options of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The start angle of a radar chart's axis.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartAngle")]
    pub start_angle: Option<f64>,


    /// 
    /// The color of the even-numbered alternate bands of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternateBandEvenColor")]
    pub alternate_band_even_color: Option<String>,


    /// 
    /// The color axis of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorAxis")]
    pub color_axis: Option<AxisDisplayOptions>,


    /// 
    /// The color label options of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// Determines the visibility of the colors of alternatign bands in a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternateBandColorsVisibility")]
    pub alternate_band_colors_visibility: Option<String>,


    /// 
    /// The color of the odd-numbered alternate bands of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternateBandOddColor")]
    pub alternate_band_odd_color: Option<String>,


    /// 
    /// The base sreies settings of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartSeriesSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseSeriesSettings")]
    pub base_series_settings: Option<RadarChartSeriesSettings>,

}


/// The options that determine the presentation of a line series in the visual
#[derive(Default, serde::Serialize)]
pub struct LineChartSeriesSettings {


    /// 
    /// Line styles options for a line series in LineChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: LineChartLineStyleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,


    /// 
    /// Marker styles options for a line series in LineChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: LineChartMarkerStyleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,

}


/// A TimeRangeFilter filters values that are between two specified values.
#[derive(Default, serde::Serialize)]
pub struct TimeRangeFilter {


    /// 
    /// The minimum value for the filter value range.
    /// 
    /// Required: No
    ///
    /// Type: TimeRangeFilterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMinimumValue")]
    pub range_minimum_value: Option<TimeRangeFilterValue>,


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<String>,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The maximum value for the filter value range.
    /// 
    /// Required: No
    ///
    /// Type: TimeRangeFilterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMaximumValue")]
    pub range_maximum_value: Option<TimeRangeFilterValue>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// The exclude period of the time range filter.
    /// 
    /// Required: No
    ///
    /// Type: ExcludePeriodConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,


    /// 
    /// Determines whether the minimum value in the filter value range should be included in the filtered results.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// Determines whether the maximum value in the filter value range should be included in the filtered results.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,

}


/// The field well configuration of a FunnelChartVisual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartFieldWells {


    /// 
    /// The field well configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunnelChartAggregatedFieldWells")]
    pub funnel_chart_aggregated_field_wells: Option<FunnelChartAggregatedFieldWells>,

}


/// Provides the forecast to meet the target for a particular date.
#[derive(Default, serde::Serialize)]
pub struct WhatIfPointScenario {


    /// 
    /// The date that you need the forecast results for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Date")]
    pub date: String,


    /// 
    /// The target value that you want to meet for the provided date.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,

}


/// A collection of options that configure how each panel displays in a small multiples chart.
#[derive(Default, serde::Serialize)]
pub struct PanelConfiguration {


    /// 
    /// Sets the background color for each panel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}(?:[A-F0-9]{2})?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,


    /// 
    /// Sets the line style of panel borders.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DASHED | DOTTED | SOLID
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<String>,


    /// 
    /// Configures the title display within each small multiples panel.
    /// 
    /// Required: No
    ///
    /// Type: PanelTitleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<PanelTitleOptions>,


    /// 
    /// Sets the line thickness of panel borders.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderThickness")]
    pub border_thickness: Option<String>,


    /// 
    /// Determines whether or not negative space between sibling panels is rendered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "GutterVisibility")]
    pub gutter_visibility: Option<String>,


    /// 
    /// Sets the line color of panel borders.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}(?:[A-F0-9]{2})?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderColor")]
    pub border_color: Option<String>,


    /// 
    /// Determines whether or not each panel displays a border.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderVisibility")]
    pub border_visibility: Option<String>,


    /// 
    /// Sets the total amount of negative space to display between sibling panels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GutterSpacing")]
    pub gutter_spacing: Option<String>,


    /// 
    /// Determines whether or not a background for each small multiples panel is rendered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundVisibility")]
    pub background_visibility: Option<String>,

}


/// Configuration options for the canvas of a grid layout.
#[derive(Default, serde::Serialize)]
pub struct GridLayoutCanvasSizeOptions {


    /// 
    /// The options that determine the sizing of the canvas used in a grid layout.
    /// 
    /// Required: No
    ///
    /// Type: GridLayoutScreenCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<GridLayoutScreenCanvasSizeOptions>,

}


/// A visual displayed on a sheet in an analysis, dashboard, or template.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct Visual {


    /// 
    /// A scatter plot.
    /// 
    /// For more information, see Using scatter plots in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: ScatterPlotVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScatterPlotVisual")]
    pub scatter_plot_visual: Option<ScatterPlotVisual>,


    /// 
    /// A bar chart.
    /// 
    /// For more information, see Using bar charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: BarChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarChartVisual")]
    pub bar_chart_visual: Option<BarChartVisual>,


    /// 
    /// A funnel chart.
    /// 
    /// For more information, see Using funnel charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunnelChartVisual")]
    pub funnel_chart_visual: Option<FunnelChartVisual>,


    /// 
    /// A visual that contains custom content.
    /// 
    /// For more information, see Using custom visual content in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomContentVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomContentVisual")]
    pub custom_content_visual: Option<CustomContentVisual>,


    /// 
    /// A radar chart visual.
    /// 
    /// For more information, see Using radar charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "RadarChartVisual")]
    pub radar_chart_visual: Option<RadarChartVisual>,


    /// 
    /// A tree map.
    /// 
    /// For more information, see Using tree maps in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: TreeMapVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreeMapVisual")]
    pub tree_map_visual: Option<TreeMapVisual>,


    /// 
    /// A box plot.
    /// 
    /// For more information, see Using box plots in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "BoxPlotVisual")]
    pub box_plot_visual: Option<BoxPlotVisual>,


    /// 
    /// A sankey diagram.
    /// 
    /// For more information, see Using Sankey diagrams in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: SankeyDiagramVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "SankeyDiagramVisual")]
    pub sankey_diagram_visual: Option<SankeyDiagramVisual>,


    /// 
    /// A pivot table.
    /// 
    /// For more information, see Using pivot tables in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "PivotTableVisual")]
    pub pivot_table_visual: Option<PivotTableVisual>,


    /// 
    /// A combo chart.
    /// 
    /// For more information, see Using combo charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: ComboChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComboChartVisual")]
    pub combo_chart_visual: Option<ComboChartVisual>,


    /// 
    /// A word cloud.
    /// 
    /// For more information, see Using word clouds in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordCloudVisual")]
    pub word_cloud_visual: Option<WordCloudVisual>,


    /// 
    /// A gauge chart.
    /// 
    /// For more information, see Using gauge charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "GaugeChartVisual")]
    pub gauge_chart_visual: Option<GaugeChartVisual>,


    /// 
    /// A filled map.
    /// 
    /// For more information, see Creating filled maps in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilledMapVisual")]
    pub filled_map_visual: Option<FilledMapVisual>,


    /// 
    /// A waterfall chart.
    /// 
    /// For more information, see Using waterfall charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaterfallVisual")]
    pub waterfall_visual: Option<WaterfallVisual>,


    /// 
    /// A table visual.
    /// 
    /// For more information, see Using tables as visuals in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: TableVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableVisual")]
    pub table_visual: Option<TableVisual>,


    /// 
    /// A line chart.
    /// 
    /// For more information, see Using line charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: LineChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineChartVisual")]
    pub line_chart_visual: Option<LineChartVisual>,


    /// 
    /// An insight visual.
    /// 
    /// For more information, see Working with insights in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: InsightVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightVisual")]
    pub insight_visual: Option<InsightVisual>,


    /// 
    /// A pie or donut chart.
    /// 
    /// For more information, see Using pie charts in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PieChartVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "PieChartVisual")]
    pub pie_chart_visual: Option<PieChartVisual>,


    /// 
    /// An empty visual.
    /// 
    /// Required: No
    ///
    /// Type: EmptyVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmptyVisual")]
    pub empty_visual: Option<EmptyVisual>,


    /// 
    /// A geospatial map or a points on map visual.
    /// 
    /// For more information, see Creating point maps in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeospatialMapVisual")]
    pub geospatial_map_visual: Option<GeospatialMapVisual>,


    /// 
    /// A histogram.
    /// 
    /// For more information, see Using histograms in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: HistogramVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "HistogramVisual")]
    pub histogram_visual: Option<HistogramVisual>,


    /// 
    /// A key performance indicator (KPI).
    /// 
    /// For more information, see Using KPIs in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: KPIVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "KPIVisual")]
    pub kpivisual: Option<KPIVisual>,


    /// 
    /// A heat map.
    /// 
    /// For more information, see Using heat maps in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: HeatMapVisual
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapVisual")]
    pub heat_map_visual: Option<HeatMapVisual>,

}


/// The exclude period of TimeRangeFilter or RelativeDatesFilter.
#[derive(Default, serde::Serialize)]
pub struct ExcludePeriodConfiguration {


    /// 
    /// The granularity or unit (day, month, year) of the exclude period.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "Granularity")]
    pub granularity: String,


    /// 
    /// The status of the exclude period. Choose from the following options:
    /// 
    /// ENABLED                                DISABLED
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// The amount or number of the exclude period.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amount")]
    pub amount: f64,

}


/// A radar chart visual.
#[derive(Default, serde::Serialize)]
pub struct RadarChartVisual {


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<RadarChartConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// The configuration of a pie chart.
#[derive(Default, serde::Serialize)]
pub struct PieChartConfiguration {


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The small multiples setup for the visual.
    /// 
    /// Required: No
    ///
    /// Type: SmallMultiplesOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The label options of the group/color that is displayed in a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The options that determine the shape of the chart. This option determines whether the chart is a pie chart or a donut chart.
    /// 
    /// Required: No
    ///
    /// Type: DonutOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DonutOptions")]
    pub donut_options: Option<DonutOptions>,


    /// 
    /// The label options for the value that is displayed in a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: PieChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PieChartSortConfiguration>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The contribution analysis (anomaly configuration) setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: List of ContributionAnalysisDefault
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: PieChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PieChartFieldWells>,

}


/// The options that determine the numeric format configuration.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct NumericFormatConfiguration {


    /// 
    /// The options that determine the number display format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NumberDisplayFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,


    /// 
    /// The options that determine the percentage display format configuration.
    /// 
    /// Required: No
    ///
    /// Type: PercentageDisplayFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,


    /// 
    /// The options that determine the currency display format configuration.
    /// 
    /// Required: No
    ///
    /// Type: CurrencyDisplayFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrencyDisplayFormatConfiguration")]
    pub currency_display_format_configuration: Option<CurrencyDisplayFormatConfiguration>,

}


/// The value input pf the numeric range filter.
#[derive(Default, serde::Serialize)]
pub struct NumericRangeFilterValue {


    /// 
    /// The static value of the numeric range filter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValue")]
    pub static_value: Option<f64>,


    /// 
    /// The parameter that is used in the numeric range.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,

}


/// The field well configuration of a waterfall visual.
#[derive(Default, serde::Serialize)]
pub struct WaterfallChartAggregatedFieldWells {


    /// 
    /// The value field wells of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The breakdown field wells of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Breakdowns")]
    pub breakdowns: Option<Vec<DimensionField>>,


    /// 
    /// The category field wells of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Categories")]
    pub categories: Option<Vec<DimensionField>>,

}


/// Conditional formatting options of a FilledMapVisual.
#[derive(Default, serde::Serialize)]
pub struct FilledMapConditionalFormattingOption {


    /// 
    /// The conditional formatting that determines the shape of the filled map.
    /// 
    /// Required: Yes
    ///
    /// Type: FilledMapShapeConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "Shape")]
    pub shape: FilledMapShapeConditionalFormatting,

}


/// The display options for the axis label.
#[derive(Default, serde::Serialize)]
pub struct AxisDisplayOptions {


    /// 
    /// The tick label options of an axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisTickLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TickLabelOptions")]
    pub tick_label_options: Option<AxisTickLabelOptions>,


    /// 
    /// The offset value that determines the starting placement of the axis within a visual's bounds.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisOffset")]
    pub axis_offset: Option<String>,


    /// 
    /// Determines whether or not the axis line is visible.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisLineVisibility")]
    pub axis_line_visibility: Option<String>,


    /// 
    /// Determines whether or not the grid line is visible.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "GridLineVisibility")]
    pub grid_line_visibility: Option<String>,


    /// 
    /// The scroll bar options for an axis.
    /// 
    /// Required: No
    ///
    /// Type: ScrollBarOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScrollbarOptions")]
    pub scrollbar_options: Option<ScrollBarOptions>,


    /// 
    /// The data options for an axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDataOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataOptions")]
    pub data_options: Option<AxisDataOptions>,

}


/// The contents of a dashboard.
#[derive(Default, serde::Serialize)]
pub struct DashboardVersionDefinition {


    /// 
    /// The filter definitions for a dashboard.
    /// 
    /// For more information, see Filtering Data in Amazon QuickSight in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of FilterGroup
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,


    /// 
    /// An array of dataset identifier declarations. With       this mapping,you can use dataset identifiers instead of dataset Amazon Resource Names (ARNs) throughout the dashboard's sub-structures.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataSetIdentifierDeclaration
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifierDeclarations")]
    pub data_set_identifier_declarations: Vec<DataSetIdentifierDeclaration>,


    /// 
    /// The parameter declarations for a dashboard. Parameters are named variables that can transfer a value for use by an action or an object.
    /// 
    /// For more information, see Parameters in Amazon QuickSight in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ParameterDeclaration
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterDeclarations")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,


    /// 
    /// An array of dashboard-level column configurations. Column configurations       are used to set the default formatting for a column that       is used throughout a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnConfiguration
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnConfigurations")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AnalysisDefaults
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnalysisDefaults")]
    pub analysis_defaults: Option<AnalysisDefaults>,


    /// 
    /// An array of calculated field definitions for the dashboard.
    /// 
    /// Required: No
    ///
    /// Type: List of CalculatedField
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFields")]
    pub calculated_fields: Option<Vec<CalculatedField>>,


    /// 
    /// An array of sheet definitions for a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: List of SheetDefinition
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<SheetDefinition>>,

}


/// The configuration for applying a filter to specific sheets or visuals. You can apply this filter to multiple visuals that are on one sheet or to all visuals on a sheet.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct SelectedSheetsFilterScopeConfiguration {


    /// 
    /// The sheet ID and visual IDs of the sheet and visuals that the filter is applied to.
    /// 
    /// Required: No
    ///
    /// Type: List of SheetVisualScopingConfiguration
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetVisualScopingConfigurations")]
    pub sheet_visual_scoping_configurations: Option<Vec<SheetVisualScopingConfiguration>>,

}


/// The field sort options for a pivot table sort configuration.
#[derive(Default, serde::Serialize)]
pub struct PivotFieldSortOptions {


    /// 
    /// The field ID for the field sort options.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The sort by field for the field sort options.
    /// 
    /// Required: Yes
    ///
    /// Type: PivotTableSortBy
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortBy")]
    pub sort_by: PivotTableSortBy,

}


/// The options that determine the sizing of the canvas used in a grid layout.
#[derive(Default, serde::Serialize)]
pub struct GridLayoutScreenCanvasSizeOptions {


    /// 
    /// This value determines the layout behavior when the viewport is resized.
    /// 
    /// FIXED: A fixed width will be used when optimizing the layout. In           the Amazon QuickSight console, this option is called Classic.                        RESPONSIVE: The width of the canvas will be responsive and           optimized to the view port. In the Amazon QuickSight console, this option is called Tiled.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FIXED | RESPONSIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResizeOption")]
    pub resize_option: String,


    /// 
    /// The width that the view port will be optimized for when the layout renders.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: Option<String>,

}


/// A sheet is an object that contains a set of visuals that       are viewed together on one page in a paginated report. Every analysis and dashboard must contain at least one sheet.
#[derive(Default, serde::Serialize)]
pub struct SheetDefinition {


    /// 
    /// Layouts define how the components of a sheet are arranged.
    /// 
    /// For more information, see Types of layout in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Layout
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Layouts")]
    pub layouts: Option<Vec<Layout>>,


    /// 
    /// The control layouts of the sheet.
    /// 
    /// Required: No
    ///
    /// Type: List of SheetControlLayout
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetControlLayouts")]
    pub sheet_control_layouts: Option<Vec<SheetControlLayout>>,


    /// 
    /// The unique identifier of a sheet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetId")]
    pub sheet_id: String,


    /// 
    /// The title of the sheet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<String>,


    /// 
    /// A description of the sheet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the sheet. This name is displayed on the sheet's tab in the Amazon QuickSight       console.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The layout content type of the sheet. Choose one of the following options:
    /// 
    /// PAGINATED: Creates a sheet for a paginated report.                        INTERACTIVE: Creates a sheet for an interactive dashboard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERACTIVE | PAGINATED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,


    /// 
    /// A list of the visuals that are on a sheet. Visual placement is determined by the layout of the sheet.
    /// 
    /// Required: No
    ///
    /// Type: List of Visual
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visuals")]
    pub visuals: Option<Vec<Visual>>,


    /// 
    /// The list of filter controls that are on a sheet.
    /// 
    /// For more information, see Adding filter controls to analysis sheets in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of FilterControl
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControls")]
    pub filter_controls: Option<Vec<FilterControl>>,


    /// 
    /// The list of parameter controls that are on a sheet.
    /// 
    /// For more information, see Using a Control with a Parameter in Amazon QuickSight in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ParameterControl
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControls")]
    pub parameter_controls: Option<Vec<ParameterControl>>,


    /// 
    /// The text boxes that are on a sheet.
    /// 
    /// Required: No
    ///
    /// Type: List of SheetTextBox
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextBoxes")]
    pub text_boxes: Option<Vec<SheetTextBox>>,

}


/// Aggregation for numerical values.
#[derive(Default, serde::Serialize)]
pub struct NumericalAggregationFunction {


    /// 
    /// An aggregation based on the percentile of values in a dimension or measure.
    /// 
    /// Required: No
    ///
    /// Type: PercentileAggregation
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentileAggregation")]
    pub percentile_aggregation: Option<PercentileAggregation>,


    /// 
    /// Built-in aggregation functions for numerical values.
    /// 
    /// SUM: The sum of a dimension or measure.                         AVERAGE: The average of a dimension or measure.                        MIN: The minimum value of a dimension or measure.                        MAX: The maximum value of a dimension or measure.                        COUNT: The count of a dimension or measure.                        DISTINCT_COUNT: The count of distinct values in a dimension or measure.                        VAR: The variance of a dimension or measure.                        VARP: The partitioned variance of a dimension or measure.                        STDEV: The standard deviation of a dimension or measure.                        STDEVP: The partitioned standard deviation of a dimension or measure.                        MEDIAN: The median value of a dimension or measure.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | DISTINCT_COUNT | MAX | MEDIAN | MIN | STDEV | STDEVP | SUM | VAR | VARP
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleNumericalAggregation")]
    pub simple_numerical_aggregation: Option<String>,

}


/// The configuration for a FilledMapVisual.
#[derive(Default, serde::Serialize)]
pub struct FilledMapConfiguration {


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The sort configuration of a FilledMapVisual.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FilledMapSortConfiguration>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FilledMapFieldWells>,


    /// 
    /// The map style options of the filled map visual.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapStyleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The window options of the filled map visual.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialWindowOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,

}


/// The conditional formatting of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIConditionalFormatting {


    /// 
    /// The conditional formatting options of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: List of KPIConditionalFormattingOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<KPIConditionalFormattingOption>>,

}


/// The field well configuration of a FunnelChartVisual.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartAggregatedFieldWells {


    /// 
    /// The value field wells of a funnel chart. Values are aggregated based on categories.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The category field wells of a funnel chart. Values are grouped by category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}


/// Allows data paths to be sorted by a specific data value.
#[derive(Default, serde::Serialize)]
pub struct DataPathSort {


    /// 
    /// Determines the sort direction.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ASC | DESC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: String,


    /// 
    /// The list of data paths that need to be sorted.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataPathValue
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortPaths")]
    pub sort_paths: Vec<DataPathValue>,

}


/// The series axis configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct LineSeriesAxisDisplayOptions {


    /// 
    /// The options that determine the presentation of the line series axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisOptions")]
    pub axis_options: Option<AxisDisplayOptions>,


    /// 
    /// The configuration options that determine how missing data is treated during the rendering of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: List of MissingDataConfiguration
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MissingDataConfigurations")]
    pub missing_data_configurations: Option<Vec<MissingDataConfiguration>>,

}


/// A control to display a dropdown list with buttons that are used to select a single value.
#[derive(Default, serde::Serialize)]
pub struct ParameterDropDownControl {


    /// 
    /// The source parameter name of the ParameterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,


    /// 
    /// The values that are displayed in a control can be configured to only show values that are valid based on what's selected in other controls.
    /// 
    /// Required: No
    ///
    /// Type: CascadingControlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,


    /// 
    /// The title of the ParameterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// A list of selectable values that are used in a control.
    /// 
    /// Required: No
    ///
    /// Type: ParameterSelectableValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,


    /// 
    /// The ID of the ParameterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: DropDownControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,


    /// 
    /// The type parameter name of the ParameterDropDownControl.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_SELECT | SINGLE_SELECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// A data set.
#[derive(Default, serde::Serialize)]
pub struct DataSetIdentifierDeclaration {


    /// 
    /// The Amazon Resource Name (ARN) of the data set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,


    /// 
    /// The identifier of the data set, typically the data set's name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifier")]
    pub identifier: String,

}


/// The series item configuration of a line chart.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct SeriesItem {


    /// 
    /// The data field series item configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: DataFieldSeriesItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataFieldSeriesItem")]
    pub data_field_series_item: Option<DataFieldSeriesItem>,


    /// 
    /// The field series item configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: FieldSeriesItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldSeriesItem")]
    pub field_series_item: Option<FieldSeriesItem>,

}


/// The text format for a subtitle.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct LongFormatText {


    /// 
    /// Plain text format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,


    /// 
    /// Rich text. Examples of rich text include bold, underline, and italics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,

}


/// The configuration of a sankey diagram.
#[derive(Default, serde::Serialize)]
pub struct SankeyDiagramChartConfiguration {


    /// 
    /// The field well configuration of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: SankeyDiagramFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<SankeyDiagramFieldWells>,


    /// 
    /// The sort configuration of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: SankeyDiagramSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<SankeyDiagramSortConfiguration>,


    /// 
    /// The data label configuration of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}


/// The configuration of the Select all options in a list control.
#[derive(Default, serde::Serialize)]
pub struct ListControlSelectAllOptions {


    /// 
    /// The visibility configuration of the Select all options in a list control.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The option that determines the text display weight, or boldness.
#[derive(Default, serde::Serialize)]
pub struct FontWeight {


    /// 
    /// The lexical name for the level of boldness of the text display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOLD | NORMAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The field wells of a GeospatialMapVisual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct GeospatialMapFieldWells {


    /// 
    /// The aggregated field well for a geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeospatialMapAggregatedFieldWells")]
    pub geospatial_map_aggregated_field_wells: Option<GeospatialMapAggregatedFieldWells>,

}


/// The field well configuration of a sankey diagram.
#[derive(Default, serde::Serialize)]
pub struct SankeyDiagramFieldWells {


    /// 
    /// The field well configuration of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: SankeyDiagramAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "SankeyDiagramAggregatedFieldWells")]
    pub sankey_diagram_aggregated_field_wells: Option<SankeyDiagramAggregatedFieldWells>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct TextAreaControlDisplayOptions {


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,


    /// 
    /// The configuration of the placeholder options in a text area control.
    /// 
    /// Required: No
    ///
    /// Type: TextControlPlaceholderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}


/// The configuration of the placeholder options in a text control.
#[derive(Default, serde::Serialize)]
pub struct TextControlPlaceholderOptions {


    /// 
    /// The visibility configuration of the placeholder options in a text control.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The minimum and maximum setup for an axis display range.
#[derive(Default, serde::Serialize)]
pub struct AxisDisplayMinMaxRange {


    /// 
    /// The maximum setup for an axis display range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Maximum")]
    pub maximum: Option<f64>,


    /// 
    /// The minimum setup for an axis display range.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minimum")]
    pub minimum: Option<f64>,

}


/// The drill down options for data points in a dashbaord.
#[derive(Default, serde::Serialize)]
pub struct DataPointDrillUpDownOption {


    /// 
    /// The status of the drill down options of data points.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// A waterfall chart.
///
/// For more information, see Using waterfall charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct WaterfallVisual {


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The configuration for a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WaterfallChartConfiguration>,

}


/// Determines the icon display configuration.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingIconDisplayConfiguration {


    /// 
    /// Determines the icon display configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ICON_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "IconDisplayOption")]
    pub icon_display_option: Option<String>,

}


/// The growth rate computation configuration.
#[derive(Default, serde::Serialize)]
pub struct GrowthRateComputation {


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,


    /// 
    /// The period size setup of a growth rate computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 2
    ///
    /// Maximum: 52
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodSize")]
    pub period_size: Option<f64>,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,

}


/// The measure type field with date type columns.
#[derive(Default, serde::Serialize)]
pub struct DateMeasureField {


    /// 
    /// The column that is used in the DateMeasureField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The aggregation function of the measure field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COUNT | DISTINCT_COUNT | MAX | MIN
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<String>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct SliderControlDisplayOptions {


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}


/// A visual that contains custom content.
///
/// For more information, see Using custom visual content in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct CustomContentVisual {


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The dataset that is used to create the custom content visual. You can't create a visual without a dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,


    /// 
    /// The configuration of a CustomContentVisual.
    /// 
    /// Required: No
    ///
    /// Type: CustomContentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<CustomContentConfiguration>,

}


/// The options that determine the negative value configuration.
#[derive(Default, serde::Serialize)]
pub struct NegativeValueConfiguration {


    /// 
    /// Determines the display mode of the negative value configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NEGATIVE | POSITIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayMode")]
    pub display_mode: String,

}


/// The limit configuration of the visual display for an axis.
#[derive(Default, serde::Serialize)]
pub struct ItemsLimitConfiguration {


    /// 
    /// The limit on how many items of a field are showed in the chart. For       example, the number of slices that are displayed in a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ItemsLimit")]
    pub items_limit: Option<f64>,


    /// 
    /// The Show         other of an axis in the chart. Choose one of the following options:
    /// 
    /// INCLUDE                                EXCLUDE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EXCLUDE | INCLUDE
    ///
    /// Update requires: No interruption
    #[serde(rename = "OtherCategories")]
    pub other_categories: Option<String>,

}


/// The filter operation that filters data included in a visual or in an entire sheet.
#[derive(Default, serde::Serialize)]
pub struct CustomActionFilterOperation {


    /// 
    /// The configuration that chooses the target visuals to be filtered.
    /// 
    /// Required: Yes
    ///
    /// Type: FilterOperationTargetVisualsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetVisualsConfiguration")]
    pub target_visuals_configuration: FilterOperationTargetVisualsConfiguration,


    /// 
    /// The configuration that chooses the fields to be filtered.
    /// 
    /// Required: Yes
    ///
    /// Type: FilterOperationSelectedFieldsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedFieldsConfiguration")]
    pub selected_fields_configuration: FilterOperationSelectedFieldsConfiguration,

}


/// Dashboard source entity.
#[derive(Default, serde::Serialize)]
pub struct DashboardSourceEntity {


    /// 
    /// Source template.
    /// 
    /// Required: No
    ///
    /// Type: DashboardSourceTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceTemplate")]
    pub source_template: Option<DashboardSourceTemplate>,

}


/// The custom text content (value, font configuration) for the table link content configuration.
#[derive(Default, serde::Serialize)]
pub struct TableFieldCustomTextContent {


    /// 
    /// The string value of the custom text content for the table URL link content.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The font configuration of the custom text content for the table URL link content.
    /// 
    /// Required: Yes
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: FontConfiguration,

}


/// The display options for the visual tooltip.
#[derive(Default, serde::Serialize)]
pub struct TooltipOptions {


    /// 
    /// Determines whether or not the tooltip is visible.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TooltipVisibility")]
    pub tooltip_visibility: Option<String>,


    /// 
    /// The selected type for the tooltip. Choose one of the following options:
    /// 
    /// BASIC: A basic tooltip.                        DETAILED: A detailed tooltip.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BASIC | DETAILED
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedTooltipType")]
    pub selected_tooltip_type: Option<String>,


    /// 
    /// The setup for the detailed tooltip. The tooltip setup is always saved. The display type is decided based on the tooltip type.
    /// 
    /// Required: No
    ///
    /// Type: FieldBasedTooltip
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldBasedTooltip")]
    pub field_based_tooltip: Option<FieldBasedTooltip>,

}


/// The point style of the geospatial map.
#[derive(Default, serde::Serialize)]
pub struct GeospatialPointStyleOptions {


    /// 
    /// The selected point styles (point, cluster) of the geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLUSTER | POINT
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedPointStyle")]
    pub selected_point_style: Option<String>,


    /// 
    /// The cluster marker configuration of the geospatial point style.
    /// 
    /// Required: No
    ///
    /// Type: ClusterMarkerConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterMarkerConfiguration")]
    pub cluster_marker_configuration: Option<ClusterMarkerConfiguration>,

}


/// The date configuration of the filter.
#[derive(Default, serde::Serialize)]
pub struct AnchorDateConfiguration {


    /// 
    /// The options for the date configuration. Choose one of the options below:
    /// 
    /// NOW
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NOW
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnchorOption")]
    pub anchor_option: Option<String>,


    /// 
    /// The name of the parameter that is used for the anchor date configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}


/// The sort configuration of a ComboChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ComboChartSortConfiguration {


    /// 
    /// The item limit configuration of the color field well in a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The item limit configuration for the category field well of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of the color field well in a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The sort configuration of the category field well in a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}


/// An integer parameter.
#[derive(Default, serde::Serialize)]
pub struct IntegerParameter {


    /// 
    /// The name of the integer parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The values for the integer parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<f64>,

}


/// The tick label options of an axis.
#[derive(Default, serde::Serialize)]
pub struct AxisTickLabelOptions {


    /// 
    /// The rotation angle of the axis tick labels.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationAngle")]
    pub rotation_angle: Option<f64>,


    /// 
    /// Determines whether or not the axis ticks are visible.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelOptions")]
    pub label_options: Option<LabelOptions>,

}


/// The conditional formatting for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct TableConditionalFormatting {


    /// 
    /// Conditional formatting options for a PivotTableVisual.
    /// 
    /// Required: No
    ///
    /// Type: List of TableConditionalFormattingOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<TableConditionalFormattingOption>>,

}


/// The options that determine how a date axis is displayed.
#[derive(Default, serde::Serialize)]
pub struct DateAxisOptions {


    /// 
    /// Determines whether or not missing dates are displayed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MissingDateVisibility")]
    pub missing_date_visibility: Option<String>,

}


/// The options that determine the presentation of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIOptions {


    /// 
    /// The options that determine the presentation of trend arrows in a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: TrendArrowOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrendArrows")]
    pub trend_arrows: Option<TrendArrowOptions>,


    /// 
    /// The comparison configuration of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: ComparisonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,


    /// 
    /// The options that determine the presentation of the secondary value of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: SecondaryValueOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryValue")]
    pub secondary_value: Option<SecondaryValueOptions>,


    /// 
    /// The options that determine the primary value display type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTUAL | COMPARISON | HIDDEN
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<String>,


    /// 
    /// The options that determine the secondary value font configuration.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryValueFontConfiguration")]
    pub secondary_value_font_configuration: Option<FontConfiguration>,


    /// 
    /// The options that determine the primary value font configuration.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,


    /// 
    /// The options that determine the presentation of the progress bar of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: ProgressBarOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<ProgressBarOptions>,

}


/// The options that determine the number display format configuration.
#[derive(Default, serde::Serialize)]
pub struct NumberDisplayFormatConfiguration {


    /// 
    /// The options that determine the null value format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NullValueFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,


    /// 
    /// The options that determine the numeric separator configuration.
    /// 
    /// Required: No
    ///
    /// Type: NumericSeparatorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,


    /// 
    /// Determines the suffix value of the number format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// Determines the prefix value of the number format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The options that determine the negative value configuration.
    /// 
    /// Required: No
    ///
    /// Type: NegativeValueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,


    /// 
    /// The option that determines the decimal places configuration.
    /// 
    /// Required: No
    ///
    /// Type: DecimalPlacesConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,


    /// 
    /// Determines the number scale value of the number format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | BILLIONS | MILLIONS | NONE | THOUSANDS | TRILLIONS
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<String>,

}


/// The sort configuration of a sankey diagram.
#[derive(Default, serde::Serialize)]
pub struct SankeyDiagramSortConfiguration {


    /// 
    /// The sort configuration of the weight fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightSort")]
    pub weight_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of source nodes that are displayed in a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceItemsLimit")]
    pub source_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The limit on the number of destination nodes that are displayed in a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationItemsLimit")]
    pub destination_items_limit: Option<ItemsLimitConfiguration>,

}


/// The field well configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct LineChartAggregatedFieldWells {


    /// 
    /// The color field wells of a line chart. Values are grouped by category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,


    /// 
    /// The value field wells of a line chart. Values are aggregated based on categories.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The category field wells of a line chart. Values are grouped by category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,


    /// 
    /// The small multiples field well of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}


/// The configured style settings of a radar chart.
#[derive(Default, serde::Serialize)]
pub struct RadarChartAreaStyleSettings {


    /// 
    /// The visibility settings of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The forecast computation configuration.
#[derive(Default, serde::Serialize)]
pub struct ForecastComputation {


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The custom seasonality value setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 180
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomSeasonalityValue")]
    pub custom_seasonality_value: Option<f64>,


    /// 
    /// The seasonality setup of a forecast computation. Choose one of the following options:
    /// 
    /// AUTOMATIC                                CUSTOM: Checks the custom seasonality value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | CUSTOM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<String>,


    /// 
    /// The upper boundary setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The lower boundary setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The periods forward setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,


    /// 
    /// The prediction interval setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 50
    ///
    /// Maximum: 95
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,


    /// 
    /// The periods backward setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,

}


/// The configuration for a waterfall visual.
#[derive(Default, serde::Serialize)]
pub struct WaterfallChartConfiguration {


    /// 
    /// The legend configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The visual palette configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The data label configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The options that determine the presentation of the category axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxisLabelOptions")]
    pub category_axis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The field well configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WaterfallChartFieldWells>,


    /// 
    /// The options that determine the presentation of the y-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The options that determine the presentation of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallChartOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaterfallChartOptions")]
    pub waterfall_chart_options: Option<WaterfallChartOptions>,


    /// 
    /// The options that determine the presentation of the y-axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WaterfallChartSortConfiguration>,


    /// 
    /// The options that determine the presentation of the category axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxisDisplayOptions")]
    pub category_axis_display_options: Option<AxisDisplayOptions>,

}


/// A parameter declaration for the DateTime data type.
#[derive(Default, serde::Serialize)]
pub struct DateTimeParameterDeclaration {


    /// 
    /// The configuration that defines the default value of a DateTime parameter when a value has not been set.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeValueWhenUnsetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DateTimeValueWhenUnsetConfiguration>,


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of MappedDataSetParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,


    /// 
    /// The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeDefaultValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DateTimeDefaultValues>,


    /// 
    /// The name of the parameter that is being declared.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// The pagination configuration for a table visual or boxplot.
#[derive(Default, serde::Serialize)]
pub struct PaginationConfiguration {


    /// 
    /// Indicates how many items render in one page.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageSize")]
    pub page_size: f64,


    /// 
    /// Indicates the page number.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageNumber")]
    pub page_number: f64,

}


/// The configuration of a ComboChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ComboChartConfiguration {


    /// 
    /// The sort configuration of a ComboChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ComboChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<ComboChartSortConfiguration>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of a combo chart's secondary y-axis(line) field well.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// The data label options for a bar in a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarDataLabels")]
    pub bar_data_labels: Option<DataLabelOptions>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of a combo chart's color field well.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The label display options (grid line, range, scale, and axis step) of a combo chart's primary y-axis (bar) field well.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// Determines the bar arrangement in a combo chart. The following are valid values in this structure:
    /// 
    /// CLUSTERED: For clustered bar combo charts.                        STACKED: For stacked bar combo charts.                        STACKED_PERCENT: Do not use. If you use this value, the operation returns a validation error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLUSTERED | STACKED | STACKED_PERCENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<String>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of a combo chart category (group/color) field well.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of a combo chart's primary y-axis (bar) field well.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The reference line setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: List of ReferenceLine
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,


    /// 
    /// The category axis of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// The data label options for a line in a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineDataLabels")]
    pub line_data_labels: Option<DataLabelOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: ComboChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ComboChartFieldWells>,


    /// 
    /// The label display options (grid line, range, scale, axis step) of a combo chart's secondary y-axis (line) field well.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<AxisDisplayOptions>,

}


/// The control of a parameter that users can interact with in a dashboard or an analysis.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ParameterControl {


    /// 
    /// A control to display a text box that is used to enter multiple entries.
    /// 
    /// Required: No
    ///
    /// Type: ParameterTextAreaControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextArea")]
    pub text_area: Option<ParameterTextAreaControl>,


    /// 
    /// A control to display a list with buttons or boxes that are used to select either a single value or multiple values.
    /// 
    /// Required: No
    ///
    /// Type: ParameterListControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "List")]
    pub list: Option<ParameterListControl>,


    /// 
    /// A control to display a dropdown list with buttons that are used to select a single value.
    /// 
    /// Required: No
    ///
    /// Type: ParameterDropDownControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<ParameterDropDownControl>,


    /// 
    /// A control to display a horizontal toggle bar. This is used to change a value by sliding the toggle.
    /// 
    /// Required: No
    ///
    /// Type: ParameterSliderControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slider")]
    pub slider: Option<ParameterSliderControl>,


    /// 
    /// A control to display a text box that is used to enter a single entry.
    /// 
    /// Required: No
    ///
    /// Type: ParameterTextFieldControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextField")]
    pub text_field: Option<ParameterTextFieldControl>,


    /// 
    /// A control from a date parameter that specifies date and time.
    /// 
    /// Required: No
    ///
    /// Type: ParameterDateTimePickerControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<ParameterDateTimePickerControl>,

}


/// A funnel chart.
///
/// For more information, see Using funnel charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartVisual {


    /// 
    /// The configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FunnelChartConfiguration>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}


/// The border options for a table border.
#[derive(Default, serde::Serialize)]
pub struct TableBorderOptions {


    /// 
    /// The style (none, solid) of a table border.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SOLID
    ///
    /// Update requires: No interruption
    #[serde(rename = "Style")]
    pub style: Option<String>,


    /// 
    /// The thickness of a table border.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,


    /// 
    /// The color of a table border.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// The options that determine the presentation of a waterfall visual.
#[derive(Default, serde::Serialize)]
pub struct WaterfallChartOptions {


    /// 
    /// This option determines the total bar label of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalBarLabel")]
    pub total_bar_label: Option<String>,

}


/// The cluster marker that is a part of the cluster marker       configuration.
#[derive(Default, serde::Serialize)]
pub struct ClusterMarker {


    /// 
    /// The simple cluster marker of the cluster marker.
    /// 
    /// Required: No
    ///
    /// Type: SimpleClusterMarker
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleClusterMarker")]
    pub simple_cluster_marker: Option<SimpleClusterMarker>,

}


/// The option that determines the hierarchy of the fields for a visual element.
#[derive(Default, serde::Serialize)]
pub struct ColumnHierarchy {


    /// 
    /// The option that determines the hierarchy of the fields that are defined during data preparation. These fields are available to use in any analysis that uses the data source.
    /// 
    /// Required: No
    ///
    /// Type: PredefinedHierarchy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedHierarchy")]
    pub predefined_hierarchy: Option<PredefinedHierarchy>,


    /// 
    /// The option that determines the hierarchy of the fields that are built within a visual's field wells. These fields can't be duplicated to other visuals.
    /// 
    /// Required: No
    ///
    /// Type: ExplicitHierarchy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplicitHierarchy")]
    pub explicit_hierarchy: Option<ExplicitHierarchy>,


    /// 
    /// The option that determines the hierarchy of any DateTime fields.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeHierarchy
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeHierarchy")]
    pub date_time_hierarchy: Option<DateTimeHierarchy>,

}


/// An aggregation based on the percentile of values in a dimension or measure.
#[derive(Default, serde::Serialize)]
pub struct PercentileAggregation {


    /// 
    /// The percentile value. This value can be any numeric constant 0–100. A percentile value of 50 computes the median value of the measure.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentileValue")]
    pub percentile_value: Option<f64>,

}


/// The aggregated field wells of a bar chart.
#[derive(Default, serde::Serialize)]
pub struct BarChartAggregatedFieldWells {


    /// 
    /// The small multiples field well of a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,


    /// 
    /// The category (y-axis) field well of a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,


    /// 
    /// The color (group/color) field well of a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,


    /// 
    /// The value field wells of a bar chart. Values are aggregated by       category.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// A heat map.
///
/// For more information, see Using heat maps in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct HeatMapVisual {


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The configuration of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: HeatMapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HeatMapConfiguration>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}


/// The configuration of spacing (often a margin or padding).
#[derive(Default, serde::Serialize)]
pub struct Spacing {


    /// 
    /// Define the bottom spacing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bottom")]
    pub bottom: Option<String>,


    /// 
    /// Define the left spacing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Left")]
    pub left: Option<String>,


    /// 
    /// Define the right spacing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Right")]
    pub right: Option<String>,


    /// 
    /// Define the top spacing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Top")]
    pub top: Option<String>,

}


/// Marker styles options for a line series in LineChartVisual.
#[derive(Default, serde::Serialize)]
pub struct LineChartMarkerStyleSettings {


    /// 
    /// Color of marker in the series.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerColor")]
    pub marker_color: Option<String>,


    /// 
    /// Configuration option that determines whether to show the markers in the series.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerVisibility")]
    pub marker_visibility: Option<String>,


    /// 
    /// Size of marker in the series.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerSize")]
    pub marker_size: Option<String>,


    /// 
    /// Shape option for markers in the series.
    /// 
    /// CIRCLE: Show marker as a circle.                        TRIANGLE: Show marker as a triangle.                        SQUARE: Show marker as a square.                        DIAMOND: Show marker as a diamond.                        ROUNDED_SQUARE: Show marker as a rounded square.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CIRCLE | DIAMOND | ROUNDED_SQUARE | SQUARE | TRIANGLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerShape")]
    pub marker_shape: Option<String>,

}


/// The configuration of selected fields in theCustomActionFilterOperation.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FilterOperationSelectedFieldsConfiguration {


    /// 
    /// Chooses the fields that are filtered in CustomActionFilterOperation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedFields")]
    pub selected_fields: Option<Vec<String>>,


    /// 
    /// A structure that contains the options that choose which fields are filtered in the CustomActionFilterOperation.
    /// 
    /// Valid values are defined as follows:
    /// 
    /// ALL_FIELDS: Applies the filter operation to all fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_FIELDS
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<String>,

}


/// The sort configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct LineChartSortConfiguration {


    /// 
    /// The sort configuration of the small multiples field.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of categories that are displayed in a line chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimitConfiguration")]
    pub category_items_limit_configuration: Option<ItemsLimitConfiguration>,


    /// 
    /// The limit on the number of lines that are displayed in a line chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorItemsLimitConfiguration")]
    pub color_items_limit_configuration: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of the category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of small multiples panels that are displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}


/// A custom action defined on a visual.
#[derive(Default, serde::Serialize)]
pub struct VisualCustomAction {


    /// 
    /// The name of the VisualCustomAction.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The trigger of the VisualCustomAction.
    /// 
    /// Valid values are defined as follows:
    /// 
    /// DATA_POINT_CLICK: Initiates a custom action by a left pointer click on a data point.                        DATA_POINT_MENU: Initiates a custom action by right pointer click from the menu.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DATA_POINT_CLICK | DATA_POINT_MENU
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trigger")]
    pub trigger: String,


    /// 
    /// The ID of the VisualCustomAction.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomActionId")]
    pub custom_action_id: String,


    /// 
    /// A list of VisualCustomActionOperations.
    /// 
    /// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
    /// 
    /// Required: Yes
    ///
    /// Type: List of VisualCustomActionOperation
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionOperations")]
    pub action_operations: Vec<VisualCustomActionOperation>,


    /// 
    /// The status of the VisualCustomAction.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}


/// The tooltip item for the columns that are not part of a field well.
#[derive(Default, serde::Serialize)]
pub struct ColumnTooltipItem {


    /// 
    /// The visibility of the tooltip item.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The aggregation function of the column tooltip item.
    /// 
    /// Required: No
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    pub aggregation: Option<AggregationFunction>,


    /// 
    /// The target column of the tooltip item.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The label of the tooltip item.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,

}


/// The sort configuration of a RadarChartVisual.
#[derive(Default, serde::Serialize)]
pub struct RadarChartSortConfiguration {


    /// 
    /// The category items limit for a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The category sort options of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The color items limit of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The color sort configuration of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,

}


/// The configuration of a body section.
#[derive(Default, serde::Serialize)]
pub struct BodySectionConfiguration {


    /// 
    /// The configuration of content in a body section.
    /// 
    /// Required: Yes
    ///
    /// Type: BodySectionContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: BodySectionContent,


    /// 
    /// The configuration of a page break for a section.
    /// 
    /// Required: No
    ///
    /// Type: SectionPageBreakConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageBreakConfiguration")]
    pub page_break_configuration: Option<SectionPageBreakConfiguration>,


    /// 
    /// The style options of a body section.
    /// 
    /// Required: No
    ///
    /// Type: SectionStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,


    /// 
    /// The unique identifier of a body section.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SectionId")]
    pub section_id: String,

}


/// The sort configuration of a word cloud visual.
#[derive(Default, serde::Serialize)]
pub struct WordCloudSortConfiguration {


    /// 
    /// The limit on the number of groups that are displayed in a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}


/// The field wells for a pivot table visual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct PivotTableFieldWells {


    /// 
    /// The aggregated field well for the pivot table.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "PivotTableAggregatedFieldWells")]
    pub pivot_table_aggregated_field_wells: Option<PivotTableAggregatedFieldWells>,

}


/// The options that determine the presentation of histogram bins.
#[derive(Default, serde::Serialize)]
pub struct HistogramBinOptions {


    /// 
    /// The options that determine the bin width of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: BinWidthOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "BinWidth")]
    pub bin_width: Option<BinWidthOptions>,


    /// 
    /// The options that determine the selected bin type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BIN_COUNT | BIN_WIDTH
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedBinType")]
    pub selected_bin_type: Option<String>,


    /// 
    /// The options that determine the bin count of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: BinCountOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "BinCount")]
    pub bin_count: Option<BinCountOptions>,


    /// 
    /// The options that determine the bin start value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartValue")]
    pub start_value: Option<f64>,

}


/// The drill down filter for the column hierarchies.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct DrillDownFilter {


    /// 
    /// The numeric equality type drill down filter. This filter is used for number type columns.
    /// 
    /// Required: No
    ///
    /// Type: NumericEqualityDrillDownFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityDrillDownFilter>,


    /// 
    /// The category type drill down filter. This filter is used for string type columns.
    /// 
    /// Required: No
    ///
    /// Type: CategoryDrillDownFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryDrillDownFilter>,


    /// 
    /// The time range drill down filter. This filter is used for date time columns.
    /// 
    /// Required: No
    ///
    /// Type: TimeRangeDrillDownFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeDrillDownFilter>,

}


/// The control of a filter that is used to interact with a dashboard or an analysis.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FilterControl {


    /// 
    /// A control to display a horizontal toggle bar. This is used to change a value by sliding the toggle.
    /// 
    /// Required: No
    ///
    /// Type: FilterSliderControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slider")]
    pub slider: Option<FilterSliderControl>,


    /// 
    /// A control to display a list of buttons or boxes. This is used to select either a single value or multiple values.
    /// 
    /// Required: No
    ///
    /// Type: FilterListControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "List")]
    pub list: Option<FilterListControl>,


    /// 
    /// A control to display a text box that is used to enter a single entry.
    /// 
    /// Required: No
    ///
    /// Type: FilterTextFieldControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextField")]
    pub text_field: Option<FilterTextFieldControl>,


    /// 
    /// A control from a date filter that is used to specify the relative date.
    /// 
    /// Required: No
    ///
    /// Type: FilterRelativeDateTimeControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateTime")]
    pub relative_date_time: Option<FilterRelativeDateTimeControl>,


    /// 
    /// A control to display a dropdown list with buttons that are used to select a single value.
    /// 
    /// Required: No
    ///
    /// Type: FilterDropDownControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<FilterDropDownControl>,


    /// 
    /// A control from a date filter that is used to specify date and time.
    /// 
    /// Required: No
    ///
    /// Type: FilterDateTimePickerControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<FilterDateTimePickerControl>,


    /// 
    /// A control to display a text box that is used to enter multiple entries.
    /// 
    /// Required: No
    ///
    /// Type: FilterTextAreaControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextArea")]
    pub text_area: Option<FilterTextAreaControl>,

}


/// The configuration for default new sheet settings.
#[derive(Default, serde::Serialize)]
pub struct DefaultNewSheetConfiguration {


    /// 
    /// The option that determines the sheet content type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERACTIVE | PAGINATED
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetContentType")]
    pub sheet_content_type: Option<String>,


    /// 
    /// The options that determine the default settings for interactive layout configuration.
    /// 
    /// Required: No
    ///
    /// Type: DefaultInteractiveLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InteractiveLayoutConfiguration")]
    pub interactive_layout_configuration: Option<DefaultInteractiveLayoutConfiguration>,


    /// 
    /// The options that determine the default settings for a paginated layout configuration.
    /// 
    /// Required: No
    ///
    /// Type: DefaultPaginatedLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaginatedLayoutConfiguration")]
    pub paginated_layout_configuration: Option<DefaultPaginatedLayoutConfiguration>,

}


/// The conditional formatting of a FilledMapVisual.
#[derive(Default, serde::Serialize)]
pub struct FilledMapConditionalFormatting {


    /// 
    /// Conditional formatting options of a FilledMapVisual.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FilledMapConditionalFormattingOption
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Vec<FilledMapConditionalFormattingOption>,

}


/// The calculated field of an analysis.
#[derive(Default, serde::Serialize)]
pub struct CalculatedField {


    /// 
    /// The data set that is used in this calculated field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,


    /// 
    /// The name of the calculated field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The expression of the calculated field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

}


/// Provides the forecast to meet the target for a particular date range.
#[derive(Default, serde::Serialize)]
pub struct WhatIfRangeScenario {


    /// 
    /// The end date in the date range that you need the forecast results for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndDate")]
    pub end_date: String,


    /// 
    /// The target value that you want to meet for the provided date range.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,


    /// 
    /// The start date in the date range that you need the forecast results for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartDate")]
    pub start_date: String,

}


/// The options that determine the sizing of the canvas used in a free-form layout.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutScreenCanvasSizeOptions {


    /// 
    /// The width that the view port will be optimized for when the layout renders.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: String,

}


/// Determines the row alternate color options.
#[derive(Default, serde::Serialize)]
pub struct RowAlternateColorOptions {


    /// 
    /// Determines the list of row alternate colors.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowAlternateColors")]
    pub row_alternate_colors: Option<Vec<String>>,


    /// 
    /// Determines the widget status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}


/// The dimension type field with date type columns.
#[derive(Default, serde::Serialize)]
pub struct DateDimensionField {


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The date granularity of the DateDimensionField. Choose one of the following options:
    /// 
    /// YEAR                                QUARTER                                MONTH                                WEEK                                DAY                                HOUR                                MINUTE                                SECOND                                MILLISECOND
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateGranularity")]
    pub date_granularity: Option<String>,


    /// 
    /// The column that is used in the DateDimensionField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The custom hierarchy ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}


/// The configuration of target visuals that you want to be filtered.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FilterOperationTargetVisualsConfiguration {


    /// 
    /// The configuration of the same-sheet target visuals that you want to be filtered.
    /// 
    /// Required: No
    ///
    /// Type: SameSheetTargetVisualConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SameSheetTargetVisualConfiguration")]
    pub same_sheet_target_visual_configuration: Option<SameSheetTargetVisualConfiguration>,

}


/// The arc axis configuration of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct ArcAxisConfiguration {


    /// 
    /// The arc axis range of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ArcAxisDisplayRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<ArcAxisDisplayRange>,


    /// 
    /// The reserved range of the arc axis.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReserveRange")]
    pub reserve_range: Option<f64>,

}


/// The configuration for a grid layout. Also called a tiled layout.
///
/// Visuals snap to a grid with standard spacing and alignment. Dashboards are displayed as designed, with options to fit to screen or view at actual size.
#[derive(Default, serde::Serialize)]
pub struct GridLayoutConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: GridLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<GridLayoutCanvasSizeOptions>,


    /// 
    /// The elements that are included in a grid layout.
    /// 
    /// Required: Yes
    ///
    /// Type: List of GridLayoutElement
    ///
    /// Maximum: 430
    ///
    /// Update requires: No interruption
    #[serde(rename = "Elements")]
    pub elements: Vec<GridLayoutElement>,

}


/// A list of selectable values that are used in a control.
#[derive(Default, serde::Serialize)]
pub struct FilterSelectableValues {


    /// 
    /// The values that are used in the FilterSelectableValues.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}


/// Dataset reference.
#[derive(Default, serde::Serialize)]
pub struct DataSetReference {


    /// 
    /// Dataset Amazon Resource Name (ARN).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,


    /// 
    /// Dataset placeholder.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetPlaceholder")]
    pub data_set_placeholder: String,

}


/// The configuration that defines the default value of a Decimal parameter when a value has not been set.
#[derive(Default, serde::Serialize)]
pub struct DecimalValueWhenUnsetConfiguration {


    /// 
    /// A custom value that's used when the value of a parameter isn't set.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,


    /// 
    /// The built-in options for default values. The value can be one of the following:
    /// 
    /// RECOMMENDED: The recommended value.                        NULL: The NULL value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NULL | RECOMMENDED_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<String>,

}


/// The conditional formatting for the text.
#[derive(Default, serde::Serialize)]
pub struct TextConditionalFormat {


    /// 
    /// The conditional formatting for the text color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,


    /// 
    /// The conditional formatting for the icon.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingIcon
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,


    /// 
    /// The conditional formatting for the text background color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,

}


/// The field well configuration of a sankey diagram.
#[derive(Default, serde::Serialize)]
pub struct SankeyDiagramAggregatedFieldWells {


    /// 
    /// The source field wells of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Option<Vec<DimensionField>>,


    /// 
    /// The destination field wells of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<Vec<DimensionField>>,


    /// 
    /// The weight field wells of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<Vec<MeasureField>>,

}


/// Custom icon options for an icon set.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingCustomIconOptions {


    /// 
    /// Determines the Unicode icon type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[^\u0000-\u00FF]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnicodeIcon")]
    pub unicode_icon: Option<String>,


    /// 
    /// Determines the type of icon.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ARROW_DOWN | ARROW_DOWN_LEFT | ARROW_DOWN_RIGHT | ARROW_LEFT | ARROW_RIGHT | ARROW_UP | ARROW_UP_LEFT | ARROW_UP_RIGHT | CARET_DOWN | CARET_UP | CHECKMARK | CIRCLE | FACE_DOWN | FACE_FLAT | FACE_UP | FLAG | MINUS | ONE_BAR | PLUS | SQUARE | THREE_BAR | THUMBS_DOWN | THUMBS_UP | TRIANGLE | TWO_BAR | X
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icon")]
    pub icon: Option<String>,

}


/// The field well configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct LineChartFieldWells {


    /// 
    /// The field well configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineChartAggregatedFieldWells")]
    pub line_chart_aggregated_field_wells: Option<LineChartAggregatedFieldWells>,

}


/// The maximum label of a data path label.
#[derive(Default, serde::Serialize)]
pub struct MaximumLabelType {


    /// 
    /// The visibility of the maximum label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The measure type field with categorical type columns.
#[derive(Default, serde::Serialize)]
pub struct CategoricalMeasureField {


    /// 
    /// The aggregation function of the measure field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COUNT | DISTINCT_COUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<String>,


    /// 
    /// The column that is used in the CategoricalMeasureField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: StringFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,

}


/// A string parameter.
#[derive(Default, serde::Serialize)]
pub struct StringParameter {


    /// 
    /// The values of a string parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,


    /// 
    /// A display name for a string parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Formatting configuration for solid color.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingSolidColor {


    /// 
    /// The expression that determines the formatting configuration for solid color.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,


    /// 
    /// Determines the color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// Sheet controls option.
#[derive(Default, serde::Serialize)]
pub struct SheetControlsOption {


    /// 
    /// Visibility state.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COLLAPSED | EXPANDED
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibilityState")]
    pub visibility_state: Option<String>,

}


/// The visual display options for a data zoom scroll bar.
#[derive(Default, serde::Serialize)]
pub struct ScrollBarOptions {


    /// 
    /// The visibility of the data zoom scroll bar.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The visibility range for the data zoom scroll bar.
    /// 
    /// Required: No
    ///
    /// Type: VisibleRangeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibleRange")]
    pub visible_range: Option<VisibleRangeOptions>,

}


/// The sort configuration of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPISortConfiguration {


    /// 
    /// The sort configuration of the trend group fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrendGroupSort")]
    pub trend_group_sort: Option<Vec<FieldSortOptions>>,

}


/// The paginated report options for a pivot table visual.
#[derive(Default, serde::Serialize)]
pub struct PivotTablePaginatedReportOptions {


    /// 
    /// The visibility of the printing table overflow across pages.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<String>,


    /// 
    /// The visibility of the repeating header rows on each page.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<String>,

}


/// The CustomColor property type specifies Property description not available. for an AWS::QuickSight::Dashboard.
#[derive(Default, serde::Serialize)]
pub struct CustomColor {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpecialValue")]
    pub special_value: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,

}


/// The scope of the cell for conditional formatting.
#[derive(Default, serde::Serialize)]
pub struct PivotTableConditionalFormattingScope {


    /// 
    /// The role (field, field total, grand total) of the cell for conditional formatting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FIELD | FIELD_TOTAL | GRAND_TOTAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: Option<String>,

}


/// The configuration for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableConfiguration {


    /// 
    /// The paginated report options for a pivot table visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTablePaginatedReportOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<PivotTablePaginatedReportOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PivotTableFieldWells>,


    /// 
    /// The table options for a pivot table visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableOptions")]
    pub table_options: Option<PivotTableOptions>,


    /// 
    /// The field options for a pivot table visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableFieldOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<PivotTableFieldOptions>,


    /// 
    /// The sort configuration for a PivotTableVisual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PivotTableSortConfiguration>,


    /// 
    /// The total options for a pivot table visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableTotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<PivotTableTotalOptions>,

}


/// The rendering rules of a sheet that uses a free-form layout.
#[derive(Default, serde::Serialize)]
pub struct SheetElementRenderingRule {


    /// 
    /// The override configuration of the rendering rules of a sheet.
    /// 
    /// Required: Yes
    ///
    /// Type: SheetElementConfigurationOverrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationOverrides")]
    pub configuration_overrides: SheetElementConfigurationOverrides,


    /// 
    /// The expression of the rendering rules of a sheet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

}


/// The layout configuration of a section.
#[derive(Default, serde::Serialize)]
pub struct SectionLayoutConfiguration {


    /// 
    /// The free-form layout configuration of a section.
    /// 
    /// Required: Yes
    ///
    /// Type: FreeFormSectionLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: FreeFormSectionLayoutConfiguration,

}


/// The options for a paper canvas of a section-based layout.
#[derive(Default, serde::Serialize)]
pub struct SectionBasedLayoutPaperCanvasSizeOptions {


    /// 
    /// The paper size that is used to define canvas dimensions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: A0 | A1 | A2 | A3 | A4 | A5 | JIS_B4 | JIS_B5 | US_LEGAL | US_LETTER | US_TABLOID_LEDGER
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<String>,


    /// 
    /// The paper orientation that       is used to define canvas dimensions. Choose one of the following       options:
    /// 
    /// PORTRAIT               LANDSCAPE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LANDSCAPE | PORTRAIT
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaperOrientation")]
    pub paper_orientation: Option<String>,


    /// 
    /// Defines the spacing between the canvas content and the top, bottom, left, and right edges.
    /// 
    /// Required: No
    ///
    /// Type: Spacing
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaperMargin")]
    pub paper_margin: Option<Spacing>,

}


/// The configuration for a CategoryFilter.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct CategoryFilterConfiguration {


    /// 
    /// A list of filter configurations. In the Amazon QuickSight console, this filter type is called a filter list.
    /// 
    /// Required: No
    ///
    /// Type: FilterListConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterListConfiguration")]
    pub filter_list_configuration: Option<FilterListConfiguration>,


    /// 
    /// A list of custom filter values. In the Amazon QuickSight console, this filter type is called a custom filter list.
    /// 
    /// Required: No
    ///
    /// Type: CustomFilterListConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomFilterListConfiguration")]
    pub custom_filter_list_configuration: Option<CustomFilterListConfiguration>,


    /// 
    /// A custom filter that filters based on a single value. This filter can be partially matched.
    /// 
    /// Required: No
    ///
    /// Type: CustomFilterConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomFilterConfiguration")]
    pub custom_filter_configuration: Option<CustomFilterConfiguration>,

}


/// The configuration for a       section-based layout.
#[derive(Default, serde::Serialize)]
pub struct SectionBasedLayoutConfiguration {


    /// 
    /// The options for the canvas of a section-based layout.
    /// 
    /// Required: Yes
    ///
    /// Type: SectionBasedLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,


    /// 
    /// A list of footer section configurations.
    /// 
    /// Required: Yes
    ///
    /// Type: List of HeaderFooterSectionConfiguration
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "FooterSections")]
    pub footer_sections: Vec<HeaderFooterSectionConfiguration>,


    /// 
    /// A list of body section configurations.
    /// 
    /// Required: Yes
    ///
    /// Type: List of BodySectionConfiguration
    ///
    /// Maximum: 28
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodySections")]
    pub body_sections: Vec<BodySectionConfiguration>,


    /// 
    /// A list of header section configurations.
    /// 
    /// Required: Yes
    ///
    /// Type: List of HeaderFooterSectionConfiguration
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderSections")]
    pub header_sections: Vec<HeaderFooterSectionConfiguration>,

}


/// The shape conditional formatting of a filled map visual.
#[derive(Default, serde::Serialize)]
pub struct ShapeConditionalFormat {


    /// 
    /// The conditional formatting for the shape background color of a filled map visual.
    /// 
    /// Required: Yes
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: ConditionalFormattingColor,

}


/// The options that determine the default settings for a grid layout configuration.
#[derive(Default, serde::Serialize)]
pub struct DefaultGridLayoutConfiguration {


    /// 
    /// Determines the screen canvas size options for a grid layout.
    /// 
    /// Required: Yes
    ///
    /// Type: GridLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: GridLayoutCanvasSizeOptions,

}


/// The field wells of a tree map.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct TreeMapFieldWells {


    /// 
    /// The aggregated field wells of a tree map.
    /// 
    /// Required: No
    ///
    /// Type: TreeMapAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreeMapAggregatedFieldWells")]
    pub tree_map_aggregated_field_wells: Option<TreeMapAggregatedFieldWells>,

}


/// The configuration of a heat map.
#[derive(Default, serde::Serialize)]
pub struct HeatMapConfiguration {


    /// 
    /// The label options of the column that is displayed in a heat map.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnLabelOptions")]
    pub column_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a heat map.
    /// 
    /// Required: No
    ///
    /// Type: HeatMapSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<HeatMapSortConfiguration>,


    /// 
    /// The label options of the row that is displayed in a heat map.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowLabelOptions")]
    pub row_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: HeatMapFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HeatMapFieldWells>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The color options (gradient color, point of divergence) in a heat map.
    /// 
    /// Required: No
    ///
    /// Type: ColorScale
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,

}


/// The range ends label type of a data path label.
#[derive(Default, serde::Serialize)]
pub struct RangeEndsLabelType {


    /// 
    /// The visibility of the range ends label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The aggregated field well of a scatter plot.
#[derive(Default, serde::Serialize)]
pub struct ScatterPlotCategoricallyAggregatedFieldWells {


    /// 
    /// The size field well of a scatter plot.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,


    /// 
    /// The x-axis field well of a scatter plot.
    /// 
    /// The x-axis is aggregated by category.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<MeasureField>>,


    /// 
    /// The category field well of a scatter plot.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,


    /// 
    /// The y-axis field well of a scatter plot.
    /// 
    /// The y-axis is aggregated by category.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<MeasureField>>,

}


/// The options for an axis with a numeric field.
#[derive(Default, serde::Serialize)]
pub struct NumericAxisOptions {


    /// 
    /// The scale setup of a numeric axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisScale
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scale")]
    pub scale: Option<AxisScale>,


    /// 
    /// The range setup of a numeric axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<AxisDisplayRange>,

}


/// The options of a box plot visual.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotOptions {


    /// 
    /// The style options of the box plot.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotStyleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "StyleOptions")]
    pub style_options: Option<BoxPlotStyleOptions>,


    /// 
    /// Determines the visibility of the outlier in a box plot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlierVisibility")]
    pub outlier_visibility: Option<String>,


    /// 
    /// Determines the visibility of all data points of the box plot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllDataPointsVisibility")]
    pub all_data_points_visibility: Option<String>,

}


/// The configuration of loading animation in free-form layout.
#[derive(Default, serde::Serialize)]
pub struct LoadingAnimation {


    /// 
    /// The visibility configuration of LoadingAnimation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// A list of Amazon QuickSight parameters and the list's override values.
#[derive(Default, serde::Serialize)]
pub struct Parameters {


    /// 
    /// The parameters that have a data type of decimal.
    /// 
    /// Required: No
    ///
    /// Type: List of DecimalParameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalParameters")]
    pub decimal_parameters: Option<Vec<DecimalParameter>>,


    /// 
    /// The parameters that have a data type of integer.
    /// 
    /// Required: No
    ///
    /// Type: List of IntegerParameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerParameters")]
    pub integer_parameters: Option<Vec<IntegerParameter>>,


    /// 
    /// The parameters that have a data type of date-time.
    /// 
    /// Required: No
    ///
    /// Type: List of DateTimeParameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeParameters")]
    pub date_time_parameters: Option<Vec<DateTimeParameter>>,


    /// 
    /// The parameters that have a data type of string.
    /// 
    /// Required: No
    ///
    /// Type: List of StringParameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringParameters")]
    pub string_parameters: Option<Vec<StringParameter>>,

}


/// The scale setup       options for a numeric axis display.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct AxisScale {


    /// 
    /// The logarithmic axis scale setup.
    /// 
    /// Required: No
    ///
    /// Type: AxisLogarithmicScale
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logarithmic")]
    pub logarithmic: Option<AxisLogarithmicScale>,


    /// 
    /// The linear axis scale setup.
    /// 
    /// Required: No
    ///
    /// Type: AxisLinearScale
    ///
    /// Update requires: No interruption
    #[serde(rename = "Linear")]
    pub linear: Option<AxisLinearScale>,

}


/// The conditional formatting for the progress bar of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIProgressBarConditionalFormatting {


    /// 
    /// The conditional formatting of the progress bar's foreground color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}


/// The axis sort options for a visual.
#[derive(Default, serde::Serialize)]
pub struct VisualAxisSortOption {


    /// 
    /// The availaiblity status of a visual's axis sort options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The comparison display configuration of a KPI or gauge chart.
#[derive(Default, serde::Serialize)]
pub struct ComparisonConfiguration {


    /// 
    /// The format of the comparison.
    /// 
    /// Required: No
    ///
    /// Type: ComparisonFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonFormat")]
    pub comparison_format: Option<ComparisonFormatConfiguration>,


    /// 
    /// The method of the comparison. Choose from the following options:
    /// 
    /// DIFFERENCE                                PERCENT_DIFFERENCE                                PERCENT
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIFFERENCE | PERCENT | PERCENT_DIFFERENCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonMethod")]
    pub comparison_method: Option<String>,

}


/// The sort configuration of a FilledMapVisual.
#[derive(Default, serde::Serialize)]
pub struct FilledMapSortConfiguration {


    /// 
    /// The sort configuration of the location fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}


/// The measure (metric) type field.
#[derive(Default, serde::Serialize)]
pub struct MeasureField {


    /// 
    /// The measure type field with numerical type columns.
    /// 
    /// Required: No
    ///
    /// Type: NumericalMeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericalMeasureField")]
    pub numerical_measure_field: Option<NumericalMeasureField>,


    /// 
    /// The measure type field with categorical type columns.
    /// 
    /// Required: No
    ///
    /// Type: CategoricalMeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoricalMeasureField")]
    pub categorical_measure_field: Option<CategoricalMeasureField>,


    /// 
    /// The calculated measure field only used in pivot tables.
    /// 
    /// Required: No
    ///
    /// Type: CalculatedMeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedMeasureField")]
    pub calculated_measure_field: Option<CalculatedMeasureField>,


    /// 
    /// The measure type field with date type columns.
    /// 
    /// Required: No
    ///
    /// Type: DateMeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateMeasureField")]
    pub date_measure_field: Option<DateMeasureField>,

}


/// A list of selectable values that are used in a control.
#[derive(Default, serde::Serialize)]
pub struct ParameterSelectableValues {


    /// 
    /// The values that are used in ParameterSelectableValues.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,


    /// 
    /// The column identifier that fetches values from the data set.
    /// 
    /// Required: No
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "LinkToDataSetColumn")]
    pub link_to_data_set_column: Option<ColumnIdentifier>,

}


/// The sort configuration for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableSortConfiguration {


    /// 
    /// The field sort options for a pivot table sort configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of PivotFieldSortOptions
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldSortOptions")]
    pub field_sort_options: Option<Vec<PivotFieldSortOptions>>,

}


/// The tooltip item for the fields.
#[derive(Default, serde::Serialize)]
pub struct FieldTooltipItem {


    /// 
    /// The label of the tooltip item.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,


    /// 
    /// The unique ID of the field that is targeted by the tooltip.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The visibility of the tooltip item.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// A custom filter that filters based on a single value. This filter can be partially matched.
#[derive(Default, serde::Serialize)]
pub struct CustomFilterConfiguration {


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// The category value for the filter.
    /// 
    /// This field is mutually exclusive to ParameterName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryValue")]
    pub category_value: Option<String>,


    /// 
    /// The match operator that is used to determine if a filter should be applied.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | DOES_NOT_CONTAIN | DOES_NOT_EQUAL | ENDS_WITH | EQUALS | STARTS_WITH
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchOperator")]
    pub match_operator: String,


    /// 
    /// The parameter whose value should be used for the filter value.
    /// 
    /// This field is mutually exclusive to CategoryValue.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,


    /// 
    /// Select all of the values. Null is not the assigned value of select all.
    /// 
    /// FILTER_ALL_VALUES
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<String>,

}


/// The configuration for a HistogramVisual.
#[derive(Default, serde::Serialize)]
pub struct HistogramConfiguration {


    /// 
    /// The options that determine the presentation of the x-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The tooltip configuration of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The visual palette configuration of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The options that determine the presentation of the y-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The data label configuration of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The options that determine the presentation of histogram bins.
    /// 
    /// Required: No
    ///
    /// Type: HistogramBinOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "BinOptions")]
    pub bin_options: Option<HistogramBinOptions>,


    /// 
    /// The field well configuration of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: HistogramFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HistogramFieldWells>,


    /// 
    /// The options that determine the presentation of the x-axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,

}


/// A bar chart.
///
/// The BarChartVisual structure describes a visual that is a member of the bar chart family. The following charts can be described using this structure:
///
/// For more information, see Using bar charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct BarChartVisual {


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: BarChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BarChartConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}


/// The period to date computation configuration.
#[derive(Default, serde::Serialize)]
pub struct PeriodToDateComputation {


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,


    /// 
    /// The time granularity setup of period to date computation. Choose from the following options:
    /// 
    /// YEAR: Year to date.               MONTH: Month to date.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodTimeGranularity")]
    pub period_time_granularity: Option<String>,

}


/// The field wells of a FilledMapVisual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FilledMapFieldWells {


    /// 
    /// The aggregated field well of the filled map.
    /// 
    /// Required: No
    ///
    /// Type: FilledMapAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilledMapAggregatedFieldWells")]
    pub filled_map_aggregated_field_wells: Option<FilledMapAggregatedFieldWells>,

}


/// The declaration definition of a parameter.
///
/// For more information, see Parameters in Amazon QuickSight in the Amazon QuickSight User Guide.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ParameterDeclaration {


    /// 
    /// A parameter declaration for the Decimal data type.
    /// 
    /// Required: No
    ///
    /// Type: DecimalParameterDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalParameterDeclaration")]
    pub decimal_parameter_declaration: Option<DecimalParameterDeclaration>,


    /// 
    /// A parameter declaration for the Integer data type.
    /// 
    /// Required: No
    ///
    /// Type: IntegerParameterDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerParameterDeclaration")]
    pub integer_parameter_declaration: Option<IntegerParameterDeclaration>,


    /// 
    /// A parameter declaration for the DateTime data type.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeParameterDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeParameterDeclaration")]
    pub date_time_parameter_declaration: Option<DateTimeParameterDeclaration>,


    /// 
    /// A parameter declaration for the String data type.
    /// 
    /// Required: No
    ///
    /// Type: StringParameterDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringParameterDeclaration")]
    pub string_parameter_declaration: Option<StringParameterDeclaration>,

}


/// The free-form layout configuration of a section.
#[derive(Default, serde::Serialize)]
pub struct FreeFormSectionLayoutConfiguration {


    /// 
    /// The elements that are included in the free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FreeFormLayoutElement
    ///
    /// Maximum: 430
    ///
    /// Update requires: No interruption
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,

}


/// A CategoryFilter filters text values.
///
/// For more information, see Adding text filters in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct CategoryFilter {


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// The configuration for a CategoryFilter.
    /// 
    /// Required: Yes
    ///
    /// Type: CategoryFilterConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: CategoryFilterConfiguration,

}


/// The options that determine the default settings for a paginated layout configuration.
#[derive(Default, serde::Serialize)]
pub struct DefaultPaginatedLayoutConfiguration {


    /// 
    /// The options that determine the default settings for a section-based layout configuration.
    /// 
    /// Required: No
    ///
    /// Type: DefaultSectionBasedLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SectionBased")]
    pub section_based: Option<DefaultSectionBasedLayoutConfiguration>,

}


/// The conditional formatting options of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIConditionalFormattingOption {


    /// 
    /// The conditional formatting for the primary value of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIPrimaryValueConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<KPIPrimaryValueConditionalFormatting>,


    /// 
    /// The conditional formatting for the progress bar of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIProgressBarConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<KPIProgressBarConditionalFormatting>,

}


/// The navigation configuration for CustomActionNavigationOperation.
#[derive(Default, serde::Serialize)]
pub struct LocalNavigationConfiguration {


    /// 
    /// The sheet that is targeted for navigation in the same analysis.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetSheetId")]
    pub target_sheet_id: String,

}


/// The style configuration of the reference       line.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineStyleConfiguration {


    /// 
    /// The pattern type of the line style. Choose one of the following options:
    /// 
    /// SOLID                                DASHED                                DOTTED
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DASHED | DOTTED | SOLID
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,


    /// 
    /// The hex color of the reference line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// The sizing options for the table image configuration.
#[derive(Default, serde::Serialize)]
pub struct TableCellImageSizingConfiguration {


    /// 
    /// The cell scaling configuration of the sizing options for the table image configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DO_NOT_SCALE | FIT_TO_CELL_HEIGHT | FIT_TO_CELL_WIDTH
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableCellImageScalingConfiguration")]
    pub table_cell_image_scaling_configuration: Option<String>,

}


/// The sheet layout maximization options of a dashbaord.
#[derive(Default, serde::Serialize)]
pub struct SheetLayoutElementMaximizationOption {


    /// 
    /// The status of the sheet layout maximization options of a dashbaord.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// A combo chart.
///
/// The ComboChartVisual includes stacked bar combo charts and clustered bar combo charts
///
/// For more information, see Using combo charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct ComboChartVisual {


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: ComboChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ComboChartConfiguration>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}


/// The field sort options in a chart configuration.
#[derive(Default, serde::Serialize)]
pub struct FieldSortOptions {


    /// 
    /// The sort configuration for a column that is not used in a field well.
    /// 
    /// Required: No
    ///
    /// Type: ColumnSort
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSort")]
    pub column_sort: Option<ColumnSort>,


    /// 
    /// The sort configuration for a field in a field well.
    /// 
    /// Required: No
    ///
    /// Type: FieldSort
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldSort")]
    pub field_sort: Option<FieldSort>,

}


/// Conditional formatting options for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct TableConditionalFormattingOption {


    /// 
    /// The row conditional formatting option for a table.
    /// 
    /// Required: No
    ///
    /// Type: TableRowConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "Row")]
    pub row: Option<TableRowConditionalFormatting>,


    /// 
    /// The cell conditional formatting option for a table.
    /// 
    /// Required: No
    ///
    /// Type: TableCellConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cell")]
    pub cell: Option<TableCellConditionalFormatting>,

}


/// The options for the legend setup of a visual.
#[derive(Default, serde::Serialize)]
pub struct LegendOptions {


    /// 
    /// The width of the legend. If this value is omitted, a default width is used when rendering.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    pub width: Option<String>,


    /// 
    /// The height of the legend. If this value is omitted, a default height is used when       rendering.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    pub height: Option<String>,


    /// 
    /// The custom title for the legend.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<LabelOptions>,


    /// 
    /// The positions for the legend. Choose one of the following       options:
    /// 
    /// AUTO                                RIGHT                                BOTTOM                                LEFT
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | BOTTOM | RIGHT | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<String>,


    /// 
    /// Determines whether or not the legend is visible.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The configuration of content in a body section.
#[derive(Default, serde::Serialize)]
pub struct BodySectionContent {


    /// 
    /// The layout configuration of a body section.
    /// 
    /// Required: No
    ///
    /// Type: SectionLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Layout")]
    pub layout: Option<SectionLayoutConfiguration>,

}


/// Options that determine the layout and display options of a chart's small multiples.
#[derive(Default, serde::Serialize)]
pub struct SmallMultiplesOptions {


    /// 
    /// Sets the maximum number of visible columns to display in the grid of small multiples panels.
    /// 
    /// The default is Auto, which automatically adjusts the columns in the grid to fit the overall layout and size of the given chart.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxVisibleColumns")]
    pub max_visible_columns: Option<f64>,


    /// 
    /// Configures the display options for each small multiples panel.
    /// 
    /// Required: No
    ///
    /// Type: PanelConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PanelConfiguration")]
    pub panel_configuration: Option<PanelConfiguration>,


    /// 
    /// Sets the maximum number of visible rows to display in the grid of small multiples panels.
    /// 
    /// The default value is Auto,       which automatically adjusts the rows in the grid       to fit the overall layout and size of the given chart.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxVisibleRows")]
    pub max_visible_rows: Option<f64>,

}


/// The default values of the StringParameterDeclaration.
#[derive(Default, serde::Serialize)]
pub struct StringDefaultValues {


    /// 
    /// The static values of the DecimalDefaultValues.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,


    /// 
    /// The dynamic value of the StringDefaultValues. Different defaults displayed according to users, groups, and values mapping.
    /// 
    /// Required: No
    ///
    /// Type: DynamicDefaultValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}


/// The conditional formatting of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartConditionalFormatting {


    /// 
    /// Conditional formatting options of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: List of GaugeChartConditionalFormattingOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<GaugeChartConditionalFormattingOption>>,

}


/// Dashboard version.
#[derive(Default, serde::Serialize)]
pub struct DashboardVersion {


    /// 
    /// The Amazon Resource Numbers (ARNs) for the datasets that are associated with this       version of the dashboard.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetArns")]
    pub data_set_arns: Option<Vec<String>>,


    /// 
    /// The time that this dashboard version was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,


    /// 
    /// Source entity ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceEntityArn")]
    pub source_entity_arn: Option<String>,


    /// 
    /// The HTTP status of the request.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CREATION_FAILED | CREATION_IN_PROGRESS | CREATION_SUCCESSFUL | DELETED | UPDATE_FAILED | UPDATE_IN_PROGRESS | UPDATE_SUCCESSFUL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// Errors associated with this dashboard version.
    /// 
    /// Required: No
    ///
    /// Type: List of DashboardError
    ///
    /// Update requires: No interruption
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<DashboardError>>,


    /// 
    /// Version number for this version of the dashboard.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<f64>,


    /// 
    /// Description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ARN of the theme associated with a version of the dashboard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,


    /// 
    /// A list of the associated sheets with the unique identifier and name of each sheet.
    /// 
    /// Required: No
    ///
    /// Type: List of Sheet
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<Sheet>>,


    /// 
    /// The Amazon Resource Name (ARN) of the resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}


/// The value of a time range filter.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct TimeRangeFilterValue {


    /// 
    /// The rolling date input value.
    /// 
    /// Required: No
    ///
    /// Type: RollingDateConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,


    /// 
    /// The parameter type input value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,


    /// 
    /// The static input value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValue")]
    pub static_value: Option<String>,

}


/// The measure type field with numerical type columns.
#[derive(Default, serde::Serialize)]
pub struct NumericalMeasureField {


    /// 
    /// The aggregation function of the measure field.
    /// 
    /// Required: No
    ///
    /// Type: NumericalAggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<NumericalAggregationFunction>,


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: NumberFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The column that is used in the NumericalMeasureField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}


/// A parameter declaration for the Decimal data type.
#[derive(Default, serde::Serialize)]
pub struct DecimalParameterDeclaration {


    /// 
    /// The name of the parameter that is being declared.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The configuration that defines the default value of a Decimal parameter when a value has not been set.
    /// 
    /// Required: No
    ///
    /// Type: DecimalValueWhenUnsetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DecimalValueWhenUnsetConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of MappedDataSetParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,


    /// 
    /// The value type determines whether the parameter is a single-value or multi-value parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_VALUED | SINGLE_VALUED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: String,


    /// 
    /// The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.
    /// 
    /// Required: No
    ///
    /// Type: DecimalDefaultValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DecimalDefaultValues>,

}


/// The conditional formatting that determines the shape of the filled map.
#[derive(Default, serde::Serialize)]
pub struct FilledMapShapeConditionalFormatting {


    /// 
    /// The field ID of the filled map shape.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The conditional formatting that determines the background color of a filled map's shape.
    /// 
    /// Required: No
    ///
    /// Type: ShapeConditionalFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<ShapeConditionalFormat>,

}


/// The options that determine the presentation of trend arrows in a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct TrendArrowOptions {


    /// 
    /// The visibility of the trend arrows.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// A list of custom filter values.
#[derive(Default, serde::Serialize)]
pub struct CustomFilterListConfiguration {


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// The list of category values for the filter.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,


    /// 
    /// The match operator that is used to determine if a filter should be applied.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | DOES_NOT_CONTAIN | DOES_NOT_EQUAL | ENDS_WITH | EQUALS | STARTS_WITH
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchOperator")]
    pub match_operator: String,


    /// 
    /// Select all of the values. Null is not the assigned value of select all.
    /// 
    /// FILTER_ALL_VALUES
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<String>,

}


/// The options that determine the presentation of the progress bar of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct ProgressBarOptions {


    /// 
    /// The visibility of the progress bar.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The URL content (text, icon) for the table link configuration.
#[derive(Default, serde::Serialize)]
pub struct TableFieldLinkContentConfiguration {


    /// 
    /// The custom text content (value, font configuration) for the table link content configuration.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldCustomTextContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomTextContent")]
    pub custom_text_content: Option<TableFieldCustomTextContent>,


    /// 
    /// The custom icon content for the table link content configuration.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldCustomIconContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomIconContent")]
    pub custom_icon_content: Option<TableFieldCustomIconContent>,

}


/// A control to display a text box that is used to enter multiple entries.
#[derive(Default, serde::Serialize)]
pub struct FilterTextAreaControl {


    /// 
    /// The delimiter that is used to separate the lines in text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,


    /// 
    /// The title of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: TextAreaControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,


    /// 
    /// The ID of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,


    /// 
    /// The source filter ID of the FilterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}


/// A control to display a text box that is used to enter multiple entries.
#[derive(Default, serde::Serialize)]
pub struct ParameterTextAreaControl {


    /// 
    /// The delimiter that is used to separate the lines in text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,


    /// 
    /// The title of the ParameterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The source parameter name of the ParameterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,


    /// 
    /// The ID of the ParameterTextAreaControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: TextAreaControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,

}


/// With a Filter, you can remove portions of data from a particular visual or view.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct Filter {


    /// 
    /// A RelativeDatesFilter filters date values that are relative to a given date.
    /// 
    /// Required: No
    ///
    /// Type: RelativeDatesFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDatesFilter")]
    pub relative_dates_filter: Option<RelativeDatesFilter>,


    /// 
    /// A TopBottomFilter filters data to the top or bottom values for a given column.
    /// 
    /// Required: No
    ///
    /// Type: TopBottomFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopBottomFilter")]
    pub top_bottom_filter: Option<TopBottomFilter>,


    /// 
    /// A TimeEqualityFilter filters date-time values that equal or do not equal       a given date/time value.
    /// 
    /// Required: No
    ///
    /// Type: TimeEqualityFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeEqualityFilter")]
    pub time_equality_filter: Option<TimeEqualityFilter>,


    /// 
    /// A CategoryFilter filters text values.
    /// 
    /// For more information, see Adding text filters in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: CategoryFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryFilter>,


    /// 
    /// A NumericEqualityFilter filters numeric values that equal or do not equal a given numeric value.
    /// 
    /// Required: No
    ///
    /// Type: NumericEqualityFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityFilter>,


    /// 
    /// A TimeRangeFilter filters date-time values that are either inside or outside a given date/time range.
    /// 
    /// Required: No
    ///
    /// Type: TimeRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeFilter>,


    /// 
    /// A NumericRangeFilter filters numeric values that are either inside or outside a given numeric range.
    /// 
    /// Required: No
    ///
    /// Type: NumericRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericRangeFilter")]
    pub numeric_range_filter: Option<NumericRangeFilter>,

}


/// The configuration of a tree map.
#[derive(Default, serde::Serialize)]
pub struct TreeMapConfiguration {


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TreeMapFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TreeMapFieldWells>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The label options (label text, label visibility) of the sizes that are displayed in a tree map.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeLabelOptions")]
    pub size_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a tree map.
    /// 
    /// Required: No
    ///
    /// Type: TreeMapSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TreeMapSortConfiguration>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The label options (label text, label visibility) of the groups that are displayed in a tree map.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupLabelOptions")]
    pub group_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The label options (label text, label visibility) for the colors displayed in a tree map.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The color options (gradient color, point of divergence) of a tree map.
    /// 
    /// Required: No
    ///
    /// Type: ColorScale
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,

}


/// The configuration of custom values for the destination parameter in DestinationParameterValueConfiguration.
#[derive(Default, serde::Serialize)]
pub struct CustomValuesConfiguration {


    /// 
    /// Includes the null value in custom action parameter values.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeNullValue")]
    pub include_null_value: Option<bool>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: CustomParameterValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValues")]
    pub custom_values: CustomParameterValues,

}


/// The source controls that are used in a CascadingControlConfiguration.
#[derive(Default, serde::Serialize)]
pub struct CascadingControlSource {


    /// 
    /// The column identifier that determines which column to look up for the source sheet control.
    /// 
    /// Required: No
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnToMatch")]
    pub column_to_match: Option<ColumnIdentifier>,


    /// 
    /// The source sheet control ID of a CascadingControlSource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceSheetControlId")]
    pub source_sheet_control_id: Option<String>,

}


/// The configuration options that determine how missing data is treated during the rendering of a line chart.
#[derive(Default, serde::Serialize)]
pub struct MissingDataConfiguration {


    /// 
    /// The treatment option that determines how missing data should be rendered. Choose       from the following options:
    /// 
    /// INTERPOLATE: Interpolate missing values between the prior and the next known value.                        SHOW_AS_ZERO: Show missing values as the value 0.                        SHOW_AS_BLANK: Display a blank space when rendering missing data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERPOLATE | SHOW_AS_BLANK | SHOW_AS_ZERO
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatmentOption")]
    pub treatment_option: Option<String>,

}


/// The conditional formatting for a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableConditionalFormatting {


    /// 
    /// Conditional formatting options for a PivotTableVisual.
    /// 
    /// Required: No
    ///
    /// Type: List of PivotTableConditionalFormattingOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<PivotTableConditionalFormattingOption>>,

}


/// The cluster marker configuration of the geospatial map selected point style.
#[derive(Default, serde::Serialize)]
pub struct ClusterMarkerConfiguration {


    /// 
    /// The cluster marker that is a part of the cluster marker configuration
    /// 
    /// Required: No
    ///
    /// Type: ClusterMarker
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterMarker")]
    pub cluster_marker: Option<ClusterMarker>,

}


/// The configuration of a page break after a section.
#[derive(Default, serde::Serialize)]
pub struct SectionAfterPageBreak {


    /// 
    /// The option that enables or disables a page break at the end of a section.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}


/// The simple cluster marker of the cluster marker.
#[derive(Default, serde::Serialize)]
pub struct SimpleClusterMarker {


    /// 
    /// The color of the simple cluster marker.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// The field well configuration of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct KPIFieldWells {


    /// 
    /// The trend group field wells of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrendGroups")]
    pub trend_groups: Option<Vec<DimensionField>>,


    /// 
    /// The target value field wells of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,


    /// 
    /// The value field wells of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// A control to display a text box that is used to enter a single entry.
#[derive(Default, serde::Serialize)]
pub struct FilterTextFieldControl {


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: TextFieldControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,


    /// 
    /// The title of the FilterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The ID of the FilterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,


    /// 
    /// The source filter ID of the FilterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}


/// The operation that is defined by the custom action.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct VisualCustomActionOperation {


    /// 
    /// The navigation operation that navigates between different sheets in the same analysis.
    /// 
    /// Required: No
    ///
    /// Type: CustomActionNavigationOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "NavigationOperation")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,


    /// 
    /// The set parameter operation that sets parameters in custom action.
    /// 
    /// Required: No
    ///
    /// Type: CustomActionSetParametersOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetParametersOperation")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,


    /// 
    /// The URL operation that opens a link to another webpage.
    /// 
    /// Required: No
    ///
    /// Type: CustomActionURLOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLOperation")]
    pub urloperation: Option<CustomActionURLOperation>,


    /// 
    /// The filter operation that filters data included in a visual or in an entire sheet.
    /// 
    /// Required: No
    ///
    /// Type: CustomActionFilterOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<CustomActionFilterOperation>,

}


/// The table cell style for a cell in pivot table or table visual.
#[derive(Default, serde::Serialize)]
pub struct TableCellStyle {


    /// 
    /// The visibility of the table cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The horizontal text alignment (left, center, right, auto) for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | CENTER | LEFT | RIGHT
    ///
    /// Update requires: No interruption
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<String>,


    /// 
    /// The text wrap (none, wrap) for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | WRAP
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextWrap")]
    pub text_wrap: Option<String>,


    /// 
    /// The borders for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: GlobalTableBorderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Border")]
    pub border: Option<GlobalTableBorderOptions>,


    /// 
    /// The font configuration of the table cells.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,


    /// 
    /// The height color for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 8
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    pub height: Option<f64>,


    /// 
    /// The background color for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,


    /// 
    /// The vertical text alignment (top, middle, bottom) for the table cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOTTOM | MIDDLE | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerticalTextAlignment")]
    pub vertical_text_alignment: Option<String>,

}


/// The unaggregated field well for the table.
#[derive(Default, serde::Serialize)]
pub struct TableUnaggregatedFieldWells {


    /// 
    /// The values field well for a pivot table. Values are unaggregated for an unaggregated table.
    /// 
    /// Required: No
    ///
    /// Type: List of UnaggregatedField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<UnaggregatedField>>,

}


/// The configuration of a free-form layout.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutConfiguration {


    /// 
    /// The elements that are included in a free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FreeFormLayoutElement
    ///
    /// Maximum: 430
    ///
    /// Update requires: No interruption
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FreeFormLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<FreeFormLayoutCanvasSizeOptions>,

}


/// The table options for a table visual.
#[derive(Default, serde::Serialize)]
pub struct TableOptions {


    /// 
    /// The orientation (vertical, horizontal) for a table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HORIZONTAL | VERTICAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Orientation")]
    pub orientation: Option<String>,


    /// 
    /// The table cell style of a table header.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderStyle")]
    pub header_style: Option<TableCellStyle>,


    /// 
    /// The table cell style of table cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,


    /// 
    /// The row alternate color options (widget status, row alternate colors) for a table.
    /// 
    /// Required: No
    ///
    /// Type: RowAlternateColorOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,

}


/// The options that determine the presentation of the secondary value of a KPI visual.
#[derive(Default, serde::Serialize)]
pub struct SecondaryValueOptions {


    /// 
    /// Determines the visibility of the secondary value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The navigation operation that navigates between different sheets in the same analysis.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct CustomActionNavigationOperation {


    /// 
    /// The configuration that chooses the navigation target.
    /// 
    /// Required: No
    ///
    /// Type: LocalNavigationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalNavigationConfiguration")]
    pub local_navigation_configuration: Option<LocalNavigationConfiguration>,

}


/// The visual display options for the visual palette.
#[derive(Default, serde::Serialize)]
pub struct VisualPalette {


    /// 
    /// The color map options for the visual palette.
    /// 
    /// Required: No
    ///
    /// Type: List of DataPathColor
    ///
    /// Maximum: 5000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorMap")]
    pub color_map: Option<Vec<DataPathColor>>,


    /// 
    /// The chart color options for the visual palette.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartColor")]
    pub chart_color: Option<String>,

}


/// The sort configuration of a heat map.
#[derive(Default, serde::Serialize)]
pub struct HeatMapSortConfiguration {


    /// 
    /// The column sort configuration for heat map for columns that aren't a part of a field well.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapColumnSort")]
    pub heat_map_column_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The field sort configuration of the rows fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapRowSort")]
    pub heat_map_row_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of columns that are displayed in a heat map.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapColumnItemsLimitConfiguration")]
    pub heat_map_column_items_limit_configuration: Option<ItemsLimitConfiguration>,


    /// 
    /// The limit on the number of rows that are displayed in a heat map.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeatMapRowItemsLimitConfiguration")]
    pub heat_map_row_items_limit_configuration: Option<ItemsLimitConfiguration>,

}


/// Defines different defaults to the users or groups based on mapping.
#[derive(Default, serde::Serialize)]
pub struct DynamicDefaultValue {


    /// 
    /// The column that contains the group name.
    /// 
    /// Required: No
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupNameColumn")]
    pub group_name_column: Option<ColumnIdentifier>,


    /// 
    /// The column that contains the username.
    /// 
    /// Required: No
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserNameColumn")]
    pub user_name_column: Option<ColumnIdentifier>,


    /// 
    /// The column that contains the default value of each user or group.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValueColumn")]
    pub default_value_column: ColumnIdentifier,

}


/// The cell conditional formatting option for a pivot table.
#[derive(Default, serde::Serialize)]
pub struct PivotTableCellConditionalFormatting {


    /// 
    /// The field ID of the cell for conditional formatting.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The text format of the cell for conditional formatting.
    /// 
    /// Required: No
    ///
    /// Type: TextConditionalFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,


    /// 
    /// The scope of the cell for conditional formatting.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableConditionalFormattingScope
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: Option<PivotTableConditionalFormattingScope>,

}


/// A histogram.
///
/// For more information, see Using histograms in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct HistogramVisual {


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The configuration for a HistogramVisual.
    /// 
    /// Required: No
    ///
    /// Type: HistogramConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HistogramConfiguration>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}


/// The background style configuration of a free-form layout element.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutElementBorderStyle {


    /// 
    /// The border visibility of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The border color of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}(?:[A-F0-9]{2})?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// The aggregated field wells of a combo chart.
#[derive(Default, serde::Serialize)]
pub struct ComboChartAggregatedFieldWells {


    /// 
    /// The aggregated colors field well of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,


    /// 
    /// The aggregated LineValues field well of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineValues")]
    pub line_values: Option<Vec<MeasureField>>,


    /// 
    /// The aggregated BarValues field well of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarValues")]
    pub bar_values: Option<Vec<MeasureField>>,


    /// 
    /// The aggregated category field wells of a combo chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}


/// A control to display a list with buttons or boxes that are used to select either a single value or multiple values.
#[derive(Default, serde::Serialize)]
pub struct ParameterListControl {


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: ListControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,


    /// 
    /// The title of the ParameterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The type of ParameterListControl.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_SELECT | SINGLE_SELECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The ID of the ParameterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The values that are displayed in a control can be configured to only show values that are valid based on what's selected in other controls.
    /// 
    /// Required: No
    ///
    /// Type: CascadingControlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,


    /// 
    /// A list of selectable values that are used in a control.
    /// 
    /// Required: No
    ///
    /// Type: ParameterSelectableValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,


    /// 
    /// The source parameter name of the ParameterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,

}


/// The default values of the DateTimeParameterDeclaration.
#[derive(Default, serde::Serialize)]
pub struct DateTimeDefaultValues {


    /// 
    /// The static values of the DataTimeDefaultValues.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,


    /// 
    /// The rolling date of the DataTimeDefaultValues. The date is determined from the dataset based on input expression.
    /// 
    /// Required: No
    ///
    /// Type: RollingDateConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,


    /// 
    /// The dynamic value of the DataTimeDefaultValues. Different defaults are displayed according to users, groups, and values mapping.
    /// 
    /// Required: No
    ///
    /// Type: DynamicDefaultValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}


/// The field well configuration of a histogram.
#[derive(Default, serde::Serialize)]
pub struct HistogramFieldWells {


    /// 
    /// The field well configuration of a histogram.
    /// 
    /// Required: No
    ///
    /// Type: HistogramAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "HistogramAggregatedFieldWells")]
    pub histogram_aggregated_field_wells: Option<HistogramAggregatedFieldWells>,

}


/// The option that determines the hierarchy of the fields that are defined during data preparation. These fields are available to use in any analysis that uses the data source.
#[derive(Default, serde::Serialize)]
pub struct PredefinedHierarchy {


    /// 
    /// The hierarchy ID of the predefined hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,


    /// 
    /// The option that determines the drill down filters for the predefined hierarchy.
    /// 
    /// Required: No
    ///
    /// Type: List of DrillDownFilter
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,


    /// 
    /// The list of columns that define the predefined hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ColumnIdentifier
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,

}


/// The data point menu options of a dashboard.
#[derive(Default, serde::Serialize)]
pub struct DataPointMenuLabelOption {


    /// 
    /// The status of the data point menu options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The top ranked and bottom ranked computation configuration.
#[derive(Default, serde::Serialize)]
pub struct TopBottomRankedComputation {


    /// 
    /// The result size of a top and bottom ranked computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResultSize")]
    pub result_size: Option<f64>,


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The category field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: DimensionField,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The computation type. Choose one of the following options:
    /// 
    /// TOP: A top ranked computation.               BOTTOM: A bottom ranked computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BOTTOM | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// The options that style a section.
#[derive(Default, serde::Serialize)]
pub struct SectionStyle {


    /// 
    /// The height of a section.
    /// 
    /// Heights can only be defined for header and footer sections. The default height margin is 0.5 inches.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    pub height: Option<String>,


    /// 
    /// The spacing between section content and its top, bottom, left, and right edges.
    /// 
    /// There is no padding by default.
    /// 
    /// Required: No
    ///
    /// Type: Spacing
    ///
    /// Update requires: No interruption
    #[serde(rename = "Padding")]
    pub padding: Option<Spacing>,

}


/// Determines the gradient color settings.
#[derive(Default, serde::Serialize)]
pub struct GradientColor {


    /// 
    /// The list of gradient color stops.
    /// 
    /// Required: No
    ///
    /// Type: List of GradientStop
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stops")]
    pub stops: Option<Vec<GradientStop>>,

}


/// A control from a date filter that is used to specify date and time.
#[derive(Default, serde::Serialize)]
pub struct FilterDateTimePickerControl {


    /// 
    /// The ID of the FilterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,


    /// 
    /// The source filter ID of the FilterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,


    /// 
    /// The title of the FilterDateTimePickerControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The date time picker type of a FilterDateTimePickerControl. Choose one of the following options:
    /// 
    /// SINGLE_VALUED: The filter condition is a fixed date.                        DATE_RANGE: The filter condition is a date time range.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DATE_RANGE | SINGLE_VALUED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: DateTimePickerControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,

}


/// The URL configuration for a table field.
#[derive(Default, serde::Serialize)]
pub struct TableFieldURLConfiguration {


    /// 
    /// The link configuration of a table field URL.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldLinkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LinkConfiguration")]
    pub link_configuration: Option<TableFieldLinkConfiguration>,


    /// 
    /// The image configuration of a table field URL.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldImageConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<TableFieldImageConfiguration>,

}


/// The general configuration of a column.
#[derive(Default, serde::Serialize)]
pub struct ColumnConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ColorsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorsConfiguration")]
    pub colors_configuration: Option<ColorsConfiguration>,


    /// 
    /// The column.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The role of the column.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIMENSION | MEASURE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: Option<String>,


    /// 
    /// The format configuration of a column.
    /// 
    /// Required: No
    ///
    /// Type: FormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,

}


/// A geospatial map or a points on map visual.
///
/// For more information, see Creating point maps in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct GeospatialMapVisual {


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GeospatialMapConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}


/// The maximum and minimum computation configuration.
#[derive(Default, serde::Serialize)]
pub struct MaximumMinimumComputation {


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The type of computation. Choose one of the following options:
    /// 
    /// MAXIMUM: A maximum computation.               MINIMUM: A minimum computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MAXIMUM | MINIMUM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,

}


/// A key performance indicator (KPI).
///
/// For more information, see Using KPIs in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct KPIVisual {


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The conditional formatting of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<KPIConditionalFormatting>,


    /// 
    /// The configuration of a KPI visual.
    /// 
    /// Required: No
    ///
    /// Type: KPIConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<KPIConfiguration>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}


/// The default values of the DecimalParameterDeclaration.
#[derive(Default, serde::Serialize)]
pub struct DecimalDefaultValues {


    /// 
    /// The dynamic value of the DecimalDefaultValues. Different defaults are displayed according to users, groups, and values mapping.
    /// 
    /// Required: No
    ///
    /// Type: DynamicDefaultValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,


    /// 
    /// The static values of the DecimalDefaultValues.
    /// 
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,

}


/// The color map that determines the color options for a particular element.
#[derive(Default, serde::Serialize)]
pub struct DataPathColor {


    /// 
    /// The color that needs to be applied to the element.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: String,


    /// 
    /// The element that the color needs to be applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: DataPathValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Element")]
    pub element: DataPathValue,


    /// 
    /// The time granularity of the field that the color needs to be applied to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<String>,

}


/// The link configuration of a table field URL.
#[derive(Default, serde::Serialize)]
pub struct TableFieldLinkConfiguration {


    /// 
    /// The URL target (new tab, new window, same tab) for the table link configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NEW_TAB | NEW_WINDOW | SAME_TAB
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: String,


    /// 
    /// The URL content (text, icon) for the table link configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: TableFieldLinkContentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: TableFieldLinkContentConfiguration,

}


/// Determines the gradient stop configuration.
#[derive(Default, serde::Serialize)]
pub struct GradientStop {


    /// 
    /// Determines the color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,


    /// 
    /// Determines the data value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,


    /// 
    /// Determines gradient offset value.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "GradientOffset")]
    pub gradient_offset: f64,

}


/// The style options of the box plot.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotStyleOptions {


    /// 
    /// The fill styles (solid, transparent) of the box plot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SOLID | TRANSPARENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "FillStyle")]
    pub fill_style: Option<String>,

}


/// An ad hoc (one-time) filtering option.
#[derive(Default, serde::Serialize)]
pub struct AdHocFilteringOption {


    /// 
    /// Availability status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The data path that needs to be sorted.
#[derive(Default, serde::Serialize)]
pub struct DataPathValue {


    /// 
    /// The actual value of the field that needs to be sorted.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldValue")]
    pub field_value: String,


    /// 
    /// The field ID of the field that needs to be sorted.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// A control to display a text box that is used to enter a single entry.
#[derive(Default, serde::Serialize)]
pub struct ParameterTextFieldControl {


    /// 
    /// The ID of the ParameterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The source parameter name of the ParameterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,


    /// 
    /// The title of the ParameterTextFieldControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: TextFieldControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,

}


/// The optional configuration of subtotals cells.
#[derive(Default, serde::Serialize)]
pub struct PivotTableFieldSubtotalOptions {


    /// 
    /// The field ID of the subtotal options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}


/// The label options for a chart axis. You must specify the field that the label is targeted to.
#[derive(Default, serde::Serialize)]
pub struct AxisLabelOptions {


    /// 
    /// The options that indicate which field the label belongs to.
    /// 
    /// Required: No
    ///
    /// Type: AxisLabelReferenceOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyTo")]
    pub apply_to: Option<AxisLabelReferenceOptions>,


    /// 
    /// The text for the axis label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The font configuration of the axis label.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,

}


/// The options that determine the presentation of the GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartOptions {


    /// 
    /// The comparison configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ComparisonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,


    /// 
    /// The options that determine the primary value font configuration.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,


    /// 
    /// The arc axis configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ArcAxisConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArcAxis")]
    pub arc_axis: Option<ArcAxisConfiguration>,


    /// 
    /// The arc configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ArcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arc")]
    pub arc: Option<ArcConfiguration>,


    /// 
    /// The options that determine the primary value display type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTUAL | COMPARISON | HIDDEN
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<String>,

}


/// A scatter plot.
///
/// For more information, see Using scatter plots in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct ScatterPlotVisual {


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: ScatterPlotConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ScatterPlotConfiguration>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}


/// The text format for the title.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ShortFormatText {


    /// 
    /// Plain text format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,


    /// 
    /// Rich text. Examples of rich text include bold, underline, and italics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,

}


/// The sort by field for the field sort options.
#[derive(Default, serde::Serialize)]
pub struct PivotTableSortBy {


    /// 
    /// The column sort (field id, direction) for the pivot table sort by options.
    /// 
    /// Required: No
    ///
    /// Type: ColumnSort
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: Option<ColumnSort>,


    /// 
    /// The data path sort (data path value, direction) for the pivot table sort by options.
    /// 
    /// Required: No
    ///
    /// Type: DataPathSort
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPath")]
    pub data_path: Option<DataPathSort>,


    /// 
    /// The field sort (field id, direction) for the pivot table sort by options.
    /// 
    /// Required: No
    ///
    /// Type: FieldSort
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<FieldSort>,

}


/// The filter that is applied to the options.
#[derive(Default, serde::Serialize)]
pub struct SheetVisualScopingConfiguration {


    /// 
    /// The scope of the applied entities. Choose one of the following options:
    /// 
    /// ALL_VISUALS                                SELECTED_VISUALS
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VISUALS | SELECTED_VISUALS
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: String,


    /// 
    /// The selected sheet that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetId")]
    pub sheet_id: String,


    /// 
    /// The selected visuals that the filter is applied to.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualIds")]
    pub visual_ids: Option<Vec<String>>,

}


/// A list of filter configurations.
#[derive(Default, serde::Serialize)]
pub struct FilterListConfiguration {


    /// 
    /// The match operator that is used to determine if a filter should be applied.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | DOES_NOT_CONTAIN | DOES_NOT_EQUAL | ENDS_WITH | EQUALS | STARTS_WITH
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchOperator")]
    pub match_operator: String,


    /// 
    /// The list of category values for the filter.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,


    /// 
    /// Select all of the values. Null is not the assigned value of select all.
    /// 
    /// FILTER_ALL_VALUES
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<String>,

}


/// The rolling date configuration of a date time filter.
#[derive(Default, serde::Serialize)]
pub struct RollingDateConfiguration {


    /// 
    /// The expression of the rolling date configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,


    /// 
    /// The data set that is used in the rolling date configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: Option<String>,

}


/// The dimension type field with categorical type columns..
#[derive(Default, serde::Serialize)]
pub struct CategoricalDimensionField {


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: StringFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,


    /// 
    /// The custom hierarchy ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The column that is used in the CategoricalDimensionField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}


/// The field well configuration of a histogram.
#[derive(Default, serde::Serialize)]
pub struct HistogramAggregatedFieldWells {


    /// 
    /// The value field wells of a histogram. Values are aggregated by COUNT or DISTINCT_COUNT.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The sort configuration of a FunnelChartVisual.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartSortConfiguration {


    /// 
    /// The sort configuration of the category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of categories displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}


/// A grid layout to define the placement of sheet control.
#[derive(Default, serde::Serialize)]
pub struct SheetControlLayout {


    /// 
    /// The configuration that determines the elements and canvas size options of sheet control.
    /// 
    /// Required: Yes
    ///
    /// Type: SheetControlLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: SheetControlLayoutConfiguration,

}


/// The dimension type field with numerical type columns.
#[derive(Default, serde::Serialize)]
pub struct NumericalDimensionField {


    /// 
    /// The custom hierarchy ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,


    /// 
    /// The column that is used in the NumericalDimensionField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: NumberFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,

}


/// The forecast configuration that is used in a line chart's display properties.
#[derive(Default, serde::Serialize)]
pub struct ForecastConfiguration {


    /// 
    /// The forecast properties setup of a forecast in the line       chart.
    /// 
    /// Required: No
    ///
    /// Type: TimeBasedForecastProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForecastProperties")]
    pub forecast_properties: Option<TimeBasedForecastProperties>,


    /// 
    /// The forecast scenario of a forecast in the line chart.
    /// 
    /// Required: No
    ///
    /// Type: ForecastScenario
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scenario")]
    pub scenario: Option<ForecastScenario>,

}


/// The sort configuration for a TableVisual.
#[derive(Default, serde::Serialize)]
pub struct TableSortConfiguration {


    /// 
    /// The field sort options for rows in the table.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowSort")]
    pub row_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The pagination configuration (page size, page number) for the table.
    /// 
    /// Required: No
    ///
    /// Type: PaginationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}


/// The custom icon content for the table link content configuration.
#[derive(Default, serde::Serialize)]
pub struct TableFieldCustomIconContent {


    /// 
    /// The icon set type (link) of the custom icon content for table URL link content.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LINK
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icon")]
    pub icon: Option<String>,

}


/// The unique values computation configuration.
#[derive(Default, serde::Serialize)]
pub struct UniqueValuesComputation {


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The category field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: DimensionField,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The configuration of a BoxPlotVisual.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotChartConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The label display options (grid line, range, scale, axis step) of a box plot category.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The box plot chart options for a box plot visual
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "BoxPlotOptions")]
    pub box_plot_options: Option<BoxPlotOptions>,


    /// 
    /// The reference line setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: List of ReferenceLine
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BoxPlotFieldWells>,


    /// 
    /// The label display options (grid line, range, scale, axis step) of a box plot category.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The sort configuration of a BoxPlotVisual.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BoxPlotSortConfiguration>,


    /// 
    /// The label options (label text, label visibility and sort icon visibility) of a box plot value.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The label options (label text, label visibility and sort Icon visibility) of a box plot category.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}


/// The configuration that defines the default value of a String parameter when a value has not been set.
#[derive(Default, serde::Serialize)]
pub struct StringValueWhenUnsetConfiguration {


    /// 
    /// The built-in options for default values. The value can be one of the following:
    /// 
    /// RECOMMENDED: The recommended value.                        NULL: The NULL value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NULL | RECOMMENDED_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<String>,


    /// 
    /// A custom value that's used when the value of a parameter isn't set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}


/// A pivot table.
///
/// For more information, see Using pivot tables in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct PivotTableVisual {


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PivotTableConfiguration>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The conditional formatting for a PivotTableVisual.
    /// 
    /// Required: No
    ///
    /// Type: PivotTableConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<PivotTableConditionalFormatting>,

}


/// The value label configuration of the label in a reference line.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineValueLabelConfiguration {


    /// 
    /// The relative position of the value label. Choose one of the following options:
    /// 
    /// BEFORE_CUSTOM_LABEL                                AFTER_CUSTOM_LABEL
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AFTER_CUSTOM_LABEL | BEFORE_CUSTOM_LABEL
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativePosition")]
    pub relative_position: Option<String>,


    /// 
    /// The format configuration of the value label.
    /// 
    /// Required: No
    ///
    /// Type: NumericFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}


/// The static data configuration of the reference line data configuration.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineStaticDataConfiguration {


    /// 
    /// The double input of the static data.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,

}


/// The field well configuration of a pie chart.
#[derive(Default, serde::Serialize)]
pub struct PieChartAggregatedFieldWells {


    /// 
    /// The value field wells of a pie chart. Values are aggregated based on categories.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The category (group/color) field wells of a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,


    /// 
    /// The small multiples field well of a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct ListControlDisplayOptions {


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,


    /// 
    /// The configuration of the search options in a list control.
    /// 
    /// Required: No
    ///
    /// Type: ListControlSearchOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SearchOptions")]
    pub search_options: Option<ListControlSearchOptions>,


    /// 
    /// The configuration of the Select all options in a list control.
    /// 
    /// Required: No
    ///
    /// Type: ListControlSelectAllOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,

}


/// The format of the comparison.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ComparisonFormatConfiguration {


    /// 
    /// The number display format.
    /// 
    /// Required: No
    ///
    /// Type: NumberDisplayFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,


    /// 
    /// The percentage display format.
    /// 
    /// Required: No
    ///
    /// Type: PercentageDisplayFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,

}


/// The field options for a pivot table visual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableFieldOptions {


    /// 
    /// The data path options for the pivot table field options.
    /// 
    /// Required: No
    ///
    /// Type: List of PivotTableDataPathOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPathOptions")]
    pub data_path_options: Option<Vec<PivotTableDataPathOption>>,


    /// 
    /// The selected field options for the pivot table field options.
    /// 
    /// Required: No
    ///
    /// Type: List of PivotTableFieldOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<PivotTableFieldOption>>,

}


/// The configuration of an insight visual.
#[derive(Default, serde::Serialize)]
pub struct InsightConfiguration {


    /// 
    /// The computations configurations of the insight visual
    /// 
    /// Required: No
    ///
    /// Type: List of Computation
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Computations")]
    pub computations: Option<Vec<Computation>>,


    /// 
    /// The custom narrative of the insight visual.
    /// 
    /// Required: No
    ///
    /// Type: CustomNarrativeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomNarrative")]
    pub custom_narrative: Option<CustomNarrativeOptions>,

}


/// The total options for a pivot table visual.
#[derive(Default, serde::Serialize)]
pub struct PivotTableTotalOptions {


    /// 
    /// The row total options.
    /// 
    /// Required: No
    ///
    /// Type: PivotTotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowTotalOptions")]
    pub row_total_options: Option<PivotTotalOptions>,


    /// 
    /// The column total options.
    /// 
    /// Required: No
    ///
    /// Type: PivotTotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnTotalOptions")]
    pub column_total_options: Option<PivotTotalOptions>,


    /// 
    /// The row subtotal options.
    /// 
    /// Required: No
    ///
    /// Type: SubtotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowSubtotalOptions")]
    pub row_subtotal_options: Option<SubtotalOptions>,


    /// 
    /// The column subtotal options.
    /// 
    /// Required: No
    ///
    /// Type: SubtotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSubtotalOptions")]
    pub column_subtotal_options: Option<SubtotalOptions>,

}


/// A decimal parameter.
#[derive(Default, serde::Serialize)]
pub struct DecimalParameter {


    /// 
    /// A display name for the decimal parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The values for the decimal parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<f64>,

}


/// A control to display a dropdown list with buttons that are used to select a single value.
#[derive(Default, serde::Serialize)]
pub struct FilterDropDownControl {


    /// 
    /// The display options of the FilterDropDownControl.
    /// 
    /// Required: No
    ///
    /// Type: DropDownControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,


    /// 
    /// A list of selectable values that are used in a control.
    /// 
    /// Required: No
    ///
    /// Type: FilterSelectableValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,


    /// 
    /// The values that are displayed in a control can be configured to only show values that are valid based on what's selected in other controls.
    /// 
    /// Required: No
    ///
    /// Type: CascadingControlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,


    /// 
    /// The title of the FilterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The type of the FilterDropDownControl. Choose one of the following options:
    /// 
    /// MULTI_SELECT: The user can select multiple entries from a dropdown menu.                        SINGLE_SELECT: The user can select a single entry from a dropdown menu.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_SELECT | SINGLE_SELECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The ID of the FilterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,


    /// 
    /// The source filter ID of the FilterDropDownControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}


/// The contribution analysis visual display for a line, pie, or bar chart.
#[derive(Default, serde::Serialize)]
pub struct ContributionAnalysisDefault {


    /// 
    /// The dimensions columns that are used in the contribution analysis,       usually a list of ColumnIdentifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ColumnIdentifier
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorDimensions")]
    pub contributor_dimensions: Vec<ColumnIdentifier>,


    /// 
    /// The measure field that is used in the contribution analysis.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureFieldId")]
    pub measure_field_id: String,

}


/// The field wells of a radar chart visual.
#[derive(Default, serde::Serialize)]
pub struct RadarChartFieldWells {


    /// 
    /// The aggregated field wells of a radar chart visual.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "RadarChartAggregatedFieldWells")]
    pub radar_chart_aggregated_field_wells: Option<RadarChartAggregatedFieldWells>,

}


/// Formatting configuration for string fields.
#[derive(Default, serde::Serialize)]
pub struct StringFormatConfiguration {


    /// 
    /// The formatting configuration for numeric strings.
    /// 
    /// Required: No
    ///
    /// Type: NumericFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,


    /// 
    /// The options that determine the null value format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NullValueFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct DateTimePickerControlDisplayOptions {


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,


    /// 
    /// Customize how dates are formatted in controls.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

}


/// A date-time parameter.
#[derive(Default, serde::Serialize)]
pub struct DateTimeParameter {


    /// 
    /// The values for the date-time parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,


    /// 
    /// A display name for the date-time parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// A sheet, which is an object that contains a set of visuals that       are viewed together on one page in Amazon QuickSight. Every analysis and dashboard       contains at least one sheet. Each sheet contains at least one visualization widget, for       example a chart, pivot table, or narrative insight. Sheets can be associated with other       components, such as controls, filters, and so on.
#[derive(Default, serde::Serialize)]
pub struct Sheet {


    /// 
    /// The unique identifier associated with a sheet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetId")]
    pub sheet_id: Option<String>,


    /// 
    /// The name of a sheet. This name is displayed on the sheet's tab in the Amazon QuickSight       console.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// A table visual.
///
/// For more information, see Using tables as visuals in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct TableVisual {


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The conditional formatting for a PivotTableVisual.
    /// 
    /// Required: No
    ///
    /// Type: TableConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<TableConditionalFormatting>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TableConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TableConfiguration>,

}


/// The formatting configuration for the icon.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingIcon {


    /// 
    /// Formatting configuration for icon set.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingIconSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "IconSet")]
    pub icon_set: Option<ConditionalFormattingIconSet>,


    /// 
    /// Determines the custom condition for an icon set.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingCustomIconCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomCondition")]
    pub custom_condition: Option<ConditionalFormattingCustomIconCondition>,

}


/// The configuration of a BarChartVisual.
#[derive(Default, serde::Serialize)]
pub struct BarChartConfiguration {


    /// 
    /// The reference line setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: List of ReferenceLine
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The small multiples setup for the visual.
    /// 
    /// Required: No
    ///
    /// Type: SmallMultiplesOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,


    /// 
    /// The label options (label text, label visibility and sort icon visibility) for a bar chart value.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The sort configuration of a BarChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: BarChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BarChartSortConfiguration>,


    /// 
    /// The label display options (grid line, range, scale, axis step) for bar chart category.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The label options (label text, label visibility and sort icon visibility) for a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: BarChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BarChartFieldWells>,


    /// 
    /// The label options (label text, label visibility and sort icon visibility) for a color that is used in a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// Determines the arrangement of the bars. The orientation and arrangement of bars determine the type of bar that is used in the visual.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLUSTERED | STACKED | STACKED_PERCENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<String>,


    /// 
    /// The contribution analysis (anomaly configuration) setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: List of ContributionAnalysisDefault
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,


    /// 
    /// The orientation of the bars in a bar chart visual. There are two valid values in this structure:
    /// 
    /// HORIZONTAL: Used for charts that have horizontal bars. Visuals that use this value are horizontal bar charts, horizontal stacked bar charts, and horizontal stacked 100% bar charts.                        VERTICAL: Used for charts that have vertical bars. Visuals that use this value are vertical bar charts, vertical stacked bar charts, and vertical stacked 100% bar charts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HORIZONTAL | VERTICAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Orientation")]
    pub orientation: Option<String>,


    /// 
    /// The label display options (grid line, range, scale, axis step) for a bar chart value.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueAxis")]
    pub value_axis: Option<AxisDisplayOptions>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct RelativeDateTimeControlDisplayOptions {


    /// 
    /// Customize how dates are formatted in controls.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}


/// The configuration of a GeospatialMapVisual.
#[derive(Default, serde::Serialize)]
pub struct GeospatialMapConfiguration {


    /// 
    /// The window options of the geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialWindowOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,


    /// 
    /// The point style options of the geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialPointStyleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PointStyleOptions")]
    pub point_style_options: Option<GeospatialPointStyleOptions>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The map style options of the geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapStyleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: GeospatialMapFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GeospatialMapFieldWells>,


    /// 
    /// The tooltip display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,

}


/// The setup for the detailed tooltip.
#[derive(Default, serde::Serialize)]
pub struct FieldBasedTooltip {


    /// 
    /// The visibility of Show aggregations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationVisibility")]
    pub aggregation_visibility: Option<String>,


    /// 
    /// The fields configuration in the       tooltip.
    /// 
    /// Required: No
    ///
    /// Type: List of TooltipItem
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TooltipFields")]
    pub tooltip_fields: Option<Vec<TooltipItem>>,


    /// 
    /// The type for the >tooltip title. Choose one of the following options:
    /// 
    /// NONE: Doesn't use the primary value as the title.                        PRIMARY_VALUE: Uses primary value as the title.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | PRIMARY_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TooltipTitleType")]
    pub tooltip_title_type: Option<String>,

}


/// The sort configuration for a field in a       field well.
#[derive(Default, serde::Serialize)]
pub struct FieldSort {


    /// 
    /// The sort direction. Choose one of the following       options:
    /// 
    /// ASC: Ascending                        DESC: Descending
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ASC | DESC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: String,


    /// 
    /// The sort configuration target field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// The options that determine the thousands separator configuration.
#[derive(Default, serde::Serialize)]
pub struct ThousandSeparatorOptions {


    /// 
    /// Determines the visibility of the thousands separator.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// Determines the thousands separator symbol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMMA | DOT | SPACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,

}


/// The configuration of adding parameters in action.
#[derive(Default, serde::Serialize)]
pub struct SetParameterValueConfiguration {


    /// 
    /// The destination parameter name of the SetParameterValueConfiguration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationParameterName")]
    pub destination_parameter_name: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: DestinationParameterValueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: DestinationParameterValueConfiguration,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct DropDownControlDisplayOptions {


    /// 
    /// The configuration of the Select all options in a       dropdown control.
    /// 
    /// Required: No
    ///
    /// Type: ListControlSelectAllOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}


/// An element within a free-form layout.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutElement {


    /// 
    /// The border style configuration of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: FreeFormLayoutElementBorderStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<FreeFormLayoutElementBorderStyle>,


    /// 
    /// The rendering rules that determine when an element should be displayed within a free-form layout.
    /// 
    /// Required: No
    ///
    /// Type: List of SheetElementRenderingRule
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenderingRules")]
    pub rendering_rules: Option<Vec<SheetElementRenderingRule>>,


    /// 
    /// A unique identifier for an element within a free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElementId")]
    pub element_id: String,


    /// 
    /// The border style configuration of a free-form layout element. This border style is used when the element is selected.
    /// 
    /// Required: No
    ///
    /// Type: FreeFormLayoutElementBorderStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedBorderStyle")]
    pub selected_border_style: Option<FreeFormLayoutElementBorderStyle>,


    /// 
    /// The loading animation configuration of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: LoadingAnimation
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadingAnimation")]
    pub loading_animation: Option<LoadingAnimation>,


    /// 
    /// The height of an element within a free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    pub height: String,


    /// 
    /// The visibility of an element within a free-form layout.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The background style configuration of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: FreeFormLayoutElementBackgroundStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundStyle")]
    pub background_style: Option<FreeFormLayoutElementBackgroundStyle>,


    /// 
    /// The type of element.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_CONTROL | PARAMETER_CONTROL | TEXT_BOX | VISUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElementType")]
    pub element_type: String,


    /// 
    /// The x-axis coordinate of the element.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisLocation")]
    pub xaxis_location: String,


    /// 
    /// The y-axis coordinate of the element.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxisLocation")]
    pub yaxis_location: String,


    /// 
    /// The width of an element within a free-form layout.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    pub width: String,

}


/// Determines the color that is applied to a particular data value.
#[derive(Default, serde::Serialize)]
pub struct DataColor {


    /// 
    /// The color that is applied to the data value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,


    /// 
    /// The data value that the color is applied to.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,

}


/// The configuration options to sort aggregated values.
#[derive(Default, serde::Serialize)]
pub struct AggregationSortConfiguration {


    /// 
    /// The function that aggregates the values in Column.
    /// 
    /// Required: Yes
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: AggregationFunction,


    /// 
    /// The column that determines the sort order of aggregated values.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The sort direction of values.
    /// 
    /// ASC: Sort in ascending order.                        DESC: Sort in descending order.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ASC | DESC
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortDirection")]
    pub sort_direction: String,

}


/// A parameter declaration for the Integer data type.
#[derive(Default, serde::Serialize)]
pub struct IntegerParameterDeclaration {


    /// 
    /// A parameter declaration for the Integer data type.
    /// 
    /// Required: No
    ///
    /// Type: IntegerValueWhenUnsetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<IntegerValueWhenUnsetConfiguration>,


    /// 
    /// The value type determines whether the parameter is a single-value or multi-value parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_VALUED | SINGLE_VALUED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of MappedDataSetParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,


    /// 
    /// The name of the parameter that is being declared.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.
    /// 
    /// Required: No
    ///
    /// Type: IntegerDefaultValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<IntegerDefaultValues>,

}


/// Formatting configuration for gradient color.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingGradientColor {


    /// 
    /// The expression that determines the formatting configuration for gradient color.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,


    /// 
    /// Determines the color.
    /// 
    /// Required: Yes
    ///
    /// Type: GradientColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: GradientColor,

}


/// The options that determine the title styles for each small multiples       panel.
#[derive(Default, serde::Serialize)]
pub struct PanelTitleOptions {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,


    /// 
    /// Determines whether or not panel titles are displayed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// Sets the horizontal text alignment of the title within each panel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | CENTER | LEFT | RIGHT
    ///
    /// Update requires: No interruption
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<String>,

}


/// An object, structure, or sub-structure of an analysis, template, or dashboard.
#[derive(Default, serde::Serialize)]
pub struct Entity {


    /// 
    /// The hierarchical path of the entity within the analysis, template, or dashboard definition tree.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,

}


/// The options that determine the presentation of the arc of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartArcConditionalFormatting {


    /// 
    /// The conditional formatting of the arc foreground color.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}


/// A control to display a horizontal toggle bar. This is used to change a value by sliding the toggle.
#[derive(Default, serde::Serialize)]
pub struct ParameterSliderControl {


    /// 
    /// The source parameter name of the ParameterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,


    /// 
    /// The title of the ParameterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The larger value that is displayed at the right of the slider.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,


    /// 
    /// The smaller value that is displayed at the left of the slider.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,


    /// 
    /// The ID of the ParameterSliderControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: SliderControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,


    /// 
    /// The number of increments that the slider bar is divided into.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepSize")]
    pub step_size: f64,

}


/// The field options for a table visual.
#[derive(Default, serde::Serialize)]
pub struct TableFieldOptions {


    /// 
    /// The selected field options for the table field options.
    /// 
    /// Required: No
    ///
    /// Type: List of TableFieldOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<TableFieldOption>>,


    /// 
    /// The order of field IDs of the field options for a table visual.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Order")]
    pub order: Option<Vec<String>>,

}


/// The sort configuration of a BoxPlotVisual.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotSortConfiguration {


    /// 
    /// The sort configuration of a group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The pagination configuration of a table visual or box plot.
    /// 
    /// Required: No
    ///
    /// Type: PaginationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}


/// The options that determine the currency display format configuration.
#[derive(Default, serde::Serialize)]
pub struct CurrencyDisplayFormatConfiguration {


    /// 
    /// Determines the symbol for the currency format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [A-Z]{3}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,


    /// 
    /// The options that determine the null value format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NullValueFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,


    /// 
    /// The options that determine the negative value configuration.
    /// 
    /// Required: No
    ///
    /// Type: NegativeValueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,


    /// 
    /// Determines the suffix value of the currency format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// Determines the number scale value for the currency format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | BILLIONS | MILLIONS | NONE | THOUSANDS | TRILLIONS
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<String>,


    /// 
    /// The option that determines the decimal places configuration.
    /// 
    /// Required: No
    ///
    /// Type: DecimalPlacesConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,


    /// 
    /// The options that determine the numeric separator configuration.
    /// 
    /// Required: No
    ///
    /// Type: NumericSeparatorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,


    /// 
    /// Determines the prefix value of the currency format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}


/// The menu options for a visual.
#[derive(Default, serde::Serialize)]
pub struct VisualMenuOption {


    /// 
    /// The availaiblity status of a visual's menu options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,

}


/// The tooltip.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct TooltipItem {


    /// 
    /// The tooltip item for the fields.
    /// 
    /// Required: No
    ///
    /// Type: FieldTooltipItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldTooltipItem")]
    pub field_tooltip_item: Option<FieldTooltipItem>,


    /// 
    /// The tooltip item for the columns that are not part of a field well.
    /// 
    /// Required: No
    ///
    /// Type: ColumnTooltipItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnTooltipItem")]
    pub column_tooltip_item: Option<ColumnTooltipItem>,

}


/// A box plot.
///
/// For more information, see Using box plots in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotVisual {


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BoxPlotChartConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}


/// Dashboard publish options.
#[derive(Default, serde::Serialize)]
pub struct DashboardPublishOptions {


    /// 
    /// Determines if hidden fields are exported with a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: ExportWithHiddenFieldsOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExportWithHiddenFieldsOption")]
    pub export_with_hidden_fields_option: Option<ExportWithHiddenFieldsOption>,


    /// 
    /// Export to .csv option.
    /// 
    /// Required: No
    ///
    /// Type: ExportToCSVOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExportToCSVOption")]
    pub export_to_csvoption: Option<ExportToCSVOption>,


    /// 
    /// The data point menu label options of a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: DataPointMenuLabelOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPointMenuLabelOption")]
    pub data_point_menu_label_option: Option<DataPointMenuLabelOption>,


    /// 
    /// The visual publish options of a visual in a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: DashboardVisualPublishOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPublishOptions")]
    pub visual_publish_options: Option<DashboardVisualPublishOptions>,


    /// 
    /// The menu options of a visual in a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: VisualMenuOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualMenuOption")]
    pub visual_menu_option: Option<VisualMenuOption>,


    /// 
    /// The drill-down options of data points in a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: DataPointDrillUpDownOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPointDrillUpDownOption")]
    pub data_point_drill_up_down_option: Option<DataPointDrillUpDownOption>,


    /// 
    /// Ad hoc (one-time) filtering option.
    /// 
    /// Required: No
    ///
    /// Type: AdHocFilteringOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdHocFilteringOption")]
    pub ad_hoc_filtering_option: Option<AdHocFilteringOption>,


    /// 
    /// The axis sort options of a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: VisualAxisSortOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualAxisSortOption")]
    pub visual_axis_sort_option: Option<VisualAxisSortOption>,


    /// 
    /// The data point tool tip options of a dashboard.
    /// 
    /// Required: No
    ///
    /// Type: DataPointTooltipOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPointTooltipOption")]
    pub data_point_tooltip_option: Option<DataPointTooltipOption>,


    /// 
    /// Sheet controls option.
    /// 
    /// Required: No
    ///
    /// Type: SheetControlsOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetControlsOption")]
    pub sheet_controls_option: Option<SheetControlsOption>,


    /// 
    /// The sheet layout maximization options of a dashbaord.
    /// 
    /// Required: No
    ///
    /// Type: SheetLayoutElementMaximizationOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetLayoutElementMaximizationOption")]
    pub sheet_layout_element_maximization_option: Option<SheetLayoutElementMaximizationOption>,

}


/// The option that determines the hierarchy of the fields that are built within a visual's field wells. These fields can't be duplicated to other visuals.
#[derive(Default, serde::Serialize)]
pub struct ExplicitHierarchy {


    /// 
    /// The hierarchy ID of the explicit hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,


    /// 
    /// The option that determines the drill down filters for the explicit hierarchy.
    /// 
    /// Required: No
    ///
    /// Type: List of DrillDownFilter
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,


    /// 
    /// The list of columns that define the explicit hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ColumnIdentifier
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,

}


/// The URL operation that opens a link to another webpage.
#[derive(Default, serde::Serialize)]
pub struct CustomActionURLOperation {


    /// 
    /// The target of the CustomActionURLOperation.
    /// 
    /// Valid values are defined as follows:
    /// 
    /// NEW_TAB: Opens the target URL in a new browser tab.                        NEW_WINDOW: Opens the target URL in a new browser window.                        SAME_TAB: Opens the target URL in the same browser tab.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NEW_TAB | NEW_WINDOW | SAME_TAB
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLTarget")]
    pub urltarget: String,


    /// 
    /// THe URL link of the CustomActionURLOperation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLTemplate")]
    pub urltemplate: String,

}


/// Configuration options for the canvas of a free-form layout.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutCanvasSizeOptions {


    /// 
    /// The options that determine the sizing of the canvas used in a free-form layout.
    /// 
    /// Required: No
    ///
    /// Type: FreeFormLayoutScreenCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<FreeFormLayoutScreenCanvasSizeOptions>,

}


/// An element within a grid layout.
#[derive(Default, serde::Serialize)]
pub struct GridLayoutElement {


    /// 
    /// A unique identifier for an element within a grid layout.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElementId")]
    pub element_id: String,


    /// 
    /// The height of a grid element expressed as a number of grid rows.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 21
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowSpan")]
    pub row_span: f64,


    /// 
    /// The type of element.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FILTER_CONTROL | PARAMETER_CONTROL | TEXT_BOX | VISUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElementType")]
    pub element_type: String,


    /// 
    /// The row index for the upper left corner of an element.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 9009
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowIndex")]
    pub row_index: Option<f64>,


    /// 
    /// The width of a grid element expressed as a number of grid columns.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 36
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSpan")]
    pub column_span: f64,


    /// 
    /// The column index for the upper left corner of an element.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 35
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnIndex")]
    pub column_index: Option<f64>,

}


/// A parameter declaration for the String data type.
#[derive(Default, serde::Serialize)]
pub struct StringParameterDeclaration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of MappedDataSetParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,


    /// 
    /// The name of the parameter that is being declared.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.
    /// 
    /// Required: No
    ///
    /// Type: StringDefaultValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<StringDefaultValues>,


    /// 
    /// The value type determines whether the parameter is a single-value or multi-value parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_VALUED | SINGLE_VALUED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: String,


    /// 
    /// The configuration that defines the default value of a String parameter when a value has not been set.
    /// 
    /// Required: No
    ///
    /// Type: StringValueWhenUnsetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<StringValueWhenUnsetConfiguration>,

}


/// The configuration of a scatter plot.
#[derive(Default, serde::Serialize)]
pub struct ScatterPlotConfiguration {


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: ScatterPlotFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ScatterPlotFieldWells>,


    /// 
    /// The label display options (grid line, range, scale, and axis step) of the scatter plot's y-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The options that determine if visual data labels are displayed.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of the scatter plot's y-axis.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxisLabelOptions")]
    pub yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The palette (chart color) display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The legend display setup of the visual.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The label display options (grid line, range, scale, and axis step) of the scatter plot's x-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The label options (label text, label visibility, and sort icon visibility) of the scatter plot's x-axis.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,

}


/// The sort configuration of a waterfall visual.
#[derive(Default, serde::Serialize)]
pub struct WaterfallChartSortConfiguration {


    /// 
    /// The limit on the number of bar groups that are displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BreakdownItemsLimit")]
    pub breakdown_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of the category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}


/// The options for a table field.
#[derive(Default, serde::Serialize)]
pub struct TableFieldOption {


    /// 
    /// The field ID for a table field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The URL configuration for a table field.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldURLConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLStyling")]
    pub urlstyling: Option<TableFieldURLConfiguration>,


    /// 
    /// The visibility of a table field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The custom label for a table field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The width for a table field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    pub width: Option<String>,

}


/// The field wells of a BoxPlotVisual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct BoxPlotFieldWells {


    /// 
    /// The aggregated field wells of a box plot.
    /// 
    /// Required: No
    ///
    /// Type: BoxPlotAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "BoxPlotAggregatedFieldWells")]
    pub box_plot_aggregated_field_wells: Option<BoxPlotAggregatedFieldWells>,

}


/// The category drill down filter.
#[derive(Default, serde::Serialize)]
pub struct NumericEqualityDrillDownFilter {


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The value of the double input numeric drill down filter.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,

}


/// Conditional formatting options of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartConditionalFormattingOption {


    /// 
    /// The options that determine the presentation of the arc of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartArcConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arc")]
    pub arc: Option<GaugeChartArcConditionalFormatting>,


    /// 
    /// The conditional formatting for the primary value of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartPrimaryValueConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<GaugeChartPrimaryValueConditionalFormatting>,

}


/// The data options for an axis.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct AxisDataOptions {


    /// 
    /// The options for an axis with a date field.
    /// 
    /// Required: No
    ///
    /// Type: DateAxisOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateAxisOptions")]
    pub date_axis_options: Option<DateAxisOptions>,


    /// 
    /// The options for an axis with a numeric field.
    /// 
    /// Required: No
    ///
    /// Type: NumericAxisOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericAxisOptions")]
    pub numeric_axis_options: Option<NumericAxisOptions>,

}


/// The image configuration of a table field URL.
#[derive(Default, serde::Serialize)]
pub struct TableFieldImageConfiguration {


    /// 
    /// The sizing options for the table image configuration.
    /// 
    /// Required: No
    ///
    /// Type: TableCellImageSizingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizingOptions")]
    pub sizing_options: Option<TableCellImageSizingConfiguration>,

}


/// The forecast properties setup of a forecast in the line chart.
#[derive(Default, serde::Serialize)]
pub struct TimeBasedForecastProperties {


    /// 
    /// The periods backward setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,


    /// 
    /// The lower boundary setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,


    /// 
    /// The periods forward setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,


    /// 
    /// The seasonality setup of a forecast computation. Choose one of the following options:
    /// 
    /// NULL: The input is set to NULL.                        NON_NULL: The input is set to a custom value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 180
    ///
    /// Update requires: No interruption
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<f64>,


    /// 
    /// The upper boundary setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,


    /// 
    /// The prediction interval setup of a forecast computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 50
    ///
    /// Maximum: 95
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,

}


/// The options that determine the bin width of a histogram.
#[derive(Default, serde::Serialize)]
pub struct BinWidthOptions {


    /// 
    /// The options that determine the bin count limit.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "BinCountLimit")]
    pub bin_count_limit: Option<f64>,


    /// 
    /// The options that determine the bin width value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}


/// The unaggregated field wells of a scatter plot.
#[derive(Default, serde::Serialize)]
pub struct ScatterPlotUnaggregatedFieldWells {


    /// 
    /// The y-axis field well of a scatter plot.
    /// 
    /// The y-axis is a dimension field and cannot be aggregated.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<DimensionField>>,


    /// 
    /// The size field well of a scatter plot.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,


    /// 
    /// The x-axis field well of a scatter plot.
    /// 
    /// The x-axis is a dimension field and cannot be aggregated.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<DimensionField>>,

}


/// The override configuration of the rendering rules of a sheet.
#[derive(Default, serde::Serialize)]
pub struct SheetElementConfigurationOverrides {


    /// 
    /// Determines whether or not the overrides are visible. Choose one of the following options:
    /// 
    /// VISIBLE                                HIDDEN
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The visual publish options of a visual in a dashboard
#[derive(Default, serde::Serialize)]
pub struct DashboardVisualPublishOptions {


    /// 
    /// Determines if hidden fields are included in an exported dashboard.
    /// 
    /// Required: No
    ///
    /// Type: ExportHiddenFieldsOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExportHiddenFieldsOption")]
    pub export_hidden_fields_option: Option<ExportHiddenFieldsOption>,

}


/// A word cloud.
///
/// For more information, see Using word clouds in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct WordCloudVisual {


    /// 
    /// The configuration settings of the visual.
    /// 
    /// Required: No
    ///
    /// Type: WordCloudChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WordCloudChartConfiguration>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers..
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}


/// The background style configuration of a free-form layout element.
#[derive(Default, serde::Serialize)]
pub struct FreeFormLayoutElementBackgroundStyle {


    /// 
    /// The background color of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}(?:[A-F0-9]{2})?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,


    /// 
    /// The background visibility of a free-form layout element.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// Dashboard error.
#[derive(Default, serde::Serialize)]
pub struct DashboardError {


    /// 
    /// Message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    pub message: Option<String>,


    /// 
    /// Lists the violated entities that caused the dashboard error.
    /// 
    /// Required: No
    ///
    /// Type: List of Entity
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViolatedEntities")]
    pub violated_entities: Option<Vec<Entity>>,


    /// 
    /// Type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACCESS_DENIED | COLUMN_GEOGRAPHIC_ROLE_MISMATCH | COLUMN_REPLACEMENT_MISSING | COLUMN_TYPE_MISMATCH | DATA_SET_NOT_FOUND | INTERNAL_FAILURE | PARAMETER_NOT_FOUND | PARAMETER_TYPE_INVALID | PARAMETER_VALUE_INCOMPATIBLE | SOURCE_NOT_FOUND
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// The display options of a control.
#[derive(Default, serde::Serialize)]
pub struct TextFieldControlDisplayOptions {


    /// 
    /// The options to configure the title visibility, name, and font size.
    /// 
    /// Required: No
    ///
    /// Type: LabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,


    /// 
    /// The configuration of the placeholder options in a text field control.
    /// 
    /// Required: No
    ///
    /// Type: TextControlPlaceholderOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}


/// A RelativeDatesFilter filters relative dates values.
#[derive(Default, serde::Serialize)]
pub struct RelativeDatesFilter {


    /// 
    /// The minimum granularity (period granularity) of the relative dates filter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumGranularity")]
    pub minimum_granularity: Option<String>,


    /// 
    /// The configuration for the exclude period of the filter.
    /// 
    /// Required: No
    ///
    /// Type: ExcludePeriodConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,


    /// 
    /// The date value of the filter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateValue")]
    pub relative_date_value: Option<f64>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// This option determines how null values should be treated when filtering data.
    /// 
    /// ALL_VALUES: Include null values in filtered results.                        NULLS_ONLY: Only include null values in filtered results.                        NON_NULLS_ONLY: Exclude null values from filtered results.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES | NON_NULLS_ONLY | NULLS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullOption")]
    pub null_option: String,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: String,


    /// 
    /// The date configuration of the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: AnchorDateConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnchorDateConfiguration")]
    pub anchor_date_configuration: AnchorDateConfiguration,


    /// 
    /// The parameter whose value should be used for the filter value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,


    /// 
    /// The range date type of the filter. Choose one of the options below:
    /// 
    /// PREVIOUS                                THIS                                LAST                                NOW                                NEXT
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LAST | NEXT | NOW | PREVIOUS | THIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateType")]
    pub relative_date_type: String,

}


/// An insight visual.
///
/// For more information, see Working with insights in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct InsightVisual {


    /// 
    /// The configuration of an insight visual.
    /// 
    /// Required: No
    ///
    /// Type: InsightConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightConfiguration")]
    pub insight_configuration: Option<InsightConfiguration>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The dataset that is used in the insight visual.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// Permission for the resource.
#[derive(Default, serde::Serialize)]
pub struct ResourcePermission {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resource")]
    pub resource: Option<String>,


    /// 
    /// The IAM action to grant or revoke permissions on.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the principal. This can be one of the following:
    /// 
    /// The ARN of an Amazon QuickSight user or group associated with a data source or dataset. (This is   common.)     The ARN of an Amazon QuickSight user, group, or namespace associated with an analysis, dashboard,   template, or theme. (This is common.)     The ARN of an AWS account root: This is an IAM ARN rather than a Amazon QuickSight ARN. Use this option only to share resources (templates) across AWS accounts. (This is   less common.)
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principal")]
    pub principal: String,

}


/// The reference that specifies where the axis label is applied to.
#[derive(Default, serde::Serialize)]
pub struct AxisLabelReferenceOptions {


    /// 
    /// The column that the axis label is targeted to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The field that the axis label is targeted to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// Formatting configuration for DateTime fields.
#[derive(Default, serde::Serialize)]
pub struct DateTimeFormatConfiguration {


    /// 
    /// Determines the DateTime format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,


    /// 
    /// The formatting configuration for numeric DateTime fields.
    /// 
    /// Required: No
    ///
    /// Type: NumericFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,


    /// 
    /// The options that determine the null value format configuration.
    /// 
    /// Required: No
    ///
    /// Type: NullValueFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}


/// The data path options for the pivot table field options.
#[derive(Default, serde::Serialize)]
pub struct PivotTableDataPathOption {


    /// 
    /// The width of the data path option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    pub width: Option<String>,


    /// 
    /// The list of data path values for the data path options.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataPathValue
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPathList")]
    pub data_path_list: Vec<DataPathValue>,

}


/// Determines the color scale that is applied to the visual.
#[derive(Default, serde::Serialize)]
pub struct ColorScale {


    /// 
    /// Determines the color fill type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISCRETE | GRADIENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorFillType")]
    pub color_fill_type: String,


    /// 
    /// Determines the color that is applied to null values.
    /// 
    /// Required: No
    ///
    /// Type: DataColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullValueColor")]
    pub null_value_color: Option<DataColor>,


    /// 
    /// Determines the list of colors that are applied to the visual.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataColor
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Vec<DataColor>,

}


/// The reference line visual display options.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLine {


    /// 
    /// The style configuration of the reference line.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineStyleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StyleConfiguration")]
    pub style_configuration: Option<ReferenceLineStyleConfiguration>,


    /// 
    /// The data configuration of the reference line.
    /// 
    /// Required: Yes
    ///
    /// Type: ReferenceLineDataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataConfiguration")]
    pub data_configuration: ReferenceLineDataConfiguration,


    /// 
    /// The label configuration of the reference line.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineLabelConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelConfiguration")]
    pub label_configuration: Option<ReferenceLineLabelConfiguration>,


    /// 
    /// The status of the reference line. Choose one of the following options:
    /// 
    /// ENABLE                                DISABLE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}


/// The series settings of a radar chart.
#[derive(Default, serde::Serialize)]
pub struct RadarChartSeriesSettings {


    /// 
    /// The area style settings of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: RadarChartAreaStyleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AreaStyleSettings")]
    pub area_style_settings: Option<RadarChartAreaStyleSettings>,

}


/// The total aggregation computation configuration.
#[derive(Default, serde::Serialize)]
pub struct TotalAggregationComputation {


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: MeasureField,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}


/// The configuration of a header or footer section.
#[derive(Default, serde::Serialize)]
pub struct HeaderFooterSectionConfiguration {


    /// 
    /// The unique identifier of the header or footer section.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SectionId")]
    pub section_id: String,


    /// 
    /// The layout configuration of the header or footer section.
    /// 
    /// Required: Yes
    ///
    /// Type: SectionLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Layout")]
    pub layout: SectionLayoutConfiguration,


    /// 
    /// The style options of a header or footer section.
    /// 
    /// Required: No
    ///
    /// Type: SectionStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,

}


/// The minimum label of a data path label.
#[derive(Default, serde::Serialize)]
pub struct MinimumLabelType {


    /// 
    /// The visibility of the minimum label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The sort configuration for a column that is not used in a field well.
#[derive(Default, serde::Serialize)]
pub struct ColumnSort {


    /// 
    /// The aggregation function that is defined in the column sort.
    /// 
    /// Required: No
    ///
    /// Type: AggregationFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortBy")]
    pub sort_by: ColumnIdentifier,


    /// 
    /// The sort direction.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ASC | DESC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: String,

}


/// The configuration for a custom label on a ReferenceLine.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineCustomLabelConfiguration {


    /// 
    /// The string text of the custom label.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: String,

}


/// The field well configuration of a waterfall visual.
#[derive(Default, serde::Serialize)]
pub struct WaterfallChartFieldWells {


    /// 
    /// The field well configuration of a waterfall visual.
    /// 
    /// Required: No
    ///
    /// Type: WaterfallChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaterfallChartAggregatedFieldWells")]
    pub waterfall_chart_aggregated_field_wells: Option<WaterfallChartAggregatedFieldWells>,

}


/// The range options for the data zoom scroll bar.
#[derive(Default, serde::Serialize)]
pub struct VisibleRangeOptions {


    /// 
    /// The percent range in the visible range.
    /// 
    /// Required: No
    ///
    /// Type: PercentVisibleRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentRange")]
    pub percent_range: Option<PercentVisibleRange>,

}


/// The custom narrative options.
#[derive(Default, serde::Serialize)]
pub struct CustomNarrativeOptions {


    /// 
    /// The string input of custom narrative.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 150000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Narrative")]
    pub narrative: String,

}


/// sort-configuration-description
#[derive(Default, serde::Serialize)]
pub struct BarChartSortConfiguration {


    /// 
    /// The limit on the number of categories displayed in a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of color fields in a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The sort configuration of category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of small multiples panels that are displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of the small multiples field.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of values displayed in a bar chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,

}


/// The configuration of the same-sheet target visuals that you want to be filtered.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct SameSheetTargetVisualConfiguration {


    /// 
    /// A list of the target visual IDs that are located in the same sheet of the analysis.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetVisuals")]
    pub target_visuals: Option<Vec<String>>,


    /// 
    /// The options that choose the target visual in the same sheet.
    /// 
    /// Valid values are defined as follows:
    /// 
    /// ALL_VISUALS: Applies the filter operation to all visuals in the same sheet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VISUALS
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetVisualOptions")]
    pub target_visual_options: Option<String>,

}


/// Dashboard source template.
#[derive(Default, serde::Serialize)]
pub struct DashboardSourceTemplate {


    /// 
    /// Dataset references.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataSetReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,


    /// 
    /// The Amazon Resource Name (ARN) of the resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,

}


/// A gauge chart.
///
/// For more information, see Using gauge charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartVisual {


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The conditional formatting of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartConditionalFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<GaugeChartConditionalFormatting>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The configuration of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: GaugeChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GaugeChartConfiguration>,

}


/// The scope configuration for a FilterGroup.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct FilterScopeConfiguration {


    /// 
    /// The configuration for applying a filter to specific sheets.
    /// 
    /// Required: No
    ///
    /// Type: SelectedSheetsFilterScopeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedSheets")]
    pub selected_sheets: Option<SelectedSheetsFilterScopeConfiguration>,

}


/// The liner axis scale setup.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct AxisLinearScale {


    /// 
    /// The step size setup of a linear axis.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepSize")]
    pub step_size: Option<f64>,


    /// 
    /// The step count setup of a linear axis.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepCount")]
    pub step_count: Option<f64>,

}


/// The formatting configuration for all types of field.
#[derive(Default, serde::Serialize)]
pub struct FormatConfiguration {


    /// 
    /// Formatting configuration for number fields.
    /// 
    /// Required: No
    ///
    /// Type: NumberFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberFormatConfiguration")]
    pub number_format_configuration: Option<NumberFormatConfiguration>,


    /// 
    /// Formatting configuration for string fields.
    /// 
    /// Required: No
    ///
    /// Type: StringFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringFormatConfiguration")]
    pub string_format_configuration: Option<StringFormatConfiguration>,


    /// 
    /// Formatting configuration for DateTime fields.
    /// 
    /// Required: No
    ///
    /// Type: DateTimeFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormatConfiguration")]
    pub date_time_format_configuration: Option<DateTimeFormatConfiguration>,

}


/// The subtitle label options for a visual.
#[derive(Default, serde::Serialize)]
pub struct VisualSubtitleLabelOptions {


    /// 
    /// The long text format of the subtitle label, such as plain text or rich text.
    /// 
    /// Required: No
    ///
    /// Type: LongFormatText
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatText")]
    pub format_text: Option<LongFormatText>,


    /// 
    /// The visibility of the subtitle label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The label options of the label that is displayed in the center of a donut chart. This option isn't available for pie charts.
#[derive(Default, serde::Serialize)]
pub struct DonutCenterOptions {


    /// 
    /// Determines the visibility of the label in a donut chart. In the Amazon QuickSight console, this option is called 'Show total'.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelVisibility")]
    pub label_visibility: Option<String>,

}


/// A line chart.
///
/// For more information, see Using line charts in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct LineChartVisual {


    /// 
    /// The configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<LineChartConfiguration>,


    /// 
    /// The column hierarchy that is used during drill-downs and drill-ups.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnHierarchy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// The top movers and bottom movers computation setup.
#[derive(Default, serde::Serialize)]
pub struct TopBottomMoversComputation {


    /// 
    /// The category field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: DimensionField,


    /// 
    /// The mover size setup of the top and bottom movers computation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "MoverSize")]
    pub mover_size: Option<f64>,


    /// 
    /// The value field that is used in a computation.
    /// 
    /// Required: No
    ///
    /// Type: MeasureField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,


    /// 
    /// The name of a computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The ID for a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputationId")]
    pub computation_id: String,


    /// 
    /// The time field that is used in a computation.
    /// 
    /// Required: Yes
    ///
    /// Type: DimensionField
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: DimensionField,


    /// 
    /// The computation type. Choose from the following options:
    /// 
    /// TOP: Top movers computation.               BOTTOM: Bottom movers computation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BOTTOM | TOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The sort order setup of the top and bottom movers computation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ABSOLUTE_DIFFERENCE | PERCENT_DIFFERENCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<String>,

}


/// An empty visual.
///
/// Empty visuals are used in layouts but have not been configured to show any data. A new visual created in the Amazon QuickSight console is considered an EmptyVisual until a visual type is selected.
#[derive(Default, serde::Serialize)]
pub struct EmptyVisual {


    /// 
    /// The data set that is used in the empty visual. Every visual requires a dataset to render.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}


/// The options for data bars.
#[derive(Default, serde::Serialize)]
pub struct DataBarsOptions {


    /// 
    /// The color of the negative data bar.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeColor")]
    pub negative_color: Option<String>,


    /// 
    /// The field ID for the data bars options.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The color of the positive data bar.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PositiveColor")]
    pub positive_color: Option<String>,

}


/// The aggregated field well for the pivot table.
#[derive(Default, serde::Serialize)]
pub struct PivotTableAggregatedFieldWells {


    /// 
    /// The values field well for a pivot table. Values are aggregated based on rows and columns fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 40
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The columns field well for a pivot table. Values are grouped by columns fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 40
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,


    /// 
    /// The rows field well for a pivot table. Values are grouped by rows fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 40
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,

}


/// The options that determine the default settings for a section-based layout configuration.
#[derive(Default, serde::Serialize)]
pub struct DefaultSectionBasedLayoutConfiguration {


    /// 
    /// Determines the screen canvas size options for a section-based layout.
    /// 
    /// Required: Yes
    ///
    /// Type: SectionBasedLayoutCanvasSizeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}


/// The option that determines the decimal places configuration.
#[derive(Default, serde::Serialize)]
pub struct DecimalPlacesConfiguration {


    /// 
    /// The values of the decimal places.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalPlaces")]
    pub decimal_places: f64,

}


/// The configuration for a TableVisual.
#[derive(Default, serde::Serialize)]
pub struct TableConfiguration {


    /// 
    /// A collection of inline visualizations to display within a chart.
    /// 
    /// Required: No
    ///
    /// Type: List of TableInlineVisualization
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableInlineVisualizations")]
    pub table_inline_visualizations: Option<Vec<TableInlineVisualization>>,


    /// 
    /// The sort configuration for a TableVisual.
    /// 
    /// Required: No
    ///
    /// Type: TableSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TableSortConfiguration>,


    /// 
    /// The table options for a table visual.
    /// 
    /// Required: No
    ///
    /// Type: TableOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableOptions")]
    pub table_options: Option<TableOptions>,


    /// 
    /// The paginated report options for a table visual.
    /// 
    /// Required: No
    ///
    /// Type: TablePaginatedReportOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<TablePaginatedReportOptions>,


    /// 
    /// The total options for a table visual.
    /// 
    /// Required: No
    ///
    /// Type: TotalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<TotalOptions>,


    /// 
    /// The field options for a table visual.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<TableFieldOptions>,


    /// 
    /// The field wells of the visual.
    /// 
    /// Required: No
    ///
    /// Type: TableFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TableFieldWells>,

}


/// The aggregated field well configuration of a RadarChartVisual.
#[derive(Default, serde::Serialize)]
pub struct RadarChartAggregatedFieldWells {


    /// 
    /// The aggregated field well categories of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,


    /// 
    /// The values that are assigned to the aggregated field wells of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,


    /// 
    /// The color that are assigned to the aggregated field wells of a radar chart.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<Vec<DimensionField>>,

}


/// The ColorsConfiguration property type specifies Property description not available. for an AWS::QuickSight::Dashboard.
#[derive(Default, serde::Serialize)]
pub struct ColorsConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of CustomColor
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomColors")]
    pub custom_colors: Option<Vec<CustomColor>>,

}


/// The aggregated field well of the filled map.
#[derive(Default, serde::Serialize)]
pub struct FilledMapAggregatedFieldWells {


    /// 
    /// The aggregated location field well of the filled map. Values are grouped by location fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,


    /// 
    /// The aggregated color field well of a filled map. Values are aggregated based on location fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The cell conditional formatting option for a table.
#[derive(Default, serde::Serialize)]
pub struct TableCellConditionalFormatting {


    /// 
    /// The text format of the cell for conditional formatting.
    /// 
    /// Required: No
    ///
    /// Type: TextConditionalFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,


    /// 
    /// The field ID of the cell for conditional formatting.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,

}


/// The aggregated field well for the table.
#[derive(Default, serde::Serialize)]
pub struct TableAggregatedFieldWells {


    /// 
    /// The group by field well for a pivot table. Values are grouped by group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,


    /// 
    /// The values field well for a pivot table. Values are aggregated based on group by fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The aggregated field wells for a geospatial map.
#[derive(Default, serde::Serialize)]
pub struct GeospatialMapAggregatedFieldWells {


    /// 
    /// The color field wells of a geospatial map.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,


    /// 
    /// The geospatial field wells of a geospatial map. Values are grouped by geospatial fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DimensionField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,


    /// 
    /// The size field wells of a geospatial map. Values are aggregated based on geospatial fields.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The options that determine the null value format configuration.
#[derive(Default, serde::Serialize)]
pub struct NullValueFormatConfiguration {


    /// 
    /// Determines the null string of null values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullString")]
    pub null_string: String,

}


/// The configuration that determines the elements and canvas size options of sheet control.
#[derive(Default, serde::Serialize)]
pub struct SheetControlLayoutConfiguration {


    /// 
    /// The configuration that determines the elements and canvas size options of sheet control.
    /// 
    /// Required: No
    ///
    /// Type: GridLayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}


/// The total options for a table visual.
#[derive(Default, serde::Serialize)]
pub struct TotalOptions {


    /// 
    /// Cell styling options for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,


    /// 
    /// The custom label string for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The scroll status (pinned, scrolled) for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PINNED | SCROLLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<String>,


    /// 
    /// The visibility configuration for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<String>,


    /// 
    /// The placement (start, end) for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: END | START
    ///
    /// Update requires: No interruption
    #[serde(rename = "Placement")]
    pub placement: Option<String>,

}


/// A TopBottomFilter filters values that are at the top or the bottom.
#[derive(Default, serde::Serialize)]
pub struct TopBottomFilter {


    /// 
    /// The parameter whose value should be used for the filter value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,


    /// 
    /// An identifier that uniquely identifies a filter within a dashboard, analysis, or template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterId")]
    pub filter_id: String,


    /// 
    /// The aggregation and sort configuration of the top bottom filter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of AggregationSortConfiguration
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationSortConfigurations")]
    pub aggregation_sort_configurations: Vec<AggregationSortConfiguration>,


    /// 
    /// The number of items to include in the top bottom filter results.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limit")]
    pub limit: Option<f64>,


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<String>,

}


/// A control to display a list of buttons or boxes. This is used to select either a single value or multiple values.
#[derive(Default, serde::Serialize)]
pub struct FilterListControl {


    /// 
    /// The type of FilterListControl. Choose one of the following options:
    /// 
    /// MULTI_SELECT: The user can select multiple entries from the list.                        SINGLE_SELECT: The user can select a single entry from the list.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_SELECT | SINGLE_SELECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The title of the FilterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,


    /// 
    /// The ID of the FilterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,


    /// 
    /// The display options of a control.
    /// 
    /// Required: No
    ///
    /// Type: ListControlDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,


    /// 
    /// The values that are displayed in a control can be configured to only show values that are valid based on what's selected in other controls.
    /// 
    /// Required: No
    ///
    /// Type: CascadingControlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,


    /// 
    /// A list of selectable values that are used in a control.
    /// 
    /// Required: No
    ///
    /// Type: FilterSelectableValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,


    /// 
    /// The source filter ID of the FilterListControl.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}


/// The unaggregated field for a table.
#[derive(Default, serde::Serialize)]
pub struct UnaggregatedField {


    /// 
    /// The column that is used in the UnaggregatedField.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The custom field ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The format configuration of the field.
    /// 
    /// Required: No
    ///
    /// Type: FormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,

}


/// The data field series item configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct DataFieldSeriesItem {


    /// 
    /// The options that determine the presentation of line series associated to the field.
    /// 
    /// Required: No
    ///
    /// Type: LineChartSeriesSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,


    /// 
    /// The field ID of the field that you are setting the axis binding to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldId")]
    pub field_id: String,


    /// 
    /// The field value of the field that you are setting the axis binding to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,


    /// 
    /// The axis that you are binding the field to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PRIMARY_YAXIS | SECONDARY_YAXIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisBinding")]
    pub axis_binding: String,

}


/// The inline visualization of a specific type to display within a chart.
#[derive(Default, serde::Serialize)]
pub struct TableInlineVisualization {


    /// 
    /// The configuration of the inline visualization of the data bars within a chart.
    /// 
    /// Required: No
    ///
    /// Type: DataBarsOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataBars")]
    pub data_bars: Option<DataBarsOptions>,

}


/// The values that are displayed in a control can be configured to only show values that are valid based on what's selected in other controls.
#[derive(Default, serde::Serialize)]
pub struct CascadingControlConfiguration {


    /// 
    /// A list of source controls that determine the values that are used in the current control.
    /// 
    /// Required: No
    ///
    /// Type: List of CascadingControlSource
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceControls")]
    pub source_controls: Option<Vec<CascadingControlSource>>,

}


/// The configuration of a FunnelChartVisual.
#[derive(Default, serde::Serialize)]
pub struct FunnelChartConfiguration {


    /// 
    /// The visual palette configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The options that determine the presentation of the data labels.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartDataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabelOptions")]
    pub data_label_options: Option<FunnelChartDataLabelOptions>,


    /// 
    /// The tooltip configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The field well configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FunnelChartFieldWells>,


    /// 
    /// The sort configuration of a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: FunnelChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FunnelChartSortConfiguration>,


    /// 
    /// The label options for the values that are displayed in a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The label options of the categories that are displayed in a FunnelChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

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


/// The customized parameter values.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct CustomParameterValues {


    /// 
    /// A list of string-type parameter values.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValues")]
    pub string_values: Option<Vec<String>>,


    /// 
    /// A list of integer-type parameter values.
    /// 
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerValues")]
    pub integer_values: Option<Vec<f64>>,


    /// 
    /// A list of datetime-type parameter values.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeValues")]
    pub date_time_values: Option<Vec<String>>,


    /// 
    /// A list of decimal-type parameter values.
    /// 
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalValues")]
    pub decimal_values: Option<Vec<f64>>,

}


/// The label options for an axis on a chart.
#[derive(Default, serde::Serialize)]
pub struct ChartAxisLabelOptions {


    /// 
    /// The visibility configuration of the sort icon on a chart's axis label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortIconVisibility")]
    pub sort_icon_visibility: Option<String>,


    /// 
    /// The label options for a chart axis.
    /// 
    /// Required: No
    ///
    /// Type: List of AxisLabelOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisLabelOptions")]
    pub axis_label_options: Option<Vec<AxisLabelOptions>>,


    /// 
    /// The visibility of an axis label on a chart. Choose one of the following options:
    /// 
    /// VISIBLE: Shows the axis.                        HIDDEN: Hides the axis.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The computation union that is used in an insight visual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct Computation {


    /// 
    /// The growth rate computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: GrowthRateComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrowthRate")]
    pub growth_rate: Option<GrowthRateComputation>,


    /// 
    /// The metric comparison computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: MetricComparisonComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricComparison")]
    pub metric_comparison: Option<MetricComparisonComputation>,


    /// 
    /// The unique values computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: UniqueValuesComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "UniqueValues")]
    pub unique_values: Option<UniqueValuesComputation>,


    /// 
    /// The forecast computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: ForecastComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Forecast")]
    pub forecast: Option<ForecastComputation>,


    /// 
    /// The top movers and bottom movers computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: TopBottomMoversComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopBottomMovers")]
    pub top_bottom_movers: Option<TopBottomMoversComputation>,


    /// 
    /// The total aggregation computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: TotalAggregationComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalAggregation")]
    pub total_aggregation: Option<TotalAggregationComputation>,


    /// 
    /// The period over period computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: PeriodOverPeriodComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodOverPeriod")]
    pub period_over_period: Option<PeriodOverPeriodComputation>,


    /// 
    /// The period to DataSetIdentifier computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: PeriodToDateComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodToDate")]
    pub period_to_date: Option<PeriodToDateComputation>,


    /// 
    /// The top ranked and bottom ranked computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: TopBottomRankedComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopBottomRanked")]
    pub top_bottom_ranked: Option<TopBottomRankedComputation>,


    /// 
    /// The maximum and minimum computation configuration.
    /// 
    /// Required: No
    ///
    /// Type: MaximumMinimumComputation
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumMinimum")]
    pub maximum_minimum: Option<MaximumMinimumComputation>,

}


/// The title label options for a visual.
#[derive(Default, serde::Serialize)]
pub struct VisualTitleLabelOptions {


    /// 
    /// The visibility of the title label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,


    /// 
    /// The short text format of the title label, such as plain text or rich text.
    /// 
    /// Required: No
    ///
    /// Type: ShortFormatText
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatText")]
    pub format_text: Option<ShortFormatText>,

}


/// The time range drill down filter.
#[derive(Default, serde::Serialize)]
pub struct TimeRangeDrillDownFilter {


    /// 
    /// The column that the filter is applied to.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,


    /// 
    /// The level of time precision that is used to aggregate DateTime values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MILLISECOND | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: String,


    /// 
    /// The maximum value for the filter value range.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: String,


    /// 
    /// The minimum value for the filter value range.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: String,

}


/// The field well configuration of a GaugeChartVisual.
#[derive(Default, serde::Serialize)]
pub struct GaugeChartFieldWells {


    /// 
    /// The target value field wells of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,


    /// 
    /// The value field wells of a GaugeChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: List of MeasureField
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}


/// The options that determine the bin count of a histogram.
#[derive(Default, serde::Serialize)]
pub struct BinCountOptions {


    /// 
    /// The options that determine the bin count value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}


/// A Layout defines the placement of elements within a sheet.
///
/// For more information, see Types of layout in the Amazon QuickSight User Guide.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct Layout {


    /// 
    /// The configuration that determines what the type of layout for a sheet.
    /// 
    /// Required: Yes
    ///
    /// Type: LayoutConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: LayoutConfiguration,

}


/// The configuration of the search options in a list control.
#[derive(Default, serde::Serialize)]
pub struct ListControlSearchOptions {


    /// 
    /// The visibility configuration of the search options in a list control.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The field wells of the visual.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct ComboChartFieldWells {


    /// 
    /// The aggregated field wells of a combo chart. Combo charts only have aggregated field wells. Columns in a combo chart are aggregated by category.
    /// 
    /// Required: No
    ///
    /// Type: ComboChartAggregatedFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComboChartAggregatedFieldWells")]
    pub combo_chart_aggregated_field_wells: Option<ComboChartAggregatedFieldWells>,

}


/// Determines the custom condition for an icon set.
#[derive(Default, serde::Serialize)]
pub struct ConditionalFormattingCustomIconCondition {


    /// 
    /// Determines the icon display configuration.
    /// 
    /// Required: No
    ///
    /// Type: ConditionalFormattingIconDisplayConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayConfiguration")]
    pub display_configuration: Option<ConditionalFormattingIconDisplayConfiguration>,


    /// 
    /// Custom icon options for an icon set.
    /// 
    /// Required: Yes
    ///
    /// Type: ConditionalFormattingCustomIconOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "IconOptions")]
    pub icon_options: ConditionalFormattingCustomIconOptions,


    /// 
    /// The expression that determines the condition of the icon set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,


    /// 
    /// Determines the color of the icon.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Color")]
    pub color: Option<String>,

}


/// The configuration of a line chart.
#[derive(Default, serde::Serialize)]
pub struct LineChartConfiguration {


    /// 
    /// The series item configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: List of SeriesItem
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Series")]
    pub series: Option<Vec<SeriesItem>>,


    /// 
    /// The legend configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LegendOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,


    /// 
    /// The visual palette configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: VisualPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,


    /// 
    /// The series axis configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineSeriesAxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,


    /// 
    /// The small multiples setup for the visual.
    /// 
    /// Required: No
    ///
    /// Type: SmallMultiplesOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,


    /// 
    /// The field well configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineChartFieldWells
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<LineChartFieldWells>,


    /// 
    /// The options that determine the presentation of the secondary y-axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The default configuration of a line chart's contribution analysis.
    /// 
    /// Required: No
    ///
    /// Type: List of ContributionAnalysisDefault
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,


    /// 
    /// The sort configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineChartSortConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<LineChartSortConfiguration>,


    /// 
    /// The reference lines configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: List of ReferenceLine
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,


    /// 
    /// The options that determine the default presentation of all line series in LineChartVisual.
    /// 
    /// Required: No
    ///
    /// Type: LineChartDefaultSeriesSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultSeriesSettings")]
    pub default_series_settings: Option<LineChartDefaultSeriesSettings>,


    /// 
    /// The options that determine the presentation of the x-axis.
    /// 
    /// Required: No
    ///
    /// Type: AxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,


    /// 
    /// The tooltip configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: TooltipOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,


    /// 
    /// The series axis configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: LineSeriesAxisDisplayOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,


    /// 
    /// Determines the type of the line chart.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AREA | LINE | STACKED_AREA
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The forecast configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: List of ForecastConfiguration
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForecastConfigurations")]
    pub forecast_configurations: Option<Vec<ForecastConfiguration>>,


    /// 
    /// The data label configuration of a line chart.
    /// 
    /// Required: No
    ///
    /// Type: DataLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,


    /// 
    /// The options that determine the presentation of the y-axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,


    /// 
    /// The options that determine the presentation of the x-axis label.
    /// 
    /// Required: No
    ///
    /// Type: ChartAxisLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,

}


/// Line styles options for a line series in LineChartVisual.
#[derive(Default, serde::Serialize)]
pub struct LineChartLineStyleSettings {


    /// 
    /// Interpolation style for line series.
    /// 
    /// LINEAR: Show as default, linear style.                        SMOOTH: Show as a smooth curve.                        STEPPED: Show steps in line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LINEAR | SMOOTH | STEPPED
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineInterpolation")]
    pub line_interpolation: Option<String>,


    /// 
    /// Line style for line series.
    /// 
    /// SOLID: Show as a solid line.                        DOTTED: Show as a dotted line.                        DASHED: Show as a dashed line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DASHED | DOTTED | SOLID
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineStyle")]
    pub line_style: Option<String>,


    /// 
    /// Width that determines the line thickness.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineWidth")]
    pub line_width: Option<String>,


    /// 
    /// Configuration option that determines whether to show the line for the series.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineVisibility")]
    pub line_visibility: Option<String>,

}


/// The option that determines the data label type.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct DataLabelType {


    /// 
    /// The option that specifies individual data values for labels.
    /// 
    /// Required: No
    ///
    /// Type: DataPathLabelType
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPathLabelType")]
    pub data_path_label_type: Option<DataPathLabelType>,


    /// 
    /// Determines the label configuration for the entire field.
    /// 
    /// Required: No
    ///
    /// Type: FieldLabelType
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLabelType")]
    pub field_label_type: Option<FieldLabelType>,


    /// 
    /// Determines the label configuration for the maximum value in a visual.
    /// 
    /// Required: No
    ///
    /// Type: MaximumLabelType
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumLabelType")]
    pub maximum_label_type: Option<MaximumLabelType>,


    /// 
    /// Determines the label configuration for the minimum value in a visual.
    /// 
    /// Required: No
    ///
    /// Type: MinimumLabelType
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumLabelType")]
    pub minimum_label_type: Option<MinimumLabelType>,


    /// 
    /// Determines the label configuration for range end value in a visual.
    /// 
    /// Required: No
    ///
    /// Type: RangeEndsLabelType
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeEndsLabelType")]
    pub range_ends_label_type: Option<RangeEndsLabelType>,

}


/// The data configuration of the reference line.
#[derive(Default, serde::Serialize)]
pub struct ReferenceLineDataConfiguration {


    /// 
    /// The axis binding type of the reference line. Choose one of the following options:
    /// 
    /// PrimaryY               SecondaryY
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PRIMARY_YAXIS | SECONDARY_YAXIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<String>,


    /// 
    /// The dynamic configuration of the reference line data configuration.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineDynamicDataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicConfiguration")]
    pub dynamic_configuration: Option<ReferenceLineDynamicDataConfiguration>,


    /// 
    /// The static data configuration of the reference line data       configuration.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceLineStaticDataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticConfiguration")]
    pub static_configuration: Option<ReferenceLineStaticDataConfiguration>,

}


/// The optional configuration of totals cells in a PivotTableVisual.
#[derive(Default, serde::Serialize)]
pub struct PivotTotalOptions {


    /// 
    /// The visibility configuration for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<String>,


    /// 
    /// The custom label string for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The cell styling options for the total of header cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,


    /// 
    /// The placement (start, end) for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: END | START
    ///
    /// Update requires: No interruption
    #[serde(rename = "Placement")]
    pub placement: Option<String>,


    /// 
    /// The cell styling options for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,


    /// 
    /// The scroll status (pinned, scrolled) for the total cells.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PINNED | SCROLLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<String>,


    /// 
    /// The cell styling options for the totals of value cells.
    /// 
    /// Required: No
    ///
    /// Type: TableCellStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,

}


/// The option that determines the hierarchy of any DateTime fields.
#[derive(Default, serde::Serialize)]
pub struct DateTimeHierarchy {


    /// 
    /// The option that determines the drill down filters for the         DateTime hierarchy.
    /// 
    /// Required: No
    ///
    /// Type: List of DrillDownFilter
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,


    /// 
    /// The hierarchy ID of the DateTime hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,

}


/// The configuration of a page break for a section.
#[derive(Default, serde::Serialize)]
pub struct SectionPageBreakConfiguration {


    /// 
    /// The configuration of a page break after a section.
    /// 
    /// Required: No
    ///
    /// Type: SectionAfterPageBreak
    ///
    /// Update requires: No interruption
    #[serde(rename = "After")]
    pub after: Option<SectionAfterPageBreak>,

}


/// The word cloud options for a word cloud visual.
#[derive(Default, serde::Serialize)]
pub struct WordCloudOptions {


    /// 
    /// The length limit of each word from 1-100.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumStringLength")]
    pub maximum_string_length: Option<f64>,


    /// 
    /// The word padding options (none, small, medium, large) for the words in a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LARGE | MEDIUM | NONE | SMALL
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordPadding")]
    pub word_padding: Option<String>,


    /// 
    /// The word orientation options (horizontal, horizontal_and_vertical) for the words in a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HORIZONTAL | HORIZONTAL_AND_VERTICAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordOrientation")]
    pub word_orientation: Option<String>,


    /// 
    /// The cloud layout options (fluid, normal) of a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FLUID | NORMAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudLayout")]
    pub cloud_layout: Option<String>,


    /// 
    /// The word scaling options (emphasize, normal) for the words in a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EMPHASIZE | NORMAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordScaling")]
    pub word_scaling: Option<String>,


    /// 
    /// The word casing options (lower_case, existing_case) for the words in a word cloud.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EXISTING_CASE | LOWER_CASE
    ///
    /// Update requires: No interruption
    #[serde(rename = "WordCasing")]
    pub word_casing: Option<String>,

}


/// The share label options for the labels.
#[derive(Default, serde::Serialize)]
pub struct LabelOptions {


    /// 
    /// The text for the label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,


    /// 
    /// The font configuration of the label.
    /// 
    /// Required: No
    ///
    /// Type: FontConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,


    /// 
    /// Determines whether or not the label is visible.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIDDEN | VISIBLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// The configuration of destination parameter values.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct DestinationParameterValueConfiguration {


    /// 
    /// The source parameter name of the destination parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: Option<String>,


    /// 
    /// The configuration that selects all options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_VALUES
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAllValueOptions")]
    pub select_all_value_options: Option<String>,


    /// 
    /// The configuration of custom values for destination parameter in DestinationParameterValueConfiguration.
    /// 
    /// Required: No
    ///
    /// Type: CustomValuesConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValuesConfiguration")]
    pub custom_values_configuration: Option<CustomValuesConfiguration>,


    /// 
    /// The source field ID of the destination parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceField")]
    pub source_field: Option<String>,

}


/// A sankey diagram.
///
/// For more information, see Using Sankey diagrams in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct SankeyDiagramVisual {


    /// 
    /// The configuration of a sankey diagram.
    /// 
    /// Required: No
    ///
    /// Type: SankeyDiagramChartConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<SankeyDiagramChartConfiguration>,


    /// 
    /// The title that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualTitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,


    /// 
    /// The list of custom actions that are configured for a visual.
    /// 
    /// Required: No
    ///
    /// Type: List of VisualCustomAction
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,


    /// 
    /// The subtitle that is displayed on the visual.
    /// 
    /// Required: No
    ///
    /// Type: VisualSubtitleLabelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,


    /// 
    /// The unique identifier of a visual. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have visuals with the same identifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\w\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}


/// A parameter declaration for the Integer data type.
///
/// This is a union type structure. For this structure to be valid, only one of the attributes can be defined.
#[derive(Default, serde::Serialize)]
pub struct IntegerValueWhenUnsetConfiguration {


    /// 
    /// A custom value that's used when the value of a parameter isn't set.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,


    /// 
    /// The built-in options for default values. The value can be one of the following:
    /// 
    /// RECOMMENDED: The recommended value.                        NULL: The NULL value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NULL | RECOMMENDED_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<String>,

}


/// The options that determine the default presentation of all line series in LineChartVisual.
#[derive(Default, serde::Serialize)]
pub struct LineChartDefaultSeriesSettings {


    /// 
    /// The axis to which you are binding all line series to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PRIMARY_YAXIS | SECONDARY_YAXIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<String>,


    /// 
    /// Marker styles options for all line series in the visual.
    /// 
    /// Required: No
    ///
    /// Type: LineChartMarkerStyleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,


    /// 
    /// Line styles options for all line series in the visual.
    /// 
    /// Required: No
    ///
    /// Type: LineChartLineStyleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,

}


/// The sort configuration of a pie chart.
#[derive(Default, serde::Serialize)]
pub struct PieChartSortConfiguration {


    /// 
    /// The sort configuration of the category fields.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of categories that are displayed in a pie chart.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,


    /// 
    /// The sort configuration of the small multiples field.
    /// 
    /// Required: No
    ///
    /// Type: List of FieldSortOptions
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,


    /// 
    /// The limit on the number of small multiples panels that are displayed.
    /// 
    /// Required: No
    ///
    /// Type: ItemsLimitConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}


/// The set parameter operation that sets parameters in custom action.
#[derive(Default, serde::Serialize)]
pub struct CustomActionSetParametersOperation {


    /// 
    /// The parameter that determines the value configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of SetParameterValueConfiguration
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValueConfigurations")]
    pub parameter_value_configurations: Vec<SetParameterValueConfiguration>,

}