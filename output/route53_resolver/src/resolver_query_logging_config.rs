/// The AWS::Route53Resolver::ResolverQueryLoggingConfig resource is a complex type that contains settings for one query logging configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl cfn_resources::CfnResource for CfnResolverQueryLoggingConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverQueryLoggingConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.destination_arn {
            if the_val.len() > 600 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination_arn'. {} is greater than 600",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.destination_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination_arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
