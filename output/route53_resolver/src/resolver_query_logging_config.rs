

/// The AWS::Route53Resolver::ResolverQueryLoggingConfig resource is a complex type that contains settings for one query logging configuration.
#[derive(Default, serde::Serialize)]
pub struct CfnResolverQueryLoggingConfig {


    /// 
    /// The ARN of the resource that you want Resolver to send query logs: an Amazon S3 bucket, a CloudWatch Logs log group, or 			a Kinesis Data Firehose delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 600
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,


    /// 
    /// The name of the query logging configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
