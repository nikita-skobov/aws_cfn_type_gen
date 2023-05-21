

/// Creates a new, empty repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRepository {


    /// Information about code to be committed to a repository after it is created in     an AWS CloudFormation stack. Information about code is only used in resource creation. Updates to a stack will not reflect changes made to code     properties after initial resource creation.
    /// 
    /// NoteYou can only use this property to add code when creating a repository with a AWS CloudFormation template at creation time.       This property cannot be used for updating code to an existing repository.
    /// 
    /// Required: No
    ///
    /// Type: Code
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    pub code: Option<Code>,


    /// 
    /// The JSON block of configuration information for each trigger.
    /// 
    /// Required: No
    ///
    /// Type: List of RepositoryTrigger
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Triggers")]
    pub triggers: Option<Vec<RepositoryTrigger>>,


    /// 
    /// A comment or description about the new repository.
    /// 
    /// NoteThe description field for a repository accepts all HTML characters and all valid         Unicode characters. Applications that do not HTML-encode the description and display         it in a webpage can expose users to potentially malicious code. Make sure that you         HTML-encode the description field in any application that uses this API to display         the repository description on a webpage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryDescription")]
    pub repository_description: Option<String>,


    /// 
    /// The name of the new repository to be created.
    /// 
    /// NoteThe repository name must be unique across the calling AWS account. Repository names         are limited to 100 alphanumeric, dash, and underscore characters, and cannot include         certain characters. For more information about the limits on repository names, see           Quotas in the          AWS CodeCommit User Guide. The         suffix .git is prohibited.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [\w\.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,


    /// 
    /// One or more tag key-value pairs to use when tagging this repository.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnRepository {
    fn type_string() -> &'static str {
        "AWS::CodeCommit::Repository"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information about code to be committed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Code {


    /// 
    /// Optional. Specifies a branch name to be used as the default branch when importing code into a repository on initial creation.       If this property is not set, the name main       will be used for the default branch for the repository. Changes to this property are ignored after initial resource creation.        We recommend using this parameter to set the name to main to align with the default behavior       of CodeCommit unless another name is needed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BranchName")]
    pub branch_name: Option<String>,


    /// Information about the Amazon S3 bucket that contains a ZIP file of code     to be committed to the repository. Changes to this property are ignored after initial resource creation.
    ///
    /// Required: Yes
    ///
    /// Type: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: S3,

}




/// Information about a trigger for a repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RepositoryTrigger {


    /// 
    /// The ARN of the resource that is the target for a trigger (for example, the ARN of a       topic in Amazon SNS).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,


    /// 
    /// Any custom data associated with the trigger to be included in the information sent to       the target of the trigger.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomData")]
    pub custom_data: Option<String>,


    /// 
    /// The repository events that cause the trigger to run actions in another service, such       as sending a notification through Amazon SNS.
    /// 
    /// NoteThe valid value "all" cannot be used with any other values.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Vec<String>,


    /// 
    /// The name of the trigger.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The branches to be included in the trigger configuration. If you specify an empty       array, the trigger applies to all branches.
    /// 
    /// NoteAlthough no content is required in the array, you must include the array itself.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Branches")]
    pub branches: Option<Vec<String>>,

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




/// Information about the Amazon S3 bucket that contains the code that will be committed to the new repository.     Changes to this property are ignored after initial resource creation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3 {


    /// The name of the Amazon S3 bucket that contains the ZIP file with the content that       will be committed to the new repository. This can be specified using the name of the       bucket in the AWS account. Changes to this property are ignored after       initial resource creation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// The key to use for accessing the Amazon S3 bucket. Changes to this property are     ignored after initial resource creation. For more information, see Creating object key names     and Uploading objects in the Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// The object version of the ZIP file, if versioning is enabled for the Amazon S3 bucket.     Changes to this property are ignored after initial resource creation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,

}


