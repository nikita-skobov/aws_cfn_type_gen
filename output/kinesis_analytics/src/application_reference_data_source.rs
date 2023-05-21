

/// Adds a reference data source to an existing application.
///
/// Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.
///
/// For conceptual information,       see Configuring Application Input.       For the limits on data sources you can add to your application, see       Limits.
///
/// This operation requires permissions to perform the kinesisanalytics:AddApplicationOutput action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationReferenceDataSource {


    /// 
    /// Name of an existing application.
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
    /// The reference data source can be an object in your Amazon S3 bucket. Amazon Kinesis       Analytics reads the object and copies the data into the in-application table that is       created. You provide an S3 bucket, object key name, and the resulting in-application       table that is created. You must also provide an IAM role with the necessary permissions       that Amazon Kinesis Analytics can assume to read the object from your S3 bucket on your       behalf.
    /// 
    /// Required: Yes
    ///
    /// Type: ReferenceDataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,

}



impl cfn_resources::CfnResource for CfnApplicationReferenceDataSource {
    fn type_string() -> &'static str {
        "AWS::KinesisAnalytics::ApplicationReferenceDataSource"
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

        
        self.reference_data_source.validate()?;

        Ok(())
    }
}

/// Provides additional mapping information when the record format uses delimiters, such       as CSV. For example, the following sample records use CSV format, where the records use       the '\n' as the row delimiter and a comma (",") as the column       delimiter:
///
/// "name1", "address1"
///
/// "name2", "address2"
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CSVMappingParameters {


    /// 
    /// Column delimiter. For example, in a CSV format, a comma (",") is the typical column       delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,


    /// 
    /// Row delimiter. For example, in a CSV format, '\n' is the typical       row delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,

}



impl cfn_resources::CfnResource for CSVMappingParameters {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.record_column_delimiter;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'record_column_delimiter'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.record_row_delimiter;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'record_row_delimiter'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Provides additional mapping information when JSON is the record format on the       streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JSONMappingParameters {


    /// 
    /// Path to the top-level parent that contains the records.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}



impl cfn_resources::CfnResource for JSONMappingParameters {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.record_row_path;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'record_row_path'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// When configuring application input at the time of creating or updating an application,       provides additional mapping information specific to the record format (such as JSON,       CSV, or record fields delimited by some delimiter) on the streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MappingParameters {


    /// 
    /// Provides additional mapping information when the record format uses delimiters (for       example, CSV).
    /// 
    /// Required: No
    ///
    /// Type: CSVMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,


    /// 
    /// Provides additional mapping information when JSON is the record format on the       streaming source.
    /// 
    /// Required: No
    ///
    /// Type: JSONMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,

}



impl cfn_resources::CfnResource for MappingParameters {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.csvmapping_parameters.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.jsonmapping_parameters.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the mapping of each data element in the streaming source to the       corresponding column in the in-application stream.
///
/// Also used to describe the format of the reference data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordColumn {


    /// 
    /// Reference to the data element in the streaming input or the reference data source.       This element is required if the RecordFormatType is JSON.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,


    /// 
    /// Name of the column created in the in-application input stream or reference       table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Type of column created in the in-application input stream or reference table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlType")]
    pub sql_type: String,

}



impl cfn_resources::CfnResource for RecordColumn {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.sql_type;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'sql_type'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Describes the record format and relevant mapping information that should be applied       to schematize the records on the stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordFormat {


    /// 
    /// When configuring application input at the time of creating or updating an application,       provides additional mapping information specific to the record format (such as JSON,       CSV, or record fields delimited by some delimiter) on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: MappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,


    /// 
    /// The type of record format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: RecordFormatRecordFormatTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RecordFormatRecordFormatTypeEnum {

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

}

impl Default for RecordFormatRecordFormatTypeEnum {
    fn default() -> Self {
        RecordFormatRecordFormatTypeEnum::Csv
    }
}


impl cfn_resources::CfnResource for RecordFormat {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.mapping_parameters.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the reference data source by providing the source information (S3 bucket       name and object key name), the resulting in-application table name that is created, and       the necessary schema to map the data elements in the Amazon S3 object to the       in-application table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferenceDataSource {


    /// 
    /// Describes the format of the data in the streaming source, and how each data element       maps to corresponding columns created in the in-application stream.
    /// 
    /// Required: Yes
    ///
    /// Type: ReferenceSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceSchema")]
    pub reference_schema: ReferenceSchema,


    /// 
    /// Identifies the S3 bucket and object that contains the reference data. Also identifies       the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf. An       Amazon Kinesis Analytics application loads reference data only once. If the data       changes, you call the UpdateApplication operation to trigger reloading of       data into your application.
    /// 
    /// Required: No
    ///
    /// Type: S3ReferenceDataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ReferenceDataSource")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,


    /// 
    /// Name of the in-application table to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,

}



impl cfn_resources::CfnResource for ReferenceDataSource {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.reference_schema.validate()?;

        self.s3_reference_data_source.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.table_name {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'table_name'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.table_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'table_name'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// The ReferenceSchema property type specifies the format of the data in the reference source for a SQL-based Amazon Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferenceSchema {


    /// 
    /// A list of RecordColumn objects.
    /// 
    /// Required: Yes
    ///
    /// Type: List of RecordColumn
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,


    /// 
    /// Specifies the encoding of the records in the reference source. For example, UTF-8.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,


    /// 
    /// Specifies the format of the records on the reference source.
    /// 
    /// Required: Yes
    ///
    /// Type: RecordFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,

}



impl cfn_resources::CfnResource for ReferenceSchema {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.record_format.validate()?;

        Ok(())
    }
}

/// Identifies the S3 bucket and object that contains the reference data. Also identifies       the IAM role Amazon Kinesis Analytics can assume to read this object on your       behalf.
///
/// An Amazon Kinesis Analytics application loads reference data only once. If the data       changes, you call the UpdateApplication operation to trigger reloading of data into your       application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3ReferenceDataSource {


    /// 
    /// Amazon Resource Name (ARN) of the S3 bucket.
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
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,


    /// 
    /// Object key name containing reference data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileKey")]
    pub file_key: String,


    /// 
    /// ARN of the IAM role that the service can assume to read data on your behalf. This role       must have permission for the s3:GetObject action on the object and trust       policy that allows Amazon Kinesis Analytics service principal to assume this       role.
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
    #[serde(rename = "ReferenceRoleARN")]
    pub reference_role_arn: String,

}



impl cfn_resources::CfnResource for S3ReferenceDataSource {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.bucket_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'bucket_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.bucket_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'bucket_arn'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.file_key;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'file_key'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.file_key;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'file_key'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.reference_role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'reference_role_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.reference_role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'reference_role_arn'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}