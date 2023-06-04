/// Creates an empty dataset and adds it to the specified dataset group.    Use CreateDatasetImportJob to import your training data to a    dataset.
///
/// There are three types of datasets:
///
/// Each dataset type has an associated schema with required field types.    Only the Interactions dataset is required in order to train a    model (also referred to as creating a solution).
///
/// A dataset can be in one of the following states:
///
/// To get the status of the dataset, call DescribeDataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDataset {
    ///
    /// The Amazon Resource Name (ARN) of the dataset group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: cfn_resources::StrVal,

    ///
    /// Describes a job that imports training data from a data source (Amazon S3 bucket) to an       Amazon Personalize dataset.
    ///
    /// Required: No
    ///
    /// Type: DatasetImportJob
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetImportJob")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dataset_import_job: Option<DatasetImportJob>,

    ///
    /// One of the following values:
    ///
    /// Interactions               Items               Users
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetType")]
    pub dataset_type: cfn_resources::StrVal,

    ///
    /// The name of the dataset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The ARN of the associated schema.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaArn")]
    pub schema_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_dataset_arn: CfnDatasetdatasetarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDatasetdatasetarn;
impl CfnDatasetdatasetarn {
    pub fn att_name(&self) -> &'static str {
        r#"DatasetArn"#
    }
}

impl cfn_resources::CfnResource for CfnDataset {
    fn type_string(&self) -> &'static str {
        "AWS::Personalize::Dataset"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.dataset_group_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'dataset_group_arn'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        self.dataset_import_job
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.dataset_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'dataset_type'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.schema_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'schema_arn'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The DataSource property type specifies Property description not available. for an AWS::Personalize::Dataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataSource {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLocation")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub data_location: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DataSource {
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

/// Describes a job that imports training data from a data source (Amazon S3    bucket) to an Amazon Personalize dataset. For more information, see CreateDatasetImportJob.
///
/// A dataset import job can be in one of the following states:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DatasetImportJob {
    ///
    /// The Amazon S3 bucket that contains the training data to import.
    ///
    /// Required: No
    ///
    /// Type: DataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub data_source: Option<DataSource>,

    ///
    /// The Amazon Resource Name (ARN) of the dataset that receives the    imported data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dataset_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the dataset import job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dataset_import_job_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the import job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub job_name: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM role that has permissions to read from the Amazon S3    data source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DatasetImportJob {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.dataset_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'dataset_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.dataset_import_job_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'dataset_import_job_arn'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!(
                        "Max validation failed on field 'job_name'. {} is greater than 63",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'job_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
