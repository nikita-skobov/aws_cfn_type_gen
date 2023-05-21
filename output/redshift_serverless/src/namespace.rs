

/// A collection of database objects and users.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNamespace {


    /// 
    /// The password of the administrator for the primary database created in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUserPassword")]
    pub admin_user_password: Option<String>,


    /// 
    /// The username of the administrator for the primary database created in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUsername")]
    pub admin_username: Option<String>,


    /// 
    /// The name of the primary database created in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbName")]
    pub db_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultIamRoleArn")]
    pub default_iam_role_arn: Option<String>,


    /// 
    /// The name of the snapshot to be created before the namespace is deleted.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalSnapshotName")]
    pub final_snapshot_name: Option<String>,


    /// 
    /// How long to retain the final snapshot.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalSnapshotRetentionPeriod")]
    pub final_snapshot_retention_period: Option<i64>,


    /// 
    /// A list of IAM roles to associate with the namespace.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoles")]
    pub iam_roles: Option<Vec<String>>,


    /// 
    /// The ID of the AWS Key Management Service key used to encrypt your data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The types of logs the namespace can export.   Available export types are userlog, connectionlog, and useractivitylog.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogExports")]
    pub log_exports: Option<Vec<String>>,


    /// 
    /// The name of the namespace.   Must be between 3-64 alphanumeric characters in lowercase,   and it cannot be a reserved word. A list of reserved words can be found   in Reserved Words in the Amazon Redshift Database Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,


    /// 
    /// The map of the key-value pairs used to tag the namespace.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnNamespace {
    fn type_string() -> &'static str {
        "AWS::RedshiftServerless::Namespace"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A collection of database objects and users.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Namespace {


    /// 
    /// The username of the administrator for the first database created in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUsername")]
    pub admin_username: Option<String>,


    /// 
    /// The date of when the namespace was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,


    /// 
    /// The name of the first database created in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbName")]
    pub db_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultIamRoleArn")]
    pub default_iam_role_arn: Option<String>,


    /// 
    /// A list of IAM roles to associate with the namespace.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoles")]
    pub iam_roles: Option<Vec<String>>,


    /// 
    /// The ID of the AWS Key Management Service key used to encrypt your data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The types of logs the namespace can export. Available export types are User log, Connection log, and User activity log.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogExports")]
    pub log_exports: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Name (ARN) associated with a namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceArn")]
    pub namespace_arn: Option<String>,


    /// 
    /// The unique identifier of a namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceId")]
    pub namespace_id: Option<String>,


    /// 
    /// The name of the namespace.    Must be between 3-64 alphanumeric characters in lowercase,    and it cannot be a reserved word. A list of reserved words can be found    in Reserved Words in the Amazon Redshift Database Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,


    /// 
    /// The status of the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

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


