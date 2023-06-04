/// A collection of database objects and users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<cfn_resources::StrVal>,

    ///
    /// The username of the administrator for the primary database created in the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<cfn_resources::StrVal>,

    ///
    /// The name of the primary database created in the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the snapshot to be created before the namespace is deleted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_name: Option<cfn_resources::StrVal>,

    ///
    /// How long to retain the final snapshot.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The types of logs the namespace can export.   Available export types are userlog, connectionlog, and useractivitylog.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub namespace_name: cfn_resources::StrVal,

    ///
    /// The map of the key-value pairs used to tag the namespace.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_namespace_admin_username: CfnNamespacenamespaceadminusername,

    #[serde(skip_serializing)]
    pub att_namespace_creation_date: CfnNamespacenamespacecreationdate,

    #[serde(skip_serializing)]
    pub att_namespace_db_name: CfnNamespacenamespacedbname,

    #[serde(skip_serializing)]
    pub att_namespace_default_iam_role_arn: CfnNamespacenamespacedefaultiamrolearn,

    #[serde(skip_serializing)]
    pub att_namespace_kms_key_id: CfnNamespacenamespacekmskeyid,

    #[serde(skip_serializing)]
    pub att_namespace_namespace_arn: CfnNamespacenamespacenamespacearn,

    #[serde(skip_serializing)]
    pub att_namespace_namespace_id: CfnNamespacenamespacenamespaceid,

    #[serde(skip_serializing)]
    pub att_namespace_namespace_name: CfnNamespacenamespacenamespacename,

    #[serde(skip_serializing)]
    pub att_namespace_status: CfnNamespacenamespacestatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespaceadminusername;
impl CfnNamespacenamespaceadminusername {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.AdminUsername"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacecreationdate;
impl CfnNamespacenamespacecreationdate {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.CreationDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacedbname;
impl CfnNamespacenamespacedbname {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.DbName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacedefaultiamrolearn;
impl CfnNamespacenamespacedefaultiamrolearn {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.DefaultIamRoleArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacekmskeyid;
impl CfnNamespacenamespacekmskeyid {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.KmsKeyId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacenamespacearn;
impl CfnNamespacenamespacenamespacearn {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.NamespaceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacenamespaceid;
impl CfnNamespacenamespacenamespaceid {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.NamespaceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacenamespacename;
impl CfnNamespacenamespacenamespacename {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.NamespaceName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamespacenamespacestatus;
impl CfnNamespacenamespacestatus {
    pub fn att_name(&self) -> &'static str {
        r#"Namespace.Status"#
    }
}

impl cfn_resources::CfnResource for CfnNamespace {
    fn type_string(&self) -> &'static str {
        "AWS::RedshiftServerless::Namespace"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A collection of database objects and users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<cfn_resources::StrVal>,

    ///
    /// The date of when the namespace was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<cfn_resources::StrVal>,

    ///
    /// The name of the first database created in the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// A list of IAM roles to associate with the namespace.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The types of logs the namespace can export. Available export types are User log, Connection log, and User activity log.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<cfn_resources::StrVal>,

    ///
    /// The unique identifier of a namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the namespace.    Must be between 3-64 alphanumeric characters in lowercase,    and it cannot be a reserved word. A list of reserved words can be found    in Reserved Words in the Amazon Redshift Database Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<cfn_resources::StrVal>,

    ///
    /// The status of the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Namespace {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
