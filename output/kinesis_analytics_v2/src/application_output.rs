

/// Adds an external destination to your SQL-based Amazon Kinesis Data Analytics       application.
///
/// If you want Kinesis Data Analytics to deliver data from an in-application stream       within your application to an external destination (such as an Kinesis data stream, a       Kinesis Data Firehose delivery stream, or an Amazon Lambda function), you add the relevant       configuration to your application using this operation. You can configure one or more       outputs for your application. Each output configuration maps an in-application stream       and an external destination.
///
/// You can use one of the output configurations to deliver data from your in-application       error stream to an external destination so that you can analyze the errors.
///
/// Any configuration update, including adding a streaming source using this operation,       results in a new version of the application. You can use the DescribeApplication operation to find the current application       version.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationOutput {


    /// 
    /// The name of the application.
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
    /// Describes a SQL-based Kinesis Data Analytics application's output configuration,    in which you identify an in-application stream and a destination where you want the    in-application stream data to be written. The destination can be a Kinesis data stream or a    Kinesis Data Firehose delivery stream.
    /// 
    /// 
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
    fn type_string() -> &'static str {
        "AWS::KinesisAnalyticsV2::ApplicationOutput"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.application_name;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'application_name'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.application_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'application_name'. {} is less than 1", the_val.len()));
        }

        
        self.output.validate()?;

        Ok(())
    }
}

/// Describes the data format when records are written to the destination in a SQL-based Kinesis Data Analytics application.
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// For a SQL-based Kinesis Data Analytics application, when configuring application    output, identifies a Kinesis Data Firehose delivery stream as the destination. You provide the    stream Amazon Resource Name (ARN) of the delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseOutput {


    /// 
    /// The ARN of the destination delivery stream to write to.
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

}



impl cfn_resources::CfnResource for KinesisFirehoseOutput {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'resource_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'resource_arn'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// When you configure a SQL-based Kinesis Data Analytics application's output,    identifies a Kinesis data stream as the destination. You provide the stream Amazon Resource    Name (ARN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamsOutput {


    /// 
    /// The ARN of the destination Kinesis data stream to write to.
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

}



impl cfn_resources::CfnResource for KinesisStreamsOutput {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'resource_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'resource_arn'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// When you configure a SQL-based Kinesis Data Analytics application's output,    identifies an Amazon Lambda function as the destination. You provide the function Amazon Resource    Name (ARN) of the Lambda function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaOutput {


    /// 
    /// The Amazon Resource Name (ARN) of the destination Lambda function to write to.
    /// 
    /// NoteTo specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see Example ARNs: Amazon Lambda
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

}



impl cfn_resources::CfnResource for LambdaOutput {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'resource_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.resource_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'resource_arn'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Describes a SQL-based Kinesis Data Analytics application's output configuration,    in which you identify an in-application stream and a destination where you want the    in-application stream data to be written. The destination can be a Kinesis data stream or a    Kinesis Data Firehose delivery stream.
/// 
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Output {


    /// 
    /// Describes the data format when records are written to the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: DestinationSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,


    /// 
    /// Identifies a Kinesis Data Firehose delivery stream as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisFirehoseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseOutput")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,


    /// 
    /// Identifies a Kinesis data stream    as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamsOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsOutput")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,


    /// 
    /// Identifies an Amazon Lambda function as the destination.
    /// 
    /// Required: No
    ///
    /// Type: LambdaOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaOutput")]
    pub lambda_output: Option<LambdaOutput>,


    /// 
    /// The name of the in-application stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^-\s<>&]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



impl cfn_resources::CfnResource for Output {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.destination_schema.validate()?;

        self.kinesis_firehose_output.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.kinesis_streams_output.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.lambda_output.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}