

/// Represents a report group. A report group contains a collection of reports.
#[derive(Default, serde::Serialize)]
pub struct CfnReportGroup {


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
    pub tags: Option<Vec<Tag>>,


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
    pub delete_reports: Option<bool>,


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
    pub cfn_type: String,


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
    pub name: Option<String>,

}


/// Information about the location where the run of a report is exported.
#[derive(Default, serde::Serialize)]
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
    pub export_config_type: String,


    /// 
    /// A S3ReportExportConfig object that contains information about the S3       bucket where the run of a report is exported.
    /// 
    /// Required: No
    ///
    /// Type: S3ReportExportConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Destination")]
    pub s3_destination: Option<S3ReportExportConfig>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Information about the S3 bucket where the raw data of a report are exported.
#[derive(Default, serde::Serialize)]
pub struct S3ReportExportConfig {


    /// 
    /// A boolean value that specifies if the results of a report are encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,


    /// 
    /// The AWS account identifier of the owner of the Amazon S3 bucket. This allows report data to be exported to an Amazon S3 bucket     that is owned by an account other than the account running the build.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,


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
    pub encryption_key: Option<String>,


    /// 
    /// The path to the exported report's raw data results.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,


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
    pub bucket: String,


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
    pub packaging: Option<String>,

}