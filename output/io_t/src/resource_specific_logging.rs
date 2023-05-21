

/// Configure resource-specific logging.
#[derive(Default, serde::Serialize)]
pub struct CfnResourceSpecificLogging {


    /// 
    /// The target name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetName")]
    pub target_name: String,


    /// 
    /// The target type. Valid Values: DEFAULT | THING_GROUP
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetType")]
    pub target_type: String,


    /// 
    /// The default log level.Valid Values: DEBUG | INFO | ERROR | WARN | DISABLED
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: String,

}
