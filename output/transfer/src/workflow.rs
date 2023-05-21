/// Allows you to create a workflow with specified steps and step details the workflow invokes after file transfer completes.    After creating a workflow, you can associate the workflow created with any transfer servers by specifying the workflow-details field in CreateServer and UpdateServer operations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWorkflow {
    ///
    /// Specifies the text description for the workflow.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\w- ]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the steps (actions) to take if errors are encountered during execution of the workflow.
    ///
    /// Required: No
    ///
    /// Type: List of WorkflowStep
    ///
    /// Maximum: 8
    ///
    /// Update requires: Replacement
    #[serde(rename = "OnExceptionSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exception_steps: Option<Vec<WorkflowStep>>,

    ///
    /// Specifies the details for the steps that are in the specified workflow.
    ///
    /// Required: Yes
    ///
    /// Type: List of WorkflowStep
    ///
    /// Maximum: 8
    ///
    /// Update requires: Replacement
    #[serde(rename = "Steps")]
    pub steps: Vec<WorkflowStep>,

    ///
    /// Key-value pairs that can be used to group and search for workflows. Tags are metadata attached to workflows for any purpose.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnWorkflow {
    fn type_string(&self) -> &'static str {
        "AWS::Transfer::Workflow"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.on_exception_steps {
            if the_val.len() > 8 as _ {
                return Err(format!(
                    "Max validation failed on field 'on_exception_steps'. {} is greater than 8",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.steps;

        if the_val.len() > 8 as _ {
            return Err(format!(
                "Max validation failed on field 'steps'. {} is greater than 8",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Details for a step that performs a file copy.
///
/// Consists of the following values:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CopyStepDetails {
    ///
    /// Specifies the location for the file being copied. Use ${Transfer:UserName} or     ${Transfer:UploadDate} in this field to parametrize the destination prefix by    username or uploaded date.
    ///
    /// Set the value of DestinationFileLocation to       ${Transfer:UserName} to copy uploaded files to an Amazon S3 bucket      that is prefixed with the name of the Transfer Family user that uploaded the      file.               Set the value of DestinationFileLocation to ${Transfer:UploadDate} to copy uploaded files to      an Amazon S3 bucket that is prefixed with the date of the upload.        NoteThe system resolves UploadDate to a date format of YYYY-MM-DD, based on the date the file       is uploaded in UTC.
    ///
    /// Required: No
    ///
    /// Type: S3FileLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_file_location: Option<S3FileLocation>,

    ///
    /// The name of the step, used as an identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// A flag that indicates whether to overwrite an existing file of the same name.    The default is FALSE.
    ///
    /// If the workflow is processing a file that has the same name as an existing file, the behavior is as follows:
    ///
    /// If OverwriteExisting is TRUE, the existing file is replaced with the file being processed.               If OverwriteExisting is FALSE, nothing happens, and the workflow processing stops.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OverwriteExisting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existing: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file   for the workflow.
    ///
    /// To use the previous file as the input, enter ${previous.file}.      In this case, this workflow step uses the output file from the previous workflow step as input.      This is the default value.               To use the originally uploaded file location as input for this step, enter ${original.file}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CopyStepDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_file_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details for a step that invokes an AWS Lambda function.
///
/// Consists of the Lambda function's name, target, and timeout (in seconds).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomStepDetails {
    ///
    /// The name of the step, used as an identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file   for the workflow.
    ///
    /// To use the previous file as the input, enter ${previous.file}.      In this case, this workflow step uses the output file from the previous workflow step as input.      This is the default value.               To use the originally uploaded file location as input for this step, enter ${original.file}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<cfn_resources::StrVal>,

    ///
    /// The ARN for the Lambda function that is being called.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<cfn_resources::StrVal>,

    ///
    /// Timeout, in seconds, for the step.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for CustomStepDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for a step that decrypts an encrypted file.
///
/// Consists of the following values:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DecryptStepDetails {
    ///
    /// Specifies the location for the file being decrypted. Use ${Transfer:UserName} or    ${Transfer:UploadDate} in this field to parametrize the destination prefix by    username or uploaded date.
    ///
    /// Set the value of DestinationFileLocation to      ${Transfer:UserName} to decrypt uploaded files to an Amazon S3 bucket      that is prefixed with the name of the Transfer Family user that uploaded the      file.               Set the value of DestinationFileLocation to ${Transfer:UploadDate} to decrypt uploaded files to      an Amazon S3 bucket that is prefixed with the date of the upload.        NoteThe system resolves UploadDate to a date format of YYYY-MM-DD, based on the date the file       is uploaded in UTC.
    ///
    /// Required: No
    ///
    /// Type: InputFileLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_file_location: Option<InputFileLocation>,

    ///
    /// The name of the step, used as an identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// A flag that indicates whether to overwrite an existing file of the same name.    The default is FALSE.
    ///
    /// If the workflow is processing a file that has the same name as an existing file, the behavior is as follows:
    ///
    /// If OverwriteExisting is TRUE, the existing file is replaced with the file being processed.               If OverwriteExisting is FALSE, nothing happens, and the workflow processing stops.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OverwriteExisting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existing: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file   for the workflow.
    ///
    /// To use the previous file as the input, enter ${previous.file}.      In this case, this workflow step uses the output file from the previous workflow step as input.      This is the default value.               To use the originally uploaded file location as input for this step, enter ${original.file}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<cfn_resources::StrVal>,

    ///
    /// The type of encryption used. Currently, this value must be PGP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DecryptStepDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_file_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that contains the name and file location for a file being deleted by a workflow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeleteStepDetails {
    ///
    /// The name of the step, used as an identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file   for the workflow.
    ///
    /// To use the previous file as the input, enter ${previous.file}.      In this case, this workflow step uses the output file from the previous workflow step as input.      This is the default value.               To use the originally uploaded file location as input for this step, enter ${original.file}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DeleteStepDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the Amazon EFS identifier and the path for the file being used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EfsInputFileLocation {
    ///
    /// The identifier of the file system, assigned by Amazon EFS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<cfn_resources::StrVal>,

    ///
    /// The pathname for the folder being used by a workflow.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for EfsInputFileLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the location for the file that's being processed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputFileLocation {
    ///
    /// Specifies the details for the Amazon Elastic File System (Amazon EFS) file that's being    decrypted.
    ///
    /// Required: No
    ///
    /// Type: EfsInputFileLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "EfsFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_file_location: Option<EfsInputFileLocation>,

    ///
    /// Specifies the details for the Amazon S3 file that's being copied or decrypted.
    ///
    /// Required: No
    ///
    /// Type: S3InputFileLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3FileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_file_location: Option<S3InputFileLocation>,
}

impl cfn_resources::CfnResource for InputFileLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.efs_file_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_file_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the S3 details for the file being used, such as bucket, ETag, and so    forth.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3FileLocation {
    ///
    /// Specifies the details for the file location for the file that's being used in the workflow. Only applicable if you are using Amazon S3 storage.
    ///
    /// Required: No
    ///
    /// Type: S3InputFileLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3FileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_file_location: Option<S3InputFileLocation>,
}

impl cfn_resources::CfnResource for S3FileLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_file_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the details for the Amazon S3 location for an input file to a workflow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3InputFileLocation {
    ///
    /// Specifies the S3 bucket for the customer input file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<cfn_resources::StrVal>,

    ///
    /// The name assigned to the file when it was created in Amazon S3. You use the object key to retrieve the object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3InputFileLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the key-value pair that are assigned to a file during the execution of a Tagging step.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Tag {
    ///
    /// The name assigned to the tag that you create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value that corresponds to the key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for S3Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for a step that creates one or more tags.
///
/// You specify one or more tags. Each tag contains a key-value pair.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagStepDetails {
    ///
    /// The name of the step, used as an identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file   for the workflow.
    ///
    /// To use the previous file as the input, enter ${previous.file}.      In this case, this workflow step uses the output file from the previous workflow step as input.      This is the default value.               To use the originally uploaded file location as input for this step, enter ${original.file}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceFileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<cfn_resources::StrVal>,

    ///
    /// Array that contains from 1 to 10 key/value pairs.
    ///
    /// Required: No
    ///
    /// Type: List of S3Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<S3Tag>>,
}

impl cfn_resources::CfnResource for TagStepDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The basic building block of a workflow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WorkflowStep {
    ///
    /// Details for a step that performs a file copy.
    ///
    /// Consists of the following values:
    ///
    /// A description               An Amazon S3 location for the destination of the file copy.               A flag that indicates whether to overwrite an existing file of the same name. The default is       FALSE.
    ///
    /// Required: No
    ///
    /// Type: CopyStepDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "CopyStepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_step_details: Option<CopyStepDetails>,

    ///
    /// Details for a step that invokes an AWS Lambda function.
    ///
    /// Consists of the Lambda function's name, target, and timeout (in seconds).
    ///
    /// Required: No
    ///
    /// Type: CustomStepDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomStepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_step_details: Option<CustomStepDetails>,

    ///
    /// Details for a step that decrypts an encrypted file.
    ///
    /// Consists of the following values:
    ///
    /// A descriptive name               An Amazon S3 or Amazon Elastic File System (Amazon EFS) location for the source file to      decrypt.               An S3 or Amazon EFS location for the destination of the file decryption.               A flag that indicates whether to overwrite an existing file of the same name. The default is       FALSE.               The type of encryption that's used. Currently, only PGP encryption is supported.
    ///
    /// Required: No
    ///
    /// Type: DecryptStepDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "DecryptStepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypt_step_details: Option<DecryptStepDetails>,

    ///
    /// Details for a step that deletes the file.
    ///
    /// Required: No
    ///
    /// Type: DeleteStepDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeleteStepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_step_details: Option<DeleteStepDetails>,

    ///
    /// Details for a step that creates one or more tags.
    ///
    /// You specify one or more tags. Each tag contains a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: TagStepDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagStepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_step_details: Option<TagStepDetails>,

    ///
    /// Currently, the following step types are supported.
    ///
    /// COPY          - Copy the file to another location.                                   CUSTOM          - Perform a custom step with an AWS Lambda function target.                                   DECRYPT          - Decrypt a file that was encrypted before it was uploaded.                                   DELETE          - Delete the file.                                   TAG          - Add a tag to the file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for WorkflowStep {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.copy_step_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.custom_step_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.decrypt_step_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.delete_step_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.tag_step_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
