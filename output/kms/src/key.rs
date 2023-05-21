

/// The AWS::KMS::Key resource specifies an KMS key in AWS Key Management Service. You can use this resource to create symmetric encryption KMS keys,    asymmetric KMS keys for encryption or signing, and symmetric HMAC KMS keys. You can use     AWS::KMS::Key to create multi-Region     primary keys of all supported types. To replicate a multi-Region key, use the     AWS::KMS::ReplicaKey resource.
///
/// You can use symmetric encryption KMS keys to encrypt and decrypt small amounts of data,    but they are more commonly used to generate data keys and data key pairs. You can also use a    symmetric encryption KMS key to encrypt data stored in AWS services that are integrated with AWS KMS. For more information, see Symmetric encryption KMS keys in the      AWS Key Management Service Developer Guide.
///
/// You can use asymmetric KMS keys to encrypt and decrypt data or sign messages and verify    signatures. To create an asymmetric key, you must specify an asymmetric KeySpec    value and a KeyUsage value. For details, see Asymmetric keys in AWS KMS in the    AWS Key Management Service Developer Guide.
///
/// You can use HMAC KMS keys (which are also symmetric keys) to generate and verify    hash-based message authentication codes. To create an HMAC key, you must specify an HMAC     KeySpec value and a KeyUsage value of     GENERATE_VERIFY_MAC. For details, see HMAC keys in AWS KMS in the      AWS Key Management Service Developer Guide.
///
/// You can also create symmetric encryption, asymmetric, and HMAC multi-Region primary keys.    To create a multi-Region primary key, set the MultiRegion property to     true. For information about multi-Region keys, see Multi-Region keys in AWS KMS in the AWS Key Management Service Developer    Guide.
///
/// You cannot use the AWS::KMS::Key resource to specify a KMS key with imported key     material or a KMS key in a custom key    store.
///
/// Regions
///
/// AWS KMS CloudFormation resources are available in all Regions in which AWS KMS and AWS CloudFormation are supported.    You can use the AWS::KMS::Key resource to create and manage all KMS key types that are supported in a Region.
#[derive(Default, serde::Serialize)]
pub struct CfnKey {


    /// 
    /// Assigns one or more tags to the replica key.
    /// 
    /// NoteTagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see      ABAC for       AWS KMS in the AWS Key Management Service Developer      Guide.
    /// 
    /// For information about tags in AWS KMS, see Tagging keys in the      AWS Key Management Service Developer Guide. For information about tags    in CloudFormation, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Determines the cryptographic     operations for which you can use the KMS key. The default value is     ENCRYPT_DECRYPT. This property is required for asymmetric KMS keys and HMAC KMS keys. You can't    change the KeyUsage value after the KMS key is created.
    /// 
    /// ImportantIf you change the value of the KeyUsage property on an existing KMS key,     the update request fails, regardless of the value of the UpdateReplacePolicy attribute. This prevents you from accidentally     deleting a KMS key by changing an immutable property value.
    /// 
    /// Select only one valid value.
    /// 
    /// For symmetric encryption KMS keys, omit the property or specify ENCRYPT_DECRYPT.        For asymmetric KMS keys with RSA key material, specify ENCRYPT_DECRYPT or       SIGN_VERIFY.        For asymmetric KMS keys with ECC key material, specify SIGN_VERIFY.        For asymmetric KMS keys with SM2 (China Regions only) key material, specify ENCRYPT_DECRYPT or      SIGN_VERIFY.        For HMAC KMS keys, specify GENERATE_VERIFY_MAC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ENCRYPT_DECRYPT | GENERATE_VERIFY_MAC | SIGN_VERIFY
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<String>,


    /// 
    /// Specifies the type of KMS key to create. The default value, SYMMETRIC_DEFAULT,    creates a KMS key with a 256-bit symmetric key for encryption and decryption. In China Regions, SYMMETRIC_DEFAULT    creates a 128-bit symmetric key that uses SM4 encryption. You can't change the    KeySpec value after the KMS key is created. For help choosing a    key spec for your KMS key, see Choosing a KMS key type in the AWS Key Management Service Developer     Guide.
    /// 
    /// The KeySpec property determines    the type of key material in the KMS key and the algorithms that the KMS key supports. To further restrict the algorithms that can    be used with the KMS key, use a condition key in its key policy or IAM policy. For more    information, see AWS KMS condition keys in the AWS Key Management Service Developer     Guide.
    /// 
    /// ImportantIf you change the value of the KeySpec property on an existing KMS key, the update request fails, regardless of the value of the     UpdateReplacePolicy attribute.     This prevents you from accidentally deleting a KMS key by changing an immutable property value.
    /// 
    /// NoteAWS     services that are integrated with AWS KMS use symmetric encryption KMS keys to     protect your data. These services do not support encryption with asymmetric KMS keys. For help determining     whether a KMS key is asymmetric, see Identifying asymmetric      KMS keys in the AWS Key Management Service Developer Guide.
    /// 
    /// AWS KMS supports the following key specs for KMS keys:
    /// 
    /// Symmetric encryption key (default)                                                  SYMMETRIC_DEFAULT (AES-256-GCM)                        HMAC keys (symmetric)                                                                                HMAC_224                                            HMAC_256                                            HMAC_384                                            HMAC_512                                   Asymmetric RSA key pairs                                                                      RSA_2048                                            RSA_3072                                            RSA_4096                                   Asymmetric NIST-recommended elliptic curve key pairs                                                                      ECC_NIST_P256 (secp256r1)                                 ECC_NIST_P384 (secp384r1)                                 ECC_NIST_P521 (secp521r1)                        Other asymmetric elliptic curve key pairs                                                  ECC_SECG_P256K1 (secp256k1), commonly used for        cryptocurrencies.                SM2 key pairs (China Regions only)                       SM2
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECC_NIST_P256 | ECC_NIST_P384 | ECC_NIST_P521 | ECC_SECG_P256K1 | HMAC_224 | HMAC_256 | HMAC_384 | HMAC_512 | RSA_2048 | RSA_3072 | RSA_4096 | SM2 | SYMMETRIC_DEFAULT
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeySpec")]
    pub key_spec: Option<String>,


    /// 
    /// A description of the KMS key. Use a description that helps you to distinguish this KMS key from    others in the account, such as its intended use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Creates a multi-Region primary key that you can replicate in other AWS Regions. You can't change the    MultiRegion value after the KMS key is created.
    /// 
    /// For a list of AWS Regions in which multi-Region keys are supported, see Multi-Region keys in AWS KMS in the AWS Key Management Service Developer Guide.
    /// 
    /// ImportantIf you change the value of the MultiRegion property on an existing KMS key,     the update request fails, regardless of the value of the UpdateReplacePolicy attribute. This prevents you from accidentally     deleting a KMS key by changing an immutable property value.
    /// 
    /// For a multi-Region key, set to this property to true. For a single-Region    key, omit this property or set it to false. The default value is    false.
    /// 
    /// Multi-Region keys are an AWS KMS feature that lets you    create multiple interoperable KMS keys in different AWS Regions. Because these    KMS keys have the same key ID, key material, and other metadata, you can use them to encrypt data    in one AWS Region and decrypt it in a different AWS Region    without making a cross-Region call or exposing the plaintext data. For more information, see    Multi-Region keys in the AWS Key Management Service Developer     Guide.
    /// 
    /// You can create a symmetric encryption, HMAC, or asymmetric multi-Region KMS key, and you can    create a multi-Region key with imported key material. However, you cannot create a    multi-Region key in a custom key store.
    /// 
    /// To create a replica of this primary key in a different AWS Region ,    create an AWS::KMS::ReplicaKey resource in a CloudFormation stack in the replica Region.    Specify the key ARN of this primary key.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "MultiRegion")]
    pub multi_region: Option<bool>,


    /// 
    /// The key policy that authorizes use of the KMS key. The key policy must conform to the    following rules.
    /// 
    /// The key policy must allow the caller to make a subsequent PutKeyPolicy request on the      KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, refer      to the scenario in the Default key policy section of the       AWS Key Management Service Developer Guide      .        Each statement in the key policy must contain one or more principals. The principals      in the key policy must exist and be visible to AWS KMS. When you create a new       AWS principal (for example, an IAM user or role), you might need to      enforce a delay before including the new principal in a key policy because the new      principal might not be immediately visible to AWS KMS. For more information,      see Changes that I make are not always immediately visible in the AWS Identity and Access Management User Guide.
    /// 
    /// If you are unsure of which policy to use, consider the default key    policy. This is the key policy that AWS KMS applies to KMS keys that are    created by using the CreateKey API with no specified key policy. It gives the AWS account that owns the key permission to perform all operations on the key. It    also allows you write IAM policies to authorize access to the key. For details, see Default key policy in the AWS Key Management Service Developer     Guide.
    /// 
    /// A key policy document can include only the following characters:
    /// 
    /// Printable ASCII characters        Printable characters in the Basic Latin and Latin-1 Supplement character set        The tab (\u0009), line feed (\u000A), and carriage return (\u000D) special characters
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 32768
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPolicy")]
    pub key_policy: serde_json::Value,


    /// 
    /// Specifies whether the KMS key is enabled. Disabled KMS keys cannot be used in cryptographic    operations.
    /// 
    /// When Enabled is true, the key state of the    KMS key is Enabled. When Enabled is false, the key state of    the KMS key is Disabled. The default value is true.
    /// 
    /// The actual key state of the KMS key might be affected by actions taken outside of    CloudFormation, such as running the EnableKey, DisableKey,    or ScheduleKeyDeletion operations.
    /// 
    /// For information about the key states of a KMS key, see Key state: Effect on your KMS key in the      AWS Key Management Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Specifies the number of days in the waiting period before AWS KMS deletes a    KMS key that has been removed from a CloudFormation stack. Enter a value between 7 and 30 days.    The default value is 30 days.
    /// 
    /// When you remove a KMS key from a CloudFormation stack, AWS KMS schedules the    KMS key for deletion and starts the mandatory waiting period. The PendingWindowInDays    property determines the length of waiting period. During the waiting period, the key state of    KMS key is Pending Deletion or Pending Replica Deletion, which prevents    the KMS key from being used in cryptographic operations. When the waiting period expires, AWS KMS permanently deletes the KMS key.
    /// 
    /// AWS KMS will not delete a multi-Region primary     key that has replica keys. If you remove a multi-Region primary key from a    CloudFormation stack, its key state changes to PendingReplicaDeletion so it    cannot be replicated or used in cryptographic operations. This state can persist indefinitely.    When the last of its replica keys is deleted, the key state of the primary key changes to     PendingDeletion and the waiting period specified by     PendingWindowInDays begins. When this waiting period expires, AWS KMS deletes the primary key. For details, see Deleting multi-Region     keys in the AWS Key Management Service Developer Guide.
    /// 
    /// You cannot use a CloudFormation template to cancel deletion of the KMS key after you remove it    from the stack, regardless of the waiting period. If you specify a KMS key in your template, even    one with the same name, CloudFormation creates a new KMS key. To cancel deletion of a KMS key, use the     AWS KMS console or the CancelKeyDeletion    operation.
    /// 
    /// For information about the Pending Deletion and Pending Replica     Deletion key states, see Key state: Effect on your KMS key in the      AWS Key Management Service Developer Guide. For more information about    deleting KMS keys, see the ScheduleKeyDeletion    operation in the AWS Key Management Service API Reference and Deleting KMS     keys in the AWS Key Management Service Developer Guide.
    /// 
    /// Minimum: 7
    /// 
    /// Maximum: 30
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: Option<i64>,


    /// 
    /// Enables automatic rotation of the key material for the specified KMS key. By default,    automatic key rotation is not enabled.
    /// 
    /// AWS KMS supports automatic rotation only for symmetric encryption KMS keys (KeySpec = SYMMETRIC_DEFAULT).    For asymmetric KMS keys and HMAC KMS keys, omit the EnableKeyRotation property or set it to     false.
    /// 
    /// To enable automatic key rotation of the key material for a multi-Region KMS key, set EnableKeyRotation to true on the primary key (created by using AWS::KMS::Key).      AWS KMS copies the rotation status to all replica keys. For details, see Rotating multi-Region keys in the AWS Key Management Service Developer Guide.
    /// 
    /// When you enable automatic rotation, AWS KMS automatically creates new key    material for the KMS key one year after the enable date and every year    thereafter. AWS KMS retains all key material until you delete the KMS key. For    detailed information about automatic key rotation, see Rotating KMS keys in the      AWS Key Management Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableKeyRotation")]
    pub enable_key_rotation: Option<bool>,

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