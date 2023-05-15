
pub mod cfn_analysis {

#[derive(serde::Serialize, Default)]
pub struct CfnAnalysis {
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<ResourceStatus>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AnalysisId")]
    pub analysis_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,
    /// No documentation provided by AWS
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,
    /// List of ResourcePermission
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceEntity")]
    pub source_entity: Option<AnalysisSourceEntity>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Option<AnalysisDefinition>,

}


#[derive(serde::Serialize, Default)]
pub struct PieChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterControl {
    #[serde(rename = "Slider")]
    pub slider: Option<ParameterSliderControl>,
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<ParameterDateTimePickerControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<ParameterDropDownControl>,
    #[serde(rename = "List")]
    pub list: Option<ParameterListControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<ParameterTextAreaControl>,
    #[serde(rename = "TextField")]
    pub text_field: Option<ParameterTextFieldControl>,

}
pub type HorizontalTextAlignment = String;
#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramFieldWells {
    #[serde(rename = "SankeyDiagramAggregatedFieldWells")]
    pub sankey_diagram_aggregated_field_wells: Option<SankeyDiagramAggregatedFieldWells>,

}
pub type SelectedTooltipType = String;pub type CategoryFilterMatchOperator = String;
#[derive(serde::Serialize, Default)]
pub struct AxisDisplayMinMaxRange {
    #[serde(rename = "Maximum")]
    pub maximum: Option<f64>,
    #[serde(rename = "Minimum")]
    pub minimum: Option<f64>,

}
pub type ConditionalFormattingIconDisplayOption = String;
#[derive(serde::Serialize, Default)]
pub struct FilledMapConfiguration {
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FilledMapFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FilledMapSortConfiguration>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct RowAlternateColorOptions {
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "RowAlternateColors")]
    pub row_alternate_colors: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<KPISortConfiguration>,
    #[serde(rename = "KPIOptions")]
    pub kpioptions: Option<KPIOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<KPIFieldWells>,

}
pub type FontStyle = String;
#[derive(serde::Serialize, Default)]
pub struct DateTimeFormatConfiguration {
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,

}
pub type TableOrientation = String;pub type PanelBorderStyle = String;
#[derive(serde::Serialize, Default)]
pub struct CustomParameterValues {
    #[serde(rename = "DateTimeValues")]
    pub date_time_values: Option<Vec<String>>,
    #[serde(rename = "DecimalValues")]
    pub decimal_values: Option<Vec<f64>>,
    #[serde(rename = "StringValues")]
    pub string_values: Option<Vec<String>>,
    #[serde(rename = "IntegerValues")]
    pub integer_values: Option<Vec<f64>>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcConfiguration {
    #[serde(rename = "ArcAngle")]
    pub arc_angle: Option<f64>,
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThicknessOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetIdentifierDeclaration {
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    #[serde(rename = "Identifier")]
    pub identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedField {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Name")]
    pub name: String,

}
pub type PivotTableConditionalFormattingScopeRole = String;
#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutConfiguration {
    #[serde(rename = "FooterSections")]
    pub footer_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "BodySections")]
    pub body_sections: Vec<BodySectionConfiguration>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,
    #[serde(rename = "HeaderSections")]
    pub header_sections: Vec<HeaderFooterSectionConfiguration>,

}
pub type ValueWhenUnsetOption = String;
#[derive(serde::Serialize, Default)]
pub struct TableSortConfiguration {
    #[serde(rename = "RowSort")]
    pub row_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AggregationFunction {
    #[serde(rename = "NumericalAggregationFunction")]
    pub numerical_aggregation_function: Option<NumericalAggregationFunction>,
    #[serde(rename = "CategoricalAggregationFunction")]
    pub categorical_aggregation_function: Option<CategoricalAggregationFunction>,
    #[serde(rename = "DateAggregationFunction")]
    pub date_aggregation_function: Option<DateAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<RadarChartFieldWells>,
    #[serde(rename = "AlternateBandColorsVisibility")]
    pub alternate_band_colors_visibility: Option<Visibility>,
    #[serde(rename = "Shape")]
    pub shape: Option<RadarChartShape>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<RadarChartSortConfiguration>,
    #[serde(rename = "AlternateBandEvenColor")]
    pub alternate_band_even_color: Option<String>,
    #[serde(rename = "AlternateBandOddColor")]
    pub alternate_band_odd_color: Option<String>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ColorAxis")]
    pub color_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "StartAngle")]
    pub start_angle: Option<f64>,
    #[serde(rename = "BaseSeriesSettings")]
    pub base_series_settings: Option<RadarChartSeriesSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramChartConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<SankeyDiagramSortConfiguration>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<SankeyDiagramFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct SmallMultiplesOptions {
    #[serde(rename = "MaxVisibleColumns")]
    pub max_visible_columns: Option<f64>,
    #[serde(rename = "PanelConfiguration")]
    pub panel_configuration: Option<PanelConfiguration>,
    #[serde(rename = "MaxVisibleRows")]
    pub max_visible_rows: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapStyleOptions {
    #[serde(rename = "BaseMapStyle")]
    pub base_map_style: Option<BaseMapStyleType>,

}

#[derive(serde::Serialize, Default)]
pub struct PercentageDisplayFormatConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialWindowOptions {
    #[serde(rename = "MapZoomMode")]
    pub map_zoom_mode: Option<MapZoomMode>,
    #[serde(rename = "Bounds")]
    pub bounds: Option<GeospatialCoordinateBounds>,

}
pub type DataLabelContent = String;
#[derive(serde::Serialize, Default)]
pub struct HistogramBinOptions {
    #[serde(rename = "StartValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "BinWidth")]
    pub bin_width: Option<BinWidthOptions>,
    #[serde(rename = "BinCount")]
    pub bin_count: Option<BinCountOptions>,
    #[serde(rename = "SelectedBinType")]
    pub selected_bin_type: Option<HistogramBinType>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConfiguration {
    #[serde(rename = "TooltipOptions")]
    pub tooltip_options: Option<TooltipOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GaugeChartFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "GaugeChartOptions")]
    pub gauge_chart_options: Option<GaugeChartOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLabelReferenceOptions {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct BodySectionConfiguration {
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,
    #[serde(rename = "SectionId")]
    pub section_id: String,
    #[serde(rename = "Content")]
    pub content: BodySectionContent,
    #[serde(rename = "PageBreakConfiguration")]
    pub page_break_configuration: Option<SectionPageBreakConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionAfterPageBreak {
    #[serde(rename = "Status")]
    pub status: Option<SectionPageBreakStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct TrendArrowOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SelectedSheetsFilterScopeConfiguration {
    #[serde(rename = "SheetVisualScopingConfigurations")]
    pub sheet_visual_scoping_configurations: Option<Vec<SheetVisualScopingConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeBasedForecastProperties {
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<f64>,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "BreakdownItemsLimit")]
    pub breakdown_items_limit: Option<ItemsLimitConfiguration>,

}
pub type LineChartType = String;
#[derive(serde::Serialize, Default)]
pub struct Entity {
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellImageSizingConfiguration {
    #[serde(rename = "TableCellImageScalingConfiguration")]
    pub table_cell_image_scaling_configuration: Option<TableCellImageScalingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightConfiguration {
    #[serde(rename = "CustomNarrative")]
    pub custom_narrative: Option<CustomNarrativeOptions>,
    #[serde(rename = "Computations")]
    pub computations: Option<Vec<Computation>>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilterValue {
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotFieldWells {
    #[serde(rename = "ScatterPlotUnaggregatedFieldWells")]
    pub scatter_plot_unaggregated_field_wells: Option<ScatterPlotUnaggregatedFieldWells>,
    #[serde(rename = "ScatterPlotCategoricallyAggregatedFieldWells")]
    pub scatter_plot_categorically_aggregated_field_wells: Option<ScatterPlotCategoricallyAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<IntegerDefaultValues>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<IntegerValueWhenUnsetConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<FreeFormLayoutCanvasSizeOptions>,
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,

}
pub type BaseMapStyleType = String;
#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBorderStyle {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutElement {
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "ColumnIndex")]
    pub column_index: Option<f64>,
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,
    #[serde(rename = "ColumnSpan")]
    pub column_span: f64,
    #[serde(rename = "RowIndex")]
    pub row_index: Option<f64>,
    #[serde(rename = "RowSpan")]
    pub row_span: f64,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathColor {
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Element")]
    pub element: DataPathValue,
    #[serde(rename = "Color")]
    pub color: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableSortConfiguration {
    #[serde(rename = "FieldSortOptions")]
    pub field_sort_options: Option<Vec<PivotFieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartConfiguration {
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ComboChartFieldWells>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "BarDataLabels")]
    pub bar_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<ComboChartSortConfiguration>,
    #[serde(rename = "LineDataLabels")]
    pub line_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetControlLayout {
    #[serde(rename = "Configuration")]
    pub configuration: SheetControlLayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct CustomContentConfiguration {
    #[serde(rename = "ImageScaling")]
    pub image_scaling: Option<CustomContentImageScalingConfiguration>,
    #[serde(rename = "ContentUrl")]
    pub content_url: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<CustomContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisTickLabelOptions {
    #[serde(rename = "LabelOptions")]
    pub label_options: Option<LabelOptions>,
    #[serde(rename = "RotationAngle")]
    pub rotation_angle: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Parameters {
    #[serde(rename = "IntegerParameters")]
    pub integer_parameters: Option<Vec<IntegerParameter>>,
    #[serde(rename = "DateTimeParameters")]
    pub date_time_parameters: Option<Vec<DateTimeParameter>>,
    #[serde(rename = "DecimalParameters")]
    pub decimal_parameters: Option<Vec<DecimalParameter>>,
    #[serde(rename = "StringParameters")]
    pub string_parameters: Option<Vec<StringParameter>>,

}
pub type WordCloudWordOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct GlobalTableBorderOptions {
    #[serde(rename = "UniformBorder")]
    pub uniform_border: Option<TableBorderOptions>,
    #[serde(rename = "SideSpecificBorder")]
    pub side_specific_border: Option<TableSideBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ExplicitHierarchy {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapFieldWells {
    #[serde(rename = "TreeMapAggregatedFieldWells")]
    pub tree_map_aggregated_field_wells: Option<TreeMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotOptions {
    #[serde(rename = "OutlierVisibility")]
    pub outlier_visibility: Option<Visibility>,
    #[serde(rename = "AllDataPointsVisibility")]
    pub all_data_points_visibility: Option<Visibility>,
    #[serde(rename = "StyleOptions")]
    pub style_options: Option<BoxPlotStyleOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartConfiguration {
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "DataLabelOptions")]
    pub data_label_options: Option<FunnelChartDataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FunnelChartFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FunnelChartSortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TextAreaControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PaginationConfiguration {
    #[serde(rename = "PageNumber")]
    pub page_number: f64,
    #[serde(rename = "PageSize")]
    pub page_size: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartFieldWells {
    #[serde(rename = "PieChartAggregatedFieldWells")]
    pub pie_chart_aggregated_field_wells: Option<PieChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilter {
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: Option<NumericRangeFilterValue>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: Option<NumericRangeFilterValue>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,

}
pub type SectionPageBreakStatus = String;
#[derive(serde::Serialize, Default)]
pub struct AxisLogarithmicScale {
    #[serde(rename = "Base")]
    pub base: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FilledMapConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<FilledMapConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SubtotalOptions {
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,
    #[serde(rename = "FieldLevel")]
    pub field_level: Option<PivotTableSubtotalLevel>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "FieldLevelOptions")]
    pub field_level_options: Option<Vec<PivotTableFieldSubtotalOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct CascadingControlConfiguration {
    #[serde(rename = "SourceControls")]
    pub source_controls: Option<Vec<CascadingControlSource>>,

}
pub type ComparisonMethod = String;
#[derive(serde::Serialize, Default)]
pub struct FieldSort {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Direction")]
    pub direction: SortDirection,

}

#[derive(serde::Serialize, Default)]
pub struct CurrencyDisplayFormatConfiguration {
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TablePaginatedReportOptions {
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartConfiguration {
    #[serde(rename = "WaterfallChartOptions")]
    pub waterfall_chart_options: Option<WaterfallChartOptions>,
    #[serde(rename = "CategoryAxisLabelOptions")]
    pub category_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WaterfallChartSortConfiguration>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WaterfallChartFieldWells>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryAxisDisplayOptions")]
    pub category_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WaterfallChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOptions {
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<PivotTableFieldOption>>,
    #[serde(rename = "DataPathOptions")]
    pub data_path_options: Option<Vec<PivotTableDataPathOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterTextAreaControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}
pub type SelectedFieldOptions = String;
#[derive(serde::Serialize, Default)]
pub struct DecimalValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramSortConfiguration {
    #[serde(rename = "DestinationItemsLimit")]
    pub destination_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "WeightSort")]
    pub weight_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SourceItemsLimit")]
    pub source_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultPaginatedLayoutConfiguration {
    #[serde(rename = "SectionBased")]
    pub section_based: Option<DefaultSectionBasedLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PivotTableConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<PivotTableConditionalFormatting>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialCoordinateBounds {
    #[serde(rename = "West")]
    pub west: f64,
    #[serde(rename = "East")]
    pub east: f64,
    #[serde(rename = "South")]
    pub south: f64,
    #[serde(rename = "North")]
    pub north: f64,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayRange {
    #[serde(rename = "MinMax")]
    pub min_max: Option<AxisDisplayMinMaxRange>,
    #[serde(rename = "DataDriven")]
    pub data_driven: Option<AxisDisplayDataDrivenRange>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: String,

}
pub type HistogramBinType = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingScope {
    #[serde(rename = "Role")]
    pub role: Option<PivotTableConditionalFormattingScopeRole>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Color")]
    pub color: Option<Vec<DimensionField>>,

}
pub type SimpleNumericalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct HeatMapFieldWells {
    #[serde(rename = "HeatMapAggregatedFieldWells")]
    pub heat_map_aggregated_field_wells: Option<HeatMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ScatterPlotConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}
pub type FontWeightName = String;
#[derive(serde::Serialize, Default)]
pub struct CustomFilterListConfiguration {
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberDisplayFormatConfiguration {
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PeriodToDateComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "PeriodTimeGranularity")]
    pub period_time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOptions {
    #[serde(rename = "Order")]
    pub order: Option<Vec<String>>,
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<TableFieldOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisDisplayRange {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconSet {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "IconSetType")]
    pub icon_set_type: Option<ConditionalFormattingIconSetType>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionFilterOperation {
    #[serde(rename = "TargetVisualsConfiguration")]
    pub target_visuals_configuration: FilterOperationTargetVisualsConfiguration,
    #[serde(rename = "SelectedFieldsConfiguration")]
    pub selected_fields_configuration: FilterOperationSelectedFieldsConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterMarker {
    #[serde(rename = "SimpleClusterMarker")]
    pub simple_cluster_marker: Option<SimpleClusterMarker>,

}
pub type RadarChartShape = String;
#[derive(serde::Serialize, Default)]
pub struct BarChartAggregatedFieldWells {
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartFieldWells {
    #[serde(rename = "ComboChartAggregatedFieldWells")]
    pub combo_chart_aggregated_field_wells: Option<ComboChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultInteractiveLayoutConfiguration {
    #[serde(rename = "Grid")]
    pub grid: Option<DefaultGridLayoutConfiguration>,
    #[serde(rename = "FreeForm")]
    pub free_form: Option<DefaultFreeFormLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct Spacing {
    #[serde(rename = "Right")]
    pub right: Option<String>,
    #[serde(rename = "Left")]
    pub left: Option<String>,
    #[serde(rename = "Top")]
    pub top: Option<String>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapAggregatedFieldWells {
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct Layout {
    #[serde(rename = "Configuration")]
    pub configuration: LayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisError {
    #[serde(rename = "ViolatedEntities")]
    pub violated_entities: Option<Vec<Entity>>,
    #[serde(rename = "Type")]
    pub ty: Option<AnalysisErrorType>,
    #[serde(rename = "Message")]
    pub message: Option<String>,

}
pub type SelectAllValueOptions = String;
#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilter {
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "RangeMinimumValue")]
    pub range_minimum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMaximumValue")]
    pub range_maximum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct SimpleClusterMarker {
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayDataDrivenRange {

}

#[derive(serde::Serialize, Default)]
pub struct FilterDropDownControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotStyleOptions {
    #[serde(rename = "FillStyle")]
    pub fill_style: Option<BoxPlotFillStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimePickerControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

}
pub type MissingDataTreatmentOption = String;
#[derive(serde::Serialize, Default)]
pub struct HistogramAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricComparisonComputation {
    #[serde(rename = "FromValue")]
    pub from_value: MeasureField,
    #[serde(rename = "TargetValue")]
    pub target_value: MeasureField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,

}

#[derive(serde::Serialize, Default)]
pub struct StringDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,

}
pub type WordCloudWordCasing = String;
#[derive(serde::Serialize, Default)]
pub struct AxisScale {
    #[serde(rename = "Linear")]
    pub linear: Option<AxisLinearScale>,
    #[serde(rename = "Logarithmic")]
    pub logarithmic: Option<AxisLogarithmicScale>,

}

#[derive(serde::Serialize, Default)]
pub struct ColorsConfiguration {
    #[serde(rename = "CustomColors")]
    pub custom_colors: Option<Vec<CustomColor>>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconDisplayConfiguration {
    #[serde(rename = "IconDisplayOption")]
    pub icon_display_option: Option<ConditionalFormattingIconDisplayOption>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormattingOption {
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<KPIPrimaryValueConditionalFormatting>,
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<KPIProgressBarConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIOptions {
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "SecondaryValue")]
    pub secondary_value: Option<SecondaryValueOptions>,
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<ProgressBarOptions>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "SecondaryValueFontConfiguration")]
    pub secondary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "TrendArrows")]
    pub trend_arrows: Option<TrendArrowOptions>,

}
pub type LineInterpolation = String;
#[derive(serde::Serialize, Default)]
pub struct BodySectionContent {
    #[serde(rename = "Layout")]
    pub layout: Option<SectionLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DrillDownFilter {
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeDrillDownFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityDrillDownFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryDrillDownFilter>,

}
pub type ResizeOption = String;
#[derive(serde::Serialize, Default)]
pub struct FilledMapShapeConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Format")]
    pub format: Option<ShapeConditionalFormat>,

}

#[derive(serde::Serialize, Default)]
pub struct SameSheetTargetVisualConfiguration {
    #[serde(rename = "TargetVisuals")]
    pub target_visuals: Option<Vec<String>>,
    #[serde(rename = "TargetVisualOptions")]
    pub target_visual_options: Option<TargetVisualOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutCanvasSizeOptions {
    #[serde(rename = "PaperCanvasSizeOptions")]
    pub paper_canvas_size_options: Option<SectionBasedLayoutPaperCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableCellConditionalFormatting {
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,
    #[serde(rename = "Scope")]
    pub scope: Option<PivotTableConditionalFormattingScope>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct SliderControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldWells {
    #[serde(rename = "TableUnaggregatedFieldWells")]
    pub table_unaggregated_field_wells: Option<TableUnaggregatedFieldWells>,
    #[serde(rename = "TableAggregatedFieldWells")]
    pub table_aggregated_field_wells: Option<TableAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionSetParametersOperation {
    #[serde(rename = "ParameterValueConfigurations")]
    pub parameter_value_configurations: Vec<SetParameterValueConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionPageBreakConfiguration {
    #[serde(rename = "After")]
    pub after: Option<SectionAfterPageBreak>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldSubtotalOptions {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ThousandSeparatorOptions {
    #[serde(rename = "Symbol")]
    pub symbol: Option<NumericSeparatorSymbol>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartAreaStyleSettings {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBackgroundStyle {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<GaugeChartConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct ComparisonConfiguration {
    #[serde(rename = "ComparisonFormat")]
    pub comparison_format: Option<ComparisonFormatConfiguration>,
    #[serde(rename = "ComparisonMethod")]
    pub comparison_method: Option<ComparisonMethod>,

}
pub type PaperSize = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterDeclaration {
    #[serde(rename = "StringParameterDeclaration")]
    pub string_parameter_declaration: Option<StringParameterDeclaration>,
    #[serde(rename = "IntegerParameterDeclaration")]
    pub integer_parameter_declaration: Option<IntegerParameterDeclaration>,
    #[serde(rename = "DateTimeParameterDeclaration")]
    pub date_time_parameter_declaration: Option<DateTimeParameterDeclaration>,
    #[serde(rename = "DecimalParameterDeclaration")]
    pub decimal_parameter_declaration: Option<DecimalParameterDeclaration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableOptions {
    #[serde(rename = "ColumnNamesVisibility")]
    pub column_names_visibility: Option<Visibility>,
    #[serde(rename = "SingleMetricVisibility")]
    pub single_metric_visibility: Option<Visibility>,
    #[serde(rename = "RowFieldNamesStyle")]
    pub row_field_names_style: Option<TableCellStyle>,
    #[serde(rename = "RowHeaderStyle")]
    pub row_header_style: Option<TableCellStyle>,
    #[serde(rename = "ColumnHeaderStyle")]
    pub column_header_style: Option<TableCellStyle>,
    #[serde(rename = "MetricPlacement")]
    pub metric_placement: Option<PivotTableMetricPlacement>,
    #[serde(rename = "ToggleButtonsVisibility")]
    pub toggle_buttons_visibility: Option<Visibility>,
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperationTargetVisualsConfiguration {
    #[serde(rename = "SameSheetTargetVisualConfiguration")]
    pub same_sheet_target_visual_configuration: Option<SameSheetTargetVisualConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct MappedDataSetParameter {
    #[serde(rename = "DataSetParameterName")]
    pub data_set_parameter_name: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudOptions {
    #[serde(rename = "WordCasing")]
    pub word_casing: Option<WordCloudWordCasing>,
    #[serde(rename = "CloudLayout")]
    pub cloud_layout: Option<WordCloudCloudLayout>,
    #[serde(rename = "WordOrientation")]
    pub word_orientation: Option<WordCloudWordOrientation>,
    #[serde(rename = "WordScaling")]
    pub word_scaling: Option<WordCloudWordScaling>,
    #[serde(rename = "WordPadding")]
    pub word_padding: Option<WordCloudWordPadding>,
    #[serde(rename = "MaximumStringLength")]
    pub maximum_string_length: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualCustomActionOperation {
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<CustomActionFilterOperation>,
    #[serde(rename = "NavigationOperation")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,
    #[serde(rename = "URLOperation")]
    pub urloperation: Option<CustomActionURLOperation>,
    #[serde(rename = "SetParametersOperation")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,

}

#[derive(serde::Serialize, Default)]
pub struct LegendOptions {
    #[serde(rename = "Title")]
    pub title: Option<LabelOptions>,
    #[serde(rename = "Position")]
    pub position: Option<LegendPosition>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartFieldWells {
    #[serde(rename = "LineChartAggregatedFieldWells")]
    pub line_chart_aggregated_field_wells: Option<LineChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadingAnimation {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type TargetVisualOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PanelConfiguration {
    #[serde(rename = "GutterVisibility")]
    pub gutter_visibility: Option<Visibility>,
    #[serde(rename = "GutterSpacing")]
    pub gutter_spacing: Option<String>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "BorderColor")]
    pub border_color: Option<String>,
    #[serde(rename = "BackgroundVisibility")]
    pub background_visibility: Option<Visibility>,
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<PanelBorderStyle>,
    #[serde(rename = "Title")]
    pub title: Option<PanelTitleOptions>,
    #[serde(rename = "BorderVisibility")]
    pub border_visibility: Option<Visibility>,
    #[serde(rename = "BorderThickness")]
    pub border_thickness: Option<String>,

}
pub type VisualCustomActionTrigger = String;
#[derive(serde::Serialize, Default)]
pub struct WordCloudChartConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WordCloudFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WordCloudSortConfiguration>,
    #[serde(rename = "WordCloudOptions")]
    pub word_cloud_options: Option<WordCloudOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapSortConfiguration {
    #[serde(rename = "TreeMapGroupItemsLimitConfiguration")]
    pub tree_map_group_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "TreeMapSort")]
    pub tree_map_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartDefaultSeriesSettings {
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayOptions {
    #[serde(rename = "TickLabelOptions")]
    pub tick_label_options: Option<AxisTickLabelOptions>,
    #[serde(rename = "DataOptions")]
    pub data_options: Option<AxisDataOptions>,
    #[serde(rename = "ScrollbarOptions")]
    pub scrollbar_options: Option<ScrollBarOptions>,
    #[serde(rename = "AxisOffset")]
    pub axis_offset: Option<String>,
    #[serde(rename = "GridLineVisibility")]
    pub grid_line_visibility: Option<Visibility>,
    #[serde(rename = "AxisLineVisibility")]
    pub axis_line_visibility: Option<Visibility>,

}
pub type LayoutElementType = String;pub type TableTotalsPlacement = String;
#[derive(serde::Serialize, Default)]
pub struct SheetControlLayoutConfiguration {
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DataFieldSeriesItem {
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct BinCountOptions {
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Visual {
    #[serde(rename = "GaugeChartVisual")]
    pub gauge_chart_visual: Option<GaugeChartVisual>,
    #[serde(rename = "FilledMapVisual")]
    pub filled_map_visual: Option<FilledMapVisual>,
    #[serde(rename = "FunnelChartVisual")]
    pub funnel_chart_visual: Option<FunnelChartVisual>,
    #[serde(rename = "BoxPlotVisual")]
    pub box_plot_visual: Option<BoxPlotVisual>,
    #[serde(rename = "TableVisual")]
    pub table_visual: Option<TableVisual>,
    #[serde(rename = "WaterfallVisual")]
    pub waterfall_visual: Option<WaterfallVisual>,
    #[serde(rename = "HeatMapVisual")]
    pub heat_map_visual: Option<HeatMapVisual>,
    #[serde(rename = "EmptyVisual")]
    pub empty_visual: Option<EmptyVisual>,
    #[serde(rename = "RadarChartVisual")]
    pub radar_chart_visual: Option<RadarChartVisual>,
    #[serde(rename = "KPIVisual")]
    pub kpivisual: Option<KPIVisual>,
    #[serde(rename = "InsightVisual")]
    pub insight_visual: Option<InsightVisual>,
    #[serde(rename = "BarChartVisual")]
    pub bar_chart_visual: Option<BarChartVisual>,
    #[serde(rename = "CustomContentVisual")]
    pub custom_content_visual: Option<CustomContentVisual>,
    #[serde(rename = "GeospatialMapVisual")]
    pub geospatial_map_visual: Option<GeospatialMapVisual>,
    #[serde(rename = "LineChartVisual")]
    pub line_chart_visual: Option<LineChartVisual>,
    #[serde(rename = "TreeMapVisual")]
    pub tree_map_visual: Option<TreeMapVisual>,
    #[serde(rename = "ComboChartVisual")]
    pub combo_chart_visual: Option<ComboChartVisual>,
    #[serde(rename = "PivotTableVisual")]
    pub pivot_table_visual: Option<PivotTableVisual>,
    #[serde(rename = "PieChartVisual")]
    pub pie_chart_visual: Option<PieChartVisual>,
    #[serde(rename = "HistogramVisual")]
    pub histogram_visual: Option<HistogramVisual>,
    #[serde(rename = "WordCloudVisual")]
    pub word_cloud_visual: Option<WordCloudVisual>,
    #[serde(rename = "ScatterPlotVisual")]
    pub scatter_plot_visual: Option<ScatterPlotVisual>,
    #[serde(rename = "SankeyDiagramVisual")]
    pub sankey_diagram_visual: Option<SankeyDiagramVisual>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualSubtitleLabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FormatText")]
    pub format_text: Option<LongFormatText>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLinearScale {
    #[serde(rename = "StepCount")]
    pub step_count: Option<f64>,
    #[serde(rename = "StepSize")]
    pub step_size: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Vec<FilledMapConditionalFormattingOption>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramFieldWells {
    #[serde(rename = "HistogramAggregatedFieldWells")]
    pub histogram_aggregated_field_wells: Option<HistogramAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LongFormatText {
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<RadarChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "SearchOptions")]
    pub search_options: Option<ListControlSearchOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CascadingControlSource {
    #[serde(rename = "ColumnToMatch")]
    pub column_to_match: Option<ColumnIdentifier>,
    #[serde(rename = "SourceSheetControlId")]
    pub source_sheet_control_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DateMeasureField {
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<DateAggregationFunction>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}
pub type FilterNullOption = String;pub type RelativeFontSize = String;
#[derive(serde::Serialize, Default)]
pub struct TableBorderOptions {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Style")]
    pub style: Option<TableBorderStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartDataLabelOptions {
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,
    #[serde(rename = "MeasureDataLabelStyle")]
    pub measure_data_label_style: Option<FunnelChartMeasureDataLabelStyle>,
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramAggregatedFieldWells {
    #[serde(rename = "Weight")]
    pub weight: Option<Vec<MeasureField>>,
    #[serde(rename = "Destination")]
    pub destination: Option<Vec<DimensionField>>,
    #[serde(rename = "Source")]
    pub source: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct DateAxisOptions {
    #[serde(rename = "MissingDateVisibility")]
    pub missing_date_visibility: Option<Visibility>,

}
pub type NumericFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct LineSeriesAxisDisplayOptions {
    #[serde(rename = "MissingDataConfigurations")]
    pub missing_data_configurations: Option<Vec<MissingDataConfiguration>>,
    #[serde(rename = "AxisOptions")]
    pub axis_options: Option<AxisDisplayOptions>,

}
pub type MapZoomMode = String;
#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<SankeyDiagramChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryFilterConfiguration {
    #[serde(rename = "FilterListConfiguration")]
    pub filter_list_configuration: Option<FilterListConfiguration>,
    #[serde(rename = "CustomFilterConfiguration")]
    pub custom_filter_configuration: Option<CustomFilterConfiguration>,
    #[serde(rename = "CustomFilterListConfiguration")]
    pub custom_filter_list_configuration: Option<CustomFilterListConfiguration>,

}
pub type ResourceStatus = String;pub type BoxPlotFillStyle = String;
#[derive(serde::Serialize, Default)]
pub struct GridLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: Option<String>,
    #[serde(rename = "ResizeOption")]
    pub resize_option: ResizeOption,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnHierarchy {
    #[serde(rename = "DateTimeHierarchy")]
    pub date_time_hierarchy: Option<DateTimeHierarchy>,
    #[serde(rename = "PredefinedHierarchy")]
    pub predefined_hierarchy: Option<PredefinedHierarchy>,
    #[serde(rename = "ExplicitHierarchy")]
    pub explicit_hierarchy: Option<ExplicitHierarchy>,

}
pub type ReferenceLineValueLabelRelativePosition = String;pub type WordCloudWordScaling = String;
#[derive(serde::Serialize, Default)]
pub struct VisualPalette {
    #[serde(rename = "ChartColor")]
    pub chart_color: Option<String>,
    #[serde(rename = "ColorMap")]
    pub color_map: Option<Vec<DataPathColor>>,

}
pub type DataLabelPosition = String;
#[derive(serde::Serialize, Default)]
pub struct RadarChartSeriesSettings {
    #[serde(rename = "AreaStyleSettings")]
    pub area_style_settings: Option<RadarChartAreaStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct MaximumMinimumComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub ty: MaximumMinimumComputationType,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOption {
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "URLStyling")]
    pub urlstyling: Option<TableFieldURLConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type CategoricalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct FilterListControl {
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalDimensionField {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableRowConditionalFormatting {
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct BinWidthOptions {
    #[serde(rename = "BinCountLimit")]
    pub bin_count_limit: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineCustomLabelConfiguration {
    #[serde(rename = "CustomLabel")]
    pub custom_label: String,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterSliderControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartFieldWells {
    #[serde(rename = "FunnelChartAggregatedFieldWells")]
    pub funnel_chart_aggregated_field_wells: Option<FunnelChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct TableSideBorderOptions {
    #[serde(rename = "Left")]
    pub left: Option<TableBorderOptions>,
    #[serde(rename = "Right")]
    pub right: Option<TableBorderOptions>,
    #[serde(rename = "Top")]
    pub top: Option<TableBorderOptions>,
    #[serde(rename = "InnerVertical")]
    pub inner_vertical: Option<TableBorderOptions>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<TableBorderOptions>,
    #[serde(rename = "InnerHorizontal")]
    pub inner_horizontal: Option<TableBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TreeMapFieldWells>,
    #[serde(rename = "GroupLabelOptions")]
    pub group_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "SizeLabelOptions")]
    pub size_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TreeMapSortConfiguration>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,

}
pub type LineChartLineStyle = String;
#[derive(serde::Serialize, Default)]
pub struct ComboChartVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ComboChartConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartFieldWells {
    #[serde(rename = "BarChartAggregatedFieldWells")]
    pub bar_chart_aggregated_field_wells: Option<BarChartAggregatedFieldWells>,

}
pub type RelativeDateType = String;
#[derive(serde::Serialize, Default)]
pub struct SheetVisualScopingConfiguration {
    #[serde(rename = "VisualIds")]
    pub visual_ids: Option<Vec<String>>,
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "Scope")]
    pub scope: FilterVisualScope,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HeatMapFieldWells>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ColumnLabelOptions")]
    pub column_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<HeatMapSortConfiguration>,
    #[serde(rename = "RowLabelOptions")]
    pub row_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BarChartSortConfiguration>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BarChartFieldWells>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<BarChartOrientation>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "ValueAxis")]
    pub value_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableTotalOptions {
    #[serde(rename = "RowSubtotalOptions")]
    pub row_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "ColumnSubtotalOptions")]
    pub column_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "ColumnTotalOptions")]
    pub column_total_options: Option<PivotTotalOptions>,
    #[serde(rename = "RowTotalOptions")]
    pub row_total_options: Option<PivotTotalOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTotalOptions {
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableUnaggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<UnaggregatedField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<TableConditionalFormatting>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TableConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapFieldWells {
    #[serde(rename = "FilledMapAggregatedFieldWells")]
    pub filled_map_aggregated_field_wells: Option<FilledMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldWells {
    #[serde(rename = "PivotTableAggregatedFieldWells")]
    pub pivot_table_aggregated_field_wells: Option<PivotTableAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartAggregatedFieldWells {
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamicDefaultValue {
    #[serde(rename = "GroupNameColumn")]
    pub group_name_column: Option<ColumnIdentifier>,
    #[serde(rename = "DefaultValueColumn")]
    pub default_value_column: ColumnIdentifier,
    #[serde(rename = "UserNameColumn")]
    pub user_name_column: Option<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOption {
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartSortConfiguration {
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotConfiguration {
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "YAxisLabelOptions")]
    pub yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ScatterPlotFieldWells>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct WhatIfRangeScenario {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,

}
pub type FilterVisualScope = String;
#[derive(serde::Serialize, Default)]
pub struct LineChartSeriesSettings {
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberFormatConfiguration {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ShapeConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    pub background_color: ConditionalFormattingColor,

}

#[derive(serde::Serialize, Default)]
pub struct AggregationSortConfiguration {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "SortDirection")]
    pub sort_direction: SortDirection,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: AggregationFunction,

}
pub type DataLabelOverlap = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterListControl {
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "Title")]
    pub title: String,

}
pub type ParameterValueType = String;
#[derive(serde::Serialize, Default)]
pub struct ReferenceLine {
    #[serde(rename = "LabelConfiguration")]
    pub label_configuration: Option<ReferenceLineLabelConfiguration>,
    #[serde(rename = "DataConfiguration")]
    pub data_configuration: ReferenceLineDataConfiguration,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "StyleConfiguration")]
    pub style_configuration: Option<ReferenceLineStyleConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct WhatIfPointScenario {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}
pub type WordCloudWordPadding = String;
#[derive(serde::Serialize, Default)]
pub struct DonutCenterOptions {
    #[serde(rename = "LabelVisibility")]
    pub label_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct PanelTitleOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type PaperOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormattingOption {
    #[serde(rename = "Arc")]
    pub arc: Option<GaugeChartArcConditionalFormatting>,
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<GaugeChartPrimaryValueConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramConfiguration {
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "BinOptions")]
    pub bin_options: Option<HistogramBinOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HistogramFieldWells>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconOptions {
    #[serde(rename = "Icon")]
    pub icon: Option<Icon>,
    #[serde(rename = "UnicodeIcon")]
    pub unicode_icon: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DimensionField {
    #[serde(rename = "DateDimensionField")]
    pub date_dimension_field: Option<DateDimensionField>,
    #[serde(rename = "NumericalDimensionField")]
    pub numerical_dimension_field: Option<NumericalDimensionField>,
    #[serde(rename = "CategoricalDimensionField")]
    pub categorical_dimension_field: Option<CategoricalDimensionField>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedMeasureField {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}
pub type SheetControlDateTimePickerType = String;
#[derive(serde::Serialize, Default)]
pub struct ShortFormatText {
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,

}
pub type CrossDatasetTypes = String;
#[derive(serde::Serialize, Default)]
pub struct DateTimeHierarchy {
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct GradientColor {
    #[serde(rename = "Stops")]
    pub stops: Option<Vec<GradientStop>>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FunnelChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct GrowthRateComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "PeriodSize")]
    pub period_size: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapAggregatedFieldWells {
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDataConfiguration {
    #[serde(rename = "StaticConfiguration")]
    pub static_configuration: Option<ReferenceLineStaticDataConfiguration>,
    #[serde(rename = "DynamicConfiguration")]
    pub dynamic_configuration: Option<ReferenceLineDynamicDataConfiguration>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,

}
pub type ReferenceLineLabelVerticalPosition = String;pub type SheetContentType = String;
#[derive(serde::Serialize, Default)]
pub struct TopBottomFilter {
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Limit")]
    pub limit: Option<f64>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "AggregationSortConfigurations")]
    pub aggregation_sort_configurations: Vec<AggregationSortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualTitleLabelOptions {
    #[serde(rename = "FormatText")]
    pub format_text: Option<ShortFormatText>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct EmptyVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TreeMapConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericEqualityDrillDownFilter {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartArcConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomNarrativeOptions {
    #[serde(rename = "Narrative")]
    pub narrative: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldImageConfiguration {
    #[serde(rename = "SizingOptions")]
    pub sizing_options: Option<TableCellImageSizingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellStyle {
    #[serde(rename = "TextWrap")]
    pub text_wrap: Option<TextWrap>,
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Height")]
    pub height: Option<f64>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "Border")]
    pub border: Option<GlobalTableBorderOptions>,
    #[serde(rename = "VerticalTextAlignment")]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconCondition {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DisplayConfiguration")]
    pub display_configuration: Option<ConditionalFormattingIconDisplayConfiguration>,
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "IconOptions")]
    pub icon_options: ConditionalFormattingCustomIconOptions,

}

#[derive(serde::Serialize, Default)]
pub struct FilterTextFieldControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkContentConfiguration {
    #[serde(rename = "CustomIconContent")]
    pub custom_icon_content: Option<TableFieldCustomIconContent>,
    #[serde(rename = "CustomTextContent")]
    pub custom_text_content: Option<TableFieldCustomTextContent>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartPrimaryValueConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}

#[derive(serde::Serialize, Default)]
pub struct TableInlineVisualization {
    #[serde(rename = "DataBars")]
    pub data_bars: Option<DataBarsOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStaticDataConfiguration {
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct ScrollBarOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "VisibleRange")]
    pub visible_range: Option<VisibleRangeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomIconContent {
    #[serde(rename = "Icon")]
    pub icon: Option<TableFieldIconSetType>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSelectAllOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapFieldWells {
    #[serde(rename = "GeospatialMapAggregatedFieldWells")]
    pub geospatial_map_aggregated_field_wells: Option<GeospatialMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterScopeConfiguration {
    #[serde(rename = "SelectedSheets")]
    pub selected_sheets: Option<SelectedSheetsFilterScopeConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisConfiguration {
    #[serde(rename = "ReserveRange")]
    pub reserve_range: Option<f64>,
    #[serde(rename = "Range")]
    pub range: Option<ArcAxisDisplayRange>,

}
pub type MaximumMinimumComputationType = String;pub type NumericSeparatorSymbol = String;
#[derive(serde::Serialize, Default)]
pub struct FilterListConfiguration {
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,

}
pub type ReferenceLineLabelHorizontalPosition = String;
#[derive(serde::Serialize, Default)]
pub struct FilterControl {
    #[serde(rename = "TextField")]
    pub text_field: Option<FilterTextFieldControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<FilterTextAreaControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<FilterDropDownControl>,
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<FilterDateTimePickerControl>,
    #[serde(rename = "Slider")]
    pub slider: Option<FilterSliderControl>,
    #[serde(rename = "RelativeDateTime")]
    pub relative_date_time: Option<FilterRelativeDateTimeControl>,
    #[serde(rename = "List")]
    pub list: Option<FilterListControl>,

}
pub type CustomContentImageScalingConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct LayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: Option<FreeFormLayoutConfiguration>,
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,
    #[serde(rename = "SectionBasedLayout")]
    pub section_based_layout: Option<SectionBasedLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetElementRenderingRule {
    #[serde(rename = "ConfigurationOverrides")]
    pub configuration_overrides: SheetElementConfigurationOverrides,
    #[serde(rename = "Expression")]
    pub expression: String,

}
pub type TableCellImageScalingConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct TopBottomRankedComputation {
    #[serde(rename = "ResultSize")]
    pub result_size: Option<f64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,

}
pub type WordCloudCloudLayout = String;
#[derive(serde::Serialize, Default)]
pub struct LineChartConfiguration {
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<LineChartType>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    pub default_series_settings: Option<LineChartDefaultSeriesSettings>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "Series")]
    pub series: Option<Vec<SeriesItem>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<LineChartSortConfiguration>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ForecastConfigurations")]
    pub forecast_configurations: Option<Vec<ForecastConfiguration>>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<LineChartFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct RelativeDatesFilter {
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,
    #[serde(rename = "AnchorDateConfiguration")]
    pub anchor_date_configuration: AnchorDateConfiguration,
    #[serde(rename = "RelativeDateValue")]
    pub relative_date_value: Option<f64>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "MinimumGranularity")]
    pub minimum_granularity: Option<TimeGranularity>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RelativeDateType")]
    pub relative_date_type: RelativeDateType,

}
pub type TopBottomSortOrder = String;
#[derive(serde::Serialize, Default)]
pub struct DecimalParameter {
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
    #[serde(rename = "Name")]
    pub name: String,

}
pub type AxisBinding = String;
#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<TableConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomContentVisual {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<CustomContentConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}
pub type SheetControlSliderType = String;
#[derive(serde::Serialize, Default)]
pub struct FieldBasedTooltip {
    #[serde(rename = "AggregationVisibility")]
    pub aggregation_visibility: Option<Visibility>,
    #[serde(rename = "TooltipTitleType")]
    pub tooltip_title_type: Option<TooltipTitleType>,
    #[serde(rename = "TooltipFields")]
    pub tooltip_fields: Option<Vec<TooltipItem>>,

}
pub type BarsArrangement = String;
#[derive(serde::Serialize, Default)]
pub struct NumericEqualityFilter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "MatchOperator")]
    pub match_operator: NumericEqualityMatchOperator,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludePeriodConfiguration {
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Granularity")]
    pub granularity: TimeGranularity,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionLayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: FreeFormSectionLayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSeriesItem {
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<KPIConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct RangeEndsLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeEqualityFilter {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VisibleRangeOptions {
    #[serde(rename = "PercentRange")]
    pub percent_range: Option<PercentVisibleRange>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Values")]
    pub values: Vec<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetReference {
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    #[serde(rename = "DataSetPlaceholder")]
    pub data_set_placeholder: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormSectionLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,

}
pub type TableBorderStyle = String;
#[derive(serde::Serialize, Default)]
pub struct DataPathSort {
    #[serde(rename = "Direction")]
    pub direction: SortDirection,
    #[serde(rename = "SortPaths")]
    pub sort_paths: Vec<DataPathValue>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartConfiguration {
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "DonutOptions")]
    pub donut_options: Option<DonutOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PieChartFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PieChartSortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SetParameterValueConfiguration {
    #[serde(rename = "DestinationParameterName")]
    pub destination_parameter_name: String,
    #[serde(rename = "Value")]
    pub value: DestinationParameterValueConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct Sheet {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SheetId")]
    pub sheet_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterDateTimePickerControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,

}

#[derive(serde::Serialize, Default)]
pub struct ContributionAnalysisDefault {
    #[serde(rename = "ContributorDimensions")]
    pub contributor_dimensions: Vec<ColumnIdentifier>,
    #[serde(rename = "MeasureFieldId")]
    pub measure_field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct AnchorDateConfiguration {
    #[serde(rename = "AnchorOption")]
    pub anchor_option: Option<AnchorOption>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}
pub type DateAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct AxisLabelOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "ApplyTo")]
    pub apply_to: Option<AxisLabelReferenceOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingSolidColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}
pub type NumericEqualityMatchOperator = String;pub type ArcThicknessOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableDataPathOption {
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "DataPathList")]
    pub data_path_list: Vec<DataPathValue>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<PivotTableConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeDefaultValues {
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct FontWeight {
    #[serde(rename = "Name")]
    pub name: Option<FontWeightName>,

}

#[derive(serde::Serialize, Default)]
pub struct SeriesItem {
    #[serde(rename = "DataFieldSeriesItem")]
    pub data_field_series_item: Option<DataFieldSeriesItem>,
    #[serde(rename = "FieldSeriesItem")]
    pub field_series_item: Option<FieldSeriesItem>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnIdentifier {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "ColumnName")]
    pub column_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationParameterValueConfiguration {
    #[serde(rename = "CustomValuesConfiguration")]
    pub custom_values_configuration: Option<CustomValuesConfiguration>,
    #[serde(rename = "SelectAllValueOptions")]
    pub select_all_value_options: Option<SelectAllValueOptions>,
    #[serde(rename = "SourceField")]
    pub source_field: Option<String>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: Option<String>,

}
pub type SheetControlListType = String;
#[derive(serde::Serialize, Default)]
pub struct WordCloudSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeParameter {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilterValue {
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastConfiguration {
    #[serde(rename = "ForecastProperties")]
    pub forecast_properties: Option<TimeBasedForecastProperties>,
    #[serde(rename = "Scenario")]
    pub scenario: Option<ForecastScenario>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormattingOption {
    #[serde(rename = "Shape")]
    pub shape: FilledMapShapeConditionalFormatting,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartAggregatedFieldWells {
    #[serde(rename = "Categories")]
    pub categories: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Breakdowns")]
    pub breakdowns: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableOptions {
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<TableOrientation>,
    #[serde(rename = "HeaderStyle")]
    pub header_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericSeparatorConfiguration {
    #[serde(rename = "ThousandsSeparator")]
    pub thousands_separator: Option<ThousandSeparatorOptions>,
    #[serde(rename = "DecimalSeparator")]
    pub decimal_separator: Option<NumericSeparatorSymbol>,

}

#[derive(serde::Serialize, Default)]
pub struct DateDimensionField {
    #[serde(rename = "DateGranularity")]
    pub date_granularity: Option<TimeGranularity>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldURLConfiguration {
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<TableFieldImageConfiguration>,
    #[serde(rename = "LinkConfiguration")]
    pub link_configuration: Option<TableFieldLinkConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalParameterDeclaration {
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DecimalDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DecimalValueWhenUnsetConfiguration>,

}
pub type TopBottomComputationType = String;
#[derive(serde::Serialize, Default)]
pub struct TotalAggregationComputation {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: MeasureField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<FreeFormLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterDateTimePickerControl {
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlDateTimePickerType>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct StringParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<StringDefaultValues>,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<StringValueWhenUnsetConfiguration>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    pub name: String,

}
pub type PrimaryValueDisplayType = String;
#[derive(serde::Serialize, Default)]
pub struct FontSize {
    #[serde(rename = "Relative")]
    pub relative: Option<RelativeFontSize>,

}
pub type Icon = String;
#[derive(serde::Serialize, Default)]
pub struct NegativeValueConfiguration {
    #[serde(rename = "DisplayMode")]
    pub display_mode: NegativeValueDisplayMode,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisDefinition {
    #[serde(rename = "ColumnConfigurations")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "DataSetIdentifierDeclarations")]
    pub data_set_identifier_declarations: Vec<DataSetIdentifierDeclaration>,
    #[serde(rename = "AnalysisDefaults")]
    pub analysis_defaults: Option<AnalysisDefaults>,
    #[serde(rename = "ParameterDeclarations")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "CalculatedFields")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,

}
pub type BarChartOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct FilterOperationSelectedFieldsConfiguration {
    #[serde(rename = "SelectedFields")]
    pub selected_fields: Option<Vec<String>>,
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<SelectedFieldOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineLabelConfiguration {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "ValueLabelConfiguration")]
    pub value_label_configuration: Option<ReferenceLineValueLabelConfiguration>,
    #[serde(rename = "CustomLabelConfiguration")]
    pub custom_label_configuration: Option<ReferenceLineCustomLabelConfiguration>,
    #[serde(rename = "VerticalPosition")]
    pub vertical_position: Option<ReferenceLineLabelVerticalPosition>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "HorizontalPosition")]
    pub horizontal_position: Option<ReferenceLineLabelHorizontalPosition>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,

}
pub type OtherCategories = String;
#[derive(serde::Serialize, Default)]
pub struct LineChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimitConfiguration")]
    pub category_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimitConfiguration")]
    pub color_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeDrillDownFilter {
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: String,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalMeasureField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<CategoricalAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnSort {
    #[serde(rename = "Direction")]
    pub direction: SortDirection,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "SortBy")]
    pub sort_by: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct KPIPrimaryValueConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}

#[derive(serde::Serialize, Default)]
pub struct RelativeDateTimeControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    pub cell: Option<TableCellConditionalFormatting>,
    #[serde(rename = "Row")]
    pub row: Option<TableRowConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextFieldControl {
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterSliderControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlSliderType>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastComputation {
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,
    #[serde(rename = "CustomSeasonalityValue")]
    pub custom_seasonality_value: Option<f64>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<ForecastComputationSeasonality>,

}
pub type GeospatialSelectedPointStyle = String;
#[derive(serde::Serialize, Default)]
pub struct TooltipItem {
    #[serde(rename = "ColumnTooltipItem")]
    pub column_tooltip_item: Option<ColumnTooltipItem>,
    #[serde(rename = "FieldTooltipItem")]
    pub field_tooltip_item: Option<FieldTooltipItem>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualCustomAction {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CustomActionId")]
    pub custom_action_id: String,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "ActionOperations")]
    pub action_operations: Vec<VisualCustomActionOperation>,
    #[serde(rename = "Trigger")]
    pub trigger: VisualCustomActionTrigger,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisSourceEntity {
    #[serde(rename = "SourceTemplate")]
    pub source_template: Option<AnalysisSourceTemplate>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomFilterConfiguration {
    #[serde(rename = "CategoryValue")]
    pub category_value: Option<String>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSearchOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TooltipOptions {
    #[serde(rename = "FieldBasedTooltip")]
    pub field_based_tooltip: Option<FieldBasedTooltip>,
    #[serde(rename = "TooltipVisibility")]
    pub tooltip_visibility: Option<Visibility>,
    #[serde(rename = "SelectedTooltipType")]
    pub selected_tooltip_type: Option<SelectedTooltipType>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapAggregatedFieldWells {
    #[serde(rename = "Sizes")]
    pub sizes: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<MeasureField>>,
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldLabelType {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultGridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: GridLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct MeasureField {
    #[serde(rename = "DateMeasureField")]
    pub date_measure_field: Option<DateMeasureField>,
    #[serde(rename = "CategoricalMeasureField")]
    pub categorical_measure_field: Option<CategoricalMeasureField>,
    #[serde(rename = "NumericalMeasureField")]
    pub numerical_measure_field: Option<NumericalMeasureField>,
    #[serde(rename = "CalculatedMeasureField")]
    pub calculated_measure_field: Option<CalculatedMeasureField>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TableSortConfiguration>,
    #[serde(rename = "TableInlineVisualizations")]
    pub table_inline_visualizations: Option<Vec<TableInlineVisualization>>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TableFieldWells>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<TablePaginatedReportOptions>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<TotalOptions>,
    #[serde(rename = "TableOptions")]
    pub table_options: Option<TableOptions>,
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<TableFieldOptions>,

}
pub type ArcThickness = String;
#[derive(serde::Serialize, Default)]
pub struct PredefinedHierarchy {
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DonutOptions {
    #[serde(rename = "DonutCenterOptions")]
    pub donut_center_options: Option<DonutCenterOptions>,
    #[serde(rename = "ArcOptions")]
    pub arc_options: Option<ArcOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ColorScale {
    #[serde(rename = "Colors")]
    pub colors: Vec<DataColor>,
    #[serde(rename = "ColorFillType")]
    pub color_fill_type: ColorFillType,
    #[serde(rename = "NullValueColor")]
    pub null_value_color: Option<DataColor>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnConfiguration {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,
    #[serde(rename = "Role")]
    pub role: Option<ColumnRole>,
    #[serde(rename = "ColorsConfiguration")]
    pub colors_configuration: Option<ColorsConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableAggregatedFieldWells {
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,

}
pub type SpecialValue = String;
#[derive(serde::Serialize, Default)]
pub struct DateTimeParameterDeclaration {
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DateTimeValueWhenUnsetConfiguration>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DateTimeDefaultValues>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSortOptions {
    #[serde(rename = "FieldSort")]
    pub field_sort: Option<FieldSort>,
    #[serde(rename = "ColumnSort")]
    pub column_sort: Option<ColumnSort>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterDropDownControl {
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathLabelType {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartFieldWells {
    #[serde(rename = "WaterfallChartAggregatedFieldWells")]
    pub waterfall_chart_aggregated_field_wells: Option<WaterfallChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultNewSheetConfiguration {
    #[serde(rename = "InteractiveLayoutConfiguration")]
    pub interactive_layout_configuration: Option<DefaultInteractiveLayoutConfiguration>,
    #[serde(rename = "PaginatedLayoutConfiguration")]
    pub paginated_layout_configuration: Option<DefaultPaginatedLayoutConfiguration>,
    #[serde(rename = "SheetContentType")]
    pub sheet_content_type: Option<SheetContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionStyle {
    #[serde(rename = "Padding")]
    pub padding: Option<Spacing>,
    #[serde(rename = "Height")]
    pub height: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisDefaults {
    #[serde(rename = "DefaultNewSheetConfiguration")]
    pub default_new_sheet_configuration: DefaultNewSheetConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<GridLayoutElement>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<GridLayoutCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetTextBox {
    #[serde(rename = "Content")]
    pub content: Option<String>,
    #[serde(rename = "SheetTextBoxId")]
    pub sheet_text_box_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PieChartConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStyleConfiguration {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Pattern")]
    pub pattern: Option<ReferenceLinePatternType>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultFreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: FreeFormLayoutCanvasSizeOptions,

}
pub type CustomContentType = String;pub type ReferenceLinePatternType = String;pub type PivotTableSubtotalLevel = String;pub type AnchorOption = String;
#[derive(serde::Serialize, Default)]
pub struct WordCloudFieldWells {
    #[serde(rename = "WordCloudAggregatedFieldWells")]
    pub word_cloud_aggregated_field_wells: Option<WordCloudAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct FormatConfiguration {
    #[serde(rename = "StringFormatConfiguration")]
    pub string_format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "NumberFormatConfiguration")]
    pub number_format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "DateTimeFormatConfiguration")]
    pub date_time_format_configuration: Option<DateTimeFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIProgressBarConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct SecondaryValueOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartSortConfiguration {
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomValuesConfiguration {
    #[serde(rename = "IncludeNullValue")]
    pub include_null_value: Option<bool>,
    #[serde(rename = "CustomValues")]
    pub custom_values: CustomParameterValues,

}

#[derive(serde::Serialize, Default)]
pub struct FilterGroup {
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
    #[serde(rename = "FilterGroupId")]
    pub filter_group_id: String,
    #[serde(rename = "CrossDataset")]
    pub cross_dataset: CrossDatasetTypes,
    #[serde(rename = "ScopeConfiguration")]
    pub scope_configuration: FilterScopeConfiguration,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineValueLabelConfiguration {
    #[serde(rename = "RelativePosition")]
    pub relative_position: Option<ReferenceLineValueLabelRelativePosition>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}
pub type TableFieldIconSetType = String;
#[derive(serde::Serialize, Default)]
pub struct CustomActionURLOperation {
    #[serde(rename = "URLTemplate")]
    pub urltemplate: String,
    #[serde(rename = "URLTarget")]
    pub urltarget: URLTargetConfiguration,

}
pub type TimeGranularity = String;
#[derive(serde::Serialize, Default)]
pub struct LocalNavigationConfiguration {
    #[serde(rename = "TargetSheetId")]
    pub target_sheet_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataColor {
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TotalOptions {
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDynamicDataConfiguration {
    #[serde(rename = "MeasureAggregationFunction")]
    pub measure_aggregation_function: AggregationFunction,
    #[serde(rename = "Calculation")]
    pub calculation: NumericalAggregationFunction,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartOptions {
    #[serde(rename = "TotalBarLabel")]
    pub total_bar_label: Option<String>,

}
pub type NumberScale = String;
#[derive(serde::Serialize, Default)]
pub struct ComparisonFormatConfiguration {
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ItemsLimitConfiguration {
    #[serde(rename = "ItemsLimit")]
    pub items_limit: Option<f64>,
    #[serde(rename = "OtherCategories")]
    pub other_categories: Option<OtherCategories>,

}

#[derive(serde::Serialize, Default)]
pub struct UniqueValuesComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Category")]
    pub category: DimensionField,

}
pub type NegativeValueDisplayMode = String;
#[derive(serde::Serialize, Default)]
pub struct HistogramVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HistogramConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct MinimumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Configuration")]
    pub configuration: CategoryFilterConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct CustomColor {
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "SpecialValue")]
    pub special_value: Option<SpecialValue>,

}

#[derive(serde::Serialize, Default)]
pub struct StringValueWhenUnsetConfiguration {
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TextFieldControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GeospatialMapConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}
pub type TableTotalsScrollStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "NumericRangeFilter")]
    pub numeric_range_filter: Option<NumericRangeFilter>,
    #[serde(rename = "TimeEqualityFilter")]
    pub time_equality_filter: Option<TimeEqualityFilter>,
    #[serde(rename = "RelativeDatesFilter")]
    pub relative_dates_filter: Option<RelativeDatesFilter>,
    #[serde(rename = "TopBottomFilter")]
    pub top_bottom_filter: Option<TopBottomFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityFilter>,
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartMarkerStyleSettings {
    #[serde(rename = "MarkerSize")]
    pub marker_size: Option<String>,
    #[serde(rename = "MarkerShape")]
    pub marker_shape: Option<LineChartMarkerShape>,
    #[serde(rename = "MarkerVisibility")]
    pub marker_visibility: Option<Visibility>,
    #[serde(rename = "MarkerColor")]
    pub marker_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionNavigationOperation {
    #[serde(rename = "LocalNavigationConfiguration")]
    pub local_navigation_configuration: Option<LocalNavigationConfiguration>,

}
pub type AnalysisErrorType = String;
#[derive(serde::Serialize, Default)]
pub struct LineChartVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<LineChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct MissingDataConfiguration {
    #[serde(rename = "TreatmentOption")]
    pub treatment_option: Option<MissingDataTreatmentOption>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<KPIConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<KPIConditionalFormatting>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PivotTableSortConfiguration>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<PivotTableTotalOptions>,
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<PivotTableFieldOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PivotTableFieldWells>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<PivotTablePaginatedReportOptions>,
    #[serde(rename = "TableOptions")]
    pub table_options: Option<PivotTableOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartFieldWells {
    #[serde(rename = "RadarChartAggregatedFieldWells")]
    pub radar_chart_aggregated_field_wells: Option<RadarChartAggregatedFieldWells>,

}
pub type ForecastComputationSeasonality = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "LinkToDataSetColumn")]
    pub link_to_data_set_column: Option<ColumnIdentifier>,

}
pub type SortDirection = String;
#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkConfiguration {
    #[serde(rename = "Content")]
    pub content: TableFieldLinkContentConfiguration,
    #[serde(rename = "Target")]
    pub target: URLTargetConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct TextControlPlaceholderOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelOptions {
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,
    #[serde(rename = "LabelContent")]
    pub label_content: Option<DataLabelContent>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,
    #[serde(rename = "DataLabelTypes")]
    pub data_label_types: Option<Vec<DataLabelType>>,
    #[serde(rename = "Overlap")]
    pub overlap: Option<DataLabelOverlap>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDataOptions {
    #[serde(rename = "DateAxisOptions")]
    pub date_axis_options: Option<DateAxisOptions>,
    #[serde(rename = "NumericAxisOptions")]
    pub numeric_axis_options: Option<NumericAxisOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapSortConfiguration {
    #[serde(rename = "HeatMapColumnSort")]
    pub heat_map_column_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "HeatMapColumnItemsLimitConfiguration")]
    pub heat_map_column_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapRowItemsLimitConfiguration")]
    pub heat_map_row_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapRowSort")]
    pub heat_map_row_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct KPISortConfiguration {
    #[serde(rename = "TrendGroupSort")]
    pub trend_group_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetElementConfigurationOverrides {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcOptions {
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThickness>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WordCloudChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotChartConfiguration {
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BoxPlotFieldWells>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BoxPlotSortConfiguration>,
    #[serde(rename = "BoxPlotOptions")]
    pub box_plot_options: Option<BoxPlotOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathValue {
    #[serde(rename = "FieldValue")]
    pub field_value: String,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FieldTooltipItem {
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotFieldSortOptions {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "SortBy")]
    pub sort_by: PivotTableSortBy,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomTextContent {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: FontConfiguration,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RollingDateConfiguration {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElement {
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,
    #[serde(rename = "RenderingRules")]
    pub rendering_rules: Option<Vec<SheetElementRenderingRule>>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "XAxisLocation")]
    pub xaxis_location: String,
    #[serde(rename = "SelectedBorderStyle")]
    pub selected_border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "YAxisLocation")]
    pub yaxis_location: String,
    #[serde(rename = "Height")]
    pub height: String,
    #[serde(rename = "BackgroundStyle")]
    pub background_style: Option<FreeFormLayoutElementBackgroundStyle>,
    #[serde(rename = "Width")]
    pub width: String,
    #[serde(rename = "LoadingAnimation")]
    pub loading_animation: Option<LoadingAnimation>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartOptions {
    #[serde(rename = "ArcAxis")]
    pub arc_axis: Option<ArcAxisConfiguration>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "Arc")]
    pub arc: Option<ArcConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartAggregatedFieldWells {
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "BarValues")]
    pub bar_values: Option<Vec<MeasureField>>,
    #[serde(rename = "LineValues")]
    pub line_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomMoversComputation {
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<TopBottomSortOrder>,
    #[serde(rename = "MoverSize")]
    pub mover_size: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelType {
    #[serde(rename = "RangeEndsLabelType")]
    pub range_ends_label_type: Option<RangeEndsLabelType>,
    #[serde(rename = "FieldLabelType")]
    pub field_label_type: Option<FieldLabelType>,
    #[serde(rename = "DataPathLabelType")]
    pub data_path_label_type: Option<DataPathLabelType>,
    #[serde(rename = "MaximumLabelType")]
    pub maximum_label_type: Option<MaximumLabelType>,
    #[serde(rename = "MinimumLabelType")]
    pub minimum_label_type: Option<MinimumLabelType>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataBarsOptions {
    #[serde(rename = "NegativeColor")]
    pub negative_color: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "PositiveColor")]
    pub positive_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct UnaggregatedField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryDrillDownFilter {
    #[serde(rename = "CategoryValues")]
    pub category_values: Vec<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIcon {
    #[serde(rename = "CustomCondition")]
    pub custom_condition: Option<ConditionalFormattingCustomIconCondition>,
    #[serde(rename = "IconSet")]
    pub icon_set: Option<ConditionalFormattingIconSet>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapConfiguration {
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GeospatialMapFieldWells>,
    #[serde(rename = "PointStyleOptions")]
    pub point_style_options: Option<GeospatialPointStyleOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightVisual {
    #[serde(rename = "InsightConfiguration")]
    pub insight_configuration: Option<InsightConfiguration>,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultSectionBasedLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct SheetDefinition {
    #[serde(rename = "ParameterControls")]
    pub parameter_controls: Option<Vec<ParameterControl>>,
    #[serde(rename = "Layouts")]
    pub layouts: Option<Vec<Layout>>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TextBoxes")]
    pub text_boxes: Option<Vec<SheetTextBox>>,
    #[serde(rename = "FilterControls")]
    pub filter_controls: Option<Vec<FilterControl>>,
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "SheetControlLayouts")]
    pub sheet_control_layouts: Option<Vec<SheetControlLayout>>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<SheetContentType>,
    #[serde(rename = "Visuals")]
    pub visuals: Option<Vec<Visual>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerValueWhenUnsetConfiguration {
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct PercentVisibleRange {
    #[serde(rename = "From")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    pub to: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<GridLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisSourceTemplate {
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalMeasureField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<NumericalAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderFooterSectionConfiguration {
    #[serde(rename = "SectionId")]
    pub section_id: String,
    #[serde(rename = "Layout")]
    pub layout: SectionLayoutConfiguration,
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct StringFormatConfiguration {
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TextConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}

#[derive(serde::Serialize, Default)]
pub struct GradientStop {
    #[serde(rename = "GradientOffset")]
    pub gradient_offset: f64,
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}
pub type VerticalTextAlignment = String;
#[derive(serde::Serialize, Default)]
pub struct ClusterMarkerConfiguration {
    #[serde(rename = "ClusterMarker")]
    pub cluster_marker: Option<ClusterMarker>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalDimensionField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PeriodOverPeriodComputation {
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FontConfiguration {
    #[serde(rename = "FontWeight")]
    pub font_weight: Option<FontWeight>,
    #[serde(rename = "FontStyle")]
    pub font_style: Option<FontStyle>,
    #[serde(rename = "FontDecoration")]
    pub font_decoration: Option<FontDecoration>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<FontSize>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotFieldWells {
    #[serde(rename = "BoxPlotAggregatedFieldWells")]
    pub box_plot_aggregated_field_wells: Option<BoxPlotAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<GaugeChartConditionalFormatting>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GaugeChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}
pub type PivotTableMetricPlacement = String;pub type FontDecoration = String;
#[derive(serde::Serialize, Default)]
pub struct StringParameter {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutPaperCanvasSizeOptions {
    #[serde(rename = "PaperMargin")]
    pub paper_margin: Option<Spacing>,
    #[serde(rename = "PaperOrientation")]
    pub paper_orientation: Option<PaperOrientation>,
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<PaperSize>,

}
pub type LineChartMarkerShape = String;
#[derive(serde::Serialize, Default)]
pub struct NumericAxisOptions {
    #[serde(rename = "Scale")]
    pub scale: Option<AxisScale>,
    #[serde(rename = "Range")]
    pub range: Option<AxisDisplayRange>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotCategoricallyAggregatedFieldWells {
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<MeasureField>>,
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,

}
pub type WidgetStatus = String;
#[derive(serde::Serialize, Default)]
pub struct TableAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterRelativeDateTimeControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}
pub type TextWrap = String;pub type LegendPosition = String;pub type ColorFillType = String;
#[derive(serde::Serialize, Default)]
pub struct ProgressBarOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnTooltipItem {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Aggregation")]
    pub aggregation: Option<AggregationFunction>,
    #[serde(rename = "Label")]
    pub label: Option<String>,

}
pub type CategoryFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    pub cell: Option<PivotTableCellConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIFieldWells {
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "TrendGroups")]
    pub trend_groups: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct Computation {
    #[serde(rename = "MaximumMinimum")]
    pub maximum_minimum: Option<MaximumMinimumComputation>,
    #[serde(rename = "PeriodToDate")]
    pub period_to_date: Option<PeriodToDateComputation>,
    #[serde(rename = "TopBottomRanked")]
    pub top_bottom_ranked: Option<TopBottomRankedComputation>,
    #[serde(rename = "PeriodOverPeriod")]
    pub period_over_period: Option<PeriodOverPeriodComputation>,
    #[serde(rename = "Forecast")]
    pub forecast: Option<ForecastComputation>,
    #[serde(rename = "TotalAggregation")]
    pub total_aggregation: Option<TotalAggregationComputation>,
    #[serde(rename = "TopBottomMovers")]
    pub top_bottom_movers: Option<TopBottomMoversComputation>,
    #[serde(rename = "MetricComparison")]
    pub metric_comparison: Option<MetricComparisonComputation>,
    #[serde(rename = "UniqueValues")]
    pub unique_values: Option<UniqueValuesComputation>,
    #[serde(rename = "GrowthRate")]
    pub growth_rate: Option<GrowthRateComputation>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    #[serde(rename = "Resource")]
    pub resource: Option<String>,
    #[serde(rename = "Principal")]
    pub principal: String,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalPlacesConfiguration {
    #[serde(rename = "DecimalPlaces")]
    pub decimal_places: f64,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartLineStyleSettings {
    #[serde(rename = "LineWidth")]
    pub line_width: Option<String>,
    #[serde(rename = "LineInterpolation")]
    pub line_interpolation: Option<LineInterpolation>,
    #[serde(rename = "LineVisibility")]
    pub line_visibility: Option<Visibility>,
    #[serde(rename = "LineStyle")]
    pub line_style: Option<LineChartLineStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct DropDownControlDisplayOptions {
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingColor {
    #[serde(rename = "Solid")]
    pub solid: Option<ConditionalFormattingSolidColor>,
    #[serde(rename = "Gradient")]
    pub gradient: Option<ConditionalFormattingGradientColor>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericFormatConfiguration {
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,
    #[serde(rename = "CurrencyDisplayFormatConfiguration")]
    pub currency_display_format_configuration: Option<CurrencyDisplayFormatConfiguration>,

}
pub type URLTargetConfiguration = String;pub type ConditionalFormattingIconSetType = String;
#[derive(serde::Serialize, Default)]
pub struct MaximumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ChartAxisLabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "SortIconVisibility")]
    pub sort_icon_visibility: Option<Visibility>,
    #[serde(rename = "AxisLabelOptions")]
    pub axis_label_options: Option<Vec<AxisLabelOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct NullValueFormatConfiguration {
    #[serde(rename = "NullString")]
    pub null_string: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableSortBy {
    #[serde(rename = "Field")]
    pub field: Option<FieldSort>,
    #[serde(rename = "Column")]
    pub column: Option<ColumnSort>,
    #[serde(rename = "DataPath")]
    pub data_path: Option<DataPathSort>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialPointStyleOptions {
    #[serde(rename = "SelectedPointStyle")]
    pub selected_point_style: Option<GeospatialSelectedPointStyle>,
    #[serde(rename = "ClusterMarkerConfiguration")]
    pub cluster_marker_configuration: Option<ClusterMarkerConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BoxPlotChartConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotUnaggregatedFieldWells {
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastScenario {
    #[serde(rename = "WhatIfPointScenario")]
    pub what_if_point_scenario: Option<WhatIfPointScenario>,
    #[serde(rename = "WhatIfRangeScenario")]
    pub what_if_range_scenario: Option<WhatIfRangeScenario>,

}
pub type FunnelChartMeasureDataLabelStyle = String;
#[derive(serde::Serialize, Default)]
pub struct NumericalAggregationFunction {
    #[serde(rename = "SimpleNumericalAggregation")]
    pub simple_numerical_aggregation: Option<SimpleNumericalAggregationFunction>,
    #[serde(rename = "PercentileAggregation")]
    pub percentile_aggregation: Option<PercentileAggregation>,

}
pub type Visibility = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTablePaginatedReportOptions {
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextAreaControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}
pub type TooltipTitleType = String;pub type ColumnRole = String;
#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingGradientColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: GradientColor,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HeatMapConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BarChartConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct PercentileAggregation {
    #[serde(rename = "PercentileValue")]
    pub percentile_value: Option<f64>,

}


}

pub mod cfn_dashboard {

#[derive(serde::Serialize, Default)]
pub struct CfnDashboard {
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,
    /// List of ResourcePermission
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// No documentation provided by AWS
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SourceEntity")]
    pub source_entity: Option<DashboardSourceEntity>,
    /// No documentation provided by AWS
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DashboardPublishOptions")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "DashboardId")]
    pub dashboard_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Option<DashboardVersionDefinition>,

}


#[derive(serde::Serialize, Default)]
pub struct PivotTableSortBy {
    #[serde(rename = "Field")]
    pub field: Option<FieldSort>,
    #[serde(rename = "Column")]
    pub column: Option<ColumnSort>,
    #[serde(rename = "DataPath")]
    pub data_path: Option<DataPathSort>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetLayoutElementMaximizationOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeParameter {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterListControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnSort {
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "SortBy")]
    pub sort_by: ColumnIdentifier,
    #[serde(rename = "Direction")]
    pub direction: SortDirection,

}

#[derive(serde::Serialize, Default)]
pub struct LegendOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "Position")]
    pub position: Option<LegendPosition>,
    #[serde(rename = "Title")]
    pub title: Option<LabelOptions>,
    #[serde(rename = "Height")]
    pub height: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSeriesItem {
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,

}
pub type PanelBorderStyle = String;
#[derive(serde::Serialize, Default)]
pub struct CustomContentVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<CustomContentConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartConfiguration {
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "DonutOptions")]
    pub donut_options: Option<DonutOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PieChartSortConfiguration>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PieChartFieldWells>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineLabelConfiguration {
    #[serde(rename = "ValueLabelConfiguration")]
    pub value_label_configuration: Option<ReferenceLineValueLabelConfiguration>,
    #[serde(rename = "HorizontalPosition")]
    pub horizontal_position: Option<ReferenceLineLabelHorizontalPosition>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "VerticalPosition")]
    pub vertical_position: Option<ReferenceLineLabelVerticalPosition>,
    #[serde(rename = "CustomLabelConfiguration")]
    pub custom_label_configuration: Option<ReferenceLineCustomLabelConfiguration>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,

}
pub type TextWrap = String;
#[derive(serde::Serialize, Default)]
pub struct WhatIfRangeScenario {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,

}
pub type WordCloudWordPadding = String;
#[derive(serde::Serialize, Default)]
pub struct CategoryFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Configuration")]
    pub configuration: CategoryFilterConfiguration,
    #[serde(rename = "FilterId")]
    pub filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct CustomNarrativeOptions {
    #[serde(rename = "Narrative")]
    pub narrative: String,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultNewSheetConfiguration {
    #[serde(rename = "PaginatedLayoutConfiguration")]
    pub paginated_layout_configuration: Option<DefaultPaginatedLayoutConfiguration>,
    #[serde(rename = "InteractiveLayoutConfiguration")]
    pub interactive_layout_configuration: Option<DefaultInteractiveLayoutConfiguration>,
    #[serde(rename = "SheetContentType")]
    pub sheet_content_type: Option<SheetContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalTableBorderOptions {
    #[serde(rename = "UniformBorder")]
    pub uniform_border: Option<TableBorderOptions>,
    #[serde(rename = "SideSpecificBorder")]
    pub side_specific_border: Option<TableSideBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberFormatConfiguration {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ColorScale {
    #[serde(rename = "NullValueColor")]
    pub null_value_color: Option<DataColor>,
    #[serde(rename = "Colors")]
    pub colors: Vec<DataColor>,
    #[serde(rename = "ColorFillType")]
    pub color_fill_type: ColorFillType,

}

#[derive(serde::Serialize, Default)]
pub struct PercentVisibleRange {
    #[serde(rename = "From")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    pub to: Option<f64>,

}
pub type BoxPlotFillStyle = String;pub type AnchorOption = String;
#[derive(serde::Serialize, Default)]
pub struct AxisDisplayRange {
    #[serde(rename = "MinMax")]
    pub min_max: Option<AxisDisplayMinMaxRange>,
    #[serde(rename = "DataDriven")]
    pub data_driven: Option<AxisDisplayDataDrivenRange>,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedField {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ScatterPlotConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}
pub type LayoutElementType = String;
#[derive(serde::Serialize, Default)]
pub struct SheetVisualScopingConfiguration {
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "VisualIds")]
    pub visual_ids: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    pub scope: FilterVisualScope,

}

#[derive(serde::Serialize, Default)]
pub struct TotalAggregationComputation {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Value")]
    pub value: MeasureField,

}

#[derive(serde::Serialize, Default)]
pub struct CustomFilterConfiguration {
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "CategoryValue")]
    pub category_value: Option<String>,
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComparisonConfiguration {
    #[serde(rename = "ComparisonMethod")]
    pub comparison_method: Option<ComparisonMethod>,
    #[serde(rename = "ComparisonFormat")]
    pub comparison_format: Option<ComparisonFormatConfiguration>,

}
pub type GeospatialSelectedPointStyle = String;
#[derive(serde::Serialize, Default)]
pub struct GeospatialMapFieldWells {
    #[serde(rename = "GeospatialMapAggregatedFieldWells")]
    pub geospatial_map_aggregated_field_wells: Option<GeospatialMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BarChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapConfiguration {
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GeospatialMapFieldWells>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "PointStyleOptions")]
    pub point_style_options: Option<GeospatialPointStyleOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<GaugeChartConditionalFormattingOption>>,

}
pub type CategoryFilterMatchOperator = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableConfiguration {
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<PivotTableFieldOptions>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<PivotTablePaginatedReportOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PivotTableFieldWells>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<PivotTableTotalOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PivotTableSortConfiguration>,
    #[serde(rename = "TableOptions")]
    pub table_options: Option<PivotTableOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "LinkToDataSetColumn")]
    pub link_to_data_set_column: Option<ColumnIdentifier>,

}
pub type DataLabelPosition = String;
#[derive(serde::Serialize, Default)]
pub struct FilledMapShapeConditionalFormatting {
    #[serde(rename = "Format")]
    pub format: Option<ShapeConditionalFormat>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeHierarchy {
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionURLOperation {
    #[serde(rename = "URLTemplate")]
    pub urltemplate: String,
    #[serde(rename = "URLTarget")]
    pub urltarget: URLTargetConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct NumericAxisOptions {
    #[serde(rename = "Scale")]
    pub scale: Option<AxisScale>,
    #[serde(rename = "Range")]
    pub range: Option<AxisDisplayRange>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartFieldWells {
    #[serde(rename = "PieChartAggregatedFieldWells")]
    pub pie_chart_aggregated_field_wells: Option<PieChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerParameterDeclaration {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<IntegerValueWhenUnsetConfiguration>,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<IntegerDefaultValues>,

}

#[derive(serde::Serialize, Default)]
pub struct LayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: Option<FreeFormLayoutConfiguration>,
    #[serde(rename = "SectionBasedLayout")]
    pub section_based_layout: Option<SectionBasedLayoutConfiguration>,
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: Option<String>,
    #[serde(rename = "ResizeOption")]
    pub resize_option: ResizeOption,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormattingOption {
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<KPIProgressBarConditionalFormatting>,
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<KPIPrimaryValueConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcConfiguration {
    #[serde(rename = "ArcAngle")]
    pub arc_angle: Option<f64>,
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThicknessOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOption {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconDisplayConfiguration {
    #[serde(rename = "IconDisplayOption")]
    pub icon_display_option: Option<ConditionalFormattingIconDisplayOption>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericEqualityFilter {
    #[serde(rename = "MatchOperator")]
    pub match_operator: NumericEqualityMatchOperator,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionAfterPageBreak {
    #[serde(rename = "Status")]
    pub status: Option<SectionPageBreakStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartDataLabelOptions {
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "MeasureDataLabelStyle")]
    pub measure_data_label_style: Option<FunnelChartMeasureDataLabelStyle>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelOptions {
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextFieldControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilterValue {
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartConfiguration {
    #[serde(rename = "BaseSeriesSettings")]
    pub base_series_settings: Option<RadarChartSeriesSettings>,
    #[serde(rename = "AlternateBandEvenColor")]
    pub alternate_band_even_color: Option<String>,
    #[serde(rename = "StartAngle")]
    pub start_angle: Option<f64>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "Shape")]
    pub shape: Option<RadarChartShape>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorAxis")]
    pub color_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "AlternateBandColorsVisibility")]
    pub alternate_band_colors_visibility: Option<Visibility>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "AlternateBandOddColor")]
    pub alternate_band_odd_color: Option<String>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<RadarChartSortConfiguration>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<RadarChartFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,

}
pub type TableBorderStyle = String;pub type NegativeValueDisplayMode = String;
#[derive(serde::Serialize, Default)]
pub struct PanelConfiguration {
    #[serde(rename = "BorderVisibility")]
    pub border_visibility: Option<Visibility>,
    #[serde(rename = "BorderThickness")]
    pub border_thickness: Option<String>,
    #[serde(rename = "BackgroundVisibility")]
    pub background_visibility: Option<Visibility>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "GutterVisibility")]
    pub gutter_visibility: Option<Visibility>,
    #[serde(rename = "BorderColor")]
    pub border_color: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<PanelTitleOptions>,
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<PanelBorderStyle>,
    #[serde(rename = "GutterSpacing")]
    pub gutter_spacing: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormattingOption {
    #[serde(rename = "Row")]
    pub row: Option<TableRowConditionalFormatting>,
    #[serde(rename = "Cell")]
    pub cell: Option<TableCellConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartSortConfiguration {
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct StringDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,

}
pub type FontDecoration = String;
#[derive(serde::Serialize, Default)]
pub struct WordCloudOptions {
    #[serde(rename = "WordPadding")]
    pub word_padding: Option<WordCloudWordPadding>,
    #[serde(rename = "WordScaling")]
    pub word_scaling: Option<WordCloudWordScaling>,
    #[serde(rename = "MaximumStringLength")]
    pub maximum_string_length: Option<f64>,
    #[serde(rename = "WordCasing")]
    pub word_casing: Option<WordCloudWordCasing>,
    #[serde(rename = "WordOrientation")]
    pub word_orientation: Option<WordCloudWordOrientation>,
    #[serde(rename = "CloudLayout")]
    pub cloud_layout: Option<WordCloudCloudLayout>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualPalette {
    #[serde(rename = "ColorMap")]
    pub color_map: Option<Vec<DataPathColor>>,
    #[serde(rename = "ChartColor")]
    pub chart_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<SankeyDiagramChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultPaginatedLayoutConfiguration {
    #[serde(rename = "SectionBased")]
    pub section_based: Option<DefaultSectionBasedLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBorderStyle {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStyleConfiguration {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Pattern")]
    pub pattern: Option<ReferenceLinePatternType>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadingAnimation {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct UnaggregatedField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartSortConfiguration {
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}
pub type DataLabelContent = String;
#[derive(serde::Serialize, Default)]
pub struct WaterfallVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WaterfallChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldWells {
    #[serde(rename = "PivotTableAggregatedFieldWells")]
    pub pivot_table_aggregated_field_wells: Option<PivotTableAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotFieldWells {
    #[serde(rename = "BoxPlotAggregatedFieldWells")]
    pub box_plot_aggregated_field_wells: Option<BoxPlotAggregatedFieldWells>,

}
pub type ReferenceLineLabelHorizontalPosition = String;
#[derive(serde::Serialize, Default)]
pub struct SeriesItem {
    #[serde(rename = "FieldSeriesItem")]
    pub field_series_item: Option<FieldSeriesItem>,
    #[serde(rename = "DataFieldSeriesItem")]
    pub data_field_series_item: Option<DataFieldSeriesItem>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisTickLabelOptions {
    #[serde(rename = "RotationAngle")]
    pub rotation_angle: Option<f64>,
    #[serde(rename = "LabelOptions")]
    pub label_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathColor {
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Element")]
    pub element: DataPathValue,

}

#[derive(serde::Serialize, Default)]
pub struct VisibleRangeOptions {
    #[serde(rename = "PercentRange")]
    pub percent_range: Option<PercentVisibleRange>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextAreaControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}
pub type SheetControlDateTimePickerType = String;pub type PivotTableMetricPlacement = String;
#[derive(serde::Serialize, Default)]
pub struct DonutOptions {
    #[serde(rename = "ArcOptions")]
    pub arc_options: Option<ArcOptions>,
    #[serde(rename = "DonutCenterOptions")]
    pub donut_center_options: Option<DonutCenterOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterControl {
    #[serde(rename = "TextField")]
    pub text_field: Option<FilterTextFieldControl>,
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<FilterDateTimePickerControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<FilterTextAreaControl>,
    #[serde(rename = "Slider")]
    pub slider: Option<FilterSliderControl>,
    #[serde(rename = "RelativeDateTime")]
    pub relative_date_time: Option<FilterRelativeDateTimeControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<FilterDropDownControl>,
    #[serde(rename = "List")]
    pub list: Option<FilterListControl>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ScatterPlotFieldWells>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "YAxisLabelOptions")]
    pub yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,

}

#[derive(serde::Serialize, Default)]
pub struct ThousandSeparatorOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Symbol")]
    pub symbol: Option<NumericSeparatorSymbol>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionPageBreakConfiguration {
    #[serde(rename = "After")]
    pub after: Option<SectionAfterPageBreak>,

}

#[derive(serde::Serialize, Default)]
pub struct MinimumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperationTargetVisualsConfiguration {
    #[serde(rename = "SameSheetTargetVisualConfiguration")]
    pub same_sheet_target_visual_configuration: Option<SameSheetTargetVisualConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalAggregationFunction {
    #[serde(rename = "SimpleNumericalAggregation")]
    pub simple_numerical_aggregation: Option<SimpleNumericalAggregationFunction>,
    #[serde(rename = "PercentileAggregation")]
    pub percentile_aggregation: Option<PercentileAggregation>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSortOptions {
    #[serde(rename = "ColumnSort")]
    pub column_sort: Option<ColumnSort>,
    #[serde(rename = "FieldSort")]
    pub field_sort: Option<FieldSort>,

}

#[derive(serde::Serialize, Default)]
pub struct RowAlternateColorOptions {
    #[serde(rename = "RowAlternateColors")]
    pub row_alternate_colors: Option<Vec<String>>,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetReference {
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    #[serde(rename = "DataSetPlaceholder")]
    pub data_set_placeholder: String,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartFieldWells {
    #[serde(rename = "FunnelChartAggregatedFieldWells")]
    pub funnel_chart_aggregated_field_wells: Option<FunnelChartAggregatedFieldWells>,

}
pub type ConditionalFormattingIconSetType = String;pub type PivotTableSubtotalLevel = String;
#[derive(serde::Serialize, Default)]
pub struct DefaultFreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: FreeFormLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationParameterValueConfiguration {
    #[serde(rename = "SelectAllValueOptions")]
    pub select_all_value_options: Option<SelectAllValueOptions>,
    #[serde(rename = "SourceField")]
    pub source_field: Option<String>,
    #[serde(rename = "CustomValuesConfiguration")]
    pub custom_values_configuration: Option<CustomValuesConfiguration>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KPISortConfiguration {
    #[serde(rename = "TrendGroupSort")]
    pub trend_group_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct ExportWithHiddenFieldsOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDataOptions {
    #[serde(rename = "DateAxisOptions")]
    pub date_axis_options: Option<DateAxisOptions>,
    #[serde(rename = "NumericAxisOptions")]
    pub numeric_axis_options: Option<NumericAxisOptions>,

}
pub type ArcThickness = String;
#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingGradientColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: GradientColor,

}
pub type OtherCategories = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableAggregatedFieldWells {
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomTextContent {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: FontConfiguration,

}
pub type BaseMapStyleType = String;
#[derive(serde::Serialize, Default)]
pub struct BodySectionContent {
    #[serde(rename = "Layout")]
    pub layout: Option<SectionLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOptions {
    #[serde(rename = "DataPathOptions")]
    pub data_path_options: Option<Vec<PivotTableDataPathOption>>,
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<PivotTableFieldOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapFieldWells {
    #[serde(rename = "TreeMapAggregatedFieldWells")]
    pub tree_map_aggregated_field_wells: Option<TreeMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartLineStyleSettings {
    #[serde(rename = "LineInterpolation")]
    pub line_interpolation: Option<LineInterpolation>,
    #[serde(rename = "LineStyle")]
    pub line_style: Option<LineChartLineStyle>,
    #[serde(rename = "LineWidth")]
    pub line_width: Option<String>,
    #[serde(rename = "LineVisibility")]
    pub line_visibility: Option<Visibility>,

}
pub type TableOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct TooltipItem {
    #[serde(rename = "FieldTooltipItem")]
    pub field_tooltip_item: Option<FieldTooltipItem>,
    #[serde(rename = "ColumnTooltipItem")]
    pub column_tooltip_item: Option<ColumnTooltipItem>,

}

#[derive(serde::Serialize, Default)]
pub struct StringValueWhenUnsetConfiguration {
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSelectAllOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TableOptions {
    #[serde(rename = "HeaderStyle")]
    pub header_style: Option<TableCellStyle>,
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<TableOrientation>,
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardSourceEntity {
    #[serde(rename = "SourceTemplate")]
    pub source_template: Option<DashboardSourceTemplate>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GaugeChartFieldWells>,
    #[serde(rename = "TooltipOptions")]
    pub tooltip_options: Option<TooltipOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "GaugeChartOptions")]
    pub gauge_chart_options: Option<GaugeChartOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconCondition {
    #[serde(rename = "IconOptions")]
    pub icon_options: ConditionalFormattingCustomIconOptions,
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DisplayConfiguration")]
    pub display_configuration: Option<ConditionalFormattingIconDisplayConfiguration>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LineSeriesAxisDisplayOptions {
    #[serde(rename = "MissingDataConfigurations")]
    pub missing_data_configurations: Option<Vec<MissingDataConfiguration>>,
    #[serde(rename = "AxisOptions")]
    pub axis_options: Option<AxisDisplayOptions>,

}
pub type RelativeFontSize = String;
#[derive(serde::Serialize, Default)]
pub struct SubtotalOptions {
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "FieldLevel")]
    pub field_level: Option<PivotTableSubtotalLevel>,
    #[serde(rename = "FieldLevelOptions")]
    pub field_level_options: Option<Vec<PivotTableFieldSubtotalOptions>>,
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,

}
pub type FilterVisualScope = String;
#[derive(serde::Serialize, Default)]
pub struct SimpleClusterMarker {
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisDefaults {
    #[serde(rename = "DefaultNewSheetConfiguration")]
    pub default_new_sheet_configuration: DefaultNewSheetConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct CustomParameterValues {
    #[serde(rename = "DecimalValues")]
    pub decimal_values: Option<Vec<f64>>,
    #[serde(rename = "IntegerValues")]
    pub integer_values: Option<Vec<f64>>,
    #[serde(rename = "DateTimeValues")]
    pub date_time_values: Option<Vec<String>>,
    #[serde(rename = "StringValues")]
    pub string_values: Option<Vec<String>>,

}
pub type LegendPosition = String;
#[derive(serde::Serialize, Default)]
pub struct ColorsConfiguration {
    #[serde(rename = "CustomColors")]
    pub custom_colors: Option<Vec<CustomColor>>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<KPIConditionalFormatting>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<KPIConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}
pub type DashboardUIState = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingScope {
    #[serde(rename = "Role")]
    pub role: Option<PivotTableConditionalFormattingScopeRole>,

}
pub type WidgetStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Layout {
    #[serde(rename = "Configuration")]
    pub configuration: LayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialWindowOptions {
    #[serde(rename = "MapZoomMode")]
    pub map_zoom_mode: Option<MapZoomMode>,
    #[serde(rename = "Bounds")]
    pub bounds: Option<GeospatialCoordinateBounds>,

}
pub type NumericSeparatorSymbol = String;
#[derive(serde::Serialize, Default)]
pub struct TotalOptions {
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutCanvasSizeOptions {
    #[serde(rename = "PaperCanvasSizeOptions")]
    pub paper_canvas_size_options: Option<SectionBasedLayoutPaperCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterDeclaration {
    #[serde(rename = "DateTimeParameterDeclaration")]
    pub date_time_parameter_declaration: Option<DateTimeParameterDeclaration>,
    #[serde(rename = "DecimalParameterDeclaration")]
    pub decimal_parameter_declaration: Option<DecimalParameterDeclaration>,
    #[serde(rename = "StringParameterDeclaration")]
    pub string_parameter_declaration: Option<StringParameterDeclaration>,
    #[serde(rename = "IntegerParameterDeclaration")]
    pub integer_parameter_declaration: Option<IntegerParameterDeclaration>,

}
pub type PaperSize = String;
#[derive(serde::Serialize, Default)]
pub struct Entity {
    #[serde(rename = "Path")]
    pub path: Option<String>,

}
pub type PaperOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct PanelTitleOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct CurrencyDisplayFormatConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    pub cell: Option<PivotTableCellConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct Parameters {
    #[serde(rename = "DecimalParameters")]
    pub decimal_parameters: Option<Vec<DecimalParameter>>,
    #[serde(rename = "IntegerParameters")]
    pub integer_parameters: Option<Vec<IntegerParameter>>,
    #[serde(rename = "StringParameters")]
    pub string_parameters: Option<Vec<StringParameter>>,
    #[serde(rename = "DateTimeParameters")]
    pub date_time_parameters: Option<Vec<DateTimeParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericFormatConfiguration {
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "CurrencyDisplayFormatConfiguration")]
    pub currency_display_format_configuration: Option<CurrencyDisplayFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableBorderOptions {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Style")]
    pub style: Option<TableBorderStyle>,

}
pub type SelectedFieldOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTotalOptions {
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct ExplicitHierarchy {
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct MaximumMinimumComputation {
    #[serde(rename = "Type")]
    pub ty: MaximumMinimumComputationType,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Time")]
    pub time: DimensionField,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryFilterConfiguration {
    #[serde(rename = "CustomFilterListConfiguration")]
    pub custom_filter_list_configuration: Option<CustomFilterListConfiguration>,
    #[serde(rename = "CustomFilterConfiguration")]
    pub custom_filter_configuration: Option<CustomFilterConfiguration>,
    #[serde(rename = "FilterListConfiguration")]
    pub filter_list_configuration: Option<FilterListConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct MissingDataConfiguration {
    #[serde(rename = "TreatmentOption")]
    pub treatment_option: Option<MissingDataTreatmentOption>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericSeparatorConfiguration {
    #[serde(rename = "DecimalSeparator")]
    pub decimal_separator: Option<NumericSeparatorSymbol>,
    #[serde(rename = "ThousandsSeparator")]
    pub thousands_separator: Option<ThousandSeparatorOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableDataPathOption {
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "DataPathList")]
    pub data_path_list: Vec<DataPathValue>,

}
pub type ArcThicknessOptions = String;
#[derive(serde::Serialize, Default)]
pub struct DefaultInteractiveLayoutConfiguration {
    #[serde(rename = "Grid")]
    pub grid: Option<DefaultGridLayoutConfiguration>,
    #[serde(rename = "FreeForm")]
    pub free_form: Option<DefaultFreeFormLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartAggregatedFieldWells {
    #[serde(rename = "LineValues")]
    pub line_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "BarValues")]
    pub bar_values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimePickerControlDisplayOptions {
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}
pub type LineChartType = String;
#[derive(serde::Serialize, Default)]
pub struct DashboardVisualPublishOptions {
    #[serde(rename = "ExportHiddenFieldsOption")]
    pub export_hidden_fields_option: Option<ExportHiddenFieldsOption>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BoxPlotChartConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SecondaryValueOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TableAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}
pub type DashboardErrorType = String;pub type TableFieldIconSetType = String;
#[derive(serde::Serialize, Default)]
pub struct TableInlineVisualization {
    #[serde(rename = "DataBars")]
    pub data_bars: Option<DataBarsOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataFieldSeriesItem {
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalDimensionField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLine {
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "DataConfiguration")]
    pub data_configuration: ReferenceLineDataConfiguration,
    #[serde(rename = "LabelConfiguration")]
    pub label_configuration: Option<ReferenceLineLabelConfiguration>,
    #[serde(rename = "StyleConfiguration")]
    pub style_configuration: Option<ReferenceLineStyleConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLabelReferenceOptions {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct DateMeasureField {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<DateAggregationFunction>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,

}

#[derive(serde::Serialize, Default)]
pub struct NullValueFormatConfiguration {
    #[serde(rename = "NullString")]
    pub null_string: String,

}
pub type DashboardBehavior = String;
#[derive(serde::Serialize, Default)]
pub struct ReferenceLineValueLabelConfiguration {
    #[serde(rename = "RelativePosition")]
    pub relative_position: Option<ReferenceLineValueLabelRelativePosition>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SetParameterValueConfiguration {
    #[serde(rename = "Value")]
    pub value: DestinationParameterValueConfiguration,
    #[serde(rename = "DestinationParameterName")]
    pub destination_parameter_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Spacing {
    #[serde(rename = "Top")]
    pub top: Option<String>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<String>,
    #[serde(rename = "Left")]
    pub left: Option<String>,
    #[serde(rename = "Right")]
    pub right: Option<String>,

}
pub type TableCellImageScalingConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct TooltipOptions {
    #[serde(rename = "FieldBasedTooltip")]
    pub field_based_tooltip: Option<FieldBasedTooltip>,
    #[serde(rename = "TooltipVisibility")]
    pub tooltip_visibility: Option<Visibility>,
    #[serde(rename = "SelectedTooltipType")]
    pub selected_tooltip_type: Option<SelectedTooltipType>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: String,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLabelOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "ApplyTo")]
    pub apply_to: Option<AxisLabelReferenceOptions>,

}
pub type RadarChartShape = String;
#[derive(serde::Serialize, Default)]
pub struct FontConfiguration {
    #[serde(rename = "FontSize")]
    pub font_size: Option<FontSize>,
    #[serde(rename = "FontStyle")]
    pub font_style: Option<FontStyle>,
    #[serde(rename = "FontDecoration")]
    pub font_decoration: Option<FontDecoration>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "FontWeight")]
    pub font_weight: Option<FontWeight>,

}
pub type ReferenceLineValueLabelRelativePosition = String;
#[derive(serde::Serialize, Default)]
pub struct NumericEqualityDrillDownFilter {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldWells {
    #[serde(rename = "TableUnaggregatedFieldWells")]
    pub table_unaggregated_field_wells: Option<TableUnaggregatedFieldWells>,
    #[serde(rename = "TableAggregatedFieldWells")]
    pub table_aggregated_field_wells: Option<TableAggregatedFieldWells>,

}
pub type ColumnRole = String;
#[derive(serde::Serialize, Default)]
pub struct MaximumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<PivotTableConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIcon {
    #[serde(rename = "CustomCondition")]
    pub custom_condition: Option<ConditionalFormattingCustomIconCondition>,
    #[serde(rename = "IconSet")]
    pub icon_set: Option<ConditionalFormattingIconSet>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartDefaultSeriesSettings {
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,

}
pub type PrimaryValueDisplayType = String;pub type ReferenceLinePatternType = String;
#[derive(serde::Serialize, Default)]
pub struct CategoryDrillDownFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "CategoryValues")]
    pub category_values: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIPrimaryValueConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}
pub type NumberScale = String;
#[derive(serde::Serialize, Default)]
pub struct CascadingControlConfiguration {
    #[serde(rename = "SourceControls")]
    pub source_controls: Option<Vec<CascadingControlSource>>,

}
pub type DataLabelOverlap = String;
#[derive(serde::Serialize, Default)]
pub struct IntegerValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct DateDimensionField {
    #[serde(rename = "DateGranularity")]
    pub date_granularity: Option<TimeGranularity>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramFieldWells {
    #[serde(rename = "SankeyDiagramAggregatedFieldWells")]
    pub sankey_diagram_aggregated_field_wells: Option<SankeyDiagramAggregatedFieldWells>,

}
pub type SimpleNumericalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct DashboardError {
    #[serde(rename = "Type")]
    pub ty: Option<DashboardErrorType>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "ViolatedEntities")]
    pub violated_entities: Option<Vec<Entity>>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalDimensionField {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}
pub type SortDirection = String;
#[derive(serde::Serialize, Default)]
pub struct AxisDisplayMinMaxRange {
    #[serde(rename = "Minimum")]
    pub minimum: Option<f64>,
    #[serde(rename = "Maximum")]
    pub maximum: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DimensionField {
    #[serde(rename = "NumericalDimensionField")]
    pub numerical_dimension_field: Option<NumericalDimensionField>,
    #[serde(rename = "CategoricalDimensionField")]
    pub categorical_dimension_field: Option<CategoricalDimensionField>,
    #[serde(rename = "DateDimensionField")]
    pub date_dimension_field: Option<DateDimensionField>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<GridLayoutCanvasSizeOptions>,
    #[serde(rename = "Elements")]
    pub elements: Vec<GridLayoutElement>,

}
pub type MaximumMinimumComputationType = String;pub type WordCloudWordScaling = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableSortConfiguration {
    #[serde(rename = "FieldSortOptions")]
    pub field_sort_options: Option<Vec<PivotFieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartFieldWells {
    #[serde(rename = "RadarChartAggregatedFieldWells")]
    pub radar_chart_aggregated_field_wells: Option<RadarChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOptions {
    #[serde(rename = "Order")]
    pub order: Option<Vec<String>>,
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<TableFieldOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisScale {
    #[serde(rename = "Linear")]
    pub linear: Option<AxisLinearScale>,
    #[serde(rename = "Logarithmic")]
    pub logarithmic: Option<AxisLogarithmicScale>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathValue {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FieldValue")]
    pub field_value: String,

}
pub type TopBottomSortOrder = String;pub type BarsArrangement = String;
#[derive(serde::Serialize, Default)]
pub struct TablePaginatedReportOptions {
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct AdHocFilteringOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTablePaginatedReportOptions {
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,

}
pub type ResizeOption = String;
#[derive(serde::Serialize, Default)]
pub struct TreeMapConfiguration {
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "GroupLabelOptions")]
    pub group_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TreeMapFieldWells>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TreeMapSortConfiguration>,
    #[serde(rename = "SizeLabelOptions")]
    pub size_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightConfiguration {
    #[serde(rename = "Computations")]
    pub computations: Option<Vec<Computation>>,
    #[serde(rename = "CustomNarrative")]
    pub custom_narrative: Option<CustomNarrativeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetElementRenderingRule {
    #[serde(rename = "ConfigurationOverrides")]
    pub configuration_overrides: SheetElementConfigurationOverrides,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExportToCSVOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}
pub type LineChartLineStyle = String;
#[derive(serde::Serialize, Default)]
pub struct GeospatialPointStyleOptions {
    #[serde(rename = "SelectedPointStyle")]
    pub selected_point_style: Option<GeospatialSelectedPointStyle>,
    #[serde(rename = "ClusterMarkerConfiguration")]
    pub cluster_marker_configuration: Option<ClusterMarkerConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SelectedSheetsFilterScopeConfiguration {
    #[serde(rename = "SheetVisualScopingConfigurations")]
    pub sheet_visual_scoping_configurations: Option<Vec<SheetVisualScopingConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkContentConfiguration {
    #[serde(rename = "CustomTextContent")]
    pub custom_text_content: Option<TableFieldCustomTextContent>,
    #[serde(rename = "CustomIconContent")]
    pub custom_icon_content: Option<TableFieldCustomIconContent>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HistogramConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisConfiguration {
    #[serde(rename = "Range")]
    pub range: Option<ArcAxisDisplayRange>,
    #[serde(rename = "ReserveRange")]
    pub reserve_range: Option<f64>,

}
pub type SelectAllValueOptions = String;pub type ConditionalFormattingIconDisplayOption = String;
#[derive(serde::Serialize, Default)]
pub struct GaugeChartVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GaugeChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<GaugeChartConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct StringParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<StringDefaultValues>,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<StringValueWhenUnsetConfiguration>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellStyle {
    #[serde(rename = "TextWrap")]
    pub text_wrap: Option<TextWrap>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "VerticalTextAlignment")]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
    #[serde(rename = "Height")]
    pub height: Option<f64>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Border")]
    pub border: Option<GlobalTableBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudFieldWells {
    #[serde(rename = "WordCloudAggregatedFieldWells")]
    pub word_cloud_aggregated_field_wells: Option<WordCloudAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct DropDownControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableSideBorderOptions {
    #[serde(rename = "Right")]
    pub right: Option<TableBorderOptions>,
    #[serde(rename = "Top")]
    pub top: Option<TableBorderOptions>,
    #[serde(rename = "Left")]
    pub left: Option<TableBorderOptions>,
    #[serde(rename = "InnerVertical")]
    pub inner_vertical: Option<TableBorderOptions>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<TableBorderOptions>,
    #[serde(rename = "InnerHorizontal")]
    pub inner_horizontal: Option<TableBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartFieldWells {
    #[serde(rename = "WaterfallChartAggregatedFieldWells")]
    pub waterfall_chart_aggregated_field_wells: Option<WaterfallChartAggregatedFieldWells>,

}
pub type WordCloudCloudLayout = String;
#[derive(serde::Serialize, Default)]
pub struct MappedDataSetParameter {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "DataSetParameterName")]
    pub data_set_parameter_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SmallMultiplesOptions {
    #[serde(rename = "MaxVisibleColumns")]
    pub max_visible_columns: Option<f64>,
    #[serde(rename = "MaxVisibleRows")]
    pub max_visible_rows: Option<f64>,
    #[serde(rename = "PanelConfiguration")]
    pub panel_configuration: Option<PanelConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeEqualityFilter {
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutPaperCanvasSizeOptions {
    #[serde(rename = "PaperOrientation")]
    pub paper_orientation: Option<PaperOrientation>,
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<PaperSize>,
    #[serde(rename = "PaperMargin")]
    pub paper_margin: Option<Spacing>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconSet {
    #[serde(rename = "IconSetType")]
    pub icon_set_type: Option<ConditionalFormattingIconSetType>,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct SheetDefinition {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParameterControls")]
    pub parameter_controls: Option<Vec<ParameterControl>>,
    #[serde(rename = "FilterControls")]
    pub filter_controls: Option<Vec<FilterControl>>,
    #[serde(rename = "SheetControlLayouts")]
    pub sheet_control_layouts: Option<Vec<SheetControlLayout>>,
    #[serde(rename = "TextBoxes")]
    pub text_boxes: Option<Vec<SheetTextBox>>,
    #[serde(rename = "Layouts")]
    pub layouts: Option<Vec<Layout>>,
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Visuals")]
    pub visuals: Option<Vec<Visual>>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<SheetContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartConfiguration {
    #[serde(rename = "Orientation")]
    pub orientation: Option<BarChartOrientation>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BarChartSortConfiguration>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BarChartFieldWells>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ValueAxis")]
    pub value_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardSourceTemplate {
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,
    #[serde(rename = "Arn")]
    pub arn: String,

}
pub type NumericEqualityMatchOperator = String;
#[derive(serde::Serialize, Default)]
pub struct StringFormatConfiguration {
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FilledMapFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FilledMapSortConfiguration>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartFieldWells {
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct DonutCenterOptions {
    #[serde(rename = "LabelVisibility")]
    pub label_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<FreeFormLayoutCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSearchOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetControlsOption {
    #[serde(rename = "VisibilityState")]
    pub visibility_state: Option<DashboardUIState>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderFooterSectionConfiguration {
    #[serde(rename = "SectionId")]
    pub section_id: String,
    #[serde(rename = "Layout")]
    pub layout: SectionLayoutConfiguration,
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Values")]
    pub values: Vec<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<FreeFormLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HeatMapConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}
pub type LineChartMarkerShape = String;
#[derive(serde::Serialize, Default)]
pub struct ChartAxisLabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "SortIconVisibility")]
    pub sort_icon_visibility: Option<Visibility>,
    #[serde(rename = "AxisLabelOptions")]
    pub axis_label_options: Option<Vec<AxisLabelOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotOptions {
    #[serde(rename = "AllDataPointsVisibility")]
    pub all_data_points_visibility: Option<Visibility>,
    #[serde(rename = "StyleOptions")]
    pub style_options: Option<BoxPlotStyleOptions>,
    #[serde(rename = "OutlierVisibility")]
    pub outlier_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartFieldWells {
    #[serde(rename = "LineChartAggregatedFieldWells")]
    pub line_chart_aggregated_field_wells: Option<LineChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldSubtotalOptions {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialCoordinateBounds {
    #[serde(rename = "North")]
    pub north: f64,
    #[serde(rename = "South")]
    pub south: f64,
    #[serde(rename = "East")]
    pub east: f64,
    #[serde(rename = "West")]
    pub west: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PredefinedHierarchy {
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct ShortFormatText {
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartMarkerStyleSettings {
    #[serde(rename = "MarkerSize")]
    pub marker_size: Option<String>,
    #[serde(rename = "MarkerVisibility")]
    pub marker_visibility: Option<Visibility>,
    #[serde(rename = "MarkerColor")]
    pub marker_color: Option<String>,
    #[serde(rename = "MarkerShape")]
    pub marker_shape: Option<LineChartMarkerShape>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionSetParametersOperation {
    #[serde(rename = "ParameterValueConfigurations")]
    pub parameter_value_configurations: Vec<SetParameterValueConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<TableConditionalFormattingOption>>,

}
pub type WordCloudWordOrientation = String;pub type CustomContentImageScalingConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct RangeEndsLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperationSelectedFieldsConfiguration {
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<SelectedFieldOptions>,
    #[serde(rename = "SelectedFields")]
    pub selected_fields: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomContentConfiguration {
    #[serde(rename = "ContentUrl")]
    pub content_url: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<CustomContentType>,
    #[serde(rename = "ImageScaling")]
    pub image_scaling: Option<CustomContentImageScalingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct BinWidthOptions {
    #[serde(rename = "BinCountLimit")]
    pub bin_count_limit: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterListConfiguration {
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableTotalOptions {
    #[serde(rename = "ColumnSubtotalOptions")]
    pub column_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "RowTotalOptions")]
    pub row_total_options: Option<PivotTotalOptions>,
    #[serde(rename = "RowSubtotalOptions")]
    pub row_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "ColumnTotalOptions")]
    pub column_total_options: Option<PivotTotalOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramSortConfiguration {
    #[serde(rename = "WeightSort")]
    pub weight_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SourceItemsLimit")]
    pub source_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "DestinationItemsLimit")]
    pub destination_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "SearchOptions")]
    pub search_options: Option<ListControlSearchOptions>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualAxisSortOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,
    #[serde(rename = "Scope")]
    pub scope: Option<PivotTableConditionalFormattingScope>,

}

#[derive(serde::Serialize, Default)]
pub struct RollingDateConfiguration {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: String,

}
pub type ValueWhenUnsetOption = String;
#[derive(serde::Serialize, Default)]
pub struct DataPathSort {
    #[serde(rename = "Direction")]
    pub direction: SortDirection,
    #[serde(rename = "SortPaths")]
    pub sort_paths: Vec<DataPathValue>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GeospatialMapConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}
pub type ParameterValueType = String;
#[derive(serde::Serialize, Default)]
pub struct FilterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}
pub type FontStyle = String;
#[derive(serde::Serialize, Default)]
pub struct BodySectionConfiguration {
    #[serde(rename = "Content")]
    pub content: BodySectionContent,
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,
    #[serde(rename = "SectionId")]
    pub section_id: String,
    #[serde(rename = "PageBreakConfiguration")]
    pub page_break_configuration: Option<SectionPageBreakConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct EmptyVisual {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Principal")]
    pub principal: String,
    #[serde(rename = "Resource")]
    pub resource: Option<String>,
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetIdentifierDeclaration {
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    #[serde(rename = "Identifier")]
    pub identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapFieldWells {
    #[serde(rename = "HeatMapAggregatedFieldWells")]
    pub heat_map_aggregated_field_wells: Option<HeatMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIFieldWells {
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "TrendGroups")]
    pub trend_groups: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct AnchorDateConfiguration {
    #[serde(rename = "AnchorOption")]
    pub anchor_option: Option<AnchorOption>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartConfiguration {
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<LineChartType>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    pub default_series_settings: Option<LineChartDefaultSeriesSettings>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<LineChartFieldWells>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<LineChartSortConfiguration>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ForecastConfigurations")]
    pub forecast_configurations: Option<Vec<ForecastConfiguration>>,
    #[serde(rename = "Series")]
    pub series: Option<Vec<SeriesItem>>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterRelativeDateTimeControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramConfiguration {
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "BinOptions")]
    pub bin_options: Option<HistogramBinOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HistogramFieldWells>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ShapeConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    pub background_color: ConditionalFormattingColor,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultSectionBasedLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct RelativeDateTimeControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TableConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<TableConditionalFormatting>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetElementConfigurationOverrides {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct BinCountOptions {
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<FilledMapConditionalFormatting>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FilledMapConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastConfiguration {
    #[serde(rename = "ForecastProperties")]
    pub forecast_properties: Option<TimeBasedForecastProperties>,
    #[serde(rename = "Scenario")]
    pub scenario: Option<ForecastScenario>,

}

#[derive(serde::Serialize, Default)]
pub struct GradientStop {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,
    #[serde(rename = "GradientOffset")]
    pub gradient_offset: f64,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PieChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}

#[derive(serde::Serialize, Default)]
pub struct ScrollBarOptions {
    #[serde(rename = "VisibleRange")]
    pub visible_range: Option<VisibleRangeOptions>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type FunnelChartMeasureDataLabelStyle = String;
#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "TimeEqualityFilter")]
    pub time_equality_filter: Option<TimeEqualityFilter>,
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeFilter>,
    #[serde(rename = "RelativeDatesFilter")]
    pub relative_dates_filter: Option<RelativeDatesFilter>,
    #[serde(rename = "TopBottomFilter")]
    pub top_bottom_filter: Option<TopBottomFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityFilter>,
    #[serde(rename = "NumericRangeFilter")]
    pub numeric_range_filter: Option<NumericRangeFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormattingOption {
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<GaugeChartPrimaryValueConditionalFormatting>,
    #[serde(rename = "Arc")]
    pub arc: Option<GaugeChartArcConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct GradientColor {
    #[serde(rename = "Stops")]
    pub stops: Option<Vec<GradientStop>>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetTextBox {
    #[serde(rename = "SheetTextBoxId")]
    pub sheet_text_box_id: String,
    #[serde(rename = "Content")]
    pub content: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableUnaggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<UnaggregatedField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomMoversComputation {
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,
    #[serde(rename = "MoverSize")]
    pub mover_size: Option<f64>,
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<TopBottomSortOrder>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartOptions {
    #[serde(rename = "TotalBarLabel")]
    pub total_bar_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartOptions {
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "ArcAxis")]
    pub arc_axis: Option<ArcAxisConfiguration>,
    #[serde(rename = "Arc")]
    pub arc: Option<ArcConfiguration>,
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeParameterDeclaration {
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DateTimeValueWhenUnsetConfiguration>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DateTimeDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkConfiguration {
    #[serde(rename = "Target")]
    pub target: URLTargetConfiguration,
    #[serde(rename = "Content")]
    pub content: TableFieldLinkContentConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilter {
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: Option<NumericRangeFilterValue>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: Option<NumericRangeFilterValue>,

}

#[derive(serde::Serialize, Default)]
pub struct WhatIfPointScenario {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Date")]
    pub date: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterDropDownControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}
pub type CategoricalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct GaugeChartPrimaryValueConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}
pub type SheetControlListType = String;
#[derive(serde::Serialize, Default)]
pub struct ClusterMarkerConfiguration {
    #[serde(rename = "ClusterMarker")]
    pub cluster_marker: Option<ClusterMarker>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingColor {
    #[serde(rename = "Solid")]
    pub solid: Option<ConditionalFormattingSolidColor>,
    #[serde(rename = "Gradient")]
    pub gradient: Option<ConditionalFormattingGradientColor>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterDropDownControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellImageSizingConfiguration {
    #[serde(rename = "TableCellImageScalingConfiguration")]
    pub table_cell_image_scaling_configuration: Option<TableCellImageScalingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomIconContent {
    #[serde(rename = "Icon")]
    pub icon: Option<TableFieldIconSetType>,

}

#[derive(serde::Serialize, Default)]
pub struct TrendArrowOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudChartConfiguration {
    #[serde(rename = "WordCloudOptions")]
    pub word_cloud_options: Option<WordCloudOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WordCloudSortConfiguration>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WordCloudFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct SliderControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DateAxisOptions {
    #[serde(rename = "MissingDateVisibility")]
    pub missing_date_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterDateTimePickerControl {
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlDateTimePickerType>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}
pub type SheetControlSliderType = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterDateTimePickerControl {
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalDefaultValues {
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,

}

#[derive(serde::Serialize, Default)]
pub struct TextConditionalFormat {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}

#[derive(serde::Serialize, Default)]
pub struct TextControlPlaceholderOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterMarker {
    #[serde(rename = "SimpleClusterMarker")]
    pub simple_cluster_marker: Option<SimpleClusterMarker>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionStyle {
    #[serde(rename = "Height")]
    pub height: Option<String>,
    #[serde(rename = "Padding")]
    pub padding: Option<Spacing>,

}
pub type ForecastComputationSeasonality = String;
#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingSolidColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormSectionLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartArcConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct PeriodToDateComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "PeriodTimeGranularity")]
    pub period_time_granularity: Option<TimeGranularity>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalParameterDeclaration {
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DecimalDefaultValues>,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DecimalValueWhenUnsetConfiguration>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,

}
pub type HorizontalTextAlignment = String;
#[derive(serde::Serialize, Default)]
pub struct MetricComparisonComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "TargetValue")]
    pub target_value: MeasureField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "FromValue")]
    pub from_value: MeasureField,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnTooltipItem {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Aggregation")]
    pub aggregation: Option<AggregationFunction>,
    #[serde(rename = "Label")]
    pub label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<KPISortConfiguration>,
    #[serde(rename = "KPIOptions")]
    pub kpioptions: Option<KPIOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<KPIFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomFilterListConfiguration {
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapStyleOptions {
    #[serde(rename = "BaseMapStyle")]
    pub base_map_style: Option<BaseMapStyleType>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartSeriesSettings {
    #[serde(rename = "AreaStyleSettings")]
    pub area_style_settings: Option<RadarChartAreaStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotFieldWells {
    #[serde(rename = "ScatterPlotCategoricallyAggregatedFieldWells")]
    pub scatter_plot_categorically_aggregated_field_wells: Option<ScatterPlotCategoricallyAggregatedFieldWells>,
    #[serde(rename = "ScatterPlotUnaggregatedFieldWells")]
    pub scatter_plot_unaggregated_field_wells: Option<ScatterPlotUnaggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Visual {
    #[serde(rename = "BoxPlotVisual")]
    pub box_plot_visual: Option<BoxPlotVisual>,
    #[serde(rename = "LineChartVisual")]
    pub line_chart_visual: Option<LineChartVisual>,
    #[serde(rename = "HeatMapVisual")]
    pub heat_map_visual: Option<HeatMapVisual>,
    #[serde(rename = "TreeMapVisual")]
    pub tree_map_visual: Option<TreeMapVisual>,
    #[serde(rename = "FilledMapVisual")]
    pub filled_map_visual: Option<FilledMapVisual>,
    #[serde(rename = "EmptyVisual")]
    pub empty_visual: Option<EmptyVisual>,
    #[serde(rename = "InsightVisual")]
    pub insight_visual: Option<InsightVisual>,
    #[serde(rename = "FunnelChartVisual")]
    pub funnel_chart_visual: Option<FunnelChartVisual>,
    #[serde(rename = "CustomContentVisual")]
    pub custom_content_visual: Option<CustomContentVisual>,
    #[serde(rename = "RadarChartVisual")]
    pub radar_chart_visual: Option<RadarChartVisual>,
    #[serde(rename = "GaugeChartVisual")]
    pub gauge_chart_visual: Option<GaugeChartVisual>,
    #[serde(rename = "WordCloudVisual")]
    pub word_cloud_visual: Option<WordCloudVisual>,
    #[serde(rename = "PieChartVisual")]
    pub pie_chart_visual: Option<PieChartVisual>,
    #[serde(rename = "ScatterPlotVisual")]
    pub scatter_plot_visual: Option<ScatterPlotVisual>,
    #[serde(rename = "GeospatialMapVisual")]
    pub geospatial_map_visual: Option<GeospatialMapVisual>,
    #[serde(rename = "BarChartVisual")]
    pub bar_chart_visual: Option<BarChartVisual>,
    #[serde(rename = "KPIVisual")]
    pub kpivisual: Option<KPIVisual>,
    #[serde(rename = "TableVisual")]
    pub table_visual: Option<TableVisual>,
    #[serde(rename = "ComboChartVisual")]
    pub combo_chart_visual: Option<ComboChartVisual>,
    #[serde(rename = "PivotTableVisual")]
    pub pivot_table_visual: Option<PivotTableVisual>,
    #[serde(rename = "SankeyDiagramVisual")]
    pub sankey_diagram_visual: Option<SankeyDiagramVisual>,
    #[serde(rename = "HistogramVisual")]
    pub histogram_visual: Option<HistogramVisual>,
    #[serde(rename = "WaterfallVisual")]
    pub waterfall_visual: Option<WaterfallVisual>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualSubtitleLabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FormatText")]
    pub format_text: Option<LongFormatText>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionNavigationOperation {
    #[serde(rename = "LocalNavigationConfiguration")]
    pub local_navigation_configuration: Option<LocalNavigationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PivotTableConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<PivotTableConditionalFormatting>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeBasedForecastProperties {
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<f64>,
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalParameter {
    #[serde(rename = "Values")]
    pub values: Vec<f64>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterScopeConfiguration {
    #[serde(rename = "SelectedSheets")]
    pub selected_sheets: Option<SelectedSheetsFilterScopeConfiguration>,

}
pub type CustomContentType = String;pub type BarChartOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct FilterTextAreaControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldImageConfiguration {
    #[serde(rename = "SizingOptions")]
    pub sizing_options: Option<TableCellImageSizingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapSortConfiguration {
    #[serde(rename = "TreeMapSort")]
    pub tree_map_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "TreeMapGroupItemsLimitConfiguration")]
    pub tree_map_group_items_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightVisual {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "InsightConfiguration")]
    pub insight_configuration: Option<InsightConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardVersion {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "DataSetArns")]
    pub data_set_arns: Option<Vec<String>>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<DashboardError>>,
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<Sheet>>,
    #[serde(rename = "SourceEntityArn")]
    pub source_entity_arn: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<f64>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramFieldWells {
    #[serde(rename = "HistogramAggregatedFieldWells")]
    pub histogram_aggregated_field_wells: Option<HistogramAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartAggregatedFieldWells {
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedMeasureField {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct MeasureField {
    #[serde(rename = "CalculatedMeasureField")]
    pub calculated_measure_field: Option<CalculatedMeasureField>,
    #[serde(rename = "CategoricalMeasureField")]
    pub categorical_measure_field: Option<CategoricalMeasureField>,
    #[serde(rename = "DateMeasureField")]
    pub date_measure_field: Option<DateMeasureField>,
    #[serde(rename = "NumericalMeasureField")]
    pub numerical_measure_field: Option<NumericalMeasureField>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterSliderControl {
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,

}
pub type URLTargetConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct RadarChartAreaStyleSettings {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetControlLayoutConfiguration {
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnConfiguration {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,
    #[serde(rename = "ColorsConfiguration")]
    pub colors_configuration: Option<ColorsConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Role")]
    pub role: Option<ColumnRole>,

}

#[derive(serde::Serialize, Default)]
pub struct CascadingControlSource {
    #[serde(rename = "SourceSheetControlId")]
    pub source_sheet_control_id: Option<String>,
    #[serde(rename = "ColumnToMatch")]
    pub column_to_match: Option<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalMeasureField {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<CategoricalAggregationFunction>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GrowthRateComputation {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "PeriodSize")]
    pub period_size: Option<f64>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBackgroundStyle {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type MissingDataTreatmentOption = String;
#[derive(serde::Serialize, Default)]
pub struct Sheet {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SheetId")]
    pub sheet_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconOptions {
    #[serde(rename = "Icon")]
    pub icon: Option<Icon>,
    #[serde(rename = "UnicodeIcon")]
    pub unicode_icon: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeDefaultValues {
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}

#[derive(serde::Serialize, Default)]
pub struct FontWeight {
    #[serde(rename = "Name")]
    pub name: Option<FontWeightName>,

}

#[derive(serde::Serialize, Default)]
pub struct PercentageDisplayFormatConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDataConfiguration {
    #[serde(rename = "DynamicConfiguration")]
    pub dynamic_configuration: Option<ReferenceLineDynamicDataConfiguration>,
    #[serde(rename = "StaticConfiguration")]
    pub static_configuration: Option<ReferenceLineStaticDataConfiguration>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,

}
pub type FilterNullOption = String;pub type MapZoomMode = String;
#[derive(serde::Serialize, Default)]
pub struct TableSortConfiguration {
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,
    #[serde(rename = "RowSort")]
    pub row_sort: Option<Vec<FieldSortOptions>>,

}
pub type CrossDatasetTypes = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableOptions {
    #[serde(rename = "MetricPlacement")]
    pub metric_placement: Option<PivotTableMetricPlacement>,
    #[serde(rename = "RowHeaderStyle")]
    pub row_header_style: Option<TableCellStyle>,
    #[serde(rename = "ToggleButtonsVisibility")]
    pub toggle_buttons_visibility: Option<Visibility>,
    #[serde(rename = "ColumnHeaderStyle")]
    pub column_header_style: Option<TableCellStyle>,
    #[serde(rename = "SingleMetricVisibility")]
    pub single_metric_visibility: Option<Visibility>,
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "ColumnNamesVisibility")]
    pub column_names_visibility: Option<Visibility>,
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "RowFieldNamesStyle")]
    pub row_field_names_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct DataColor {
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLogarithmicScale {
    #[serde(rename = "Base")]
    pub base: Option<f64>,

}
pub type TimeGranularity = String;
#[derive(serde::Serialize, Default)]
pub struct FormatConfiguration {
    #[serde(rename = "StringFormatConfiguration")]
    pub string_format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "DateTimeFormatConfiguration")]
    pub date_time_format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "NumberFormatConfiguration")]
    pub number_format_configuration: Option<NumberFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct StringParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSort {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Direction")]
    pub direction: SortDirection,

}

#[derive(serde::Serialize, Default)]
pub struct ExportHiddenFieldsOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapFieldWells {
    #[serde(rename = "FilledMapAggregatedFieldWells")]
    pub filled_map_aggregated_field_wells: Option<FilledMapAggregatedFieldWells>,

}
pub type SectionPageBreakStatus = String;pub type ResourceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct FilterGroup {
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "CrossDataset")]
    pub cross_dataset: CrossDatasetTypes,
    #[serde(rename = "FilterGroupId")]
    pub filter_group_id: String,
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
    #[serde(rename = "ScopeConfiguration")]
    pub scope_configuration: FilterScopeConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct DataPointDrillUpDownOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}
pub type SelectedTooltipType = String;
#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStaticDataConfiguration {
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct UniqueValuesComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Category")]
    pub category: DimensionField,

}

#[derive(serde::Serialize, Default)]
pub struct VisualMenuOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDynamicDataConfiguration {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "MeasureAggregationFunction")]
    pub measure_aggregation_function: AggregationFunction,
    #[serde(rename = "Calculation")]
    pub calculation: NumericalAggregationFunction,

}
pub type CategoryFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct FunnelChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}
pub type ReferenceLineLabelVerticalPosition = String;pub type LineInterpolation = String;pub type VisualCustomActionTrigger = String;
#[derive(serde::Serialize, Default)]
pub struct VisualTitleLabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FormatText")]
    pub format_text: Option<ShortFormatText>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<LineChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct LocalNavigationConfiguration {
    #[serde(rename = "TargetSheetId")]
    pub target_sheet_id: String,

}
pub type VerticalTextAlignment = String;
#[derive(serde::Serialize, Default)]
pub struct DataPathLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramBinOptions {
    #[serde(rename = "StartValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "SelectedBinType")]
    pub selected_bin_type: Option<HistogramBinType>,
    #[serde(rename = "BinWidth")]
    pub bin_width: Option<BinWidthOptions>,
    #[serde(rename = "BinCount")]
    pub bin_count: Option<BinCountOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayOptions {
    #[serde(rename = "TickLabelOptions")]
    pub tick_label_options: Option<AxisTickLabelOptions>,
    #[serde(rename = "GridLineVisibility")]
    pub grid_line_visibility: Option<Visibility>,
    #[serde(rename = "AxisLineVisibility")]
    pub axis_line_visibility: Option<Visibility>,
    #[serde(rename = "DataOptions")]
    pub data_options: Option<AxisDataOptions>,
    #[serde(rename = "ScrollbarOptions")]
    pub scrollbar_options: Option<ScrollBarOptions>,
    #[serde(rename = "AxisOffset")]
    pub axis_offset: Option<String>,

}
pub type PivotTableConditionalFormattingScopeRole = String;
#[derive(serde::Serialize, Default)]
pub struct PieChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultGridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: GridLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLinearScale {
    #[serde(rename = "StepSize")]
    pub step_size: Option<f64>,
    #[serde(rename = "StepCount")]
    pub step_count: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartFieldWells {
    #[serde(rename = "ComboChartAggregatedFieldWells")]
    pub combo_chart_aggregated_field_wells: Option<ComboChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct ComparisonFormatConfiguration {
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct NegativeValueConfiguration {
    #[serde(rename = "DisplayMode")]
    pub display_mode: NegativeValueDisplayMode,

}

#[derive(serde::Serialize, Default)]
pub struct SectionLayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: FreeFormSectionLayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct DrillDownFilter {
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityDrillDownFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryDrillDownFilter>,
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeDrillDownFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramAggregatedFieldWells {
    #[serde(rename = "Weight")]
    pub weight: Option<Vec<MeasureField>>,
    #[serde(rename = "Source")]
    pub source: Option<Vec<DimensionField>>,
    #[serde(rename = "Destination")]
    pub destination: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutElement {
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "ColumnSpan")]
    pub column_span: f64,
    #[serde(rename = "ColumnIndex")]
    pub column_index: Option<f64>,
    #[serde(rename = "RowIndex")]
    pub row_index: Option<f64>,
    #[serde(rename = "RowSpan")]
    pub row_span: f64,

}

#[derive(serde::Serialize, Default)]
pub struct FilterSliderControl {
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlSliderType>,
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<RadarChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Color")]
    pub color: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "RangeMaximumValue")]
    pub range_maximum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "RangeMinimumValue")]
    pub range_minimum_value: Option<TimeRangeFilterValue>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionFilterOperation {
    #[serde(rename = "TargetVisualsConfiguration")]
    pub target_visuals_configuration: FilterOperationTargetVisualsConfiguration,
    #[serde(rename = "SelectedFieldsConfiguration")]
    pub selected_fields_configuration: FilterOperationSelectedFieldsConfiguration,

}
pub type DateAggregationFunction = String;pub type SpecialValue = String;
#[derive(serde::Serialize, Default)]
pub struct BoxPlotStyleOptions {
    #[serde(rename = "FillStyle")]
    pub fill_style: Option<BoxPlotFillStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotChartConfiguration {
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BoxPlotSortConfiguration>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BoxPlotFieldWells>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "BoxPlotOptions")]
    pub box_plot_options: Option<BoxPlotOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableRowConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualCustomActionOperation {
    #[serde(rename = "NavigationOperation")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<CustomActionFilterOperation>,
    #[serde(rename = "URLOperation")]
    pub urloperation: Option<CustomActionURLOperation>,
    #[serde(rename = "SetParametersOperation")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardVersionDefinition {
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "ParameterDeclarations")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "CalculatedFields")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "DataSetIdentifierDeclarations")]
    pub data_set_identifier_declarations: Vec<DataSetIdentifierDeclaration>,
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "ColumnConfigurations")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "AnalysisDefaults")]
    pub analysis_defaults: Option<AnalysisDefaults>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartConfiguration {
    #[serde(rename = "WaterfallChartOptions")]
    pub waterfall_chart_options: Option<WaterfallChartOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WaterfallChartFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WaterfallChartSortConfiguration>,
    #[serde(rename = "CategoryAxisLabelOptions")]
    pub category_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryAxisDisplayOptions")]
    pub category_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}
pub type NumericFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PaginationConfiguration {
    #[serde(rename = "PageNumber")]
    pub page_number: f64,
    #[serde(rename = "PageSize")]
    pub page_size: f64,

}

#[derive(serde::Serialize, Default)]
pub struct Computation {
    #[serde(rename = "MaximumMinimum")]
    pub maximum_minimum: Option<MaximumMinimumComputation>,
    #[serde(rename = "TotalAggregation")]
    pub total_aggregation: Option<TotalAggregationComputation>,
    #[serde(rename = "PeriodToDate")]
    pub period_to_date: Option<PeriodToDateComputation>,
    #[serde(rename = "MetricComparison")]
    pub metric_comparison: Option<MetricComparisonComputation>,
    #[serde(rename = "TopBottomMovers")]
    pub top_bottom_movers: Option<TopBottomMoversComputation>,
    #[serde(rename = "TopBottomRanked")]
    pub top_bottom_ranked: Option<TopBottomRankedComputation>,
    #[serde(rename = "GrowthRate")]
    pub growth_rate: Option<GrowthRateComputation>,
    #[serde(rename = "UniqueValues")]
    pub unique_values: Option<UniqueValuesComputation>,
    #[serde(rename = "PeriodOverPeriod")]
    pub period_over_period: Option<PeriodOverPeriodComputation>,
    #[serde(rename = "Forecast")]
    pub forecast: Option<ForecastComputation>,

}

#[derive(serde::Serialize, Default)]
pub struct LongFormatText {
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotUnaggregatedFieldWells {
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<DimensionField>>,
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<DimensionField>>,

}
pub type FontWeightName = String;
#[derive(serde::Serialize, Default)]
pub struct DataPointTooltipOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPointMenuLabelOption {
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<DashboardBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelType {
    #[serde(rename = "FieldLabelType")]
    pub field_label_type: Option<FieldLabelType>,
    #[serde(rename = "MinimumLabelType")]
    pub minimum_label_type: Option<MinimumLabelType>,
    #[serde(rename = "DataPathLabelType")]
    pub data_path_label_type: Option<DataPathLabelType>,
    #[serde(rename = "RangeEndsLabelType")]
    pub range_ends_label_type: Option<RangeEndsLabelType>,
    #[serde(rename = "MaximumLabelType")]
    pub maximum_label_type: Option<MaximumLabelType>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterListControl {
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,

}
pub type SheetContentType = String;
#[derive(serde::Serialize, Default)]
pub struct ArcOptions {
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThickness>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardPublishOptions {
    #[serde(rename = "ExportToCSVOption")]
    pub export_to_csvoption: Option<ExportToCSVOption>,
    #[serde(rename = "ExportWithHiddenFieldsOption")]
    pub export_with_hidden_fields_option: Option<ExportWithHiddenFieldsOption>,
    #[serde(rename = "DataPointTooltipOption")]
    pub data_point_tooltip_option: Option<DataPointTooltipOption>,
    #[serde(rename = "SheetControlsOption")]
    pub sheet_controls_option: Option<SheetControlsOption>,
    #[serde(rename = "SheetLayoutElementMaximizationOption")]
    pub sheet_layout_element_maximization_option: Option<SheetLayoutElementMaximizationOption>,
    #[serde(rename = "DataPointDrillUpDownOption")]
    pub data_point_drill_up_down_option: Option<DataPointDrillUpDownOption>,
    #[serde(rename = "VisualPublishOptions")]
    pub visual_publish_options: Option<DashboardVisualPublishOptions>,
    #[serde(rename = "VisualMenuOption")]
    pub visual_menu_option: Option<VisualMenuOption>,
    #[serde(rename = "VisualAxisSortOption")]
    pub visual_axis_sort_option: Option<VisualAxisSortOption>,
    #[serde(rename = "AdHocFilteringOption")]
    pub ad_hoc_filtering_option: Option<AdHocFilteringOption>,
    #[serde(rename = "DataPointMenuLabelOption")]
    pub data_point_menu_label_option: Option<DataPointMenuLabelOption>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartSortConfiguration {
    #[serde(rename = "ColorItemsLimitConfiguration")]
    pub color_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimitConfiguration")]
    pub category_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramChartConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<SankeyDiagramSortConfiguration>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<SankeyDiagramFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnHierarchy {
    #[serde(rename = "DateTimeHierarchy")]
    pub date_time_hierarchy: Option<DateTimeHierarchy>,
    #[serde(rename = "ExplicitHierarchy")]
    pub explicit_hierarchy: Option<ExplicitHierarchy>,
    #[serde(rename = "PredefinedHierarchy")]
    pub predefined_hierarchy: Option<PredefinedHierarchy>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FunnelChartSortConfiguration>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "DataLabelOptions")]
    pub data_label_options: Option<FunnelChartDataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FunnelChartFieldWells>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapSortConfiguration {
    #[serde(rename = "HeatMapColumnSort")]
    pub heat_map_column_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "HeatMapRowItemsLimitConfiguration")]
    pub heat_map_row_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapColumnItemsLimitConfiguration")]
    pub heat_map_column_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapRowSort")]
    pub heat_map_row_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualCustomAction {
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "Trigger")]
    pub trigger: VisualCustomActionTrigger,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CustomActionId")]
    pub custom_action_id: String,
    #[serde(rename = "ActionOperations")]
    pub action_operations: Vec<VisualCustomActionOperation>,

}
pub type AxisBinding = String;
#[derive(serde::Serialize, Default)]
pub struct AggregationSortConfiguration {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: AggregationFunction,
    #[serde(rename = "SortDirection")]
    pub sort_direction: SortDirection,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastScenario {
    #[serde(rename = "WhatIfRangeScenario")]
    pub what_if_range_scenario: Option<WhatIfRangeScenario>,
    #[serde(rename = "WhatIfPointScenario")]
    pub what_if_point_scenario: Option<WhatIfPointScenario>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomColor {
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "SpecialValue")]
    pub special_value: Option<SpecialValue>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<KPIConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldBasedTooltip {
    #[serde(rename = "AggregationVisibility")]
    pub aggregation_visibility: Option<Visibility>,
    #[serde(rename = "TooltipFields")]
    pub tooltip_fields: Option<Vec<TooltipItem>>,
    #[serde(rename = "TooltipTitleType")]
    pub tooltip_title_type: Option<TooltipTitleType>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilterValue {
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<f64>,

}
pub type Visibility = String;
#[derive(serde::Serialize, Default)]
pub struct FontSize {
    #[serde(rename = "Relative")]
    pub relative: Option<RelativeFontSize>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineCustomLabelConfiguration {
    #[serde(rename = "CustomLabel")]
    pub custom_label: String,

}
pub type HistogramBinType = String;
#[derive(serde::Serialize, Default)]
pub struct ScatterPlotCategoricallyAggregatedFieldWells {
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<MeasureField>>,
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<MeasureField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ContributionAnalysisDefault {
    #[serde(rename = "MeasureFieldId")]
    pub measure_field_id: String,
    #[serde(rename = "ContributorDimensions")]
    pub contributor_dimensions: Vec<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct PeriodOverPeriodComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomValuesConfiguration {
    #[serde(rename = "IncludeNullValue")]
    pub include_null_value: Option<bool>,
    #[serde(rename = "CustomValues")]
    pub custom_values: CustomParameterValues,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnIdentifier {
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterTextFieldControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,

}
pub type TableTotalsPlacement = String;pub type TopBottomComputationType = String;
#[derive(serde::Serialize, Default)]
pub struct TopBottomFilter {
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Limit")]
    pub limit: Option<f64>,
    #[serde(rename = "AggregationSortConfigurations")]
    pub aggregation_sort_configurations: Vec<AggregationSortConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WordCloudChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}
pub type TargetVisualOptions = String;
#[derive(serde::Serialize, Default)]
pub struct TreeMapAggregatedFieldWells {
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<DimensionField>>,
    #[serde(rename = "Sizes")]
    pub sizes: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "BreakdownItemsLimit")]
    pub breakdown_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisDisplayRange {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElement {
    #[serde(rename = "RenderingRules")]
    pub rendering_rules: Option<Vec<SheetElementRenderingRule>>,
    #[serde(rename = "YAxisLocation")]
    pub yaxis_location: String,
    #[serde(rename = "Width")]
    pub width: String,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "BackgroundStyle")]
    pub background_style: Option<FreeFormLayoutElementBackgroundStyle>,
    #[serde(rename = "LoadingAnimation")]
    pub loading_animation: Option<LoadingAnimation>,
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "SelectedBorderStyle")]
    pub selected_border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,
    #[serde(rename = "Height")]
    pub height: String,
    #[serde(rename = "XAxisLocation")]
    pub xaxis_location: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOption {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "URLStyling")]
    pub urlstyling: Option<TableFieldURLConfiguration>,
    #[serde(rename = "Width")]
    pub width: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalPlacesConfiguration {
    #[serde(rename = "DecimalPlaces")]
    pub decimal_places: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PercentileAggregation {
    #[serde(rename = "PercentileValue")]
    pub percentile_value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIOptions {
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<ProgressBarOptions>,
    #[serde(rename = "SecondaryValue")]
    pub secondary_value: Option<SecondaryValueOptions>,
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "TrendArrows")]
    pub trend_arrows: Option<TrendArrowOptions>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "SecondaryValueFontConfiguration")]
    pub secondary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<GridLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartSeriesSettings {
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct TextAreaControlDisplayOptions {
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomRankedComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "ResultSize")]
    pub result_size: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartConfiguration {
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<ComboChartSortConfiguration>,
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "BarDataLabels")]
    pub bar_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "LineDataLabels")]
    pub line_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ComboChartFieldWells>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetControlLayout {
    #[serde(rename = "Configuration")]
    pub configuration: SheetControlLayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeFormatConfiguration {
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RelativeDatesFilter {
    #[serde(rename = "MinimumGranularity")]
    pub minimum_granularity: Option<TimeGranularity>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "RelativeDateValue")]
    pub relative_date_value: Option<f64>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "AnchorDateConfiguration")]
    pub anchor_date_configuration: AnchorDateConfiguration,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,
    #[serde(rename = "RelativeDateType")]
    pub relative_date_type: RelativeDateType,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapAggregatedFieldWells {
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludePeriodConfiguration {
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Granularity")]
    pub granularity: TimeGranularity,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutConfiguration {
    #[serde(rename = "BodySections")]
    pub body_sections: Vec<BodySectionConfiguration>,
    #[serde(rename = "FooterSections")]
    pub footer_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "HeaderSections")]
    pub header_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct NumberDisplayFormatConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ItemsLimitConfiguration {
    #[serde(rename = "OtherCategories")]
    pub other_categories: Option<OtherCategories>,
    #[serde(rename = "ItemsLimit")]
    pub items_limit: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapConfiguration {
    #[serde(rename = "ColumnLabelOptions")]
    pub column_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HeatMapFieldWells>,
    #[serde(rename = "RowLabelOptions")]
    pub row_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<HeatMapSortConfiguration>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}
pub type RelativeDateType = String;
#[derive(serde::Serialize, Default)]
pub struct DynamicDefaultValue {
    #[serde(rename = "DefaultValueColumn")]
    pub default_value_column: ColumnIdentifier,
    #[serde(rename = "GroupNameColumn")]
    pub group_name_column: Option<ColumnIdentifier>,
    #[serde(rename = "UserNameColumn")]
    pub user_name_column: Option<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TreeMapConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartFieldWells {
    #[serde(rename = "BarChartAggregatedFieldWells")]
    pub bar_chart_aggregated_field_wells: Option<BarChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct AggregationFunction {
    #[serde(rename = "CategoricalAggregationFunction")]
    pub categorical_aggregation_function: Option<CategoricalAggregationFunction>,
    #[serde(rename = "DateAggregationFunction")]
    pub date_aggregation_function: Option<DateAggregationFunction>,
    #[serde(rename = "NumericalAggregationFunction")]
    pub numerical_aggregation_function: Option<NumericalAggregationFunction>,

}
pub type WordCloudWordCasing = String;pub type TooltipTitleType = String;
#[derive(serde::Serialize, Default)]
pub struct TextFieldControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelOptions {
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,
    #[serde(rename = "DataLabelTypes")]
    pub data_label_types: Option<Vec<DataLabelType>>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "LabelContent")]
    pub label_content: Option<DataLabelContent>,
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "Overlap")]
    pub overlap: Option<DataLabelOverlap>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,

}
pub type ComparisonMethod = String;
#[derive(serde::Serialize, Default)]
pub struct PivotFieldSortOptions {
    #[serde(rename = "SortBy")]
    pub sort_by: PivotTableSortBy,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterControl {
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<ParameterDateTimePickerControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<ParameterDropDownControl>,
    #[serde(rename = "Slider")]
    pub slider: Option<ParameterSliderControl>,
    #[serde(rename = "List")]
    pub list: Option<ParameterListControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<ParameterTextAreaControl>,
    #[serde(rename = "TextField")]
    pub text_field: Option<ParameterTextFieldControl>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Categories")]
    pub categories: Option<Vec<DimensionField>>,
    #[serde(rename = "Breakdowns")]
    pub breakdowns: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldTooltipItem {
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Vec<FilledMapConditionalFormattingOption>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}
pub type ColorFillType = String;pub type Icon = String;
#[derive(serde::Serialize, Default)]
pub struct NumericalMeasureField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<NumericalAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldURLConfiguration {
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<TableFieldImageConfiguration>,
    #[serde(rename = "LinkConfiguration")]
    pub link_configuration: Option<TableFieldLinkConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ComboChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}

#[derive(serde::Serialize, Default)]
pub struct DataBarsOptions {
    #[serde(rename = "NegativeColor")]
    pub negative_color: Option<String>,
    #[serde(rename = "PositiveColor")]
    pub positive_color: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormattingOption {
    #[serde(rename = "Shape")]
    pub shape: FilledMapShapeConditionalFormatting,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FunnelChartConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConfiguration {
    #[serde(rename = "TableInlineVisualizations")]
    pub table_inline_visualizations: Option<Vec<TableInlineVisualization>>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TableFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TableSortConfiguration>,
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<TableFieldOptions>,
    #[serde(rename = "TableOptions")]
    pub table_options: Option<TableOptions>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<TablePaginatedReportOptions>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<TotalOptions>,

}
pub type TableTotalsScrollStatus = String;
#[derive(serde::Serialize, Default)]
pub struct ForecastComputation {
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "CustomSeasonalityValue")]
    pub custom_seasonality_value: Option<f64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<ForecastComputationSeasonality>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIProgressBarConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeDrillDownFilter {
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: String,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct ProgressBarOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SameSheetTargetVisualConfiguration {
    #[serde(rename = "TargetVisuals")]
    pub target_visuals: Option<Vec<String>>,
    #[serde(rename = "TargetVisualOptions")]
    pub target_visual_options: Option<TargetVisualOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayDataDrivenRange {

}


}

pub mod cfn_data_set {

#[derive(serde::Serialize, Default)]
pub struct CfnDataSet {
    /// No documentation provided by AWS
    #[serde(rename = "FieldFolders")]
    pub field_folders: Option<FieldFolderMap>,
    /// <p>The row-level security configuration for the dataset.</p>
    #[serde(rename = "RowLevelPermissionDataSet")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    /// No documentation provided by AWS
    #[serde(rename = "ImportMode")]
    pub import_mode: Option<DataSetImportMode>,
    /// <p>The dataset usage configuration for the dataset.</p>
    #[serde(rename = "DataSetUsageConfiguration")]
    pub data_set_usage_configuration: Option<DataSetUsageConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DataSetId")]
    pub data_set_id: Option<String>,
    /// <p>The display name for the dataset.</p>
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dataset.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Wait policy to use when creating/updating dataset. Default is to wait for SPICE ingestion to finish with timeout of 36 hours.</p>
    #[serde(rename = "IngestionWaitPolicy")]
    pub ingestion_wait_policy: Option<IngestionWaitPolicy>,
    /// <p>A list of resource permissions on the dataset.</p>
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// <p>Groupings of columns that work together in certain QuickSight features. Currently, only geospatial hierarchy is supported.</p>
    #[serde(rename = "ColumnGroups")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    /// List of ColumnLevelPermissionRule
    #[serde(rename = "ColumnLevelPermissionRules")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
    /// No documentation provided by AWS
    #[serde(rename = "PhysicalTableMap")]
    pub physical_table_map: Option<PhysicalTableMap>,
    /// No documentation provided by AWS
    #[serde(rename = "LogicalTableMap")]
    pub logical_table_map: Option<LogicalTableMap>,

}


#[derive(serde::Serialize, Default)]
pub struct JoinKeyProperties {
    #[serde(rename = "UniqueKey")]
    pub unique_key: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TagColumnOperation {
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<ColumnTag>,

}

#[derive(serde::Serialize, Default)]
pub struct RelationalTable {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Catalog")]
    pub catalog: Option<String>,
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,
    #[serde(rename = "Schema")]
    pub schema: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeoSpatialColumnGroup {
    #[serde(rename = "CountryCode")]
    pub country_code: Option<GeoSpatialCountryCode>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Columns")]
    pub columns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RenameColumnOperation {
    #[serde(rename = "NewColumnName")]
    pub new_column_name: String,
    #[serde(rename = "ColumnName")]
    pub column_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Source {
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    #[serde(rename = "UploadSettings")]
    pub upload_settings: Option<UploadSettings>,
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldFolder {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldFolderMap {

}

#[derive(serde::Serialize, Default)]
pub struct CastColumnTypeOperation {
    #[serde(rename = "NewColumnType")]
    pub new_column_type: ColumnDataType,
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    #[serde(rename = "Format")]
    pub format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LogicalTableSource {
    #[serde(rename = "PhysicalTableId")]
    pub physical_table_id: Option<String>,
    #[serde(rename = "JoinInstruction")]
    pub join_instruction: Option<JoinInstruction>,
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputColumn {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub ty: InputColumnDataType,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedColumn {
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "ColumnId")]
    pub column_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    #[serde(rename = "Principal")]
    pub principal: String,

}
pub type ColumnDataType = String;pub type TextQualifier = String;pub type GeoSpatialDataRole = String;
#[derive(serde::Serialize, Default)]
pub struct IngestionWaitPolicy {
    #[serde(rename = "IngestionWaitTimeInHours")]
    pub ingestion_wait_time_in_hours: Option<f64>,
    #[serde(rename = "WaitForSpiceIngestion")]
    pub wait_for_spice_ingestion: Option<bool>,

}
pub type RowLevelPermissionPolicy = String;
#[derive(serde::Serialize, Default)]
pub struct PhysicalTableMap {

}

#[derive(serde::Serialize, Default)]
pub struct PhysicalTable {
    #[serde(rename = "S3Source")]
    pub s3_source: Option<S3Source>,
    #[serde(rename = "RelationalTable")]
    pub relational_table: Option<RelationalTable>,
    #[serde(rename = "CustomSql")]
    pub custom_sql: Option<CustomSql>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnLevelPermissionRule {
    #[serde(rename = "Principals")]
    pub principals: Option<Vec<String>>,
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,

}
pub type RowLevelPermissionFormatVersion = String;
#[derive(serde::Serialize, Default)]
pub struct DataSetUsageConfiguration {
    #[serde(rename = "DisableUseAsImportedSource")]
    pub disable_use_as_imported_source: Option<bool>,
    #[serde(rename = "DisableUseAsDirectQuerySource")]
    pub disable_use_as_direct_query_source: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnTag {
    #[serde(rename = "ColumnDescription")]
    pub column_description: Option<ColumnDescription>,
    #[serde(rename = "ColumnGeographicRole")]
    pub column_geographic_role: Option<GeoSpatialDataRole>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnGroup {
    #[serde(rename = "GeoSpatialColumnGroup")]
    pub geo_spatial_column_group: Option<GeoSpatialColumnGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct JoinInstruction {
    #[serde(rename = "LeftJoinKeyProperties")]
    pub left_join_key_properties: Option<JoinKeyProperties>,
    #[serde(rename = "OnClause")]
    pub on_clause: String,
    #[serde(rename = "RightJoinKeyProperties")]
    pub right_join_key_properties: Option<JoinKeyProperties>,
    #[serde(rename = "Type")]
    pub ty: JoinType,
    #[serde(rename = "RightOperand")]
    pub right_operand: String,
    #[serde(rename = "LeftOperand")]
    pub left_operand: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogicalTable {
    #[serde(rename = "Source")]
    pub source: LogicalTableSource,
    #[serde(rename = "DataTransforms")]
    pub data_transforms: Option<Vec<TransformOperation>>,
    #[serde(rename = "Alias")]
    pub alias: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectOperation {
    #[serde(rename = "ProjectedColumns")]
    pub projected_columns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TransformOperation {
    #[serde(rename = "RenameColumnOperation")]
    pub rename_column_operation: Option<RenameColumnOperation>,
    #[serde(rename = "ProjectOperation")]
    pub project_operation: Option<ProjectOperation>,
    #[serde(rename = "TagColumnOperation")]
    pub tag_column_operation: Option<TagColumnOperation>,
    #[serde(rename = "CreateColumnsOperation")]
    pub create_columns_operation: Option<CreateColumnsOperation>,
    #[serde(rename = "CastColumnTypeOperation")]
    pub cast_column_type_operation: Option<CastColumnTypeOperation>,
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<FilterOperation>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomSql {
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,
    #[serde(rename = "SqlQuery")]
    pub sql_query: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Columns")]
    pub columns: Vec<InputColumn>,

}
pub type DataSetImportMode = String;pub type InputColumnDataType = String;
#[derive(serde::Serialize, Default)]
pub struct ColumnDescription {
    #[serde(rename = "Text")]
    pub text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperation {
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct UploadSettings {
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<bool>,
    #[serde(rename = "StartFromRow")]
    pub start_from_row: Option<f64>,
    #[serde(rename = "TextQualifier")]
    pub text_qualifier: Option<TextQualifier>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "Format")]
    pub format: Option<FileFormat>,

}
pub type GeoSpatialCountryCode = String;pub type FileFormat = String;pub type JoinType = String;
#[derive(serde::Serialize, Default)]
pub struct OutputColumn {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<ColumnDataType>,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RowLevelPermissionDataSet {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "FormatVersion")]
    pub format_version: Option<RowLevelPermissionFormatVersion>,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "PermissionPolicy")]
    pub permission_policy: RowLevelPermissionPolicy,

}

#[derive(serde::Serialize, Default)]
pub struct LogicalTableMap {

}

#[derive(serde::Serialize, Default)]
pub struct CreateColumnsOperation {
    #[serde(rename = "Columns")]
    pub columns: Vec<CalculatedColumn>,

}


}

pub mod cfn_data_source {

#[derive(serde::Serialize, Default)]
pub struct CfnDataSource {
    /// <p>A list of resource permissions on the data source.</p>
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<DataSourceType>,
    /// <p>A display name for the data source.</p>
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// <p>VPC connection properties.</p>
    #[serde(rename = "VpcConnectionProperties")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
    /// <p>Secure Socket Layer (SSL) properties that apply when QuickSight connects to your
    /// underlying data source.</p>
    #[serde(rename = "SslProperties")]
    pub ssl_properties: Option<SslProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "Credentials")]
    pub credentials: Option<DataSourceCredentials>,
    /// <p>The parameters that Amazon QuickSight uses to connect to your underlying data source.
    /// This is a variant type structure. For this structure to be valid, only one of the
    /// attributes can be non-null.</p>
    #[serde(rename = "DataSourceParameters")]
    pub data_source_parameters: Option<DataSourceParameters>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,
    /// <p>A set of alternate data source parameters that you want to share for the credentials
    /// stored with this data source. The credentials are applied in tandem with the data source
    /// parameters when you copy a data source by using a create or update request. The API
    /// operation compares the <code>DataSourceParameters</code> structure that's in the request
    /// with the structures in the <code>AlternateDataSourceParameters</code> allow list. If the
    /// structures are an exact match, the request is allowed to use the credentials from this
    /// existing data source. If the <code>AlternateDataSourceParameters</code> list is null,
    /// the <code>Credentials</code> originally used with this <code>DataSourceParameters</code>
    /// are automatically allowed.</p>
    #[serde(rename = "AlternateDataSourceParameters")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
    /// No documentation provided by AWS
    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<DataSourceErrorInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "DataSourceId")]
    pub data_source_id: Option<String>,
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the data source.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

pub type DataSourceType = String;
#[derive(serde::Serialize, Default)]
pub struct MySqlParameters {
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: String,

}

#[derive(serde::Serialize, Default)]
pub struct OracleParameters {
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Database")]
    pub database: String,

}

#[derive(serde::Serialize, Default)]
pub struct PostgreSqlParameters {
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Database")]
    pub database: String,

}

#[derive(serde::Serialize, Default)]
pub struct RdsParameters {
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "InstanceId")]
    pub instance_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonElasticsearchParameters {
    #[serde(rename = "Domain")]
    pub domain: String,

}

#[derive(serde::Serialize, Default)]
pub struct AthenaParameters {
    #[serde(rename = "WorkGroup")]
    pub work_group: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Principal")]
    pub principal: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSourceCredentials {
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "CopySourceArn")]
    pub copy_source_arn: Option<String>,
    #[serde(rename = "CredentialPair")]
    pub credential_pair: Option<CredentialPair>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Parameters {
    #[serde(rename = "ManifestFileLocation")]
    pub manifest_file_location: ManifestFileLocation,

}

#[derive(serde::Serialize, Default)]
pub struct DataSourceParameters {
    #[serde(rename = "TeradataParameters")]
    pub teradata_parameters: Option<TeradataParameters>,
    #[serde(rename = "PrestoParameters")]
    pub presto_parameters: Option<PrestoParameters>,
    #[serde(rename = "S3Parameters")]
    pub s3_parameters: Option<S3Parameters>,
    #[serde(rename = "AuroraParameters")]
    pub aurora_parameters: Option<AuroraParameters>,
    #[serde(rename = "AthenaParameters")]
    pub athena_parameters: Option<AthenaParameters>,
    #[serde(rename = "PostgreSqlParameters")]
    pub postgre_sql_parameters: Option<PostgreSqlParameters>,
    #[serde(rename = "SnowflakeParameters")]
    pub snowflake_parameters: Option<SnowflakeParameters>,
    #[serde(rename = "SparkParameters")]
    pub spark_parameters: Option<SparkParameters>,
    #[serde(rename = "MariaDbParameters")]
    pub maria_db_parameters: Option<MariaDbParameters>,
    #[serde(rename = "DatabricksParameters")]
    pub databricks_parameters: Option<DatabricksParameters>,
    #[serde(rename = "RdsParameters")]
    pub rds_parameters: Option<RdsParameters>,
    #[serde(rename = "SqlServerParameters")]
    pub sql_server_parameters: Option<SqlServerParameters>,
    #[serde(rename = "OracleParameters")]
    pub oracle_parameters: Option<OracleParameters>,
    #[serde(rename = "AuroraPostgreSqlParameters")]
    pub aurora_postgre_sql_parameters: Option<AuroraPostgreSqlParameters>,
    #[serde(rename = "MySqlParameters")]
    pub my_sql_parameters: Option<MySqlParameters>,
    #[serde(rename = "AmazonOpenSearchParameters")]
    pub amazon_open_search_parameters: Option<AmazonOpenSearchParameters>,
    #[serde(rename = "RedshiftParameters")]
    pub redshift_parameters: Option<RedshiftParameters>,
    #[serde(rename = "AmazonElasticsearchParameters")]
    pub amazon_elasticsearch_parameters: Option<AmazonElasticsearchParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct SparkParameters {
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: f64,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectionProperties {
    #[serde(rename = "VpcConnectionArn")]
    pub vpc_connection_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ManifestFileLocation {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,

}

#[derive(serde::Serialize, Default)]
pub struct DatabricksParameters {
    #[serde(rename = "SqlEndpointPath")]
    pub sql_endpoint_path: String,
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Host")]
    pub host: String,

}

#[derive(serde::Serialize, Default)]
pub struct AuroraPostgreSqlParameters {
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: String,

}

#[derive(serde::Serialize, Default)]
pub struct SslProperties {
    #[serde(rename = "DisableSsl")]
    pub disable_ssl: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SqlServerParameters {
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: f64,

}

#[derive(serde::Serialize, Default)]
pub struct MariaDbParameters {
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Database")]
    pub database: String,

}

#[derive(serde::Serialize, Default)]
pub struct CredentialPair {
    #[serde(rename = "AlternateDataSourceParameters")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,

}
pub type ResourceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct SnowflakeParameters {
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Warehouse")]
    pub warehouse: String,
    #[serde(rename = "Database")]
    pub database: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct AuroraParameters {
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PrestoParameters {
    #[serde(rename = "Catalog")]
    pub catalog: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: f64,

}

#[derive(serde::Serialize, Default)]
pub struct TeradataParameters {
    #[serde(rename = "Port")]
    pub port: f64,
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: String,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonOpenSearchParameters {
    #[serde(rename = "Domain")]
    pub domain: String,

}
pub type DataSourceErrorInfoType = String;
#[derive(serde::Serialize, Default)]
pub struct RedshiftParameters {
    #[serde(rename = "ClusterId")]
    pub cluster_id: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<f64>,
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "Host")]
    pub host: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSourceErrorInfo {
    #[serde(rename = "Type")]
    pub ty: Option<DataSourceErrorInfoType>,
    #[serde(rename = "Message")]
    pub message: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsIotAnalyticsParameters {
    #[serde(rename = "DataSetName")]
    pub data_set_name: String,

}


}

pub mod cfn_refresh_schedule {

#[derive(serde::Serialize, Default)]
pub struct CfnRefreshSchedule {
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: Option<RefreshScheduleMap>,
    /// No documentation provided by AWS
    #[serde(rename = "DataSetId")]
    pub data_set_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct RefreshScheduleMap {
    #[serde(rename = "StartAfterDateTime")]
    pub start_after_date_time: Option<String>,
    #[serde(rename = "ScheduleFrequency")]
    pub schedule_frequency: Option<()>,
    #[serde(rename = "RefreshType")]
    pub refresh_type: Option<String>,
    #[serde(rename = "ScheduleId")]
    pub schedule_id: Option<String>,

}


}

pub mod cfn_template {

#[derive(serde::Serialize, Default)]
pub struct CfnTemplate {
    /// List of ResourcePermission
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceEntity")]
    pub source_entity: Option<TemplateSourceEntity>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Option<TemplateVersionDefinition>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct VisualCustomActionOperation {
    #[serde(rename = "SetParametersOperation")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,
    #[serde(rename = "URLOperation")]
    pub urloperation: Option<CustomActionURLOperation>,
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<CustomActionFilterOperation>,
    #[serde(rename = "NavigationOperation")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,

}

#[derive(serde::Serialize, Default)]
pub struct ScrollBarOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "VisibleRange")]
    pub visible_range: Option<VisibleRangeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldImageConfiguration {
    #[serde(rename = "SizingOptions")]
    pub sizing_options: Option<TableCellImageSizingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DimensionField {
    #[serde(rename = "NumericalDimensionField")]
    pub numerical_dimension_field: Option<NumericalDimensionField>,
    #[serde(rename = "DateDimensionField")]
    pub date_dimension_field: Option<DateDimensionField>,
    #[serde(rename = "CategoricalDimensionField")]
    pub categorical_dimension_field: Option<CategoricalDimensionField>,

}
pub type FunnelChartMeasureDataLabelStyle = String;
#[derive(serde::Serialize, Default)]
pub struct FilterScopeConfiguration {
    #[serde(rename = "SelectedSheets")]
    pub selected_sheets: Option<SelectedSheetsFilterScopeConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapConfiguration {
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GeospatialMapFieldWells>,
    #[serde(rename = "PointStyleOptions")]
    pub point_style_options: Option<GeospatialPointStyleOptions>,

}
pub type MissingDataTreatmentOption = String;
#[derive(serde::Serialize, Default)]
pub struct WaterfallVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WaterfallChartConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,

}
pub type WordCloudWordCasing = String;
#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutPaperCanvasSizeOptions {
    #[serde(rename = "PaperMargin")]
    pub paper_margin: Option<Spacing>,
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<PaperSize>,
    #[serde(rename = "PaperOrientation")]
    pub paper_orientation: Option<PaperOrientation>,

}
pub type OtherCategories = String;
#[derive(serde::Serialize, Default)]
pub struct AxisScale {
    #[serde(rename = "Linear")]
    pub linear: Option<AxisLinearScale>,
    #[serde(rename = "Logarithmic")]
    pub logarithmic: Option<AxisLogarithmicScale>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartConfiguration {
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<ComboChartSortConfiguration>,
    #[serde(rename = "LineDataLabels")]
    pub line_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ComboChartFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "BarDataLabels")]
    pub bar_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryFilter {
    #[serde(rename = "Configuration")]
    pub configuration: CategoryFilterConfiguration,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    pub filter_id: String,

}
pub type ComparisonMethod = String;
#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormattingOption {
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<GaugeChartPrimaryValueConditionalFormatting>,
    #[serde(rename = "Arc")]
    pub arc: Option<GaugeChartArcConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberFormatConfiguration {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}
pub type SpecialValue = String;
#[derive(serde::Serialize, Default)]
pub struct ThousandSeparatorOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Symbol")]
    pub symbol: Option<NumericSeparatorSymbol>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapAggregatedFieldWells {
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<MeasureField>>,
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<DimensionField>>,
    #[serde(rename = "Sizes")]
    pub sizes: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}
pub type TooltipTitleType = String;
#[derive(serde::Serialize, Default)]
pub struct LineChartFieldWells {
    #[serde(rename = "LineChartAggregatedFieldWells")]
    pub line_chart_aggregated_field_wells: Option<LineChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct PaginationConfiguration {
    #[serde(rename = "PageNumber")]
    pub page_number: f64,
    #[serde(rename = "PageSize")]
    pub page_size: f64,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartDefaultSeriesSettings {
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,

}

#[derive(serde::Serialize, Default)]
pub struct ShapeConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    pub background_color: ConditionalFormattingColor,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<LineChartConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct StringValueWhenUnsetConfiguration {
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TemplateVersion {
    #[serde(rename = "Status")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "DataSetConfigurations")]
    pub data_set_configurations: Option<Vec<DataSetConfiguration>>,
    #[serde(rename = "SourceEntityArn")]
    pub source_entity_arn: Option<String>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<TemplateError>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ThemeArn")]
    pub theme_arn: Option<String>,
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<f64>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<Sheet>>,

}
pub type HorizontalTextAlignment = String;pub type SectionPageBreakStatus = String;pub type TimeGranularity = String;
#[derive(serde::Serialize, Default)]
pub struct FilledMapShapeConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Format")]
    pub format: Option<ShapeConditionalFormat>,

}

#[derive(serde::Serialize, Default)]
pub struct PanelConfiguration {
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<PanelBorderStyle>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "BorderThickness")]
    pub border_thickness: Option<String>,
    #[serde(rename = "BorderColor")]
    pub border_color: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<PanelTitleOptions>,
    #[serde(rename = "GutterSpacing")]
    pub gutter_spacing: Option<String>,
    #[serde(rename = "GutterVisibility")]
    pub gutter_visibility: Option<Visibility>,
    #[serde(rename = "BorderVisibility")]
    pub border_visibility: Option<Visibility>,
    #[serde(rename = "BackgroundVisibility")]
    pub background_visibility: Option<Visibility>,

}
pub type Visibility = String;
#[derive(serde::Serialize, Default)]
pub struct KPIPrimaryValueConditionalFormatting {
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDataOptions {
    #[serde(rename = "DateAxisOptions")]
    pub date_axis_options: Option<DateAxisOptions>,
    #[serde(rename = "NumericAxisOptions")]
    pub numeric_axis_options: Option<NumericAxisOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SameSheetTargetVisualConfiguration {
    #[serde(rename = "TargetVisuals")]
    pub target_visuals: Option<Vec<String>>,
    #[serde(rename = "TargetVisualOptions")]
    pub target_visual_options: Option<TargetVisualOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLinearScale {
    #[serde(rename = "StepSize")]
    pub step_size: Option<f64>,
    #[serde(rename = "StepCount")]
    pub step_count: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotCategoricallyAggregatedFieldWells {
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<MeasureField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<MeasureField>>,

}
pub type PaperOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomIconContent {
    #[serde(rename = "Icon")]
    pub icon: Option<TableFieldIconSetType>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderFooterSectionConfiguration {
    #[serde(rename = "Layout")]
    pub layout: SectionLayoutConfiguration,
    #[serde(rename = "SectionId")]
    pub section_id: String,
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeEqualityFilter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct DonutCenterOptions {
    #[serde(rename = "LabelVisibility")]
    pub label_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilterValue {
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<String>,
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,

}
pub type BarChartOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct FilterTextAreaControl {
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBackgroundStyle {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableDataPathOption {
    #[serde(rename = "DataPathList")]
    pub data_path_list: Vec<DataPathValue>,
    #[serde(rename = "Width")]
    pub width: Option<String>,

}
pub type SheetControlSliderType = String;
#[derive(serde::Serialize, Default)]
pub struct CustomFilterListConfiguration {
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialCoordinateBounds {
    #[serde(rename = "South")]
    pub south: f64,
    #[serde(rename = "West")]
    pub west: f64,
    #[serde(rename = "North")]
    pub north: f64,
    #[serde(rename = "East")]
    pub east: f64,

}
pub type FilterNullOption = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterDateTimePickerControl {
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct RowAlternateColorOptions {
    #[serde(rename = "RowAlternateColors")]
    pub row_alternate_colors: Option<Vec<String>>,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalParameterDeclaration {
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DecimalValueWhenUnsetConfiguration>,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DecimalDefaultValues>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TemplateSourceAnalysis {
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "DataSetReferences")]
    pub data_set_references: Vec<DataSetReference>,

}

#[derive(serde::Serialize, Default)]
pub struct TemplateSourceEntity {
    #[serde(rename = "SourceAnalysis")]
    pub source_analysis: Option<TemplateSourceAnalysis>,
    #[serde(rename = "SourceTemplate")]
    pub source_template: Option<TemplateSourceTemplate>,

}
pub type ParameterValueType = String;
#[derive(serde::Serialize, Default)]
pub struct ColorScale {
    #[serde(rename = "NullValueColor")]
    pub null_value_color: Option<DataColor>,
    #[serde(rename = "Colors")]
    pub colors: Vec<DataColor>,
    #[serde(rename = "ColorFillType")]
    pub color_fill_type: ColorFillType,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconDisplayConfiguration {
    #[serde(rename = "IconDisplayOption")]
    pub icon_display_option: Option<ConditionalFormattingIconDisplayOption>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConfiguration {
    #[serde(rename = "KPIOptions")]
    pub kpioptions: Option<KPIOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<KPIFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<KPISortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineLabelConfiguration {
    #[serde(rename = "CustomLabelConfiguration")]
    pub custom_label_configuration: Option<ReferenceLineCustomLabelConfiguration>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "HorizontalPosition")]
    pub horizontal_position: Option<ReferenceLineLabelHorizontalPosition>,
    #[serde(rename = "ValueLabelConfiguration")]
    pub value_label_configuration: Option<ReferenceLineValueLabelConfiguration>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "VerticalPosition")]
    pub vertical_position: Option<ReferenceLineLabelVerticalPosition>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldURLConfiguration {
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<TableFieldImageConfiguration>,
    #[serde(rename = "LinkConfiguration")]
    pub link_configuration: Option<TableFieldLinkConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeFilter {
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMaximumValue")]
    pub range_maximum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "RangeMinimumValue")]
    pub range_minimum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,

}

#[derive(serde::Serialize, Default)]
pub struct NumericFormatConfiguration {
    #[serde(rename = "CurrencyDisplayFormatConfiguration")]
    pub currency_display_format_configuration: Option<CurrencyDisplayFormatConfiguration>,
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotConfiguration {
    #[serde(rename = "YAxisLabelOptions")]
    pub yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<ScatterPlotFieldWells>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,

}
pub type NegativeValueDisplayMode = String;
#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<FreeFormLayoutCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PanelTitleOptions {
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,

}
pub type CustomContentType = String;
#[derive(serde::Serialize, Default)]
pub struct FilterRelativeDateTimeControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormSectionLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<FreeFormLayoutElement>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnSort {
    #[serde(rename = "Direction")]
    pub direction: SortDirection,
    #[serde(rename = "SortBy")]
    pub sort_by: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct ShortFormatText {
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterGroup {
    #[serde(rename = "CrossDataset")]
    pub cross_dataset: CrossDatasetTypes,
    #[serde(rename = "ScopeConfiguration")]
    pub scope_configuration: FilterScopeConfiguration,
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
    #[serde(rename = "FilterGroupId")]
    pub filter_group_id: String,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalDimensionField {
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,

}
pub type NumericSeparatorSymbol = String;
#[derive(serde::Serialize, Default)]
pub struct TableConfiguration {
    #[serde(rename = "TableOptions")]
    pub table_options: Option<TableOptions>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<TotalOptions>,
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<TableFieldOptions>,
    #[serde(rename = "TableInlineVisualizations")]
    pub table_inline_visualizations: Option<Vec<TableInlineVisualization>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TableSortConfiguration>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TableFieldWells>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<TablePaginatedReportOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalMeasureField {
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<CategoricalAggregationFunction>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}
pub type ArcThickness = String;
#[derive(serde::Serialize, Default)]
pub struct FilterTextFieldControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartConfiguration {
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "AlternateBandOddColor")]
    pub alternate_band_odd_color: Option<String>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "StartAngle")]
    pub start_angle: Option<f64>,
    #[serde(rename = "AlternateBandColorsVisibility")]
    pub alternate_band_colors_visibility: Option<Visibility>,
    #[serde(rename = "ColorAxis")]
    pub color_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "BaseSeriesSettings")]
    pub base_series_settings: Option<RadarChartSeriesSettings>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<RadarChartFieldWells>,
    #[serde(rename = "AlternateBandEvenColor")]
    pub alternate_band_even_color: Option<String>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<RadarChartSortConfiguration>,
    #[serde(rename = "Shape")]
    pub shape: Option<RadarChartShape>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableSortBy {
    #[serde(rename = "Field")]
    pub field: Option<FieldSort>,
    #[serde(rename = "Column")]
    pub column: Option<ColumnSort>,
    #[serde(rename = "DataPath")]
    pub data_path: Option<DataPathSort>,

}

#[derive(serde::Serialize, Default)]
pub struct TablePaginatedReportOptions {
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramConfiguration {
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "BinOptions")]
    pub bin_options: Option<HistogramBinOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "YAxisDisplayOptions")]
    pub yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HistogramFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}
pub type PivotTableMetricPlacement = String;
#[derive(serde::Serialize, Default)]
pub struct TextFieldControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapFieldWells {
    #[serde(rename = "TreeMapAggregatedFieldWells")]
    pub tree_map_aggregated_field_wells: Option<TreeMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartLineStyleSettings {
    #[serde(rename = "LineStyle")]
    pub line_style: Option<LineChartLineStyle>,
    #[serde(rename = "LineVisibility")]
    pub line_visibility: Option<Visibility>,
    #[serde(rename = "LineInterpolation")]
    pub line_interpolation: Option<LineInterpolation>,
    #[serde(rename = "LineWidth")]
    pub line_width: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterListControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ChartAxisLabelOptions {
    #[serde(rename = "SortIconVisibility")]
    pub sort_icon_visibility: Option<Visibility>,
    #[serde(rename = "AxisLabelOptions")]
    pub axis_label_options: Option<Vec<AxisLabelOptions>>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconCondition {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DisplayConfiguration")]
    pub display_configuration: Option<ConditionalFormattingIconDisplayConfiguration>,
    #[serde(rename = "IconOptions")]
    pub icon_options: ConditionalFormattingCustomIconOptions,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartConfiguration {
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PieChartSortConfiguration>,
    #[serde(rename = "DonutOptions")]
    pub donut_options: Option<DonutOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PieChartFieldWells>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "RelativeDatesFilter")]
    pub relative_dates_filter: Option<RelativeDatesFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryFilter>,
    #[serde(rename = "NumericRangeFilter")]
    pub numeric_range_filter: Option<NumericRangeFilter>,
    #[serde(rename = "TimeEqualityFilter")]
    pub time_equality_filter: Option<TimeEqualityFilter>,
    #[serde(rename = "TopBottomFilter")]
    pub top_bottom_filter: Option<TopBottomFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityFilter>,
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct TableOptions {
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "HeaderStyle")]
    pub header_style: Option<TableCellStyle>,
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<TableOrientation>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnHierarchy {
    #[serde(rename = "PredefinedHierarchy")]
    pub predefined_hierarchy: Option<PredefinedHierarchy>,
    #[serde(rename = "DateTimeHierarchy")]
    pub date_time_hierarchy: Option<DateTimeHierarchy>,
    #[serde(rename = "ExplicitHierarchy")]
    pub explicit_hierarchy: Option<ExplicitHierarchy>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingGradientColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: GradientColor,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapAggregatedFieldWells {
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConfiguration {
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "MapStyleOptions")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FilledMapFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FilledMapSortConfiguration>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "WindowOptions")]
    pub window_options: Option<GeospatialWindowOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastConfiguration {
    #[serde(rename = "ForecastProperties")]
    pub forecast_properties: Option<TimeBasedForecastProperties>,
    #[serde(rename = "Scenario")]
    pub scenario: Option<ForecastScenario>,

}

#[derive(serde::Serialize, Default)]
pub struct TemplateSourceTemplate {
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DonutOptions {
    #[serde(rename = "DonutCenterOptions")]
    pub donut_center_options: Option<DonutCenterOptions>,
    #[serde(rename = "ArcOptions")]
    pub arc_options: Option<ArcOptions>,

}
pub type LineInterpolation = String;pub type PanelBorderStyle = String;
#[derive(serde::Serialize, Default)]
pub struct GridLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<GridLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSearchOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextAreaControl {
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: Option<String>,
    #[serde(rename = "ResizeOption")]
    pub resize_option: ResizeOption,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramSortConfiguration {
    #[serde(rename = "DestinationItemsLimit")]
    pub destination_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SourceItemsLimit")]
    pub source_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "WeightSort")]
    pub weight_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualSubtitleLabelOptions {
    #[serde(rename = "FormatText")]
    pub format_text: Option<LongFormatText>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldWells {
    #[serde(rename = "TableUnaggregatedFieldWells")]
    pub table_unaggregated_field_wells: Option<TableUnaggregatedFieldWells>,
    #[serde(rename = "TableAggregatedFieldWells")]
    pub table_aggregated_field_wells: Option<TableAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetConfiguration {
    #[serde(rename = "DataSetSchema")]
    pub data_set_schema: Option<DataSetSchema>,
    #[serde(rename = "ColumnGroupSchemaList")]
    pub column_group_schema_list: Option<Vec<ColumnGroupSchema>>,
    #[serde(rename = "Placeholder")]
    pub placeholder: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WhatIfRangeScenario {
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "EndDate")]
    pub end_date: String,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationParameterValueConfiguration {
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: Option<String>,
    #[serde(rename = "SourceField")]
    pub source_field: Option<String>,
    #[serde(rename = "SelectAllValueOptions")]
    pub select_all_value_options: Option<SelectAllValueOptions>,
    #[serde(rename = "CustomValuesConfiguration")]
    pub custom_values_configuration: Option<CustomValuesConfiguration>,

}
pub type ForecastComputationSeasonality = String;
#[derive(serde::Serialize, Default)]
pub struct NumericalAggregationFunction {
    #[serde(rename = "SimpleNumericalAggregation")]
    pub simple_numerical_aggregation: Option<SimpleNumericalAggregationFunction>,
    #[serde(rename = "PercentileAggregation")]
    pub percentile_aggregation: Option<PercentileAggregation>,

}

#[derive(serde::Serialize, Default)]
pub struct Computation {
    #[serde(rename = "MaximumMinimum")]
    pub maximum_minimum: Option<MaximumMinimumComputation>,
    #[serde(rename = "Forecast")]
    pub forecast: Option<ForecastComputation>,
    #[serde(rename = "GrowthRate")]
    pub growth_rate: Option<GrowthRateComputation>,
    #[serde(rename = "TotalAggregation")]
    pub total_aggregation: Option<TotalAggregationComputation>,
    #[serde(rename = "PeriodToDate")]
    pub period_to_date: Option<PeriodToDateComputation>,
    #[serde(rename = "PeriodOverPeriod")]
    pub period_over_period: Option<PeriodOverPeriodComputation>,
    #[serde(rename = "TopBottomRanked")]
    pub top_bottom_ranked: Option<TopBottomRankedComputation>,
    #[serde(rename = "MetricComparison")]
    pub metric_comparison: Option<MetricComparisonComputation>,
    #[serde(rename = "TopBottomMovers")]
    pub top_bottom_movers: Option<TopBottomMoversComputation>,
    #[serde(rename = "UniqueValues")]
    pub unique_values: Option<UniqueValuesComputation>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormattingOption {
    #[serde(rename = "PrimaryValue")]
    pub primary_value: Option<KPIPrimaryValueConditionalFormatting>,
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<KPIProgressBarConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOption {
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "URLStyling")]
    pub urlstyling: Option<TableFieldURLConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultGridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: GridLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterTextFieldControl {
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    pub title: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTablePaginatedReportOptions {
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    pub overflow_column_header_visibility: Option<Visibility>,
    #[serde(rename = "VerticalOverflowVisibility")]
    pub vertical_overflow_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeBasedForecastProperties {
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<f64>,
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnGroupSchema {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ColumnGroupColumnSchemaList")]
    pub column_group_column_schema_list: Option<Vec<ColumnGroupColumnSchema>>,

}
pub type TableCellImageScalingConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct WaterfallChartFieldWells {
    #[serde(rename = "WaterfallChartAggregatedFieldWells")]
    pub waterfall_chart_aggregated_field_wells: Option<WaterfallChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomContentVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<CustomContentConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HeatMapConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartConfiguration {
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WaterfallChartFieldWells>,
    #[serde(rename = "WaterfallChartOptions")]
    pub waterfall_chart_options: Option<WaterfallChartOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryAxisDisplayOptions")]
    pub category_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryAxisLabelOptions")]
    pub category_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WaterfallChartSortConfiguration>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct MaximumMinimumComputation {
    #[serde(rename = "Type")]
    pub ty: MaximumMinimumComputationType,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartAggregatedFieldWells {
    #[serde(rename = "Breakdowns")]
    pub breakdowns: Option<Vec<DimensionField>>,
    #[serde(rename = "Categories")]
    pub categories: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionFilterOperation {
    #[serde(rename = "SelectedFieldsConfiguration")]
    pub selected_fields_configuration: FilterOperationSelectedFieldsConfiguration,
    #[serde(rename = "TargetVisualsConfiguration")]
    pub target_visuals_configuration: FilterOperationTargetVisualsConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct SecondaryValueOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormattingOption {
    #[serde(rename = "Row")]
    pub row: Option<TableRowConditionalFormatting>,
    #[serde(rename = "Cell")]
    pub cell: Option<TableCellConditionalFormatting>,

}
pub type CustomContentImageScalingConfiguration = String;pub type LineChartMarkerShape = String;
#[derive(serde::Serialize, Default)]
pub struct TableUnaggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<UnaggregatedField>>,

}

#[derive(serde::Serialize, Default)]
pub struct SimpleClusterMarker {
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<PivotTableConditionalFormattingOption>>,

}
pub type TableTotalsPlacement = String;pub type TableOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct SheetElementRenderingRule {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "ConfigurationOverrides")]
    pub configuration_overrides: SheetElementConfigurationOverrides,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapSortConfiguration {
    #[serde(rename = "HeatMapRowItemsLimitConfiguration")]
    pub heat_map_row_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapRowSort")]
    pub heat_map_row_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "HeatMapColumnItemsLimitConfiguration")]
    pub heat_map_column_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapColumnSort")]
    pub heat_map_column_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadingAnimation {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterDateTimePickerControl {
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlDateTimePickerType>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeHierarchy {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterDeclaration {
    #[serde(rename = "DecimalParameterDeclaration")]
    pub decimal_parameter_declaration: Option<DecimalParameterDeclaration>,
    #[serde(rename = "StringParameterDeclaration")]
    pub string_parameter_declaration: Option<StringParameterDeclaration>,
    #[serde(rename = "IntegerParameterDeclaration")]
    pub integer_parameter_declaration: Option<IntegerParameterDeclaration>,
    #[serde(rename = "DateTimeParameterDeclaration")]
    pub date_time_parameter_declaration: Option<DateTimeParameterDeclaration>,

}

#[derive(serde::Serialize, Default)]
pub struct ComparisonConfiguration {
    #[serde(rename = "ComparisonMethod")]
    pub comparison_method: Option<ComparisonMethod>,
    #[serde(rename = "ComparisonFormat")]
    pub comparison_format: Option<ComparisonFormatConfiguration>,

}
pub type FontDecoration = String;
#[derive(serde::Serialize, Default)]
pub struct MetricComparisonComputation {
    #[serde(rename = "FromValue")]
    pub from_value: MeasureField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "TargetValue")]
    pub target_value: MeasureField,

}

#[derive(serde::Serialize, Default)]
pub struct PercentVisibleRange {
    #[serde(rename = "From")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    pub to: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryDrillDownFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "CategoryValues")]
    pub category_values: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLabelOptions {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "ApplyTo")]
    pub apply_to: Option<AxisLabelReferenceOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldBasedTooltip {
    #[serde(rename = "AggregationVisibility")]
    pub aggregation_visibility: Option<Visibility>,
    #[serde(rename = "TooltipFields")]
    pub tooltip_fields: Option<Vec<TooltipItem>>,
    #[serde(rename = "TooltipTitleType")]
    pub tooltip_title_type: Option<TooltipTitleType>,

}

#[derive(serde::Serialize, Default)]
pub struct KPISortConfiguration {
    #[serde(rename = "TrendGroupSort")]
    pub trend_group_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionLayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: FreeFormSectionLayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayOptions {
    #[serde(rename = "GridLineVisibility")]
    pub grid_line_visibility: Option<Visibility>,
    #[serde(rename = "DataOptions")]
    pub data_options: Option<AxisDataOptions>,
    #[serde(rename = "TickLabelOptions")]
    pub tick_label_options: Option<AxisTickLabelOptions>,
    #[serde(rename = "AxisLineVisibility")]
    pub axis_line_visibility: Option<Visibility>,
    #[serde(rename = "ScrollbarOptions")]
    pub scrollbar_options: Option<ScrollBarOptions>,
    #[serde(rename = "AxisOffset")]
    pub axis_offset: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetDefinition {
    #[serde(rename = "ContentType")]
    pub content_type: Option<SheetContentType>,
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "TextBoxes")]
    pub text_boxes: Option<Vec<SheetTextBox>>,
    #[serde(rename = "Visuals")]
    pub visuals: Option<Vec<Visual>>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "SheetControlLayouts")]
    pub sheet_control_layouts: Option<Vec<SheetControlLayout>>,
    #[serde(rename = "FilterControls")]
    pub filter_controls: Option<Vec<FilterControl>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ParameterControls")]
    pub parameter_controls: Option<Vec<ParameterControl>>,
    #[serde(rename = "Layouts")]
    pub layouts: Option<Vec<Layout>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomMoversComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "MoverSize")]
    pub mover_size: Option<f64>,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<TopBottomSortOrder>,

}

#[derive(serde::Serialize, Default)]
pub struct FontWeight {
    #[serde(rename = "Name")]
    pub name: Option<FontWeightName>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericEqualityFilter {
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "MatchOperator")]
    pub match_operator: NumericEqualityMatchOperator,

}

#[derive(serde::Serialize, Default)]
pub struct VisibleRangeOptions {
    #[serde(rename = "PercentRange")]
    pub percent_range: Option<PercentVisibleRange>,

}
pub type NumericEqualityMatchOperator = String;
#[derive(serde::Serialize, Default)]
pub struct WaterfallChartOptions {
    #[serde(rename = "TotalBarLabel")]
    pub total_bar_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialPointStyleOptions {
    #[serde(rename = "ClusterMarkerConfiguration")]
    pub cluster_marker_configuration: Option<ClusterMarkerConfiguration>,
    #[serde(rename = "SelectedPointStyle")]
    pub selected_point_style: Option<GeospatialSelectedPointStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldCustomTextContent {
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: FontConfiguration,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "InsightConfiguration")]
    pub insight_configuration: Option<InsightConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelType {
    #[serde(rename = "MinimumLabelType")]
    pub minimum_label_type: Option<MinimumLabelType>,
    #[serde(rename = "DataPathLabelType")]
    pub data_path_label_type: Option<DataPathLabelType>,
    #[serde(rename = "RangeEndsLabelType")]
    pub range_ends_label_type: Option<RangeEndsLabelType>,
    #[serde(rename = "FieldLabelType")]
    pub field_label_type: Option<FieldLabelType>,
    #[serde(rename = "MaximumLabelType")]
    pub maximum_label_type: Option<MaximumLabelType>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterControl {
    #[serde(rename = "List")]
    pub list: Option<ParameterListControl>,
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<ParameterDateTimePickerControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<ParameterDropDownControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<ParameterTextAreaControl>,
    #[serde(rename = "TextField")]
    pub text_field: Option<ParameterTextFieldControl>,
    #[serde(rename = "Slider")]
    pub slider: Option<ParameterSliderControl>,

}
pub type SortDirection = String;
#[derive(serde::Serialize, Default)]
pub struct ExcludePeriodConfiguration {
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Granularity")]
    pub granularity: TimeGranularity,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct RangeEndsLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormattingOption {
    #[serde(rename = "Shape")]
    pub shape: FilledMapShapeConditionalFormatting,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ScatterPlotConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DateMeasureField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<DateAggregationFunction>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    pub screen_canvas_size_options: Option<FreeFormLayoutScreenCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DropDownControlDisplayOptions {
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}
pub type WordCloudWordPadding = String;pub type MaximumMinimumComputationType = String;pub type SelectedFieldOptions = String;
#[derive(serde::Serialize, Default)]
pub struct BarChartFieldWells {
    #[serde(rename = "BarChartAggregatedFieldWells")]
    pub bar_chart_aggregated_field_wells: Option<BarChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionNavigationOperation {
    #[serde(rename = "LocalNavigationConfiguration")]
    pub local_navigation_configuration: Option<LocalNavigationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLine {
    #[serde(rename = "DataConfiguration")]
    pub data_configuration: ReferenceLineDataConfiguration,
    #[serde(rename = "LabelConfiguration")]
    pub label_configuration: Option<ReferenceLineLabelConfiguration>,
    #[serde(rename = "StyleConfiguration")]
    pub style_configuration: Option<ReferenceLineStyleConfiguration>,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct Sheet {
    #[serde(rename = "SheetId")]
    pub sheet_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapStyleOptions {
    #[serde(rename = "BaseMapStyle")]
    pub base_map_style: Option<BaseMapStyleType>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnTooltipItem {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Aggregation")]
    pub aggregation: Option<AggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayRange {
    #[serde(rename = "MinMax")]
    pub min_max: Option<AxisDisplayMinMaxRange>,
    #[serde(rename = "DataDriven")]
    pub data_driven: Option<AxisDisplayDataDrivenRange>,

}
pub type ReferenceLineValueLabelRelativePosition = String;
#[derive(serde::Serialize, Default)]
pub struct TotalAggregationComputation {
    #[serde(rename = "Value")]
    pub value: MeasureField,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}
pub type SimpleNumericalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct RelativeDateTimeControlDisplayOptions {
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingColor {
    #[serde(rename = "Solid")]
    pub solid: Option<ConditionalFormattingSolidColor>,
    #[serde(rename = "Gradient")]
    pub gradient: Option<ConditionalFormattingGradientColor>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIcon {
    #[serde(rename = "IconSet")]
    pub icon_set: Option<ConditionalFormattingIconSet>,
    #[serde(rename = "CustomCondition")]
    pub custom_condition: Option<ConditionalFormattingCustomIconCondition>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathSort {
    #[serde(rename = "Direction")]
    pub direction: SortDirection,
    #[serde(rename = "SortPaths")]
    pub sort_paths: Vec<DataPathValue>,

}
pub type NumberScale = String;
#[derive(serde::Serialize, Default)]
pub struct FunnelChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,

}
pub type LegendPosition = String;
#[derive(serde::Serialize, Default)]
pub struct BodySectionContent {
    #[serde(rename = "Layout")]
    pub layout: Option<SectionLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CascadingControlSource {
    #[serde(rename = "SourceSheetControlId")]
    pub source_sheet_control_id: Option<String>,
    #[serde(rename = "ColumnToMatch")]
    pub column_to_match: Option<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct DateDimensionField {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "DateGranularity")]
    pub date_granularity: Option<TimeGranularity>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapFieldWells {
    #[serde(rename = "FilledMapAggregatedFieldWells")]
    pub filled_map_aggregated_field_wells: Option<FilledMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartFieldWells {
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableSortConfiguration {
    #[serde(rename = "FieldSortOptions")]
    pub field_sort_options: Option<Vec<PivotFieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct SelectedSheetsFilterScopeConfiguration {
    #[serde(rename = "SheetVisualScopingConfigurations")]
    pub sheet_visual_scoping_configurations: Option<Vec<SheetVisualScopingConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudOptions {
    #[serde(rename = "WordScaling")]
    pub word_scaling: Option<WordCloudWordScaling>,
    #[serde(rename = "CloudLayout")]
    pub cloud_layout: Option<WordCloudCloudLayout>,
    #[serde(rename = "WordCasing")]
    pub word_casing: Option<WordCloudWordCasing>,
    #[serde(rename = "WordPadding")]
    pub word_padding: Option<WordCloudWordPadding>,
    #[serde(rename = "WordOrientation")]
    pub word_orientation: Option<WordCloudWordOrientation>,
    #[serde(rename = "MaximumStringLength")]
    pub maximum_string_length: Option<f64>,

}
pub type CategoricalAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct BarChartAggregatedFieldWells {
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "SmallMultiples")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionURLOperation {
    #[serde(rename = "URLTarget")]
    pub urltarget: URLTargetConfiguration,
    #[serde(rename = "URLTemplate")]
    pub urltemplate: String,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotFieldSortOptions {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "SortBy")]
    pub sort_by: PivotTableSortBy,

}
pub type ReferenceLineLabelVerticalPosition = String;
#[derive(serde::Serialize, Default)]
pub struct SectionPageBreakConfiguration {
    #[serde(rename = "After")]
    pub after: Option<SectionAfterPageBreak>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetElementConfigurationOverrides {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct DataColor {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutElement {
    #[serde(rename = "ColumnIndex")]
    pub column_index: Option<f64>,
    #[serde(rename = "ColumnSpan")]
    pub column_span: f64,
    #[serde(rename = "RowSpan")]
    pub row_span: f64,
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "RowIndex")]
    pub row_index: Option<f64>,
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,

}
pub type MapZoomMode = String;
#[derive(serde::Serialize, Default)]
pub struct FilterListControl {
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotFieldWells {
    #[serde(rename = "ScatterPlotCategoricallyAggregatedFieldWells")]
    pub scatter_plot_categorically_aggregated_field_wells: Option<ScatterPlotCategoricallyAggregatedFieldWells>,
    #[serde(rename = "ScatterPlotUnaggregatedFieldWells")]
    pub scatter_plot_unaggregated_field_wells: Option<ScatterPlotUnaggregatedFieldWells>,

}
pub type BaseMapStyleType = String;
#[derive(serde::Serialize, Default)]
pub struct GaugeChartConfiguration {
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "GaugeChartOptions")]
    pub gauge_chart_options: Option<GaugeChartOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<GaugeChartFieldWells>,
    #[serde(rename = "TooltipOptions")]
    pub tooltip_options: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartSortConfiguration {
    #[serde(rename = "ColorItemsLimitConfiguration")]
    pub color_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimitConfiguration")]
    pub category_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BoxPlotChartConfiguration>,

}
pub type TargetVisualOptions = String;
#[derive(serde::Serialize, Default)]
pub struct NumericSeparatorConfiguration {
    #[serde(rename = "ThousandsSeparator")]
    pub thousands_separator: Option<ThousandSeparatorOptions>,
    #[serde(rename = "DecimalSeparator")]
    pub decimal_separator: Option<NumericSeparatorSymbol>,

}
pub type DataLabelOverlap = String;
#[derive(serde::Serialize, Default)]
pub struct AxisLogarithmicScale {
    #[serde(rename = "Base")]
    pub base: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartConfiguration {
    #[serde(rename = "DataLabelOptions")]
    pub data_label_options: Option<FunnelChartDataLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<FunnelChartFieldWells>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<FunnelChartSortConfiguration>,

}
pub type CategoryFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct ClusterMarkerConfiguration {
    #[serde(rename = "ClusterMarker")]
    pub cluster_marker: Option<ClusterMarker>,

}
pub type SelectAllValueOptions = String;
#[derive(serde::Serialize, Default)]
pub struct HistogramBinOptions {
    #[serde(rename = "SelectedBinType")]
    pub selected_bin_type: Option<HistogramBinType>,
    #[serde(rename = "StartValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "BinCount")]
    pub bin_count: Option<BinCountOptions>,
    #[serde(rename = "BinWidth")]
    pub bin_width: Option<BinWidthOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,

}

#[derive(serde::Serialize, Default)]
pub struct TextAreaControlDisplayOptions {
    #[serde(rename = "PlaceholderOptions")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TableRowConditionalFormatting {
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct BinWidthOptions {
    #[serde(rename = "BinCountLimit")]
    pub bin_count_limit: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElementBorderStyle {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnSchema {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
    #[serde(rename = "GeographicRole")]
    pub geographic_role: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramChartConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<SankeyDiagramSortConfiguration>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<SankeyDiagramFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramAggregatedFieldWells {
    #[serde(rename = "Source")]
    pub source: Option<Vec<DimensionField>>,
    #[serde(rename = "Destination")]
    pub destination: Option<Vec<DimensionField>>,
    #[serde(rename = "Weight")]
    pub weight: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<GaugeChartConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct ForecastScenario {
    #[serde(rename = "WhatIfPointScenario")]
    pub what_if_point_scenario: Option<WhatIfPointScenario>,
    #[serde(rename = "WhatIfRangeScenario")]
    pub what_if_range_scenario: Option<WhatIfRangeScenario>,

}

#[derive(serde::Serialize, Default)]
pub struct PeriodOverPeriodComputation {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    #[serde(rename = "Principal")]
    pub principal: String,
    #[serde(rename = "Resource")]
    pub resource: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StringDefaultValues {
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}

#[derive(serde::Serialize, Default)]
pub struct TextControlPlaceholderOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type HistogramBinType = String;
#[derive(serde::Serialize, Default)]
pub struct BoxPlotSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartSeriesSettings {
    #[serde(rename = "AreaStyleSettings")]
    pub area_style_settings: Option<RadarChartAreaStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperationTargetVisualsConfiguration {
    #[serde(rename = "SameSheetTargetVisualConfiguration")]
    pub same_sheet_target_visual_configuration: Option<SameSheetTargetVisualConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnGroupColumnSchema {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
pub type DataLabelPosition = String;
#[derive(serde::Serialize, Default)]
pub struct SheetControlLayoutConfiguration {
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GridLayoutConfiguration {
    #[serde(rename = "Elements")]
    pub elements: Vec<GridLayoutElement>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: Option<GridLayoutCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct LegendOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Position")]
    pub position: Option<LegendPosition>,
    #[serde(rename = "Title")]
    pub title: Option<LabelOptions>,
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomFilter {
    #[serde(rename = "Limit")]
    pub limit: Option<f64>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,
    #[serde(rename = "AggregationSortConfigurations")]
    pub aggregation_sort_configurations: Vec<AggregationSortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingCustomIconOptions {
    #[serde(rename = "Icon")]
    pub icon: Option<Icon>,
    #[serde(rename = "UnicodeIcon")]
    pub unicode_icon: Option<String>,

}
pub type RelativeDateType = String;
#[derive(serde::Serialize, Default)]
pub struct UnaggregatedField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldWells {
    #[serde(rename = "PivotTableAggregatedFieldWells")]
    pub pivot_table_aggregated_field_wells: Option<PivotTableAggregatedFieldWells>,

}
pub type ReferenceLineLabelHorizontalPosition = String;
#[derive(serde::Serialize, Default)]
pub struct BoxPlotOptions {
    #[serde(rename = "AllDataPointsVisibility")]
    pub all_data_points_visibility: Option<Visibility>,
    #[serde(rename = "OutlierVisibility")]
    pub outlier_visibility: Option<Visibility>,
    #[serde(rename = "StyleOptions")]
    pub style_options: Option<BoxPlotStyleOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalDefaultValues {
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisConfiguration {
    #[serde(rename = "Range")]
    pub range: Option<ArcAxisDisplayRange>,
    #[serde(rename = "ReserveRange")]
    pub reserve_range: Option<f64>,

}
pub type ArcThicknessOptions = String;
#[derive(serde::Serialize, Default)]
pub struct PeriodToDateComputation {
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "PeriodTimeGranularity")]
    pub period_time_granularity: Option<TimeGranularity>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutElement {
    #[serde(rename = "RenderingRules")]
    pub rendering_rules: Option<Vec<SheetElementRenderingRule>>,
    #[serde(rename = "SelectedBorderStyle")]
    pub selected_border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "Width")]
    pub width: String,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "ElementId")]
    pub element_id: String,
    #[serde(rename = "YAxisLocation")]
    pub yaxis_location: String,
    #[serde(rename = "ElementType")]
    pub element_type: LayoutElementType,
    #[serde(rename = "XAxisLocation")]
    pub xaxis_location: String,
    #[serde(rename = "Height")]
    pub height: String,
    #[serde(rename = "BorderStyle")]
    pub border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "BackgroundStyle")]
    pub background_style: Option<FreeFormLayoutElementBackgroundStyle>,
    #[serde(rename = "LoadingAnimation")]
    pub loading_animation: Option<LoadingAnimation>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapSortConfiguration {
    #[serde(rename = "TreeMapSort")]
    pub tree_map_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "TreeMapGroupItemsLimitConfiguration")]
    pub tree_map_group_items_limit_configuration: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualCustomAction {
    #[serde(rename = "ActionOperations")]
    pub action_operations: Vec<VisualCustomActionOperation>,
    #[serde(rename = "Trigger")]
    pub trigger: VisualCustomActionTrigger,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CustomActionId")]
    pub custom_action_id: String,
    #[serde(rename = "Status")]
    pub status: Option<WidgetStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellStyle {
    #[serde(rename = "TextWrap")]
    pub text_wrap: Option<TextWrap>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "FontConfiguration")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "VerticalTextAlignment")]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<f64>,
    #[serde(rename = "Border")]
    pub border: Option<GlobalTableBorderOptions>,
    #[serde(rename = "HorizontalTextAlignment")]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapFieldWells {
    #[serde(rename = "HeatMapAggregatedFieldWells")]
    pub heat_map_aggregated_field_wells: Option<HeatMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConfiguration {
    #[serde(rename = "TableOptions")]
    pub table_options: Option<PivotTableOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<PivotTableFieldWells>,
    #[serde(rename = "FieldOptions")]
    pub field_options: Option<PivotTableFieldOptions>,
    #[serde(rename = "PaginatedReportOptions")]
    pub paginated_report_options: Option<PivotTablePaginatedReportOptions>,
    #[serde(rename = "TotalOptions")]
    pub total_options: Option<PivotTableTotalOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<PivotTableSortConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayDataDrivenRange {

}

#[derive(serde::Serialize, Default)]
pub struct TableBorderOptions {
    #[serde(rename = "Thickness")]
    pub thickness: Option<f64>,
    #[serde(rename = "Style")]
    pub style: Option<TableBorderStyle>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudFieldWells {
    #[serde(rename = "WordCloudAggregatedFieldWells")]
    pub word_cloud_aggregated_field_wells: Option<WordCloudAggregatedFieldWells>,

}
pub type VerticalTextAlignment = String;
#[derive(serde::Serialize, Default)]
pub struct FieldTooltipItem {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}
pub type WordCloudWordScaling = String;pub type AxisBinding = String;pub type BarsArrangement = String;pub type CrossDatasetTypes = String;
#[derive(serde::Serialize, Default)]
pub struct PieChartVisual {
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PieChartConfiguration>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct RollingDateConfiguration {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSort {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Direction")]
    pub direction: SortDirection,

}
pub type PivotTableConditionalFormattingScopeRole = String;
#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutCanvasSizeOptions {
    #[serde(rename = "PaperCanvasSizeOptions")]
    pub paper_canvas_size_options: Option<SectionBasedLayoutPaperCanvasSizeOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PieChartFieldWells {
    #[serde(rename = "PieChartAggregatedFieldWells")]
    pub pie_chart_aggregated_field_wells: Option<PieChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericAxisOptions {
    #[serde(rename = "Scale")]
    pub scale: Option<AxisScale>,
    #[serde(rename = "Range")]
    pub range: Option<AxisDisplayRange>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetReference {
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: String,
    #[serde(rename = "DataSetPlaceholder")]
    pub data_set_placeholder: String,

}

#[derive(serde::Serialize, Default)]
pub struct TrendArrowOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartAggregatedFieldWells {
    #[serde(rename = "BarValues")]
    pub bar_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "LineValues")]
    pub line_values: Option<Vec<MeasureField>>,

}
pub type ReferenceLinePatternType = String;
#[derive(serde::Serialize, Default)]
pub struct DateTimePickerControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartAggregatedFieldWells {
    #[serde(rename = "Color")]
    pub color: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartOptions {
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "ArcAxis")]
    pub arc_axis: Option<ArcAxisConfiguration>,
    #[serde(rename = "Arc")]
    pub arc: Option<ArcConfiguration>,

}
pub type TopBottomComputationType = String;
#[derive(serde::Serialize, Default)]
pub struct BarChartConfiguration {
    #[serde(rename = "BarsArrangement")]
    pub bars_arrangement: Option<BarsArrangement>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "ValueAxis")]
    pub value_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BarChartFieldWells>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BarChartSortConfiguration>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<BarChartOrientation>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ValueLabelOptions")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<RadarChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FormatConfiguration {
    #[serde(rename = "NumberFormatConfiguration")]
    pub number_format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "DateTimeFormatConfiguration")]
    pub date_time_format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "StringFormatConfiguration")]
    pub string_format_configuration: Option<StringFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapFieldWells {
    #[serde(rename = "GeospatialMapAggregatedFieldWells")]
    pub geospatial_map_aggregated_field_wells: Option<GeospatialMapAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct LongFormatText {
    #[serde(rename = "PlainText")]
    pub plain_text: Option<String>,
    #[serde(rename = "RichText")]
    pub rich_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramFieldWells {
    #[serde(rename = "SankeyDiagramAggregatedFieldWells")]
    pub sankey_diagram_aggregated_field_wells: Option<SankeyDiagramAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct TooltipItem {
    #[serde(rename = "ColumnTooltipItem")]
    pub column_tooltip_item: Option<ColumnTooltipItem>,
    #[serde(rename = "FieldTooltipItem")]
    pub field_tooltip_item: Option<FieldTooltipItem>,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TreeMapConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ScatterPlotUnaggregatedFieldWells {
    #[serde(rename = "YAxis")]
    pub yaxis: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "XAxis")]
    pub xaxis: Option<Vec<DimensionField>>,

}
pub type RadarChartShape = String;pub type CategoryFilterMatchOperator = String;
#[derive(serde::Serialize, Default)]
pub struct HistogramFieldWells {
    #[serde(rename = "HistogramAggregatedFieldWells")]
    pub histogram_aggregated_field_wells: Option<HistogramAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellImageSizingConfiguration {
    #[serde(rename = "TableCellImageScalingConfiguration")]
    pub table_cell_image_scaling_configuration: Option<TableCellImageScalingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcAxisDisplayRange {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct AggregationFunction {
    #[serde(rename = "CategoricalAggregationFunction")]
    pub categorical_aggregation_function: Option<CategoricalAggregationFunction>,
    #[serde(rename = "NumericalAggregationFunction")]
    pub numerical_aggregation_function: Option<NumericalAggregationFunction>,
    #[serde(rename = "DateAggregationFunction")]
    pub date_aggregation_function: Option<DateAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct MaximumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type VisualCustomActionTrigger = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableOptions {
    #[serde(rename = "CellStyle")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "RowHeaderStyle")]
    pub row_header_style: Option<TableCellStyle>,
    #[serde(rename = "ColumnNamesVisibility")]
    pub column_names_visibility: Option<Visibility>,
    #[serde(rename = "MetricPlacement")]
    pub metric_placement: Option<PivotTableMetricPlacement>,
    #[serde(rename = "ToggleButtonsVisibility")]
    pub toggle_buttons_visibility: Option<Visibility>,
    #[serde(rename = "SingleMetricVisibility")]
    pub single_metric_visibility: Option<Visibility>,
    #[serde(rename = "RowFieldNamesStyle")]
    pub row_field_names_style: Option<TableCellStyle>,
    #[serde(rename = "RowAlternateColorOptions")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "ColumnHeaderStyle")]
    pub column_header_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartAreaStyleSettings {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type TableFieldIconSetType = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterSliderControl {
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct AnchorDateConfiguration {
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "AnchorOption")]
    pub anchor_option: Option<AnchorOption>,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartFieldWells {
    #[serde(rename = "ComboChartAggregatedFieldWells")]
    pub combo_chart_aggregated_field_wells: Option<ComboChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct NumberDisplayFormatConfiguration {
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,

}

#[derive(serde::Serialize, Default)]
pub struct SeriesItem {
    #[serde(rename = "FieldSeriesItem")]
    pub field_series_item: Option<FieldSeriesItem>,
    #[serde(rename = "DataFieldSeriesItem")]
    pub data_field_series_item: Option<DataFieldSeriesItem>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSortOptions {
    #[serde(rename = "FieldSort")]
    pub field_sort: Option<FieldSort>,
    #[serde(rename = "ColumnSort")]
    pub column_sort: Option<ColumnSort>,

}
pub type ResizeOption = String;
#[derive(serde::Serialize, Default)]
pub struct HeatMapAggregatedFieldWells {
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<DimensionField>>,
    #[serde(rename = "Rows")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotStyleOptions {
    #[serde(rename = "FillStyle")]
    pub fill_style: Option<BoxPlotFillStyle>,

}
pub type URLTargetConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct MappedDataSetParameter {
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "DataSetParameterName")]
    pub data_set_parameter_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComboChartVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<ComboChartConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ProgressBarOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type WordCloudCloudLayout = String;pub type GeospatialSelectedPointStyle = String;pub type TemplateErrorType = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterDropDownControl {
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "SourceParameterName")]
    pub source_parameter_name: String,
    #[serde(rename = "ParameterControlId")]
    pub parameter_control_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "Title")]
    pub title: String,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartSeriesSettings {
    #[serde(rename = "LineStyleSettings")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionBasedLayoutConfiguration {
    #[serde(rename = "HeaderSections")]
    pub header_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "FooterSections")]
    pub footer_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "BodySections")]
    pub body_sections: Vec<BodySectionConfiguration>,
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct AxisDisplayMinMaxRange {
    #[serde(rename = "Minimum")]
    pub minimum: Option<f64>,
    #[serde(rename = "Maximum")]
    pub maximum: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlDisplayOptions {
    #[serde(rename = "SearchOptions")]
    pub search_options: Option<ListControlSearchOptions>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerDefaultValues {
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<f64>>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalTableBorderOptions {
    #[serde(rename = "SideSpecificBorder")]
    pub side_specific_border: Option<TableSideBorderOptions>,
    #[serde(rename = "UniformBorder")]
    pub uniform_border: Option<TableBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TopBottomRankedComputation {
    #[serde(rename = "ResultSize")]
    pub result_size: Option<f64>,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Type")]
    pub ty: TopBottomComputationType,

}

#[derive(serde::Serialize, Default)]
pub struct TreeMapConfiguration {
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<TreeMapFieldWells>,
    #[serde(rename = "GroupLabelOptions")]
    pub group_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<TreeMapSortConfiguration>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "SizeLabelOptions")]
    pub size_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    pub color_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualPalette {
    #[serde(rename = "ChartColor")]
    pub chart_color: Option<String>,
    #[serde(rename = "ColorMap")]
    pub color_map: Option<Vec<DataPathColor>>,

}

#[derive(serde::Serialize, Default)]
pub struct EmptyVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableCellConditionalFormatting {
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,
    #[serde(rename = "Scope")]
    pub scope: Option<PivotTableConditionalFormattingScope>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct GeospatialMapVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GeospatialMapConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoryFilterConfiguration {
    #[serde(rename = "CustomFilterListConfiguration")]
    pub custom_filter_list_configuration: Option<CustomFilterListConfiguration>,
    #[serde(rename = "CustomFilterConfiguration")]
    pub custom_filter_configuration: Option<CustomFilterConfiguration>,
    #[serde(rename = "FilterListConfiguration")]
    pub filter_list_configuration: Option<FilterListConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CategoricalDimensionField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: Option<String>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<StringFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DateAxisOptions {
    #[serde(rename = "MissingDateVisibility")]
    pub missing_date_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct SliderControlDisplayOptions {
    #[serde(rename = "TitleOptions")]
    pub title_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOptions {
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<PivotTableFieldOption>>,
    #[serde(rename = "DataPathOptions")]
    pub data_path_options: Option<Vec<PivotTableDataPathOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct MissingDataConfiguration {
    #[serde(rename = "TreatmentOption")]
    pub treatment_option: Option<MissingDataTreatmentOption>,

}
pub type LayoutElementType = String;
#[derive(serde::Serialize, Default)]
pub struct WordCloudAggregatedFieldWells {
    #[serde(rename = "Size")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,

}

#[derive(serde::Serialize, Default)]
pub struct Entity {
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct UniqueValuesComputation {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Category")]
    pub category: DimensionField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Visual {
    #[serde(rename = "TableVisual")]
    pub table_visual: Option<TableVisual>,
    #[serde(rename = "HeatMapVisual")]
    pub heat_map_visual: Option<HeatMapVisual>,
    #[serde(rename = "ScatterPlotVisual")]
    pub scatter_plot_visual: Option<ScatterPlotVisual>,
    #[serde(rename = "WaterfallVisual")]
    pub waterfall_visual: Option<WaterfallVisual>,
    #[serde(rename = "HistogramVisual")]
    pub histogram_visual: Option<HistogramVisual>,
    #[serde(rename = "SankeyDiagramVisual")]
    pub sankey_diagram_visual: Option<SankeyDiagramVisual>,
    #[serde(rename = "KPIVisual")]
    pub kpivisual: Option<KPIVisual>,
    #[serde(rename = "CustomContentVisual")]
    pub custom_content_visual: Option<CustomContentVisual>,
    #[serde(rename = "RadarChartVisual")]
    pub radar_chart_visual: Option<RadarChartVisual>,
    #[serde(rename = "GaugeChartVisual")]
    pub gauge_chart_visual: Option<GaugeChartVisual>,
    #[serde(rename = "EmptyVisual")]
    pub empty_visual: Option<EmptyVisual>,
    #[serde(rename = "InsightVisual")]
    pub insight_visual: Option<InsightVisual>,
    #[serde(rename = "PieChartVisual")]
    pub pie_chart_visual: Option<PieChartVisual>,
    #[serde(rename = "LineChartVisual")]
    pub line_chart_visual: Option<LineChartVisual>,
    #[serde(rename = "BarChartVisual")]
    pub bar_chart_visual: Option<BarChartVisual>,
    #[serde(rename = "FunnelChartVisual")]
    pub funnel_chart_visual: Option<FunnelChartVisual>,
    #[serde(rename = "BoxPlotVisual")]
    pub box_plot_visual: Option<BoxPlotVisual>,
    #[serde(rename = "FilledMapVisual")]
    pub filled_map_visual: Option<FilledMapVisual>,
    #[serde(rename = "PivotTableVisual")]
    pub pivot_table_visual: Option<PivotTableVisual>,
    #[serde(rename = "TreeMapVisual")]
    pub tree_map_visual: Option<TreeMapVisual>,
    #[serde(rename = "GeospatialMapVisual")]
    pub geospatial_map_visual: Option<GeospatialMapVisual>,
    #[serde(rename = "WordCloudVisual")]
    pub word_cloud_visual: Option<WordCloudVisual>,
    #[serde(rename = "ComboChartVisual")]
    pub combo_chart_visual: Option<ComboChartVisual>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultNewSheetConfiguration {
    #[serde(rename = "InteractiveLayoutConfiguration")]
    pub interactive_layout_configuration: Option<DefaultInteractiveLayoutConfiguration>,
    #[serde(rename = "PaginatedLayoutConfiguration")]
    pub paginated_layout_configuration: Option<DefaultPaginatedLayoutConfiguration>,
    #[serde(rename = "SheetContentType")]
    pub sheet_content_type: Option<SheetContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<KPIConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamicDefaultValue {
    #[serde(rename = "GroupNameColumn")]
    pub group_name_column: Option<ColumnIdentifier>,
    #[serde(rename = "DefaultValueColumn")]
    pub default_value_column: ColumnIdentifier,
    #[serde(rename = "UserNameColumn")]
    pub user_name_column: Option<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct TooltipOptions {
    #[serde(rename = "TooltipVisibility")]
    pub tooltip_visibility: Option<Visibility>,
    #[serde(rename = "FieldBasedTooltip")]
    pub field_based_tooltip: Option<FieldBasedTooltip>,
    #[serde(rename = "SelectedTooltipType")]
    pub selected_tooltip_type: Option<SelectedTooltipType>,

}
pub type NumericFilterSelectAllOptions = String;
#[derive(serde::Serialize, Default)]
pub struct GradientStop {
    #[serde(rename = "DataValue")]
    pub data_value: Option<f64>,
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "GradientOffset")]
    pub gradient_offset: f64,

}

#[derive(serde::Serialize, Default)]
pub struct CascadingControlConfiguration {
    #[serde(rename = "SourceControls")]
    pub source_controls: Option<Vec<CascadingControlSource>>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeFormatConfiguration {
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalPlacesConfiguration {
    #[serde(rename = "DecimalPlaces")]
    pub decimal_places: f64,

}

#[derive(serde::Serialize, Default)]
pub struct CustomValuesConfiguration {
    #[serde(rename = "IncludeNullValue")]
    pub include_null_value: Option<bool>,
    #[serde(rename = "CustomValues")]
    pub custom_values: CustomParameterValues,

}

#[derive(serde::Serialize, Default)]
pub struct ColorsConfiguration {
    #[serde(rename = "CustomColors")]
    pub custom_colors: Option<Vec<CustomColor>>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultPaginatedLayoutConfiguration {
    #[serde(rename = "SectionBased")]
    pub section_based: Option<DefaultSectionBasedLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldSeriesItem {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Vec<FilledMapConditionalFormattingOption>,

}

#[derive(serde::Serialize, Default)]
pub struct DecimalValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,

}

#[derive(serde::Serialize, Default)]
pub struct FontConfiguration {
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<FontSize>,
    #[serde(rename = "FontDecoration")]
    pub font_decoration: Option<FontDecoration>,
    #[serde(rename = "FontWeight")]
    pub font_weight: Option<FontWeight>,
    #[serde(rename = "FontStyle")]
    pub font_style: Option<FontStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct GradientColor {
    #[serde(rename = "Stops")]
    pub stops: Option<Vec<GradientStop>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomActionSetParametersOperation {
    #[serde(rename = "ParameterValueConfigurations")]
    pub parameter_value_configurations: Vec<SetParameterValueConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnConfiguration {
    #[serde(rename = "Role")]
    pub role: Option<ColumnRole>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<FormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "ColorsConfiguration")]
    pub colors_configuration: Option<ColorsConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ListControlSelectAllOptions {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDynamicDataConfiguration {
    #[serde(rename = "Calculation")]
    pub calculation: NumericalAggregationFunction,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "MeasureAggregationFunction")]
    pub measure_aggregation_function: AggregationFunction,

}

#[derive(serde::Serialize, Default)]
pub struct BinCountOptions {
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStyleConfiguration {
    #[serde(rename = "Pattern")]
    pub pattern: Option<ReferenceLinePatternType>,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RelativeDatesFilter {
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,
    #[serde(rename = "RelativeDateValue")]
    pub relative_date_value: Option<f64>,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "MinimumGranularity")]
    pub minimum_granularity: Option<TimeGranularity>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "AnchorDateConfiguration")]
    pub anchor_date_configuration: AnchorDateConfiguration,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RelativeDateType")]
    pub relative_date_type: RelativeDateType,
    #[serde(rename = "ExcludePeriodConfiguration")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldOptions {
    #[serde(rename = "Order")]
    pub order: Option<Vec<String>>,
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<Vec<TableFieldOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct WaterfallChartSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "BreakdownItemsLimit")]
    pub breakdown_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct WhatIfPointScenario {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotFieldWells {
    #[serde(rename = "BoxPlotAggregatedFieldWells")]
    pub box_plot_aggregated_field_wells: Option<BoxPlotAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct GrowthRateComputation {
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "PeriodSize")]
    pub period_size: Option<f64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct TableAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}

#[derive(serde::Serialize, Default)]
pub struct StringFormatConfiguration {
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcOptions {
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThickness>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIProgressBarConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineValueLabelConfiguration {
    #[serde(rename = "RelativePosition")]
    pub relative_position: Option<ReferenceLineValueLabelRelativePosition>,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumericFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathValue {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FieldValue")]
    pub field_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct KPIOptions {
    #[serde(rename = "TrendArrows")]
    pub trend_arrows: Option<TrendArrowOptions>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "SecondaryValue")]
    pub secondary_value: Option<SecondaryValueOptions>,
    #[serde(rename = "SecondaryValueFontConfiguration")]
    pub secondary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "ProgressBar")]
    pub progress_bar: Option<ProgressBarOptions>,
    #[serde(rename = "PrimaryValueDisplayType")]
    pub primary_value_display_type: Option<PrimaryValueDisplayType>,
    #[serde(rename = "Comparison")]
    pub comparison: Option<ComparisonConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisLabelReferenceOptions {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct MeasureField {
    #[serde(rename = "NumericalMeasureField")]
    pub numerical_measure_field: Option<NumericalMeasureField>,
    #[serde(rename = "CalculatedMeasureField")]
    pub calculated_measure_field: Option<CalculatedMeasureField>,
    #[serde(rename = "DateMeasureField")]
    pub date_measure_field: Option<DateMeasureField>,
    #[serde(rename = "CategoricalMeasureField")]
    pub categorical_measure_field: Option<CategoricalMeasureField>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterDropDownControl {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "SelectableValues")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "CascadingControlConfiguration")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlListType>,

}

#[derive(serde::Serialize, Default)]
pub struct TableSideBorderOptions {
    #[serde(rename = "Top")]
    pub top: Option<TableBorderOptions>,
    #[serde(rename = "InnerHorizontal")]
    pub inner_horizontal: Option<TableBorderOptions>,
    #[serde(rename = "InnerVertical")]
    pub inner_vertical: Option<TableBorderOptions>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<TableBorderOptions>,
    #[serde(rename = "Left")]
    pub left: Option<TableBorderOptions>,
    #[serde(rename = "Right")]
    pub right: Option<TableBorderOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldSubtotalOptions {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericEqualityDrillDownFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FilledMapConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<FilledMapConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomNarrativeOptions {
    #[serde(rename = "Narrative")]
    pub narrative: String,

}

#[derive(serde::Serialize, Default)]
pub struct SubtotalOptions {
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "FieldLevel")]
    pub field_level: Option<PivotTableSubtotalLevel>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "FieldLevelOptions")]
    pub field_level_options: Option<Vec<PivotTableFieldSubtotalOptions>>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudSortConfiguration {
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AxisTickLabelOptions {
    #[serde(rename = "RotationAngle")]
    pub rotation_angle: Option<f64>,
    #[serde(rename = "LabelOptions")]
    pub label_options: Option<LabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingSolidColor {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Color")]
    pub color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeValueWhenUnsetConfiguration {
    #[serde(rename = "ValueWhenUnsetOption")]
    pub value_when_unset_option: Option<ValueWhenUnsetOption>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HeatMapConfiguration {
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<HeatMapSortConfiguration>,
    #[serde(rename = "ColumnLabelOptions")]
    pub column_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<HeatMapFieldWells>,
    #[serde(rename = "RowLabelOptions")]
    pub row_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ColorScale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct BodySectionConfiguration {
    #[serde(rename = "Content")]
    pub content: BodySectionContent,
    #[serde(rename = "Style")]
    pub style: Option<SectionStyle>,
    #[serde(rename = "PageBreakConfiguration")]
    pub page_break_configuration: Option<SectionPageBreakConfiguration>,
    #[serde(rename = "SectionId")]
    pub section_id: String,

}
pub type LineChartLineStyle = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableFieldOption {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct RadarChartFieldWells {
    #[serde(rename = "RadarChartAggregatedFieldWells")]
    pub radar_chart_aggregated_field_wells: Option<RadarChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct ItemsLimitConfiguration {
    #[serde(rename = "ItemsLimit")]
    pub items_limit: Option<f64>,
    #[serde(rename = "OtherCategories")]
    pub other_categories: Option<OtherCategories>,

}

#[derive(serde::Serialize, Default)]
pub struct LayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    pub free_form_layout: Option<FreeFormLayoutConfiguration>,
    #[serde(rename = "SectionBasedLayout")]
    pub section_based_layout: Option<SectionBasedLayoutConfiguration>,
    #[serde(rename = "GridLayout")]
    pub grid_layout: Option<GridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct BoxPlotAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,

}
pub type PivotTableSubtotalLevel = String;
#[derive(serde::Serialize, Default)]
pub struct ExplicitHierarchy {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct CalculatedField {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterOperationSelectedFieldsConfiguration {
    #[serde(rename = "SelectedFieldOptions")]
    pub selected_field_options: Option<SelectedFieldOptions>,
    #[serde(rename = "SelectedFields")]
    pub selected_fields: Option<Vec<String>>,

}
pub type FilterVisualScope = String;
#[derive(serde::Serialize, Default)]
pub struct GeospatialWindowOptions {
    #[serde(rename = "Bounds")]
    pub bounds: Option<GeospatialCoordinateBounds>,
    #[serde(rename = "MapZoomMode")]
    pub map_zoom_mode: Option<MapZoomMode>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartConfiguration {
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "Type")]
    pub ty: Option<LineChartType>,
    #[serde(rename = "DataLabels")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<LineChartSortConfiguration>,
    #[serde(rename = "XAxisDisplayOptions")]
    pub xaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    pub secondary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    pub secondary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ForecastConfigurations")]
    pub forecast_configurations: Option<Vec<ForecastConfiguration>>,
    #[serde(rename = "Series")]
    pub series: Option<Vec<SeriesItem>>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    pub default_series_settings: Option<LineChartDefaultSeriesSettings>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<LineChartFieldWells>,
    #[serde(rename = "SmallMultiplesOptions")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    pub xaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableVisual {
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<PivotTableConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<PivotTableConditionalFormatting>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}
pub type PrimaryValueDisplayType = String;
#[derive(serde::Serialize, Default)]
pub struct PercentageDisplayFormatConfiguration {
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineCustomLabelConfiguration {
    #[serde(rename = "CustomLabel")]
    pub custom_label: String,

}
pub type TextWrap = String;pub type SelectedTooltipType = String;
#[derive(serde::Serialize, Default)]
pub struct TotalOptions {
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,

}
pub type WidgetStatus = String;
#[derive(serde::Serialize, Default)]
pub struct CustomFilterConfiguration {
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
    #[serde(rename = "CategoryValue")]
    pub category_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerParameterDeclaration {
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<IntegerValueWhenUnsetConfiguration>,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<IntegerDefaultValues>,

}

#[derive(serde::Serialize, Default)]
pub struct LineSeriesAxisDisplayOptions {
    #[serde(rename = "MissingDataConfigurations")]
    pub missing_data_configurations: Option<Vec<MissingDataConfiguration>>,
    #[serde(rename = "AxisOptions")]
    pub axis_options: Option<AxisDisplayOptions>,

}
pub type ColumnRole = String;
#[derive(serde::Serialize, Default)]
pub struct NullValueFormatConfiguration {
    #[serde(rename = "NullString")]
    pub null_string: String,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultFreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: FreeFormLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct MinimumLabelType {
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericalMeasureField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<NumericalAggregationFunction>,

}

#[derive(serde::Serialize, Default)]
pub struct DataBarsOptions {
    #[serde(rename = "NegativeColor")]
    pub negative_color: Option<String>,
    #[serde(rename = "PositiveColor")]
    pub positive_color: Option<String>,
    #[serde(rename = "FieldId")]
    pub field_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisDefaults {
    #[serde(rename = "DefaultNewSheetConfiguration")]
    pub default_new_sheet_configuration: DefaultNewSheetConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct TableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    pub conditional_formatting_options: Option<Vec<TableConditionalFormattingOption>>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "TargetValues")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "TrendGroups")]
    pub trend_groups: Option<Vec<DimensionField>>,

}
pub type WordCloudWordOrientation = String;
#[derive(serde::Serialize, Default)]
pub struct FunnelChartDataLabelOptions {
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,
    #[serde(rename = "MeasureDataLabelStyle")]
    pub measure_data_label_style: Option<FunnelChartMeasureDataLabelStyle>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterMarker {
    #[serde(rename = "SimpleClusterMarker")]
    pub simple_cluster_marker: Option<SimpleClusterMarker>,

}

#[derive(serde::Serialize, Default)]
pub struct CurrencyDisplayFormatConfiguration {
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "NumberScale")]
    pub number_scale: Option<NumberScale>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "NegativeValueConfiguration")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "DecimalPlacesConfiguration")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "SeparatorConfiguration")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "NullValueFormatConfiguration")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct KPIVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<KPIConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<KPIConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineDataConfiguration {
    #[serde(rename = "StaticConfiguration")]
    pub static_configuration: Option<ReferenceLineStaticDataConfiguration>,
    #[serde(rename = "DynamicConfiguration")]
    pub dynamic_configuration: Option<ReferenceLineDynamicDataConfiguration>,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: Option<AxisBinding>,

}

#[derive(serde::Serialize, Default)]
pub struct TableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "TextFormat")]
    pub text_format: Option<TextConditionalFormat>,

}

#[derive(serde::Serialize, Default)]
pub struct TableSortConfiguration {
    #[serde(rename = "RowSort")]
    pub row_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    pub pagination_configuration: Option<PaginationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DateTimeParameterDeclaration {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<DateTimeValueWhenUnsetConfiguration>,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<DateTimeDefaultValues>,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,

}
pub type AnchorOption = String;
#[derive(serde::Serialize, Default)]
pub struct SectionAfterPageBreak {
    #[serde(rename = "Status")]
    pub status: Option<SectionPageBreakStatus>,

}
pub type LineChartType = String;pub type FontStyle = String;
#[derive(serde::Serialize, Default)]
pub struct AggregationSortConfiguration {
    #[serde(rename = "SortDirection")]
    pub sort_direction: SortDirection,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: AggregationFunction,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct FilterControl {
    #[serde(rename = "TextField")]
    pub text_field: Option<FilterTextFieldControl>,
    #[serde(rename = "DateTimePicker")]
    pub date_time_picker: Option<FilterDateTimePickerControl>,
    #[serde(rename = "RelativeDateTime")]
    pub relative_date_time: Option<FilterRelativeDateTimeControl>,
    #[serde(rename = "TextArea")]
    pub text_area: Option<FilterTextAreaControl>,
    #[serde(rename = "List")]
    pub list: Option<FilterListControl>,
    #[serde(rename = "Dropdown")]
    pub dropdown: Option<FilterDropDownControl>,
    #[serde(rename = "Slider")]
    pub slider: Option<FilterSliderControl>,

}

#[derive(serde::Serialize, Default)]
pub struct ArcConfiguration {
    #[serde(rename = "ArcAngle")]
    pub arc_angle: Option<f64>,
    #[serde(rename = "ArcThickness")]
    pub arc_thickness: Option<ArcThicknessOptions>,

}
pub type DataLabelContent = String;
#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilter {
    #[serde(rename = "IncludeMinimum")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: Option<NumericRangeFilterValue>,
    #[serde(rename = "IncludeMaximum")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    pub filter_id: String,
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: Option<NumericRangeFilterValue>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<NumericFilterSelectAllOptions>,
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "NullOption")]
    pub null_option: FilterNullOption,

}

#[derive(serde::Serialize, Default)]
pub struct ContributionAnalysisDefault {
    #[serde(rename = "MeasureFieldId")]
    pub measure_field_id: String,
    #[serde(rename = "ContributorDimensions")]
    pub contributor_dimensions: Vec<ColumnIdentifier>,

}
pub type SheetContentType = String;
#[derive(serde::Serialize, Default)]
pub struct ForecastComputation {
    #[serde(rename = "CustomSeasonalityValue")]
    pub custom_seasonality_value: Option<f64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "PeriodsBackward")]
    pub periods_backward: Option<f64>,
    #[serde(rename = "UpperBoundary")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<MeasureField>,
    #[serde(rename = "PeriodsForward")]
    pub periods_forward: Option<f64>,
    #[serde(rename = "ComputationId")]
    pub computation_id: String,
    #[serde(rename = "Seasonality")]
    pub seasonality: Option<ForecastComputationSeasonality>,
    #[serde(rename = "Time")]
    pub time: DimensionField,
    #[serde(rename = "LowerBoundary")]
    pub lower_boundary: Option<f64>,
    #[serde(rename = "PredictionInterval")]
    pub prediction_interval: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Layout {
    #[serde(rename = "Configuration")]
    pub configuration: LayoutConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkContentConfiguration {
    #[serde(rename = "CustomTextContent")]
    pub custom_text_content: Option<TableFieldCustomTextContent>,
    #[serde(rename = "CustomIconContent")]
    pub custom_icon_content: Option<TableFieldCustomIconContent>,

}

#[derive(serde::Serialize, Default)]
pub struct PredefinedHierarchy {
    #[serde(rename = "HierarchyId")]
    pub hierarchy_id: String,
    #[serde(rename = "DrillDownFilters")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "Columns")]
    pub columns: Vec<ColumnIdentifier>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceLineStaticDataConfiguration {
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct SheetTextBox {
    #[serde(rename = "SheetTextBoxId")]
    pub sheet_text_box_id: String,
    #[serde(rename = "Content")]
    pub content: Option<String>,

}
pub type ConditionalFormattingIconSetType = String;
#[derive(serde::Serialize, Default)]
pub struct CustomParameterValues {
    #[serde(rename = "IntegerValues")]
    pub integer_values: Option<Vec<f64>>,
    #[serde(rename = "DecimalValues")]
    pub decimal_values: Option<Vec<f64>>,
    #[serde(rename = "StringValues")]
    pub string_values: Option<Vec<String>>,
    #[serde(rename = "DateTimeValues")]
    pub date_time_values: Option<Vec<String>>,

}
pub type DateAggregationFunction = String;
#[derive(serde::Serialize, Default)]
pub struct NegativeValueConfiguration {
    #[serde(rename = "DisplayMode")]
    pub display_mode: NegativeValueDisplayMode,

}

#[derive(serde::Serialize, Default)]
pub struct DataSetSchema {
    #[serde(rename = "ColumnSchemaList")]
    pub column_schema_list: Option<Vec<ColumnSchema>>,

}

#[derive(serde::Serialize, Default)]
pub struct NumericRangeFilterValue {
    #[serde(rename = "Parameter")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    pub static_value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct LocalNavigationConfiguration {
    #[serde(rename = "TargetSheetId")]
    pub target_sheet_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataLabelOptions {
    #[serde(rename = "DataLabelTypes")]
    pub data_label_types: Option<Vec<DataLabelType>>,
    #[serde(rename = "MeasureLabelVisibility")]
    pub measure_label_visibility: Option<Visibility>,
    #[serde(rename = "Position")]
    pub position: Option<DataLabelPosition>,
    #[serde(rename = "CategoryLabelVisibility")]
    pub category_label_visibility: Option<Visibility>,
    #[serde(rename = "Overlap")]
    pub overlap: Option<DataLabelOverlap>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "LabelContent")]
    pub label_content: Option<DataLabelContent>,
    #[serde(rename = "LabelFontConfiguration")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "LabelColor")]
    pub label_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataFieldSeriesItem {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "AxisBinding")]
    pub axis_binding: AxisBinding,
    #[serde(rename = "Settings")]
    pub settings: Option<LineChartSeriesSettings>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComparisonFormatConfiguration {
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterListConfiguration {
    #[serde(rename = "MatchOperator")]
    pub match_operator: CategoryFilterMatchOperator,
    #[serde(rename = "CategoryValues")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "SelectAllOptions")]
    pub select_all_options: Option<CategoryFilterSelectAllOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeRangeDrillDownFilter {
    #[serde(rename = "Column")]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMinimum")]
    pub range_minimum: String,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: TimeGranularity,
    #[serde(rename = "RangeMaximum")]
    pub range_maximum: String,

}

#[derive(serde::Serialize, Default)]
pub struct FilterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}
pub type ConditionalFormattingIconDisplayOption = String;
#[derive(serde::Serialize, Default)]
pub struct DateTimeDefaultValues {
    #[serde(rename = "StaticValues")]
    pub static_values: Option<Vec<String>>,
    #[serde(rename = "RollingDate")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "DynamicValue")]
    pub dynamic_value: Option<DynamicDefaultValue>,

}
pub type RelativeFontSize = String;pub type ResourceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct ParameterSelectableValues {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "LinkToDataSetColumn")]
    pub link_to_data_set_column: Option<ColumnIdentifier>,

}
pub type SheetControlListType = String;
#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingScope {
    #[serde(rename = "Role")]
    pub role: Option<PivotTableConditionalFormattingScopeRole>,

}

#[derive(serde::Serialize, Default)]
pub struct TableInlineVisualization {
    #[serde(rename = "DataBars")]
    pub data_bars: Option<DataBarsOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FunnelChartFieldWells {
    #[serde(rename = "FunnelChartAggregatedFieldWells")]
    pub funnel_chart_aggregated_field_wells: Option<FunnelChartAggregatedFieldWells>,

}

#[derive(serde::Serialize, Default)]
pub struct DrillDownFilter {
    #[serde(rename = "TimeRangeFilter")]
    pub time_range_filter: Option<TimeRangeDrillDownFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<NumericEqualityDrillDownFilter>,
    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<CategoryDrillDownFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterSliderControl {
    #[serde(rename = "FilterControlId")]
    pub filter_control_id: String,
    #[serde(rename = "Type")]
    pub ty: Option<SheetControlSliderType>,
    #[serde(rename = "MinimumValue")]
    pub minimum_value: f64,
    #[serde(rename = "SourceFilterId")]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "MaximumValue")]
    pub maximum_value: f64,
    #[serde(rename = "StepSize")]
    pub step_size: f64,
    #[serde(rename = "DisplayOptions")]
    pub display_options: Option<SliderControlDisplayOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<WordCloudChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartPrimaryValueConditionalFormatting {
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,

}
pub type Icon = String;
#[derive(serde::Serialize, Default)]
pub struct TemplateError {
    #[serde(rename = "Type")]
    pub ty: Option<TemplateErrorType>,
    #[serde(rename = "ViolatedEntities")]
    pub violated_entities: Option<Vec<Entity>>,
    #[serde(rename = "Message")]
    pub message: Option<String>,

}
pub type PaperSize = String;
#[derive(serde::Serialize, Default)]
pub struct CalculatedMeasureField {
    #[serde(rename = "FieldId")]
    pub field_id: String,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct InsightConfiguration {
    #[serde(rename = "CustomNarrative")]
    pub custom_narrative: Option<CustomNarrativeOptions>,
    #[serde(rename = "Computations")]
    pub computations: Option<Vec<Computation>>,

}

#[derive(serde::Serialize, Default)]
pub struct LineChartMarkerStyleSettings {
    #[serde(rename = "MarkerVisibility")]
    pub marker_visibility: Option<Visibility>,
    #[serde(rename = "MarkerShape")]
    pub marker_shape: Option<LineChartMarkerShape>,
    #[serde(rename = "MarkerSize")]
    pub marker_size: Option<String>,
    #[serde(rename = "MarkerColor")]
    pub marker_color: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PercentileAggregation {
    #[serde(rename = "PercentileValue")]
    pub percentile_value: Option<f64>,

}
pub type TableTotalsScrollStatus = String;
#[derive(serde::Serialize, Default)]
pub struct SheetVisualScopingConfiguration {
    #[serde(rename = "SheetId")]
    pub sheet_id: String,
    #[serde(rename = "VisualIds")]
    pub visual_ids: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    pub scope: FilterVisualScope,

}

#[derive(serde::Serialize, Default)]
pub struct Spacing {
    #[serde(rename = "Right")]
    pub right: Option<String>,
    #[serde(rename = "Left")]
    pub left: Option<String>,
    #[serde(rename = "Top")]
    pub top: Option<String>,
    #[serde(rename = "Bottom")]
    pub bottom: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartVisual {
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<GaugeChartConditionalFormatting>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<GaugeChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct HistogramVisual {
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<HistogramConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,

}

#[derive(serde::Serialize, Default)]
pub struct SankeyDiagramVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<SankeyDiagramChartConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct FreeFormLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    pub optimized_view_port_width: String,

}

#[derive(serde::Serialize, Default)]
pub struct SmallMultiplesOptions {
    #[serde(rename = "MaxVisibleRows")]
    pub max_visible_rows: Option<f64>,
    #[serde(rename = "MaxVisibleColumns")]
    pub max_visible_columns: Option<f64>,
    #[serde(rename = "PanelConfiguration")]
    pub panel_configuration: Option<PanelConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableFieldLinkConfiguration {
    #[serde(rename = "Target")]
    pub target: URLTargetConfiguration,
    #[serde(rename = "Content")]
    pub content: TableFieldLinkContentConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct WordCloudChartConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<WordCloudFieldWells>,
    #[serde(rename = "WordCloudOptions")]
    pub word_cloud_options: Option<WordCloudOptions>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<WordCloudSortConfiguration>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}
pub type TableBorderStyle = String;
#[derive(serde::Serialize, Default)]
pub struct FunnelChartVisual {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<FunnelChartConfiguration>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnIdentifier {
    #[serde(rename = "ColumnName")]
    pub column_name: String,
    #[serde(rename = "DataSetIdentifier")]
    pub data_set_identifier: String,

}
pub type BoxPlotFillStyle = String;
#[derive(serde::Serialize, Default)]
pub struct CustomContentConfiguration {
    #[serde(rename = "ImageScaling")]
    pub image_scaling: Option<CustomContentImageScalingConfiguration>,
    #[serde(rename = "ContentUrl")]
    pub content_url: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<CustomContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct GaugeChartArcConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    pub foreground_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetControlLayout {
    #[serde(rename = "Configuration")]
    pub configuration: SheetControlLayoutConfiguration,

}
pub type TopBottomSortOrder = String;
#[derive(serde::Serialize, Default)]
pub struct DefaultSectionBasedLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,

}

#[derive(serde::Serialize, Default)]
pub struct DataPathLabelType {
    #[serde(rename = "FieldId")]
    pub field_id: Option<String>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}
pub type ColorFillType = String;
#[derive(serde::Serialize, Default)]
pub struct DefaultInteractiveLayoutConfiguration {
    #[serde(rename = "FreeForm")]
    pub free_form: Option<DefaultFreeFormLayoutConfiguration>,
    #[serde(rename = "Grid")]
    pub grid: Option<DefaultGridLayoutConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct TableVisual {
    #[serde(rename = "VisualId")]
    pub visual_id: String,
    #[serde(rename = "ConditionalFormatting")]
    pub conditional_formatting: Option<TableConditionalFormatting>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<TableConfiguration>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct SectionStyle {
    #[serde(rename = "Padding")]
    pub padding: Option<Spacing>,
    #[serde(rename = "Height")]
    pub height: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionalFormattingIconSet {
    #[serde(rename = "IconSetType")]
    pub icon_set_type: Option<ConditionalFormattingIconSetType>,
    #[serde(rename = "Expression")]
    pub expression: String,

}
pub type ValueWhenUnsetOption = String;
#[derive(serde::Serialize, Default)]
pub struct DataPathColor {
    #[serde(rename = "Element")]
    pub element: DataPathValue,
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "TimeGranularity")]
    pub time_granularity: Option<TimeGranularity>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualTitleLabelOptions {
    #[serde(rename = "FormatText")]
    pub format_text: Option<ShortFormatText>,
    #[serde(rename = "Visibility")]
    pub visibility: Option<Visibility>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartVisual {
    #[serde(rename = "ChartConfiguration")]
    pub chart_configuration: Option<BarChartConfiguration>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "Title")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "ColumnHierarchies")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "VisualId")]
    pub visual_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct TemplateVersionDefinition {
    #[serde(rename = "Sheets")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "CalculatedFields")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "DataSetConfigurations")]
    pub data_set_configurations: Vec<DataSetConfiguration>,
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "ColumnConfigurations")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "ParameterDeclarations")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "AnalysisDefaults")]
    pub analysis_defaults: Option<AnalysisDefaults>,

}

#[derive(serde::Serialize, Default)]
pub struct SetParameterValueConfiguration {
    #[serde(rename = "DestinationParameterName")]
    pub destination_parameter_name: String,
    #[serde(rename = "Value")]
    pub value: DestinationParameterValueConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct FontSize {
    #[serde(rename = "Relative")]
    pub relative: Option<RelativeFontSize>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTotalOptions {
    #[serde(rename = "Placement")]
    pub placement: Option<TableTotalsPlacement>,
    #[serde(rename = "ScrollStatus")]
    pub scroll_status: Option<TableTotalsScrollStatus>,
    #[serde(rename = "TotalsVisibility")]
    pub totals_visibility: Option<Visibility>,
    #[serde(rename = "ValueCellStyle")]
    pub value_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalCellStyle")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "CustomLabel")]
    pub custom_label: Option<String>,
    #[serde(rename = "MetricHeaderCellStyle")]
    pub metric_header_cell_style: Option<TableCellStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomColor {
    #[serde(rename = "SpecialValue")]
    pub special_value: Option<SpecialValue>,
    #[serde(rename = "FieldValue")]
    pub field_value: Option<String>,
    #[serde(rename = "Color")]
    pub color: String,

}

#[derive(serde::Serialize, Default)]
pub struct TextConditionalFormat {
    #[serde(rename = "Icon")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<ConditionalFormattingColor>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    pub cell: Option<PivotTableCellConditionalFormatting>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartSortConfiguration {
    #[serde(rename = "ColorSort")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "CategorySort")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesSort")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorItemsLimit")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategoryItemsLimit")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,

}
pub type SheetControlDateTimePickerType = String;
#[derive(serde::Serialize, Default)]
pub struct StringParameterDeclaration {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultValues")]
    pub default_values: Option<StringDefaultValues>,
    #[serde(rename = "ParameterValueType")]
    pub parameter_value_type: ParameterValueType,
    #[serde(rename = "MappedDataSetParameters")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "ValueWhenUnset")]
    pub value_when_unset: Option<StringValueWhenUnsetConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct FilledMapAggregatedFieldWells {
    #[serde(rename = "Values")]
    pub values: Option<Vec<MeasureField>>,
    #[serde(rename = "Geospatial")]
    pub geospatial: Option<Vec<DimensionField>>,

}
pub type FontWeightName = String;
#[derive(serde::Serialize, Default)]
pub struct BoxPlotChartConfiguration {
    #[serde(rename = "FieldWells")]
    pub field_wells: Option<BoxPlotFieldWells>,
    #[serde(rename = "CategoryAxis")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ReferenceLines")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SortConfiguration")]
    pub sort_configuration: Option<BoxPlotSortConfiguration>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    pub primary_yaxis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "BoxPlotOptions")]
    pub box_plot_options: Option<BoxPlotOptions>,
    #[serde(rename = "Legend")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    pub primary_yaxis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    pub category_label_options: Option<ChartAxisLabelOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct PivotTableTotalOptions {
    #[serde(rename = "ColumnSubtotalOptions")]
    pub column_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "RowTotalOptions")]
    pub row_total_options: Option<PivotTotalOptions>,
    #[serde(rename = "ColumnTotalOptions")]
    pub column_total_options: Option<PivotTotalOptions>,
    #[serde(rename = "RowSubtotalOptions")]
    pub row_subtotal_options: Option<SubtotalOptions>,

}


}

pub mod cfn_theme {

#[derive(serde::Serialize, Default)]
pub struct CfnTheme {
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The ID of the theme that a custom theme will inherit from. All themes inherit from one of
    /// the starting themes defined by Amazon QuickSight. For a list of the starting themes, use
    /// <code>ListThemes</code> or choose <b>Themes</b> from
    /// within a QuickSight analysis. </p>
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: Option<String>,
    /// <p>A display name for the theme.</p>
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// <p>A map of the key-value pairs for the resource tag or tags that you want to add to the
    /// resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A valid grouping of resource permissions to apply to the new theme.
    /// </p>
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<ThemeConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "ThemeId")]
    pub theme_id: String,
    /// <p>A description of the first version of the theme that you're creating. Every time
    /// <code>UpdateTheme</code> is called, a new version is created. Each version of the
    /// theme has a description of the version in the <code>VersionDescription</code>
    /// field.</p>
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,

}

pub type ThemeErrorType = String;
#[derive(serde::Serialize, Default)]
pub struct Typography {
    #[serde(rename = "FontFamilies")]
    pub font_families: Option<Vec<Font>>,

}

#[derive(serde::Serialize, Default)]
pub struct GutterStyle {
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcePermission {
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    #[serde(rename = "Principal")]
    pub principal: String,

}

#[derive(serde::Serialize, Default)]
pub struct UIColorPalette {
    #[serde(rename = "DimensionForeground")]
    pub dimension_foreground: Option<String>,
    #[serde(rename = "Dimension")]
    pub dimension: Option<String>,
    #[serde(rename = "SecondaryForeground")]
    pub secondary_foreground: Option<String>,
    #[serde(rename = "Success")]
    pub success: Option<String>,
    #[serde(rename = "SecondaryBackground")]
    pub secondary_background: Option<String>,
    #[serde(rename = "PrimaryBackground")]
    pub primary_background: Option<String>,
    #[serde(rename = "WarningForeground")]
    pub warning_foreground: Option<String>,
    #[serde(rename = "Danger")]
    pub danger: Option<String>,
    #[serde(rename = "AccentForeground")]
    pub accent_foreground: Option<String>,
    #[serde(rename = "MeasureForeground")]
    pub measure_foreground: Option<String>,
    #[serde(rename = "Warning")]
    pub warning: Option<String>,
    #[serde(rename = "DangerForeground")]
    pub danger_foreground: Option<String>,
    #[serde(rename = "PrimaryForeground")]
    pub primary_foreground: Option<String>,
    #[serde(rename = "Measure")]
    pub measure: Option<String>,
    #[serde(rename = "SuccessForeground")]
    pub success_foreground: Option<String>,
    #[serde(rename = "Accent")]
    pub accent: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BorderStyle {
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SheetStyle {
    #[serde(rename = "TileLayout")]
    pub tile_layout: Option<TileLayoutStyle>,
    #[serde(rename = "Tile")]
    pub tile: Option<TileStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct DataColorPalette {
    #[serde(rename = "EmptyFillColor")]
    pub empty_fill_color: Option<String>,
    #[serde(rename = "MinMaxGradient")]
    pub min_max_gradient: Option<Vec<String>>,
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ThemeConfiguration {
    #[serde(rename = "Typography")]
    pub typography: Option<Typography>,
    #[serde(rename = "UIColorPalette")]
    pub uicolor_palette: Option<UIColorPalette>,
    #[serde(rename = "Sheet")]
    pub sheet: Option<SheetStyle>,
    #[serde(rename = "DataColorPalette")]
    pub data_color_palette: Option<DataColorPalette>,

}
pub type ThemeType = String;
#[derive(serde::Serialize, Default)]
pub struct MarginStyle {
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Font {
    #[serde(rename = "FontFamily")]
    pub font_family: Option<String>,

}
pub type ResourceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct ThemeError {
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<ThemeErrorType>,

}

#[derive(serde::Serialize, Default)]
pub struct ThemeVersion {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "Configuration")]
    pub configuration: Option<ThemeConfiguration>,
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<f64>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<ThemeError>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TileLayoutStyle {
    #[serde(rename = "Gutter")]
    pub gutter: Option<GutterStyle>,
    #[serde(rename = "Margin")]
    pub margin: Option<MarginStyle>,

}

#[derive(serde::Serialize, Default)]
pub struct TileStyle {
    #[serde(rename = "Border")]
    pub border: Option<BorderStyle>,

}


}

pub mod cfn_vpcconnection {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCConnection {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<RoleArn>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<VPCConnectionAvailabilityStatus>,
    /// No documentation provided by AWS
    #[serde(rename = "DnsResolvers")]
    pub dns_resolvers: Option<DnsResolvers>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<SecurityGroupIds>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<SubnetIds>,
    /// No documentation provided by AWS
    #[serde(rename = "VPCConnectionId")]
    pub vpcconnection_id: Option<VPCConnectionId>,

}

pub type VPCId = String;
#[derive(serde::Serialize, Default)]
pub struct SecurityGroupIds {

}
pub type VPCConnectionResourceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type DnsResolvers = Vec<String>;pub type RoleArn = String;pub type NetworkInterfaceId = String;
#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaces {

}

#[derive(serde::Serialize, Default)]
pub struct SubnetIds {

}
pub type SecurityGroupId = String;
#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<SubnetId>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ErrorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<NetworkInterfaceId>,

}
pub type SubnetId = String;pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type VPCConnectionId = String;pub type VPCConnectionAvailabilityStatus = String;

}
