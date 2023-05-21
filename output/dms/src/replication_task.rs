

/// The AWS::DMS::ReplicationTask resource creates an AWS DMS replication task.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnReplicationTask {


    /// 
    /// Indicates when you want a change data capture (CDC) operation to start. Use either     CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start.     Specifying both values results in an error.
    /// 
    /// The value can be in date, checkpoint, log sequence number (LSN), or system change       number (SCN) format.
    /// 
    /// Here is a date example: --cdc-start-position "2018-03-08T12:12:12"
    /// 
    /// Here is a checkpoint example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"
    /// 
    /// Here is an LSN example: --cdc-start-position “mysql-bin-changelog.000024:373”
    /// 
    /// NoteWhen you use this task setting with a source PostgreSQL database, a logical         replication slot should already be created and associated with the source endpoint.         You can verify this by setting the slotName extra connection attribute to the         name of this logical replication slot. For more information, see                   Extra Connection Attributes When Using PostgreSQL as a Source for AWS DMS         in the AWS Database Migration Service User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcStartPosition")]
    pub cdc_start_position: Option<String>,


    /// 
    /// Indicates the start time for a change data capture (CDC) operation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcStartTime")]
    pub cdc_start_time: Option<f64>,


    /// 
    /// Indicates when you want a change data capture (CDC) operation to stop. The value can be     either server time or commit time.
    /// 
    /// Here is a server time example: --cdc-stop-position         "server_time:2018-02-09T12:12:12"
    /// 
    /// Here is a commit time example: --cdc-stop-position "commit_time:         2018-02-09T12:12:12"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcStopPosition")]
    pub cdc_stop_position: Option<String>,


    /// 
    /// The migration type. Valid values: full-load | cdc | full-load-and-cdc
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: cdc | full-load | full-load-and-cdc
    ///
    /// Update requires: No interruption
    #[serde(rename = "MigrationType")]
    pub migration_type: ReplicationTaskMigrationTypeEnum,


    /// 
    /// The Amazon Resource Name (ARN) of a replication instance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,


    /// 
    /// An identifier for the replication task.
    /// 
    /// Constraints:
    /// 
    /// Must contain 1-255 alphanumeric characters or hyphens.               First character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationTaskIdentifier")]
    pub replication_task_identifier: Option<String>,


    /// 
    /// Overall settings for the task, in JSON format. For more information, see                Specifying Task Settings for AWS Database Migration Service Tasks       in the AWS Database Migration Service User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationTaskSettings")]
    pub replication_task_settings: Option<String>,


    /// 
    /// A display name for the resource identifier at the end of the EndpointArn       response parameter that is returned in the created Endpoint object. The       value for this parameter can have up to 31 characters. It can contain only ASCII       letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two       consecutive hyphens, and can only begin with a letter, such as         Example-App-ARN1.
    /// 
    /// For example, this value might result in the EndpointArn value         arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1. If you don't       specify a ResourceIdentifier value, AWS DMS generates a default       identifier value for the end of EndpointArn.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,


    /// 
    /// An Amazon Resource Name (ARN) that uniquely identifies the source endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceEndpointArn")]
    pub source_endpoint_arn: String,


    /// 
    /// The table mappings for the task, in JSON format. For more information, see                Using Table Mapping to Specify Task Settings       in the AWS Database Migration Service User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableMappings")]
    pub table_mappings: String,


    /// 
    /// One or more tags to be assigned to the replication task.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// An Amazon Resource Name (ARN) that uniquely identifies the target endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetEndpointArn")]
    pub target_endpoint_arn: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskData")]
    pub task_data: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicationTaskMigrationTypeEnum {

    /// cdc
    #[serde(rename = "cdc")]
    Cdc,

    /// full-load
    #[serde(rename = "full-load")]
    Fullload,

    /// full-load-and-cdc
    #[serde(rename = "full-load-and-cdc")]
    Fullloadandcdc,

}

impl Default for ReplicationTaskMigrationTypeEnum {
    fn default() -> Self {
        ReplicationTaskMigrationTypeEnum::Cdc
    }
}


impl cfn_resources::CfnResource for CfnReplicationTask {
    fn type_string() -> &'static str {
        "AWS::DMS::ReplicationTask"
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