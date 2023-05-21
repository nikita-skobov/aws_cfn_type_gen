/// Adds an external destination to your Amazon Kinesis Analytics application.
///
/// If you want Amazon Kinesis Analytics to deliver data from an in-application stream       within your application to an external destination (such as an Amazon Kinesis stream, an       Amazon Kinesis Firehose delivery stream, or an Amazon Lambda function), you add the       relevant configuration to your application using this operation. You can configure one       or more outputs for your application. Each output configuration maps an in-application       stream and an external destination.
///
/// You can use one of the output configurations to deliver data from your       in-application error stream to an external destination so that you can analyze the       errors. For more information, see Understanding Application         Output (Destination).
///
/// Any configuration update, including adding a streaming source using this       operation, results in a new version of the application. You can use the DescribeApplication operation to find the current application       version.
///
/// For the limits on the number of application inputs and outputs       you can configure, see Limits.
///
/// This operation requires permissions to perform the kinesisanalytics:AddApplicationOutput action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationOutput {
    ///
    /// Name of the application to which you want to add the output configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

    ///
    /// An array of objects, each describing one output configuration. In the output       configuration, you specify the name of an in-application stream, a destination (that is,       an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), and record the formation to use when writing to the       destination.
    ///
    /// Required: Yes
    ///
    /// Type: Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Output")]
    pub output: Output,
}

impl cfn_resources::CfnResource for CfnApplicationOutput {
    fn type_string(&self) -> &'static str {
        "AWS::KinesisAnalytics::ApplicationOutput"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.application_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'application_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.application_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'application_name'. {} is less than 1",
                the_val.len()
            ));
        }

        self.output.validate()?;

        Ok(())
    }
}

/// Describes the data format when records are written to the destination. For more       information, see Configuring Application         Output.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationSchema {
    ///
    /// Specifies the format of the records on the output stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format_type: Option<DestinationSchemaRecordFormatTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DestinationSchemaRecordFormatTypeEnum {
    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,
}

impl Default for DestinationSchemaRecordFormatTypeEnum {
    fn default() -> Self {
        DestinationSchemaRecordFormatTypeEnum::Csv
    }
}

impl cfn_resources::CfnResource for DestinationSchema {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// When configuring application output, identifies an Amazon Kinesis Firehose delivery       stream as the destination. You provide the stream Amazon Resource Name (ARN) and an IAM       role that enables Amazon Kinesis Analytics to write to the stream on your behalf.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseOutput {
    ///
    /// ARN of the destination Amazon Kinesis Firehose delivery stream to write to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

    ///
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination stream on your behalf. You need to grant the necessary permissions to this       role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

impl cfn_resources::CfnResource for KinesisFirehoseOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'resource_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'resource_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// When configuring application output, identifies an Amazon Kinesis stream as the       destination. You provide the stream Amazon Resource Name (ARN) and also an IAM role ARN       that Amazon Kinesis Analytics can use to write to the stream on your behalf.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamsOutput {
    ///
    /// ARN of the destination Amazon Kinesis stream to write to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

    ///
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination stream on your behalf. You need to grant the necessary permissions to this       role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

impl cfn_resources::CfnResource for KinesisStreamsOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'resource_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'resource_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// When configuring application output, identifies an AWS Lambda function       as the destination. You provide the function Amazon Resource Name (ARN) and also an IAM       role ARN that Amazon Kinesis Analytics can use to write to the function on your behalf.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaOutput {
    ///
    /// Amazon Resource Name (ARN) of the destination Lambda function to write to.
    ///
    /// NoteTo specify an earlier version of the Lambda function than the latest, include the         Lambda function version in the Lambda function ARN. For more information about         Lambda ARNs, see Example           ARNs: AWS Lambda
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

    ///
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination function on your behalf. You need to grant the necessary permissions to this       role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

impl cfn_resources::CfnResource for LambdaOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'resource_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'resource_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Describes application output configuration in which you identify an in-application       stream and a destination where you want the in-application stream data to be written.       The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery       stream.
///
/// For limits on how many destinations an application can write and other limitations,       see Limits.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Output {
    ///
    /// Describes the data format when records are written to the destination. For more       information, see Configuring Application         Output.
    ///
    /// Required: Yes
    ///
    /// Type: DestinationSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,

    ///
    /// Identifies an Amazon Kinesis Firehose delivery stream as the destination.
    ///
    /// Required: No
    ///
    /// Type: KinesisFirehoseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,

    ///
    /// Identifies an Amazon Kinesis stream as the destination.
    ///
    /// Required: No
    ///
    /// Type: KinesisStreamsOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,

    ///
    /// Identifies an AWS Lambda function as the destination.
    ///
    /// Required: No
    ///
    /// Type: LambdaOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output: Option<LambdaOutput>,

    ///
    /// Name of the in-application stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl cfn_resources::CfnResource for Output {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_schema.validate()?;

        self.kinesis_firehose_output
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_streams_output
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.lambda_output
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if the_val.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 32",
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
