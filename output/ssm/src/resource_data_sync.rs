

/// The AWS::SSM::ResourceDataSync resource creates, updates, or deletes a    resource data sync for AWS Systems Manager. A resource data sync helps you view data from    multiple sources in a single location. Systems Manager offers two types of resource data sync:     SyncToDestination and SyncFromSource.
///
/// You can configure Systems Manager Inventory to use the SyncToDestination type    to synchronize Inventory data from multiple AWS Regions to a single Amazon S3 bucket.
///
/// You can configure Systems Manager Explorer to use the SyncFromSource type to    synchronize operational work items (OpsItems) and operational data (OpsData) from multiple AWS Regions. This type can synchronize OpsItems and OpsData from multiple AWS accounts and Regions    or from an EntireOrganization by using AWS Organizations.
///
/// A resource data sync is an asynchronous operation that returns immediately. After a    successful initial sync is completed, the system continuously syncs data.
///
/// By default, data is not encrypted in Amazon S3. We strongly recommend that you enable    encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure    access to the Amazon S3 bucket by creating a restrictive bucket policy.
///
/// For more information, see Configuring Inventory Collection and Setting Up Systems     Manager Explorer to Display Data from Multiple Accounts and Regions in the     AWS Systems Manager User Guide.
///
/// Important: The following Syntax section shows all fields that are    supported for a resource data sync. The Examples section below shows the    recommended way to specify configurations for each sync type. Please see the     Examples section when you create your resource data sync.
#[derive(Default, serde::Serialize)]
pub struct CfnResourceDataSync {


    /// 
    /// Configuration information for the target S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: S3Destination
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Destination")]
    pub s3_destination: Option<S3Destination>,


    /// 
    /// Information about the source where the data was synchronized.
    /// 
    /// Required: No
    ///
    /// Type: SyncSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncSource")]
    pub sync_source: Option<SyncSource>,


    /// 
    /// The name of the S3 bucket where the aggregated data is stored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// A supported sync format. The following format is currently supported: JsonSerDe
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: JsonSerDe
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncFormat")]
    pub sync_format: Option<String>,


    /// 
    /// An Amazon S3 prefix for the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The ARN of an encryption key for a destination in Amazon S3. You can use a KMS key to    encrypt inventory data in Amazon S3. You must specify a key that exist in the same region as    the destination Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KMSKeyArn")]
    pub kmskey_arn: Option<String>,


    /// 
    /// The type of resource data sync. If SyncType is SyncToDestination,  then the resource data sync synchronizes data to an S3 bucket. If the SyncType is   SyncFromSource then the resource data sync synchronizes data from AWS Organizations or from  multiple AWS Regions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncType")]
    pub sync_type: Option<String>,


    /// 
    /// A name for the resource data sync.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncName")]
    pub sync_name: String,


    /// 
    /// The AWS Region with the S3 bucket targeted by the resource data sync.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketRegion")]
    pub bucket_region: Option<String>,

}


/// Information about the source of the data included in the resource data sync.
#[derive(Default, serde::Serialize)]
pub struct SyncSource {


    /// 
    /// Whether to automatically synchronize and aggregate data from new AWS Regions when those  Regions come online.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeFutureRegions")]
    pub include_future_regions: Option<bool>,


    /// 
    /// The type of data source for the resource data sync. SourceType is either   AwsOrganizations (if an organization is present in AWS Organizations) or   SingleAccountMultiRegions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceType")]
    pub source_type: String,


    /// 
    /// The SyncSource       AWS Regions included in the resource data sync.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceRegions")]
    pub source_regions: Vec<String>,


    /// 
    /// Information about the AwsOrganizationsSource resource data sync source. A sync source of    this type can synchronize data from AWS Organizations.
    /// 
    /// Required: No
    ///
    /// Type: AwsOrganizationsSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsOrganizationsSource")]
    pub aws_organizations_source: Option<AwsOrganizationsSource>,

}


/// Information about the AwsOrganizationsSource resource data sync source. A sync  source of this type can synchronize data from AWS Organizations or, if an AWS organization isn't  present, from multiple AWS Regions.
#[derive(Default, serde::Serialize)]
pub struct AwsOrganizationsSource {


    /// 
    /// The AWS Organizations organization units included in the sync.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnits")]
    pub organizational_units: Option<Vec<String>>,


    /// 
    /// If an AWS organization is present, this is either OrganizationalUnits or   EntireOrganization. For OrganizationalUnits, the data is aggregated  from a set of organization units. For EntireOrganization, the data is aggregated  from the entire AWS organization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationSourceType")]
    pub organization_source_type: String,

}


/// Information about the target S3 bucket for the resource data sync.
#[derive(Default, serde::Serialize)]
pub struct S3Destination {


    /// 
    /// An Amazon S3 prefix for the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The name of the S3 bucket where the aggregated data is stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The ARN of an encryption key for a destination in Amazon S3. Must belong to the same  Region as the destination S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KMSKeyArn")]
    pub kmskey_arn: Option<String>,


    /// 
    /// The AWS Region with the S3 bucket targeted by the resource data sync.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketRegion")]
    pub bucket_region: String,


    /// 
    /// A supported sync format. The following format is currently supported: JsonSerDe
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: JsonSerDe
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncFormat")]
    pub sync_format: String,

}