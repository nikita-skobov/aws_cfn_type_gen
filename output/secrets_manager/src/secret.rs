/// Creates a new secret. A secret can be a password, a set of    credentials such as a user name and password, an OAuth token, or other secret information that    you store in an encrypted form in Secrets Manager.
///
/// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
///
/// To retrieve a secret in a CloudFormation template, use a dynamic    reference. For more information, see     Retrieve a secret in an AWS CloudFormation resource.
///
/// A common scenario is to first create a secret with GenerateSecretString, which    generates a password, and then use a dynamic reference to retrieve the username and password from    the secret to use as credentials for a new database. Follow these steps, as shown in the examples below:
///
/// For information about creating a secret in the console, see Create a    secret. For information about creating a secret using the CLI or SDK, see CreateSecret.
///
/// For information about retrieving a secret in code, see Retrieve     secrets from Secrets Manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecret {
    ///
    /// The description of the secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// A structure that specifies how to generate a password to encrypt and store in the secret. To include a specific string    in the secret, use SecretString instead. If you omit both GenerateSecretString and SecretString, you create an empty secret. When you make a change to this property, a new secret version is created.
    ///
    /// We recommend that you specify the maximum length and include every character type that the    system you are generating a password for can support.
    ///
    /// Required: No
    ///
    /// Type: GenerateSecretString
    ///
    /// Update requires: No interruption
    #[serde(rename = "GenerateSecretString")]
    pub generate_secret_string: Option<GenerateSecretString>,

    ///
    /// The ARN, key ID, or alias of the AWS KMS key that Secrets Manager uses to    encrypt the secret value in the secret. An alias is always prefixed by alias/, for example alias/aws/secretsmanager.   For more information, see About aliases.
    ///
    /// To use a AWS KMS key in a different account, use the key ARN or the alias ARN.
    ///
    /// If you don't specify this value, then Secrets Manager uses the key aws/secretsmanager.    If that key doesn't yet exist, then Secrets Manager creates it for you automatically the first time it    encrypts the secret value.
    ///
    /// If the secret is in a different AWS account from the credentials calling the API, then    you can't use aws/secretsmanager to encrypt the secret, and you must create    and use a customer managed AWS KMS key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// The name of the new secret.
    ///
    /// The secret name can contain ASCII letters, numbers, and the following characters:    /_+=.@-
    ///
    /// Do not end your secret name with a hyphen followed by six characters. If you do so, you     risk confusion and unexpected results when searching for a secret by partial ARN. Secrets Manager     automatically adds a hyphen and six random characters after the secret name at the end of the ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// A custom type that specifies a Region and the KmsKeyId for a replica secret.
    ///
    /// Required: No
    ///
    /// Type: List of ReplicaRegion
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicaRegions")]
    pub replica_regions: Option<Vec<ReplicaRegion>>,

    ///
    /// The text to encrypt and store in the secret. We recommend you use a JSON structure of    key/value pairs for your secret value. To generate a random password, use GenerateSecretString instead.    If you omit both GenerateSecretString and SecretString, you create an empty secret. When you make a change to this property, a new secret version is created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretString")]
    pub secret_string: Option<String>,

    ///
    /// A list of tags to attach to the secret. Each tag is a key and value pair of strings in a    JSON text string, for example:
    ///
    /// [{"Key":"CostCenter","Value":"12345"},{"Key":"environment","Value":"production"}]
    ///
    /// Secrets Manager tag key names are case sensitive. A tag with the key "ABC" is a different    tag from one with key "abc".
    ///
    /// If you check tags in permissions policies as part of your security strategy, then adding    or removing a tag can change permissions. If the completion of this operation would result in    you losing your permissions for this secret, then Secrets Manager blocks the operation and    returns an Access Denied error. For more information, see Control     access to secrets using tags and Limit access to identities with tags that match secrets' tags.
    ///
    /// For information about how to format a JSON parameter for the various command line tool    environments, see Using JSON for     Parameters. If your command-line tool or SDK requires quotation marks around the    parameter, you should use single quotes to avoid confusion with the double quotes required in    the JSON text.
    ///
    /// The following restrictions apply to tags:
    ///
    /// Maximum number of tags per secret: 50        Maximum key length: 127 Unicode characters in UTF-8        Maximum value length: 255 Unicode characters in UTF-8        Tag keys and values are case sensitive.        Do not use the aws: prefix in your tag names or values because AWS reserves it for AWS use. You can't edit or delete tag names or      values with this prefix. Tags with this prefix do not count against your tags per secret      limit.        If you use your tagging schema across multiple services and resources, other services      might have restrictions on allowed characters. Generally allowed characters: letters,      spaces, and numbers representable in UTF-8, plus the following special characters: + - = .      _ : / @.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnSecret {
    fn type_string(&self) -> &'static str {
        "AWS::SecretsManager::Secret"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.generate_secret_string
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Generates a random password. We recommend that you specify the maximum length and include    every character type that the system you are generating a password for can support.
///
/// Required permissions:    secretsmanager:GetRandomPassword. For more information, see IAM policy actions for Secrets Manager and Authentication and access control     in Secrets Manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GenerateSecretString {
    ///
    /// A string of the characters that you don't want in the password.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeCharacters")]
    pub exclude_characters: Option<String>,

    ///
    /// Specifies whether to exclude lowercase letters from the password. If    you don't include this switch, the password can contain lowercase letters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeLowercase")]
    pub exclude_lowercase: Option<bool>,

    ///
    /// Specifies whether to exclude numbers from the password. If you don't    include this switch, the password can contain numbers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeNumbers")]
    pub exclude_numbers: Option<bool>,

    ///
    /// Specifies whether to exclude the following punctuation characters from the password:    ! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~.    If you don't include this switch, the password can contain punctuation.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludePunctuation")]
    pub exclude_punctuation: Option<bool>,

    ///
    /// Specifies whether to exclude uppercase letters from the password. If you    don't include this switch, the password can contain uppercase letters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeUppercase")]
    pub exclude_uppercase: Option<bool>,

    ///
    /// The JSON key name for the key/value pair, where the value is the generated password. This    pair is added to the JSON structure specified by the SecretStringTemplate    parameter. If you specify this parameter, then you must also specify     SecretStringTemplate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GenerateStringKey")]
    pub generate_string_key: Option<String>,

    ///
    /// Specifies whether to include the space character. If you    include this switch, the password can contain space characters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSpace")]
    pub include_space: Option<bool>,

    ///
    /// The length of the password. If you don't include this parameter, the    default length is 32 characters.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordLength")]
    pub password_length: Option<i64>,

    ///
    /// Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation.    If you don't include this switch, the password contains at least one of every character type.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireEachIncludedType")]
    pub require_each_included_type: Option<bool>,

    ///
    /// A template that the generated string must match. When you make a change to this property, a new secret version is created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretStringTemplate")]
    pub secret_string_template: Option<String>,
}

impl cfn_resources::CfnResource for GenerateSecretString {
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

/// Specifies a Region and the KmsKeyId for a replica secret.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicaRegion {
    ///
    /// The ARN, key ID, or alias of the KMS key to encrypt the secret. If you don't include this field, Secrets Manager uses aws/secretsmanager.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// (Optional) A string that represents a Region, for example "us-east-1".
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,
}

impl cfn_resources::CfnResource for ReplicaRegion {
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
