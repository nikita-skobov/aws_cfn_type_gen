

/// Creates an new set of frequently asked question (FAQ) questions and answers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFaq {


    /// 
    /// A description for the FAQ.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The format of the input file. You can choose between a basic CSV       format, a CSV format that includes customs attributes in a header,       and a JSON format that includes custom attributes.
    /// 
    /// The format must match the format of the file stored in the S3       bucket identified in the S3Path parameter.
    /// 
    /// Valid values are:
    /// 
    /// CSV               CSV_WITH_HEADER               JSON
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileFormat")]
    pub file_format: Option<String>,


    /// 
    /// The identifier of the index that contains the FAQ.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IndexId")]
    pub index_id: String,


    /// 
    /// The name that you assigned the FAQ when you created or updated the FAQ.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of a role with permission to access       the S3 bucket that contains the FAQ.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon Simple Storage Service (Amazon S3) location of the FAQ       input data.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Path
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Path")]
    pub s3_path: S3Path,


    /// 
    /// An array of key-value pairs to apply to this resource
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

}



impl cfn_resources::CfnResource for CfnFaq {
    fn type_string() -> &'static str {
        "AWS::Kendra::Faq"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information required to find a specific file in an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Path {


    /// 
    /// The name of the S3 bucket that contains the file.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// The name of the file.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,

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


