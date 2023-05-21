

/// Creates an Amazon FSx for Lustre data repository association (DRA). A data       repository association is a link between a directory on the file system and       an Amazon S3 bucket or prefix. You can have a maximum of 8 data repository       associations on a file system. Data repository associations are supported only       for file systems with the Persistent_2 deployment type.
///
/// Each data repository association must have a unique Amazon FSx file       system directory and a unique S3 bucket or prefix associated with it. You       can configure a data repository association for automatic import only,       for automatic export only, or for both. To learn more about linking a       data repository to your file system, see       Linking your file system to an S3 bucket.
#[derive(Default, serde::Serialize)]
pub struct CfnDataRepositoryAssociation {


    /// 
    /// The path to the Amazon S3 data repository that will be linked to the file       system. The path can be an S3 bucket or prefix in the format       s3://myBucket/myPrefix/. This path specifies where in the S3       data repository files will be imported from or exported to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 4357
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{3,4357}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataRepositoryPath")]
    pub data_repository_path: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A path on the Amazon FSx for Lustre file system that points to a high-level directory (such       as /ns1/) or subdirectory (such as /ns1/subdir/)       that will be mapped 1-1 with DataRepositoryPath.       The leading forward slash in the name is required. Two data repository       associations cannot have overlapping file system paths. For example, if       a data repository is associated with file system path /ns1/,       then you cannot link another data repository with file system       path /ns1/ns2.
    /// 
    /// This path specifies where in your file system files will be exported       from or imported to. This file system directory can be linked to only one       Amazon S3 bucket, and no other S3 bucket can be linked to the directory.
    /// 
    /// NoteIf you specify only a forward slash (/) as the file system       path, you can link only one data repository to the file system. You can only specify       "/" as the file system path for the first data repository associated with a file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,4096}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemPath")]
    pub file_system_path: String,


    /// 
    /// The ID of the file system on which the data repository association is configured.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,


    /// 
    /// For files imported from a data repository, this value determines the stripe count and       maximum amount of data per file (in MiB) stored on a single physical disk. The maximum       number of disks that a single file can be striped across is limited by the total number       of disks that make up the file system or cache.
    /// 
    /// The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500       GiB). Amazon S3 objects have a maximum size of 5 TB.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImportedFileChunkSize")]
    pub imported_file_chunk_size: Option<i64>,


    /// 
    /// A boolean flag indicating whether an import data repository task to import       metadata should run after the data repository association is created. The       task runs if this flag is set to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "BatchImportMetaDataOnCreate")]
    pub batch_import_meta_data_on_create: Option<bool>,


    /// 
    /// The configuration for an Amazon S3 data repository linked to an       Amazon FSx Lustre file system with a data repository association.       The configuration defines which file events (new, changed, or       deleted files or directories) are automatically imported from       the linked data repository to the file system or automatically       exported from the file system to the data repository.
    /// 
    /// Required: No
    ///
    /// Type: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3>,

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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// The configuration for an Amazon S3 data repository linked to an       Amazon FSx Lustre file system with a data repository association.       The configuration defines which file events (new, changed, or       deleted files or directories) are automatically imported from       the linked data repository to the file system or automatically       exported from the file system to the data repository.
#[derive(Default, serde::Serialize)]
pub struct S3 {


    /// 
    /// Describes the data repository association's automatic import policy.       The AutoImportPolicy defines how Amazon FSx keeps your file metadata and directory       listings up to date by importing changes to your Amazon FSx for Lustre file system       as you modify objects in a linked S3 bucket.
    /// 
    /// The AutoImportPolicy is only supported on Amazon FSx for Lustre file systems       with a data repository association.
    /// 
    /// Required: No
    ///
    /// Type: AutoImportPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoImportPolicy")]
    pub auto_import_policy: Option<AutoImportPolicy>,


    /// 
    /// Describes a data repository association's automatic export policy. The       AutoExportPolicy defines the types of updated objects on the       file system that will be automatically exported to the data repository.       As you create, modify, or delete files, Amazon FSx for Lustre       automatically exports the defined changes asynchronously once your application finishes       modifying the file.
    /// 
    /// The AutoExportPolicy is only supported on Amazon FSx for Lustre file systems       with a data repository association.
    /// 
    /// Required: No
    ///
    /// Type: AutoExportPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoExportPolicy")]
    pub auto_export_policy: Option<AutoExportPolicy>,

}


/// Describes the data repository association's automatic import policy.       The AutoImportPolicy defines how Amazon FSx keeps your file metadata and directory       listings up to date by importing changes to your Amazon FSx for Lustre file system       as you modify objects in a linked S3 bucket.
///
/// The AutoImportPolicy is only supported on Amazon FSx for Lustre file systems       with a data repository association.
#[derive(Default, serde::Serialize)]
pub struct AutoImportPolicy {


    /// 
    /// The AutoImportPolicy can have the following event values:
    /// 
    /// NEW - Amazon FSx automatically imports metadata of         files added to the linked S3 bucket that do not currently exist in the FSx         file system.                        CHANGED - Amazon FSx automatically updates file         metadata and invalidates existing file content on the file system as files         change in the data repository.                        DELETED - Amazon FSx automatically deletes files         on the file system as corresponding files are deleted in the data repository.
    /// 
    /// You can define any combination of event types for your AutoImportPolicy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Vec<String>,

}


/// Describes a data repository association's automatic export policy. The       AutoExportPolicy defines the types of updated objects on the       file system that will be automatically exported to the data repository.       As you create, modify, or delete files, Amazon FSx for Lustre       automatically exports the defined changes asynchronously once your application finishes       modifying the file.
///
/// The AutoExportPolicy is only supported on Amazon FSx for Lustre file systems       with a data repository association.
#[derive(Default, serde::Serialize)]
pub struct AutoExportPolicy {


    /// 
    /// The AutoExportPolicy can have the following event values:
    /// 
    /// NEW - New files and directories are automatically exported         to the data repository as they are added to the file system.                        CHANGED - Changes to files and directories on the         file system are automatically exported to the data repository.                        DELETED - Files and directories are automatically deleted         on the data repository when they are deleted on the file system.
    /// 
    /// You can define any combination of event types for your AutoExportPolicy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Vec<String>,

}