

/// Creates an Amazon Kendra index
///
/// Once the index is active you can add documents to your index using       the BatchPutDocument operation or using one of the       supported data sources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIndex {


    /// 
    /// The identifier of the AWS KMS customer managed key (CMK) to use to       encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support       asymmetric CMKs.
    /// 
    /// Required: No
    ///
    /// Type: ServerSideEncryptionConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,


    /// 
    /// The user context policy.
    /// 
    /// ATTRIBUTE_FILTER
    /// 
    /// All indexed content is searchable and displayable for all users. If you want to filter search results on user context, you can use the attribute filters of _user_id and _group_ids or you can provide user and group information in UserContext.
    /// 
    /// USER_TOKEN
    /// 
    /// Enables token-based user access control to filter search results on user context. All documents with no access control and all documents accessible to the user will be searchable and displayable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserContextPolicy")]
    pub user_context_policy: Option<String>,


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


    /// 
    /// A description for the index.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// An IAM role that gives Amazon Kendra permissions to access your       Amazon CloudWatch logs and metrics. This is also the role used when       you use the BatchPutDocument operation to index documents from an Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Indicates whether the index is a Enterprise Edition index or a       Developer Edition index. Valid values are         DEVELOPER_EDITION and         ENTERPRISE_EDITION.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DEVELOPER_EDITION | ENTERPRISE_EDITION
    ///
    /// Update requires: Replacement
    #[serde(rename = "Edition")]
    pub edition: String,


    /// 
    /// Defines the type of user token used for the index.
    /// 
    /// Required: No
    ///
    /// Type: List of UserTokenConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserTokenConfigurations")]
    pub user_token_configurations: Option<Vec<UserTokenConfiguration>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CapacityUnitsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,


    /// 
    /// The name of the index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Specifies the properties of an index field. You can add either a       custom or a built-in field. You can add and remove built-in fields       at any time. When a built-in field is removed it's configuration       reverts to the default for the field. Custom fields can't be removed       from an index after they are added.
    /// 
    /// Required: No
    ///
    /// Type: List of DocumentMetadataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentMetadataConfigurations")]
    pub document_metadata_configurations: Option<Vec<DocumentMetadataConfiguration>>,

}

impl cfn_resources::CfnResource for CfnIndex {
    fn type_string() -> &'static str {
        "AWS::Kendra::Index"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the properties, such as relevance tuning and searchability, of an index       field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentMetadataConfiguration {


    /// 
    /// Provides information about how the field is used during a search.
    /// 
    /// Required: No
    ///
    /// Type: Search
    ///
    /// Update requires: No interruption
    #[serde(rename = "Search")]
    pub search: Option<Search>,


    /// 
    /// The name of the index field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Provides tuning parameters to determine how the field affects the search       results.
    /// 
    /// Required: No
    ///
    /// Type: Relevance
    ///
    /// Update requires: No interruption
    #[serde(rename = "Relevance")]
    pub relevance: Option<Relevance>,


    /// 
    /// The data type of the index field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DATE_VALUE | LONG_VALUE | STRING_LIST_VALUE | STRING_VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// Provides information about how a custom index field is used during a search.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Search {


    /// 
    /// Determines whether the field is returned in the query response. The default is         true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Displayable")]
    pub displayable: Option<bool>,


    /// 
    /// Determines whether the field can be used to sort the results of a query. The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sortable")]
    pub sortable: Option<bool>,


    /// 
    /// Indicates that the field can be used to create search facets, a count of results for       each value in the field. The default is false .
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Facetable")]
    pub facetable: Option<bool>,


    /// 
    /// Determines whether the field is used in the search. If the Searchable       field is true, you can use relevance tuning to manually tune how Amazon Kendra weights the field in the search. The default is true for       string fields and false for number and date fields.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Searchable")]
    pub searchable: Option<bool>,

}


/// Provides the identifier of the AWS KMS customer master key (CMK)       used to encrypt data indexed by Amazon Kendra. We suggest that you       use a CMK from your account to help secure your index. Amazon Kendra       doesn't support asymmetric CMKs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerSideEncryptionConfiguration {


    /// 
    /// The identifier of the AWS KMS key. Amazon Kendra doesn't support       asymmetric keys.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


/// Specifies a key-value pair of the search boost value       for a document when the key is part of the metadata of a       document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValueImportanceItem {


    /// 
    /// The boost value for a document when the key is part of       the metadata of a document.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<i64>,


    /// 
    /// The document metadata value used for the search boost.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


/// Provides information for tuning the relevance of a field in a search. When a query       includes terms that match the field, the results are given a boost in the response based       on these tuning parameters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Relevance {


    /// 
    /// Indicates that this field determines how "fresh" a document is. For example, if       document 1 was created on November 5, and document 2 was created on October 31, document       1 is "fresher" than document 2. You can only set the Freshness field on one         DATE type field. Only applies to DATE fields.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Freshness")]
    pub freshness: Option<bool>,


    /// 
    /// An array of key-value pairs for different boosts when they appear in the search       result list. For example, if you want to boost query terms that match       the "department" field in the result, query terms that match this field are boosted in the result. You can add entries from       the department field to boost documents with those values       higher.
    /// 
    /// For example, you can add entries to the map with names of       departments. If you add "HR", 5 and "Legal",3 those departments are       given special attention when they appear in the metadata of a       document.
    /// 
    /// Required: No
    ///
    /// Type: List of ValueImportanceItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueImportanceItems")]
    pub value_importance_items: Option<Vec<ValueImportanceItem>>,


    /// 
    /// Specifies the time period that the boost applies to. For example, to make the boost       apply to documents with the field value within the last month, you would use "2628000s".       Once the field value is beyond the specified range, the effect of the boost drops off.       The higher the importance, the faster the effect drops off. If you don't specify a       value, the default is 3 months. The value of the field is a numeric string followed by       the character "s", for example "86400s" for one day, or "604800s" for one week.
    /// 
    /// Only applies to DATE fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Pattern: [0-9]+[s]
    ///
    /// Update requires: No interruption
    #[serde(rename = "Duration")]
    pub duration: Option<String>,


    /// 
    /// The relative importance of the field in the search. Larger numbers provide more of a       boost than smaller numbers.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Importance")]
    pub importance: Option<i64>,


    /// 
    /// Determines how values should be interpreted.
    /// 
    /// When the RankOrder field is ASCENDING, higher numbers are       better. For example, a document with a rating score of 10 is higher ranking than a       document with a rating score of 1.
    /// 
    /// When the RankOrder field is DESCENDING, lower numbers are       better. For example, in a task tracking application, a priority 1 task is more important       than a priority 5 task.
    /// 
    /// Only applies to LONG and DOUBLE fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ASCENDING | DESCENDING
    ///
    /// Update requires: No interruption
    #[serde(rename = "RankOrder")]
    pub rank_order: Option<String>,

}


/// Provides the configuration information for the JSON token type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonTokenTypeConfiguration {


    /// 
    /// The group attribute field.
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
    #[serde(rename = "GroupAttributeField")]
    pub group_attribute_field: String,


    /// 
    /// The user name attribute field.
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
    #[serde(rename = "UserNameAttributeField")]
    pub user_name_attribute_field: String,

}


/// Provides the configuration information for the JWT token type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JwtTokenTypeConfiguration {


    /// 
    /// The regular expression that identifies the claim.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClaimRegex")]
    pub claim_regex: Option<String>,


    /// 
    /// The location of the key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SECRET_MANAGER | URL
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyLocation")]
    pub key_location: String,


    /// 
    /// The group attribute field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupAttributeField")]
    pub group_attribute_field: Option<String>,


    /// 
    /// The Amazon Resource Name (arn) of the secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: Option<String>,


    /// 
    /// The user name attribute field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserNameAttributeField")]
    pub user_name_attribute_field: Option<String>,


    /// 
    /// The issuer of the token.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,


    /// 
    /// The signing key URL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^(https?|ftp|file):\/\/([^\s]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "URL")]
    pub url: Option<String>,

}


/// Specifies additional capacity units configured for your Enterprise Edition index. You can    add and remove capacity units to fit your usage requirements.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityUnitsConfiguration {


    /// 
    /// The amount of extra query capacity for an index and GetQuerySuggestions    capacity.
    /// 
    /// A single extra capacity unit for an index provides 0.1 queries per second or approximately    8,000 queries per day. You can add up to 100 extra capacity units.
    /// 
    /// GetQuerySuggestions capacity is five times the provisioned query capacity for    an index, or the base capacity of 2.5 calls per second, whichever is higher. For example, the    base capacity for an index is 0.1 queries per second, and GetQuerySuggestions    capacity has a base of 2.5 calls per second. If you add another 0.1 queries per second to    total 0.2 queries per second for an index, the GetQuerySuggestions capacity is    2.5 calls per second (higher than five times 0.2 queries per second).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryCapacityUnits")]
    pub query_capacity_units: i64,


    /// 
    /// The amount of extra storage capacity for an index. A single capacity unit provides 30 GB    of storage space or 100,000 documents, whichever is reached first. You can add up to 100 extra    capacity units.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageCapacityUnits")]
    pub storage_capacity_units: i64,

}


/// Provides the configuration information for a token.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserTokenConfiguration {


    /// 
    /// Information about the JSON token type configuration.
    /// 
    /// Required: No
    ///
    /// Type: JsonTokenTypeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonTokenTypeConfiguration")]
    pub json_token_type_configuration: Option<JsonTokenTypeConfiguration>,


    /// 
    /// Information about the JWT token type configuration.
    /// 
    /// Required: No
    ///
    /// Type: JwtTokenTypeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "JwtTokenTypeConfiguration")]
    pub jwt_token_type_configuration: Option<JwtTokenTypeConfiguration>,

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
