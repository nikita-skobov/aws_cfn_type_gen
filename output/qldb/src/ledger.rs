

/// The AWS::QLDB::Ledger resource specifies a new Amazon Quantum Ledger Database     (Amazon QLDB) ledger in your AWS account. Amazon QLDB is a fully managed ledger database     that provides a transparent, immutable, and cryptographically verifiable transaction log     owned by a central trusted authority. You can use QLDB to track all application data     changes, and maintain a complete and verifiable history of changes over time.
///
/// For more information, see CreateLedger in the       Amazon QLDB API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLedger {


    /// 
    /// Specifies whether the ledger is protected from being deleted by any user. If not defined during    ledger creation, this feature is enabled (true) by default.
    /// 
    /// If deletion protection is enabled, you must first disable it before you can delete the    ledger. You can disable it by calling the UpdateLedger operation to set this parameter to false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,


    /// 
    /// The key in AWS Key Management Service (AWS KMS) to use for encryption of data at rest in the ledger. For     more information, see Encryption at rest in     the Amazon QLDB Developer Guide.
    /// 
    /// Use one of the following options to specify this parameter:
    /// 
    /// AWS_OWNED_KMS_KEY: Use an AWS KMS key that is owned and managed by AWS        on your behalf.                        Undefined: By default, use an AWS owned KMS        key.                        A valid symmetric customer managed KMS key: Use        the specified symmetric encryption KMS key in your account that you create, own, and        manage.        Amazon QLDB does not support asymmetric keys. For more information, see Using symmetric and asymmetric keys in the            AWS Key Management Service Developer          Guide.
    /// 
    /// To specify a customer managed KMS key, you can use its key ID, Amazon Resource Name     (ARN), alias name, or alias ARN. When using an alias name, prefix it with       "alias/". To specify a key in a different AWS account, you must use the key     ARN or alias ARN.
    /// 
    /// For example:
    /// 
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab                       Key ARN:          arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab                       Alias name: alias/ExampleAlias                       Alias ARN:        arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias
    /// 
    /// For more information, see Key identifiers (KeyId) in     the         AWS Key Management Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1600
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,


    /// 
    /// The name of the ledger that you want to create. The name must be unique among all of the     ledgers in your AWS account in the current Region.
    /// 
    /// Naming constraints for ledger names are defined in Quotas in Amazon QLDB     in the Amazon QLDB Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: (?!^.*--)(?!^[0-9]+$)(?!^-)(?!.*-$)^[A-Za-z0-9-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The permissions mode to assign to the ledger that you want to create. This parameter can     have one of the following values:
    /// 
    /// ALLOW_ALL: A legacy permissions mode that enables access control with        API-level granularity for ledgers.        This mode allows users who have the SendCommand API permission for        this ledger to run all PartiQL commands (hence, ALLOW_ALL) on any tables        in the specified ledger. This mode disregards any table-level or command-level IAM        permissions policies that you create for the ledger.                        STANDARD: (Recommended) A permissions mode that        enables access control with finer granularity for ledgers, tables, and PartiQL        commands.        By default, this mode denies all user requests to run any PartiQL commands on any        tables in this ledger. To allow PartiQL commands to run, you must create IAM        permissions policies for specific table resources and PartiQL actions, in addition to        the SendCommand API permission for the ledger. For information, see          Getting          started with the standard permissions mode in the Amazon QLDB          Developer Guide.
    /// 
    /// NoteWe strongly recommend using the STANDARD permissions mode to maximize       the security of your ledger data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOW_ALL | STANDARD
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsMode")]
    pub permissions_mode: LedgerPermissionsModeEnum,


    /// 
    /// An array of key-value pairs to apply to this resource.
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


#[derive(Clone, Debug, serde::Serialize)]
pub enum LedgerPermissionsModeEnum {

    /// ALLOW_ALL
    #[serde(rename = "ALLOW_ALL")]
    Allowall,

    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

}

impl Default for LedgerPermissionsModeEnum {
    fn default() -> Self {
        LedgerPermissionsModeEnum::Allowall
    }
}


impl cfn_resources::CfnResource for CfnLedger {
    fn type_string() -> &'static str {
        "AWS::QLDB::Ledger"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.kms_key {

        if the_val.len() > 1600 as _ {
            return Err(format!("Max validation failed on field 'kms_key'. {} is greater than 1600", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
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