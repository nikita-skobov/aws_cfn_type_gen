

/// The AWS::GameLift::Script resource creates a new script record for your    Realtime Servers script. Realtime scripts are JavaScript that provide configuration settings    and optional custom game logic for your game. The script is deployed when you create a    Realtime Servers fleet to host your game sessions. Script logic is executed during an active    game session.
#[derive(Default, serde::Serialize)]
pub struct CfnScript {


    /// 
    /// The location of the Amazon S3 bucket where a zipped file containing your Realtime scripts is       stored. The storage location must specify the Amazon S3 bucket name, the zip file name (the       "key"), and a role ARN that allows Amazon GameLift to access the Amazon S3 storage location. The S3       bucket must be in the same Region where you want to create a new script. By default,       Amazon GameLift uploads the latest version of the zip file; if you have S3 object versioning       turned on, you can use the ObjectVersion parameter to specify an earlier       version.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageLocation")]
    pub storage_location: S3Location,


    /// 
    /// The version that is associated with a build or script. Version strings do not need to    be unique.
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
    /// A list of labels to assign to the new script resource. Tags are developer-defined    key-value pairs. Tagging    AWS resources are useful for resource management, access management and cost allocation.    For more information, see Tagging AWS Resources in the        AWS General Reference. Once the resource is created, you can    use TagResource, UntagResource, and    ListTagsForResource to add, remove, and view tags. The    maximum tag limit may be lower than stated. See the AWS General Reference for actual    tagging limits.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A descriptive label that is associated with a script. Script names do not need to be    unique.
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


/// The location in Amazon S3 where build or script files can be stored for access by    Amazon GameLift.
#[derive(Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The name of the zip file that contains the build files or script files.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// The Amazon Resource Name (ARN) for an IAM role that       allows Amazon GameLift to access the S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The version of the file, if object versioning is turned on for the bucket. Amazon GameLift uses       this information when retrieving files from an S3 bucket that you own. Use this       parameter to specify a specific version of the file. If not set, the latest version of       the file is retrieved.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,

}
