

/// The AWS::DataSync::Task resource specifies a task. A task is a set of two     locations (source and destination) and a set of Options that you use to     control the behavior of a task. If you don't specify Options when you create a     task, AWS DataSync populates them with service defaults.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTask {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to    monitor and log events in the task.
    /// 
    /// For more information about how to use CloudWatch Logs with DataSync, see Monitoring Your     Task in the         AWS DataSync User Guide.
    /// 
    /// For more information about these groups, see Working with Log     Groups and Log Streams in the Amazon CloudWatch Logs User    Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 562
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):logs:[a-z\-0-9]+:[0-9]{12}:log-group:([^:\*]*)(:\*)?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS storage resource's     location.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationLocationArn")]
    pub destination_location_arn: String,


    /// 
    /// Specifies a list of filter rules that exclude specific data during your transfer. For more    information and examples, see Filtering data transferred by DataSync.
    /// 
    /// Required: No
    ///
    /// Type: List of FilterRule
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excludes")]
    pub excludes: Option<Vec<FilterRule>>,


    /// 
    /// Specifies a list of filter rules that include specific data during your transfer. For more    information and examples, see Filtering data transferred by DataSync.
    /// 
    /// Required: No
    ///
    /// Type: List of FilterRule
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Includes")]
    pub includes: Option<Vec<FilterRule>>,


    /// 
    /// The name of a task. This value is a text reference that is used to identify the task in    the console.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9\s+=._:@/-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Specifies the configuration options for a task. Some options include preserving file or    object metadata and verifying data integrity.
    /// 
    /// You can also override these options before starting an individual run of a task (also    known as a task execution). For more information, see StartTaskExecution.
    /// 
    /// Required: No
    ///
    /// Type: Options
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    pub options: Option<Options>,


    /// 
    /// Specifies a schedule used to periodically transfer files from a source to a destination    location. The schedule should be specified in UTC time. For more information, see Scheduling your     task.
    /// 
    /// Required: No
    ///
    /// Type: TaskSchedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<TaskSchedule>,


    /// 
    /// The Amazon Resource Name (ARN) of the source location for the task.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:location/loc-[0-9a-z]{17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceLocationArn")]
    pub source_location_arn: String,


    /// 
    /// Specifies the tags that you want to apply to the Amazon Resource Name (ARN)    representing the task.
    /// 
    /// Tags are key-value pairs that help you manage, filter, and search    for your DataSync resources.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnTask {
    fn type_string() -> &'static str {
        "AWS::DataSync::Task"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.cloud_watch_log_group_arn {

        if the_val.len() > 562 as _ {
            return Err(format!("Max validation failed on field 'cloud_watch_log_group_arn'. {} is greater than 562", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.excludes {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'excludes'. {} is greater than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.includes {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'includes'. {} is greater than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.schedule.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.source_location_arn;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'source_location_arn'. {} is greater than 128", the_val.len()));
        }

        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Specifies which files, folders, and objects to include or exclude when transferring files    from source to destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterRule {


    /// 
    /// The type of filter rule to apply. AWS DataSync only supports the SIMPLE_PATTERN    rule type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SIMPLE_PATTERN
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterType")]
    pub filter_type: Option<FilterRuleFilterTypeEnum>,


    /// 
    /// A single filter string that consists of the patterns to include or exclude. The patterns    are delimited by "|" (that is, a pipe), for example: /folder1|/folder2
    /// 
    /// 
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 102400
    ///
    /// Pattern: ^[^\x00]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FilterRuleFilterTypeEnum {

    /// SIMPLE_PATTERN
    #[serde(rename = "SIMPLE_PATTERN")]
    Simplepattern,

}

impl Default for FilterRuleFilterTypeEnum {
    fn default() -> Self {
        FilterRuleFilterTypeEnum::Simplepattern
    }
}


impl cfn_resources::CfnResource for FilterRule {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.value {

        if the_val.len() > 102400 as _ {
            return Err(format!("Max validation failed on field 'value'. {} is greater than 102400", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Represents the options that are available to control the behavior of a StartTaskExecution operation. This behavior includes preserving metadata, such     as user ID (UID), group ID (GID), and file permissions; overwriting files in the     destination; data integrity verification; and so on.
///
/// A task has a set of default options associated with it. If you don't specify an     option in StartTaskExecution,     the default value is used. You can override the default options on each task execution by     specifying an overriding Options value to StartTaskExecution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Options {


    /// 
    /// A file metadata value that shows the last time that a file was accessed (that is,     when the file was read or written to). If you set Atime to       BEST_EFFORT, AWS DataSync attempts to preserve the original       Atime attribute on all source files (that is, the version before the     PREPARING phase). However, Atime's behavior is not fully standard across     platforms, so AWS DataSync can only do this on a best-effort basis.
    /// 
    /// Default value: BEST_EFFORT
    /// 
    /// BEST_EFFORT: Attempt to preserve the per-file Atime     value (recommended).
    /// 
    /// NONE: Ignore Atime.
    /// 
    /// NoteIf Atime is set to BEST_EFFORT, Mtime must       be set to PRESERVE. If Atime is set to NONE, Mtime must also be        NONE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BEST_EFFORT | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Atime")]
    pub atime: Option<OptionsAtimeEnum>,


    /// 
    /// A value that limits the bandwidth used by AWS DataSync. For example, if you want    AWS DataSync to use a maximum of 1 MB, set this value to 1048576     (=1024*1024).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BytesPerSecond")]
    pub bytes_per_second: Option<i64>,


    /// 
    /// The group ID (GID) of the file's owners.
    /// 
    /// Default value:     INT_VALUE
    /// 
    /// INT_VALUE: Preserve the integer value of the user ID (UID) and group ID     (GID) (recommended).
    /// 
    /// NAME: Currently not supported.
    /// 
    /// NONE: Ignore the UID and GID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOTH | INT_VALUE | NAME | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gid")]
    pub gid: Option<OptionsGidEnum>,


    /// 
    /// Specifies the type of logs that DataSync publishes to a Amazon CloudWatch Logs    log group. To specify the log group, see CloudWatchLogGroupArn.
    /// 
    /// If you set LogLevel to OFF, no logs are published.     BASIC publishes logs on errors for individual files transferred.     TRANSFER publishes logs for every file or object that is transferred and    integrity checked.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BASIC | OFF | TRANSFER
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: Option<OptionsLogLevelEnum>,


    /// 
    /// A value that indicates the last time that a file was modified (that is, a file was    written to) before the PREPARING phase. This option is required for cases when you need to run    the same task more than one time.
    /// 
    /// Default value: PRESERVE
    /// 
    /// PRESERVE: Preserve original Mtime     (recommended)
    /// 
    /// NONE: Ignore Mtime.
    /// 
    /// NoteIf Mtime is set to PRESERVE, Atime must be       set to BEST_EFFORT.If Mtime is set to NONE, Atime must also be       set to NONE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | PRESERVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mtime")]
    pub mtime: Option<OptionsMtimeEnum>,


    /// 
    /// Specifies whether object tags are preserved when transferring between object storage    systems. If you want your DataSync task to ignore object tags, specify the     NONE value.
    /// 
    /// Default Value: PRESERVE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | PRESERVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectTags")]
    pub object_tags: Option<OptionsObjectTagsEnum>,


    /// 
    /// Specifies whether data at the destination location should be overwritten or preserved. If    set to NEVER, a destination file for example will not be replaced by a source    file (even if the destination file differs from the source file). If you modify files in the    destination and you sync the files, you can use this value to protect against overwriting    those changes.
    /// 
    /// Some storage classes have specific behaviors that can affect your Amazon S3    storage cost. For detailed information, see Considerations     when working with Amazon S3 storage classes in DataSync.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALWAYS | NEVER
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverwriteMode")]
    pub overwrite_mode: Option<OptionsOverwriteModeEnum>,


    /// 
    /// A value that determines which users or groups can access a file for a specific     purpose, such as reading, writing, or execution of the file. This option should be set only     for Network File System (NFS), Amazon EFS, and Amazon S3 locations. For more information     about what metadata is copied by DataSync, see Metadata Copied by       DataSync.
    /// 
    /// Default value: PRESERVE
    /// 
    /// PRESERVE: Preserve POSIX-style permissions (recommended).
    /// 
    /// NONE: Ignore permissions.
    /// 
    /// Note        AWS DataSync can preserve extant permissions of a source location.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | PRESERVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "PosixPermissions")]
    pub posix_permissions: Option<OptionsPosixPermissionsEnum>,


    /// 
    /// A value that specifies whether files in the destination that don't exist in the     source file system are preserved. This option can affect your storage costs. If your task     deletes objects, you might incur minimum storage duration charges for certain storage     classes. For detailed information, see Considerations when working with Amazon S3 storage classes in DataSync in the       AWS DataSync User Guide.
    /// 
    /// Default value: PRESERVE
    /// 
    /// PRESERVE: Ignore destination files that aren't present in the source     (recommended).
    /// 
    /// REMOVE: Delete destination files that aren't present in the     source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PRESERVE | REMOVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveDeletedFiles")]
    pub preserve_deleted_files: Option<OptionsPreserveDeletedFilesEnum>,


    /// 
    /// A value that determines whether AWS DataSync should preserve the metadata of block    and character devices in the source file system, and re-create the files with that device name    and metadata on the destination. DataSync does not copy the contents of such devices, only the    name and metadata.
    /// 
    /// Note        AWS DataSync can't sync the actual contents of such devices, because they are     nonterminal and don't return an end-of-file (EOF) marker.
    /// 
    /// Default value: NONE
    /// 
    /// NONE: Ignore special devices (recommended).
    /// 
    /// PRESERVE: Preserve character and block device metadata. This option     isn't currently supported for Amazon EFS.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | PRESERVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveDevices")]
    pub preserve_devices: Option<OptionsPreserveDevicesEnum>,


    /// 
    /// A value that determines which components of the SMB security descriptor are copied from source    to destination objects.
    /// 
    /// This value is only used for transfers    between SMB and Amazon FSx for Windows File Server locations, or between two Amazon FSx for Windows File    Server locations. For more information about how    DataSync handles metadata, see    How DataSync Handles Metadata and Special Files.
    /// 
    /// Default value: OWNER_DACL
    /// 
    /// OWNER_DACL: For each copied object, DataSync copies the following     metadata:
    /// 
    /// Object owner.               NTFS discretionary access control lists (DACLs), which determine whether to     grant access to an object.
    /// 
    /// When you use option, DataSync does NOT copy the NTFS system access control lists     (SACLs), which are used by administrators to log attempts to access a secured     object.
    /// 
    /// OWNER_DACL_SACL: For each copied object, DataSync copies the following     metadata:
    /// 
    /// Object owner.               NTFS discretionary access control lists (DACLs), which determine whether to      grant access to an object.               NTFS system access control lists (SACLs), which are used by administrators      to log attempts to access a secured object.
    /// 
    /// Copying SACLs requires granting additional permissions to the Windows user that DataSync    uses to access your SMB location. For information about choosing a user that ensures    sufficient permissions to files, folders, and metadata, see user.
    /// 
    /// NONE: None of the SMB security descriptor components are copied.     Destination objects are owned by the user that was provided for accessing the destination     location. DACLs and SACLs are set based on the destination server’s configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | OWNER_DACL | OWNER_DACL_SACL
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityDescriptorCopyFlags")]
    pub security_descriptor_copy_flags: Option<OptionsSecurityDescriptorCopyFlagsEnum>,


    /// 
    /// Specifies whether tasks should be queued before executing the tasks. The default is     ENABLED, which means the tasks will be queued.
    /// 
    /// If you use the same agent to run multiple tasks, you can enable the tasks to run in    series. For more information, see Queueing task     executions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskQueueing")]
    pub task_queueing: Option<OptionsTaskQueueingEnum>,


    /// 
    /// A value that determines whether DataSync transfers only the data and metadata that differ between the source    and the destination location, or whether DataSync transfers all the content from the source, without comparing it to    the destination location.
    /// 
    /// CHANGED: DataSync copies only data or metadata that is new or different from the source location to the    destination location.
    /// 
    /// ALL: DataSync copies all source location content to the destination, without comparing it to existing content on    the destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | CHANGED
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransferMode")]
    pub transfer_mode: Option<OptionsTransferModeEnum>,


    /// 
    /// The user ID (UID) of the file's owner.
    /// 
    /// Default value:     INT_VALUE
    /// 
    /// INT_VALUE: Preserve the integer value of the UID and group ID (GID)     (recommended).
    /// 
    /// NAME: Currently not supported
    /// 
    /// NONE: Ignore the UID and GID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BOTH | INT_VALUE | NAME | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Uid")]
    pub uid: Option<OptionsUidEnum>,


    /// 
    /// A value that determines whether a data integrity verification is performed at the     end of a task execution after all data and metadata have been transferred. For more     information, see Configure task settings.
    /// 
    /// Default value: POINT_IN_TIME_CONSISTENT
    /// 
    /// ONLY_FILES_TRANSFERRED (recommended): Perform verification only on     files that were transferred.
    /// 
    /// POINT_IN_TIME_CONSISTENT: Scan the entire source and entire     destination at the end of the transfer to verify that the source and destination are fully     synchronized. This option isn't supported when transferring to S3 Glacier or S3 Glacier     Deep Archive storage classes.
    /// 
    /// NONE: No additional verification is done at the end of the transfer,     but all data transmissions are integrity-checked with checksum verification during the     transfer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | ONLY_FILES_TRANSFERRED | POINT_IN_TIME_CONSISTENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifyMode")]
    pub verify_mode: Option<OptionsVerifyModeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsAtimeEnum {

    /// BEST_EFFORT
    #[serde(rename = "BEST_EFFORT")]
    Besteffort,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for OptionsAtimeEnum {
    fn default() -> Self {
        OptionsAtimeEnum::Besteffort
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsGidEnum {

    /// BOTH
    #[serde(rename = "BOTH")]
    Both,

    /// INT_VALUE
    #[serde(rename = "INT_VALUE")]
    Intvalue,

    /// NAME
    #[serde(rename = "NAME")]
    Name,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for OptionsGidEnum {
    fn default() -> Self {
        OptionsGidEnum::Both
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsLogLevelEnum {

    /// BASIC
    #[serde(rename = "BASIC")]
    Basic,

    /// OFF
    #[serde(rename = "OFF")]
    Off,

    /// TRANSFER
    #[serde(rename = "TRANSFER")]
    Transfer,

}

impl Default for OptionsLogLevelEnum {
    fn default() -> Self {
        OptionsLogLevelEnum::Basic
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsMtimeEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

}

impl Default for OptionsMtimeEnum {
    fn default() -> Self {
        OptionsMtimeEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsObjectTagsEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

}

impl Default for OptionsObjectTagsEnum {
    fn default() -> Self {
        OptionsObjectTagsEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsOverwriteModeEnum {

    /// ALWAYS
    #[serde(rename = "ALWAYS")]
    Always,

    /// NEVER
    #[serde(rename = "NEVER")]
    Never,

}

impl Default for OptionsOverwriteModeEnum {
    fn default() -> Self {
        OptionsOverwriteModeEnum::Always
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsPosixPermissionsEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

}

impl Default for OptionsPosixPermissionsEnum {
    fn default() -> Self {
        OptionsPosixPermissionsEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsPreserveDeletedFilesEnum {

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

    /// REMOVE
    #[serde(rename = "REMOVE")]
    Remove,

}

impl Default for OptionsPreserveDeletedFilesEnum {
    fn default() -> Self {
        OptionsPreserveDeletedFilesEnum::Preserve
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsPreserveDevicesEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

}

impl Default for OptionsPreserveDevicesEnum {
    fn default() -> Self {
        OptionsPreserveDevicesEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsSecurityDescriptorCopyFlagsEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// OWNER_DACL
    #[serde(rename = "OWNER_DACL")]
    Ownerdacl,

    /// OWNER_DACL_SACL
    #[serde(rename = "OWNER_DACL_SACL")]
    Ownerdaclsacl,

}

impl Default for OptionsSecurityDescriptorCopyFlagsEnum {
    fn default() -> Self {
        OptionsSecurityDescriptorCopyFlagsEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsTaskQueueingEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

}

impl Default for OptionsTaskQueueingEnum {
    fn default() -> Self {
        OptionsTaskQueueingEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsTransferModeEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// CHANGED
    #[serde(rename = "CHANGED")]
    Changed,

}

impl Default for OptionsTransferModeEnum {
    fn default() -> Self {
        OptionsTransferModeEnum::All
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsUidEnum {

    /// BOTH
    #[serde(rename = "BOTH")]
    Both,

    /// INT_VALUE
    #[serde(rename = "INT_VALUE")]
    Intvalue,

    /// NAME
    #[serde(rename = "NAME")]
    Name,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for OptionsUidEnum {
    fn default() -> Self {
        OptionsUidEnum::Both
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OptionsVerifyModeEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// ONLY_FILES_TRANSFERRED
    #[serde(rename = "ONLY_FILES_TRANSFERRED")]
    Onlyfilestransferred,

    /// POINT_IN_TIME_CONSISTENT
    #[serde(rename = "POINT_IN_TIME_CONSISTENT")]
    Pointintimeconsistent,

}

impl Default for OptionsVerifyModeEnum {
    fn default() -> Self {
        OptionsVerifyModeEnum::None
    }
}


impl cfn_resources::CfnResource for Options {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies the schedule you want your task to use for repeated executions. For more    information, see Schedule Expressions for     Rules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TaskSchedule {


    /// 
    /// A cron expression that specifies when AWS DataSync initiates a scheduled    transfer from a source to a destination location.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9\ \_\*\?\,\|\^\-\/\#\s\(\)\+]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,

}



impl cfn_resources::CfnResource for TaskSchedule {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.schedule_expression;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'schedule_expression'. {} is greater than 256", the_val.len()));
        }

        
        Ok(())
    }
}