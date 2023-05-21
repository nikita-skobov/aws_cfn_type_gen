

/// A real-time log configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRealtimeLogConfig {


    /// 
    /// The sampling rate for this real-time log configuration. The sampling rate determines 			the percentage of viewer requests that are represented in the real-time log data. The 			sampling rate is an integer between 1 and 100, inclusive.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: f64,


    /// 
    /// A list of fields that are included in each real-time log record. In an API response, 			the fields are provided in the same order in which they are sent to the Amazon Kinesis data 			stream.
    /// 
    /// For more information about fields, see Real-time log configuration fields in the 				Amazon CloudFront Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fields")]
    pub fields: Vec<String>,


    /// 
    /// Contains information about the Amazon Kinesis data stream where you are sending real-time 			log data for this real-time log configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of EndPoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndPoints")]
    pub end_points: Vec<EndPoint>,


    /// 
    /// The unique name of this real-time log configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}

impl cfn_resources::CfnResource for CfnRealtimeLogConfig {
    fn type_string() -> &'static str {
        "AWS::CloudFront::RealtimeLogConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains information about the Amazon Kinesis data stream where you are sending real-time 			log data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamConfig {


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that CloudFront can use to 			send real-time log data to your Kinesis data stream.
    /// 
    /// For more information the IAM role, see Real-time log configuration IAM role in the 				Amazon CloudFront Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the Kinesis data stream where you are sending 			real-time log data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,

}


/// Contains information about the Amazon Kinesis data stream where you are sending real-time 			log data in a real-time log configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EndPoint {


    /// 
    /// Contains information about the Amazon Kinesis data stream where you are sending real-time 			log data.
    /// 
    /// Required: Yes
    ///
    /// Type: KinesisStreamConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamConfig")]
    pub kinesis_stream_config: KinesisStreamConfig,


    /// 
    /// The type of data stream where you are sending real-time log data. The only valid value 			is Kinesis.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamType")]
    pub stream_type: String,

}
