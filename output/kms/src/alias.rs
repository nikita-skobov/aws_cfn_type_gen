/// The AWS::KMS::Alias resource specifies a display name for a KMS key.    You can use an alias to identify a KMS key in the AWS KMS console, in the DescribeKey    operation, and in cryptographic     operations, such as Decrypt and GenerateDataKey.
///
/// Using an alias to refer to a KMS key can help you simplify key management. For example, an    alias in your code can be associated with different KMS keys in different AWS Regions. For more information, see Using aliases in the      AWS Key Management Service Developer Guide.
///
/// When specifying an alias, observe the following rules.
///
/// Regions
///
/// AWS KMS CloudFormation resources are available in all AWS Regions in which AWS KMS and     AWS CloudFormation are supported.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAlias {
    ///
    /// Specifies the alias name. This value must begin with alias/ followed by a    name, such as alias/ExampleAlias.
    ///
    /// NoteIf you change the value of the AliasName property, the existing alias is     deleted and a new alias is created for the specified KMS key. This change can disrupt     applications that use the alias. It can also allow or deny access to a KMS key affected by     attribute-based access control (ABAC).
    ///
    /// The alias must be string of 1-256 characters. It can contain only alphanumeric characters,    forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with     alias/aws/. The alias/aws/ prefix is reserved for AWS managed keys.
    ///
    /// Pattern: ^alias/[a-zA-Z0-9/_-]+$
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AliasName")]
    pub alias_name: String,

    ///
    /// Associates the alias with the specified customer managed key. The    KMS key must be in the same AWS account and Region.
    ///
    /// A valid key ID is required. If you supply a null or empty string value, this operation    returns an error.
    ///
    /// For help finding the key ID and ARN, see Finding the key ID and     ARN in the AWS Key Management Service Developer Guide.
    ///
    /// Specify the key ID or the key ARN of the KMS key.
    ///
    /// For example:
    ///
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab             Key ARN:       arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab
    ///
    /// To get the key ID and key ARN for a KMS key, use ListKeys or DescribeKey.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

impl cfn_resources::CfnResource for CfnAlias {
    fn type_string(&self) -> &'static str {
        "AWS::KMS::Alias"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.alias_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'alias_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.alias_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'alias_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.target_key_id;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'target_key_id'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.target_key_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'target_key_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
