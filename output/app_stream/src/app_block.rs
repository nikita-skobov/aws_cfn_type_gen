

/// This resource creates an app block. App blocks store details about the virtual hard disk that     contains the files for the application in an S3 bucket. It also stores the setup script     with details about how to mount the virtual hard disk. App blocks are only supported for     Elastic fleets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAppBlock {


    /// The description of the app block.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// The display name of the app block.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,


    /// The name of the app block.
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// The setup script details of the app block.
    ///
    /// Required: Yes
    ///
    /// Type: ScriptDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "SetupScriptDetails")]
    pub setup_script_details: ScriptDetails,


    /// The source S3 location of the app block.
    ///
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceS3Location")]
    pub source_s3_location: S3Location,


    /// The tags of the app block.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnAppBlock {
    fn type_string() -> &'static str {
        "AWS::AppStream::AppBlock"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The S3 location of the app block.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The S3 bucket of the app block.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The S3 key of the S3 object of the virtual hard disk.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}




/// The details of the script.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScriptDetails {


    /// 
    /// The parameters used in the run path for the script.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutableParameters")]
    pub executable_parameters: Option<String>,


    /// 
    /// The run path for the script.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutablePath")]
    pub executable_path: String,


    /// 
    /// The S3 object location of the script.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScriptS3Location")]
    pub script_s3_location: S3Location,


    /// 
    /// The run timeout, in seconds, for the script.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: i64,

}




/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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


