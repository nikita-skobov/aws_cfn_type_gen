

/// The AWS::SSM::Document resource creates a Systems Manager (SSM) document in AWS Systems Manager. This document defines the actions that Systems Manager performs on your AWS     resources.
#[derive(Default, serde::Serialize)]
pub struct CfnDocument {


    /// 
    /// A name for the SSM document.
    /// 
    /// ImportantYou can't use the following strings as document name prefixes. These are reserved by AWS   for use as document name prefixes:                                                     aws                                      amazon                                      amzn
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The content for the new SSM document in JSON or YAML. For more information about the schemas for SSM document content, see SSM document schema features and examples in the AWS Systems Manager User Guide.
    /// 
    /// NoteThis parameter also supports String data types.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: serde_json::Value,


    /// 
    /// Specify the document format for the request. JSON is the default format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: JSON | TEXT | YAML
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentFormat")]
    pub document_format: Option<String>,


    /// 
    /// A list of SSM documents required by a document. This parameter is used exclusively by  AWS AppConfig. When a user creates an AWS AppConfig configuration in an SSM document, the user must also  specify a required document for validation purposes. In this case, an   ApplicationConfiguration document requires an   ApplicationConfigurationSchema document for validation purposes. For more  information, see What is AWS AppConfig? in the           AWS AppConfig User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of DocumentRequires
    ///
    /// Update requires: No interruption
    #[serde(rename = "Requires")]
    pub requires: Option<Vec<DocumentRequires>>,


    /// 
    /// The type of document to create.
    /// 
    /// Allowed Values: ApplicationConfigurationSchema |     Automation | Automation.ChangeTemplate | Command |       DeploymentStrategy | Package | Policy |       Session
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DocumentType")]
    pub document_type: Option<String>,


    /// 
    /// If the document resource you specify in your template already exists, this parameter determines whether a new version of the existing document is created, or the existing document is replaced. Replace is the default method. If you specify NewVersion for the UpdateMethod parameter, and the Name of the document does not match an existing resource, a new document is created. When you specify NewVersion, the default version of the document is changed to the newly created version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateMethod")]
    pub update_method: Option<String>,


    /// 
    /// Specify a target type to define the kinds of resources the document can run on. For example,  to run a document on EC2 instances, specify the following value: /AWS::EC2::Instance. If you  specify a value of '/' the document can run on all types of resources. If you don't specify a  value, the document can't run on any resources. For a list of valid resource types, see AWS resource and property types   reference in the AWS CloudFormation User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^\/[\w\.\-\:\/]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetType")]
    pub target_type: Option<String>,


    /// 
    /// A list of key-value pairs that describe attachments to a version of a document.
    /// 
    /// Required: No
    ///
    /// Type: List of AttachmentsSource
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attachments")]
    pub attachments: Option<Vec<AttachmentsSource>>,


    /// 
    /// An optional field specifying the version of the artifact you are creating with the document.  For example, Release12.1. This value is unique across all versions of a document,  and can't be changed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{1,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionName")]
    pub version_name: Option<String>,


    /// 
    /// AWS CloudFormation resource tags to apply to the document. Use tags to help you identify     and categorize resources.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// Identifying information about a document attachment, including the file name and a key-value  pair that identifies the location of an attachment to a document.
#[derive(Default, serde::Serialize)]
pub struct AttachmentsSource {


    /// 
    /// The name of the document attachment file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The key of a key-value pair that identifies the location of an attachment to a  document.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AttachmentReference | S3FileUrl | SourceUrl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The value of a key-value pair that identifies the location of an attachment to a document.  The format for Value depends on the type of key you  specify.
    /// 
    /// For the key SourceUrl, the value is an S3 bucket location. For   example:                  "Values": [ "s3://doc-example-bucket/my-folder" ]                       For the key S3FileUrl, the value is a file in an S3 bucket. For   example:                  "Values": [ "s3://doc-example-bucket/my-folder/my-file.py" ]                       For the key AttachmentReference, the value is constructed from the   name of another SSM document in your account, a version number of that document, and a file   attached to that document version that you want to reuse. For example:                  "Values": [ "MyOtherDocument/3/my-other-file.py" ]                However, if the SSM document is shared with you from another account, the full SSM   document ARN must be specified instead of the document name only. For example:                  "Values": [    "arn:aws:ssm:us-east-2:111122223333:document/OtherAccountDocument/3/their-file.py"    ]
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

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


/// An SSM document required by the current document.
#[derive(Default, serde::Serialize)]
pub struct DocumentRequires {


    /// 
    /// The document version required by the current document.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ([$]LATEST|[$]DEFAULT|^[1-9][0-9]*$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The name of the required SSM document. The name can be an Amazon Resource Name (ARN).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.:/]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
