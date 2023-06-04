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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDataSync {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_region: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmskey_arn: Option<cfn_resources::StrVal>,

    ///
    /// Configuration information for the target S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: S3Destination
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_format: Option<ResourceDataSyncSyncFormatEnum>,

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
    pub sync_name: cfn_resources::StrVal,

    ///
    /// Information about the source where the data was synchronized.
    ///
    /// Required: No
    ///
    /// Type: SyncSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_source: Option<SyncSource>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_sync_name: CfnResourceDataSyncsyncname,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResourceDataSyncSyncFormatEnum {
    /// JsonSerDe
    #[serde(rename = "JsonSerDe")]
    Jsonserde,
}

impl Default for ResourceDataSyncSyncFormatEnum {
    fn default() -> Self {
        ResourceDataSyncSyncFormatEnum::Jsonserde
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDataSyncsyncname;
impl CfnResourceDataSyncsyncname {
    pub fn att_name(&self) -> &'static str {
        r#"SyncName"#
    }
}

impl cfn_resources::CfnResource for CfnResourceDataSync {
    fn type_string(&self) -> &'static str {
        "AWS::SSM::ResourceDataSync"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bucket_name'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bucket_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bucket_prefix'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bucket_prefix'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bucket_region'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bucket_region'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kmskey_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kmskey_arn'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kmskey_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kmskey_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.s3_destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.sync_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'sync_name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.sync_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'sync_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.sync_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.sync_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'sync_type'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.sync_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sync_type'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Information about the AwsOrganizationsSource resource data sync source. A sync  source of this type can synchronize data from AWS Organizations or, if an AWS organization isn't  present, from multiple AWS Regions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AwsOrganizationsSource {
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
    pub organization_source_type: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for AwsOrganizationsSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.organization_source_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!("Max validation failed on field 'organization_source_type'. {} is greater than 64", s.len()));
            }
        }

        let the_val = &self.organization_source_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'organization_source_type'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.organizational_units {
            if the_val.len() > 1000 as _ {
                return Err(format!("Max validation failed on field 'organizational_units'. {} is greater than 1000", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Information about the target S3 bucket for the resource data sync.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3Destination {
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
    pub bucket_name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<cfn_resources::StrVal>,

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
    pub bucket_region: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmskey_arn: Option<cfn_resources::StrVal>,

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
    pub sync_format: S3DestinationSyncFormatEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum S3DestinationSyncFormatEnum {
    /// JsonSerDe
    #[serde(rename = "JsonSerDe")]
    Jsonserde,
}

impl Default for S3DestinationSyncFormatEnum {
    fn default() -> Self {
        S3DestinationSyncFormatEnum::Jsonserde
    }
}

impl cfn_resources::CfnResource for S3Destination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket_name'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.bucket_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.bucket_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bucket_prefix'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bucket_prefix'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.bucket_region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket_region'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.bucket_region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket_region'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.kmskey_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kmskey_arn'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kmskey_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kmskey_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Information about the source of the data included in the resource data sync.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SyncSource {
    ///
    /// Information about the AwsOrganizationsSource resource data sync source. A sync source of    this type can synchronize data from AWS Organizations.
    ///
    /// Required: No
    ///
    /// Type: AwsOrganizationsSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsOrganizationsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organizations_source: Option<AwsOrganizationsSource>,

    ///
    /// Whether to automatically synchronize and aggregate data from new AWS Regions when those  Regions come online.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeFutureRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_regions: Option<bool>,

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
    pub source_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SyncSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.aws_organizations_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.source_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'source_type'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.source_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'source_type'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
