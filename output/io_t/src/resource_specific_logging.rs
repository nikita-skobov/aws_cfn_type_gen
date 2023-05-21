

/// Configure resource-specific logging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceSpecificLogging {


    /// 
    /// The default log level.Valid Values: DEBUG | INFO | ERROR | WARN | DISABLED
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: ResourceSpecificLoggingLogLevelEnum,


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
    pub target_type: ResourceSpecificLoggingTargetTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceSpecificLoggingTargetTypeEnum {

    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

    /// THING_GROUP
    #[serde(rename = "THING_GROUP")]
    Thinggroup,

}

impl Default for ResourceSpecificLoggingTargetTypeEnum {
    fn default() -> Self {
        ResourceSpecificLoggingTargetTypeEnum::Default
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceSpecificLoggingLogLevelEnum {

    /// DEBUG
    #[serde(rename = "DEBUG")]
    Debug,

    /// INFO
    #[serde(rename = "INFO")]
    Info,

    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

    /// WARN
    #[serde(rename = "WARN")]
    Warn,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

}

impl Default for ResourceSpecificLoggingLogLevelEnum {
    fn default() -> Self {
        ResourceSpecificLoggingLogLevelEnum::Debug
    }
}


impl cfn_resources::CfnResource for CfnResourceSpecificLogging {
    fn type_string() -> &'static str {
        "AWS::IoT::ResourceSpecificLogging"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
