

/// Specifies a key pair for use with an Amazon Elastic Compute Cloud instance as follows:
///
/// When you import an existing key pair, you specify the public key material for the key. We      assume that you have the private key material for the key. AWS CloudFormation does not      create or return the private key material when you import a key pair.
///
/// When you create a new key pair, the private key is saved to AWS Systems Manager      Parameter Store, using a parameter with the following name: /ec2/keypair/{key_pair_id}.      For more information about retrieving private key, and the required permissions, see Create a key pair using AWS CloudFormation in the Amazon EC2 User Guide.
///
/// When AWS CloudFormation deletes a key pair that was created or imported by a stack,      it also deletes the parameter that was used to store the private key material in     Parameter Store.
#[derive(Default, serde::Serialize)]
pub struct CfnKeyPair {


    /// 
    /// The public key material. The PublicKeyMaterial property is used to import a key pair. If this property is not specified,      then a new key pair will be created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKeyMaterial")]
    pub public_key_material: Option<String>,


    /// 
    /// The tags to apply to the key pair.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A unique name for the key pair.
    /// 
    /// Constraints: Up to 255 ASCII characters
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyName")]
    pub key_name: String,


    /// 
    /// The type of key pair. Note that ED25519 keys are not supported for Windows instances.
    /// 
    /// If the PublicKeyMaterial property is specified, the KeyType property is ignored, and the key type      is inferred from the PublicKeyMaterial value.
    /// 
    /// Default: rsa
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ed25519 | rsa
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}