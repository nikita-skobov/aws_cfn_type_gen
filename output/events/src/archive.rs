

/// Creates an archive of events with the specified settings. When you create an archive,    incoming events might not immediately start being sent to the archive. Allow a short period of    time for changes to take effect. If you do not specify a pattern to filter events sent to the    archive, all events are sent to the archive except replayed events. Replayed events are not    sent to an archive.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnArchive {


    /// 
    /// An event pattern to use to filter events sent to the archive.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<serde_json::Value>,


    /// 
    /// The ARN of the event bus that sends events to the archive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceArn")]
    pub source_arn: String,


    /// 
    /// The number of days to retain events for. Default value is 0. If set to 0, events are    retained indefinitely
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionDays")]
    pub retention_days: Option<i64>,


    /// 
    /// The name for the archive to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 48
    ///
    /// Pattern: [\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ArchiveName")]
    pub archive_name: Option<String>,


    /// 
    /// A description for the archive.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



impl cfn_resources::CfnResource for CfnArchive {
    fn type_string() -> &'static str {
        "AWS::Events::Archive"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
