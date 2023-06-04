/// The AWS::Lightsail::Bucket resource specifies a bucket.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnBucket {
    ///
    /// An object that describes the access rules for the bucket.
    ///
    /// Required: No
    ///
    /// Type: AccessRules
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessRules")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub access_rules: Option<AccessRules>,

    ///
    /// The name of the bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: cfn_resources::StrVal,

    ///
    /// The bundle ID for the bucket (for example, small_1_0).
    ///
    /// A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a     bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BundleId")]
    pub bundle_id: cfn_resources::StrVal,

    ///
    /// Indicates whether object versioning is enabled for the bucket.
    ///
    /// The following options can be configured:
    ///
    /// Enabled - Object versioning is enabled.                    Suspended - Object versioning was previously enabled but is currently        suspended. Existing object versions are retained.                    NeverEnabled - Object versioning has never been enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectVersioning")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub object_versioning: Option<bool>,

    ///
    /// An array of AWS account IDs that have read-only access to the     bucket.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnlyAccessAccounts")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub read_only_access_accounts: Option<Vec<String>>,

    ///
    /// An array of Lightsail instances that have access to the bucket.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcesReceivingAccess")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resources_receiving_access: Option<Vec<String>>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    ///
    /// NoteThe Value of Tags is optional for Lightsail resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_bucket_arn: CfnBucketbucketarn,

    #[serde(skip_serializing)]
    pub att_url: CfnBucketurl,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBucketbucketarn;
impl CfnBucketbucketarn {
    pub fn att_name(&self) -> &'static str {
        r#"BucketArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBucketurl;
impl CfnBucketurl {
    pub fn att_name(&self) -> &'static str {
        r#"Url"#
    }
}

impl cfn_resources::CfnResource for CfnBucket {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::Bucket"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_rules
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.read_only_access_accounts {
            if the_val.len() > 10 as _ {
                return Err(format!("Max validation failed on field 'read_only_access_accounts'. {} is greater than 10", the_val.len()));
            }
        }

        Ok(())
    }
}

/// AccessRules is a property of the AWS::Lightsail::Bucket resource. It describes access rules for a bucket.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessRules {
    ///
    /// A Boolean value indicating whether the access control list (ACL) permissions that are     applied to individual objects override the GetObject option that is currently     specified.
    ///
    /// When this is true, you can use the PutObjectAcl Amazon S3 API     operation to set individual objects to public (read-only) or private, using either the public-read     ACL or the private ACL.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowPublicOverrides")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub allow_public_overrides: Option<bool>,

    ///
    /// Specifies the anonymous access to all objects in a bucket.
    ///
    /// The following options can be specified:
    ///
    /// public - Sets all objects in the bucket to public (read-only), making        them readable by everyone on the internet.        If the GetObject value is set to public, then all        objects in the bucket default to public regardless of the          allowPublicOverrides value.                    private - Sets all objects in the bucket to private, making them        readable only by you and anyone that you grant access to.        If the GetObject value is set to private, and the          allowPublicOverrides value is set to true, then all        objects in the bucket default to private unless they are configured with a          public-read ACL. Individual objects with a public-read        ACL are readable by everyone on the internet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: private | public
    ///
    /// Update requires: No interruption
    #[serde(rename = "GetObject")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub get_object: Option<AccessRulesGetObjectEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AccessRulesGetObjectEnum {
    /// private
    #[serde(rename = "private")]
    Private,

    /// public
    #[serde(rename = "public")]
    Public,
}

impl Default for AccessRulesGetObjectEnum {
    fn default() -> Self {
        AccessRulesGetObjectEnum::Private
    }
}

impl cfn_resources::CfnResource for AccessRules {
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
#[serde(default)]
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
