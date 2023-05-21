

/// Creates a theme.
///
/// A theme is set of configuration options for color and layout. Themes apply to analyses and  dashboards. For more information, see Using Themes in Amazon QuickSight in the  Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnTheme {


    /// 
    /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of 			the starting themes defined by Amazon QuickSight. For a list of the starting themes, use 				ListThemes or choose Themes from 			within an analysis.
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
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: Option<String>,


    /// 
    /// A description of the first version of the theme that you're creating. Every time 				UpdateTheme is called, a new version is created. Each version of the 			theme has a description of the version in the VersionDescription 			field.
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
    /// A map of the key-value pairs for the resource tag or tags that you want to add to the 			resource.
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
    /// A display name for the theme.
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
    /// The ID of the AWS account where you want to store the new theme.
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
    /// The theme configuration, which contains the theme display properties.
    /// 
    /// Required: No
    ///
    /// Type: ThemeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: Option<ThemeConfiguration>,


    /// 
    /// An ID for the theme that you want to create. The theme ID is unique per AWS Region in 			each AWS account.
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
    #[serde(rename = "ThemeId")]
    pub theme_id: String,


    /// 
    /// A valid grouping of resource permissions to apply to the new theme.
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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The display options for tile borders for visuals.
#[derive(Default, serde::Serialize)]
pub struct BorderStyle {


    /// 
    /// The option to enable display of borders for visuals.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}


/// The theme colors that apply to UI and to charts, excluding data colors. The colors description is a hexadecimal  color code that consists of six alphanumerical characters, prefixed with #, for example #37BFF5. For  more information, see Using Themes  in Amazon QuickSight in the Amazon QuickSight User Guide.
#[derive(Default, serde::Serialize)]
pub struct UIColorPalette {


    /// 
    /// The color of text and other foreground elements that appear over the primary       background regions, such as grid lines, borders, table banding, icons, and so on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryForeground")]
    pub primary_foreground: Option<String>,


    /// 
    /// This color is that applies to selected states and buttons.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Accent")]
    pub accent: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       success color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuccessForeground")]
    pub success_foreground: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       accent color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccentForeground")]
    pub accent_foreground: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       error color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DangerForeground")]
    pub danger_foreground: Option<String>,


    /// 
    /// This color that applies to warning and informational messages.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Warning")]
    pub warning: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       measure color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MeasureForeground")]
    pub measure_foreground: Option<String>,


    /// 
    /// The color that applies to error messages.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Danger")]
    pub danger: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       warning color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarningForeground")]
    pub warning_foreground: Option<String>,


    /// 
    /// The background color that applies to the sheet background and sheet controls.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryBackground")]
    pub secondary_background: Option<String>,


    /// 
    /// The color that applies to the names of fields that are identified as       dimensions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimension")]
    pub dimension: Option<String>,


    /// 
    /// The color that applies to success messages, for example the check mark for a       successful download.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Success")]
    pub success: Option<String>,


    /// 
    /// The background color that applies to visuals and other high emphasis UI.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryBackground")]
    pub primary_background: Option<String>,


    /// 
    /// The foreground color that applies to any sheet title, sheet control text, or UI that       appears over the secondary background.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryForeground")]
    pub secondary_foreground: Option<String>,


    /// 
    /// The color that applies to the names of fields that are identified as measures.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Measure")]
    pub measure: Option<String>,


    /// 
    /// The foreground color that applies to any text or other elements that appear over the       dimension color.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionForeground")]
    pub dimension_foreground: Option<String>,

}


/// Theme error.
#[derive(Default, serde::Serialize)]
pub struct ThemeError {


    /// 
    /// The type of error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERNAL_FAILURE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The error message.
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

}


/// The display options for gutter spacing between tiles on a sheet.
#[derive(Default, serde::Serialize)]
pub struct GutterStyle {


    /// 
    /// This Boolean value controls whether to display a gutter space between sheet tiles.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}


/// Permission for the resource.
#[derive(Default, serde::Serialize)]
pub struct ResourcePermission {


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

}


/// A version of a theme.
#[derive(Default, serde::Serialize)]
pub struct ThemeVersion {


    /// 
    /// The Amazon QuickSight-defined ID of the theme that a custom theme inherits from. All       themes initially inherit from a default Amazon QuickSight theme.
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
    #[serde(rename = "BaseThemeId")]
    pub base_theme_id: Option<String>,


    /// 
    /// Errors associated with the theme.
    /// 
    /// Required: No
    ///
    /// Type: List of ThemeError
    ///
    /// Update requires: No interruption
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<ThemeError>>,


    /// 
    /// The version number of the theme.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<f64>,


    /// 
    /// The theme configuration, which contains all the theme display properties.
    /// 
    /// Required: No
    ///
    /// Type: ThemeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: Option<ThemeConfiguration>,


    /// 
    /// The status of the theme version.
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
    /// The date and time that this theme version was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,


    /// 
    /// The description of the theme.
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


/// The Font property type specifies Property description not available. for an AWS::QuickSight::Theme.
#[derive(Default, serde::Serialize)]
pub struct Font {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontFamily")]
    pub font_family: Option<String>,

}


/// The theme colors that are used for data colors in charts. The colors description is a       hexadecimal color code that consists of six alphanumerical characters, prefixed with         #, for example #37BFF5.
#[derive(Default, serde::Serialize)]
pub struct DataColorPalette {


    /// 
    /// The hexadecimal codes for the colors.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Colors")]
    pub colors: Option<Vec<String>>,


    /// 
    /// The hexadecimal code of a color that applies to charts where a lack of data is       highlighted.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^#[A-F0-9]{6}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmptyFillColor")]
    pub empty_fill_color: Option<String>,


    /// 
    /// The minimum and maximum hexadecimal codes that describe a color gradient.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinMaxGradient")]
    pub min_max_gradient: Option<Vec<String>>,

}


/// The Typography property type specifies Property description not available. for an AWS::QuickSight::Theme.
#[derive(Default, serde::Serialize)]
pub struct Typography {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Font
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontFamilies")]
    pub font_families: Option<Vec<Font>>,

}


/// The display options for margins around the outside edge of sheets.
#[derive(Default, serde::Serialize)]
pub struct MarginStyle {


    /// 
    /// This Boolean value controls whether to display sheet margins.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Show")]
    pub show: Option<bool>,

}


/// The display options for the layout of tiles on a sheet.
#[derive(Default, serde::Serialize)]
pub struct TileLayoutStyle {


    /// 
    /// The gutter settings that apply between tiles.
    /// 
    /// Required: No
    ///
    /// Type: GutterStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gutter")]
    pub gutter: Option<GutterStyle>,


    /// 
    /// The margin settings that apply around the outside edge of sheets.
    /// 
    /// Required: No
    ///
    /// Type: MarginStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Margin")]
    pub margin: Option<MarginStyle>,

}


/// Display options related to tiles on a sheet.
#[derive(Default, serde::Serialize)]
pub struct TileStyle {


    /// 
    /// The border around a tile.
    /// 
    /// Required: No
    ///
    /// Type: BorderStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Border")]
    pub border: Option<BorderStyle>,

}


/// The theme configuration. This configuration contains all of the display properties for       a theme.
#[derive(Default, serde::Serialize)]
pub struct ThemeConfiguration {


    /// 
    /// Color properties that apply to the UI and to charts, excluding the colors that apply       to data.
    /// 
    /// Required: No
    ///
    /// Type: UIColorPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "UIColorPalette")]
    pub uicolor_palette: Option<UIColorPalette>,


    /// 
    /// Display options related to sheets.
    /// 
    /// Required: No
    ///
    /// Type: SheetStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sheet")]
    pub sheet: Option<SheetStyle>,


    /// 
    /// Color properties that apply to chart data colors.
    /// 
    /// Required: No
    ///
    /// Type: DataColorPalette
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataColorPalette")]
    pub data_color_palette: Option<DataColorPalette>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Typography
    ///
    /// Update requires: No interruption
    #[serde(rename = "Typography")]
    pub typography: Option<Typography>,

}


/// The theme display options for sheets.
#[derive(Default, serde::Serialize)]
pub struct SheetStyle {


    /// 
    /// The layout options for tiles.
    /// 
    /// Required: No
    ///
    /// Type: TileLayoutStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "TileLayout")]
    pub tile_layout: Option<TileLayoutStyle>,


    /// 
    /// The display options for tiles.
    /// 
    /// Required: No
    ///
    /// Type: TileStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tile")]
    pub tile: Option<TileStyle>,

}