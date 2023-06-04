/// Represents a report group. A report group contains a collection of reports.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnReportGroup {
    ///
    /// When deleting a report group, specifies if reports within the report group should be       deleted.
    ///
    /// true                   Deletes any reports that belong to the report group before deleting the             report group.                         false                   You must delete any reports in the report group. This is the default             value. If you delete a report group that contains one or more reports, an             exception is thrown.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteReports")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub delete_reports: Option<bool>,

    ///
    /// Information about the destination where the raw data of this ReportGroup       is exported.
    ///
    /// Required: Yes
    ///
    /// Type: ReportExportConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExportConfig")]
    pub export_config: ReportExportConfig,

    ///
    /// The name of the ReportGroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// A list of tag key and value pairs associated with this report group.
    ///
    /// These tags are available for use by AWS services that support AWS CodeBuild report group    tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of the ReportGroup. This can be one of the following       values:
    ///
    /// CODE_COVERAGE                  The report group contains code coverage reports.                       TEST                  The report group contains test reports.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CODE_COVERAGE | TEST
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ReportGroupTypeEnum,

    #[serde(skip_serializing)]
    pub att_arn: CfnReportGrouparn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReportGroupTypeEnum {
    /// CODE_COVERAGE
    #[serde(rename = "CODE_COVERAGE")]
    Codecoverage,

    /// TEST
    #[serde(rename = "TEST")]
    Test,
}

impl Default for ReportGroupTypeEnum {
    fn default() -> Self {
        ReportGroupTypeEnum::Codecoverage
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReportGrouparn;
impl CfnReportGrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnReportGroup {
    fn type_string(&self) -> &'static str {
        "AWS::CodeBuild::ReportGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.export_config.validate()?;

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 2 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 2",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Information about the location where the run of a report is exported.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReportExportConfig {
    ///
    /// The export configuration type. Valid values are:
    ///
    /// S3: The report results are exported to an S3 bucket.                         NO_EXPORT: The report results are not exported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NO_EXPORT | S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExportConfigType")]
    pub export_config_type: ReportExportConfigExportConfigTypeEnum,

    ///
    /// A S3ReportExportConfig object that contains information about the S3       bucket where the run of a report is exported.
    ///
    /// Required: No
    ///
    /// Type: S3ReportExportConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub s3_destination: Option<S3ReportExportConfig>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReportExportConfigExportConfigTypeEnum {
    /// NO_EXPORT
    #[serde(rename = "NO_EXPORT")]
    Noexport,

    /// S3
    #[serde(rename = "S3")]
    S3,
}

impl Default for ReportExportConfigExportConfigTypeEnum {
    fn default() -> Self {
        ReportExportConfigExportConfigTypeEnum::Noexport
    }
}

impl cfn_resources::CfnResource for ReportExportConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the S3 bucket where the raw data of a report are exported.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3ReportExportConfig {
    ///
    /// The name of the S3 bucket where the raw data of a report are exported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: cfn_resources::StrVal,

    ///
    /// The AWS account identifier of the owner of the Amazon S3 bucket. This allows report data to be exported to an Amazon S3 bucket     that is owned by an account other than the account running the build.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_owner: Option<cfn_resources::StrVal>,

    ///
    /// A boolean value that specifies if the results of a report are encrypted.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionDisabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption_disabled: Option<bool>,

    ///
    /// The encryption key for the report's encrypted raw data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption_key: Option<cfn_resources::StrVal>,

    ///
    /// The type of build output artifact to create. Valid values include:
    ///
    /// NONE: CodeBuild creates the raw data in the output bucket. This           is the default if packaging is not specified.                         ZIP: CodeBuild creates a ZIP file with the raw data in the           output bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | ZIP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Packaging")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub packaging: Option<S3ReportExportConfigPackagingEnum>,

    ///
    /// The path to the exported report's raw data results.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum S3ReportExportConfigPackagingEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// ZIP
    #[serde(rename = "ZIP")]
    Zip,
}

impl Default for S3ReportExportConfigPackagingEnum {
    fn default() -> Self {
        S3ReportExportConfigPackagingEnum::None
    }
}

impl cfn_resources::CfnResource for S3ReportExportConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.encryption_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'encryption_key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
