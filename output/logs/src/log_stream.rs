

/// The AWS::Logs::LogStream resource specifies an Amazon CloudWatch Logs log stream in a specific log group.      A log stream represents the sequence of events coming from an application instance or resource that you are monitoring.
///
/// There is no limit on the number of log streams that you can create for a log group.
///
/// You must use the following guidelines when naming a log stream:
#[derive(Default, serde::Serialize)]
pub struct CfnLogStream {


    /// 
    /// The name of the log group where the log stream is created.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,


    /// 
    /// The name of the log stream. The name must be unique within the log group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,

}
