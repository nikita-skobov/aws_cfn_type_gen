

/// The AWS::GameLift::Build resource creates a game server build that is    installed and run on instances in an Amazon GameLift fleet. This resource points to an Amazon    S3 location that contains a zip file with all of the components of the game server    build.
#[derive(Default, serde::Serialize)]
pub struct CfnBuild {


    /// 
    /// Information indicating where your game build files are stored. Use this parameter only       when creating a build with files stored in an Amazon S3 bucket that you own. The storage       location must specify an Amazon S3 bucket name and key. The location must also specify a role       ARN that you set up to allow Amazon GameLift to access your Amazon S3 bucket. The S3 bucket and your       new build must be in the same Region.
    /// 
    /// If a StorageLocation is specified, the size of your file can be found in       your Amazon S3 bucket. Amazon GameLift will report a SizeOnDisk of 0.
    /// 
    /// Required: No
    ///
    /// Type: StorageLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageLocation")]
    pub storage_location: Option<StorageLocation>,


    /// 
    /// The operating system that your game server binaries run on. This value determines the       type of fleet resources that you use for this build. If your game build contains       multiple executables, they all must run on the same operating system. You must specify a       valid operating system in this request. There is no default value. You can't change a       build's operating system later.
    /// 
    /// NoteIf you have active fleets using the Windows Server 2012 operating system, you can continue to         create new builds using this OS until October 10, 2023, when Microsoft ends its         support. All others must use Windows Server 2016 when creating new Windows-based         builds.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AMAZON_LINUX | AMAZON_LINUX_2 | WINDOWS_2012 | WINDOWS_2016
    ///
    /// Update requires: Replacement
    #[serde(rename = "OperatingSystem")]
    pub operating_system: Option<String>,


    /// 
    /// Version information that is associated with this build. Version strings do not need to be unique.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// A server SDK version you used when integrating your game server build with Amazon GameLift. For more information see Integrate games         with custom game servers. By default Amazon GameLift sets this value to         4.0.2.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^\d+\.\d+\.\d+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerSdkVersion")]
    pub server_sdk_version: Option<String>,


    /// 
    /// A descriptive label that is associated with a build. Build names do not need to be    unique.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The location in Amazon S3 where build or script files are stored for access by Amazon    GameLift.
#[derive(Default, serde::Serialize)]
pub struct StorageLocation {


    /// 
    /// The version of the file, if object versioning is turned on for the bucket. Amazon GameLift    uses this information when retrieving files from your S3 bucket. To retrieve a specific    version of the file, provide an object version. To retrieve the latest version of the    file, do not set this parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) for an IAM role that       allows Amazon GameLift to access the S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the zip file that contains the build files or script files.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// An Amazon S3 bucket identifier. Thename of the S3 bucket.
    /// 
    /// NoteAmazon GameLift doesn't support uploading from Amazon S3 buckets with names that contain a dot         (.).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,

}