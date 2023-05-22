/// Creates an Amazon Kendra index
///
/// Once the index is active you can add documents to your index using       the BatchPutDocument operation or using one of the       supported data sources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIndex {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CapacityUnitsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,

    ///
    /// A description for the index.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the properties of an index field. You can add either a       custom or a built-in field. You can add and remove built-in fields       at any time. When a built-in field is removed it's configuration       reverts to the default for the field. Custom fields can't be removed       from an index after they are added.
    ///
    /// Required: No
    ///
    /// Type: List of DocumentMetadataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentMetadataConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata_configurations: Option<Vec<DocumentMetadataConfiguration>>,

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
    pub edition: IndexEditionEnum,

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
    pub name: cfn_resources::StrVal,

    ///
    /// An IAM role that gives Amazon Kendra permissions to access your       Amazon CloudWatch logs and metrics. This is also the role used when       you use the BatchPutDocument operation to index documents from an Amazon S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The identifier of the AWS KMS customer managed key (CMK) to use to       encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support       asymmetric CMKs.
    ///
    /// Required: No
    ///
    /// Type: ServerSideEncryptionConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_policy: Option<cfn_resources::StrVal>,

    ///
    /// Defines the type of user token used for the index.
    ///
    /// Required: No
    ///
    /// Type: List of UserTokenConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserTokenConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_token_configurations: Option<Vec<UserTokenConfiguration>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnIndexarn,

    #[serde(skip_serializing)]
    pub att_id: CfnIndexid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum IndexEditionEnum {
    /// DEVELOPER_EDITION
    #[serde(rename = "DEVELOPER_EDITION")]
    Developeredition,

    /// ENTERPRISE_EDITION
    #[serde(rename = "ENTERPRISE_EDITION")]
    Enterpriseedition,
}

impl Default for IndexEditionEnum {
    fn default() -> Self {
        IndexEditionEnum::Developeredition
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIndexarn;
impl CfnIndexarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIndexid;
impl CfnIndexid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnIndex {
    fn type_string(&self) -> &'static str {
        "AWS::Kendra::Index"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.capacity_units
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 1000",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.server_side_encryption_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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

impl cfn_resources::CfnResource for CapacityUnitsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.query_capacity_units;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'query_capacity_units'. {} is less than 0",
                the_val
            ));
        }

        let the_val = &self.storage_capacity_units;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'storage_capacity_units'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// Specifies the properties, such as relevance tuning and searchability, of an index       field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentMetadataConfiguration {
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
    pub name: cfn_resources::StrVal,

    ///
    /// Provides tuning parameters to determine how the field affects the search       results.
    ///
    /// Required: No
    ///
    /// Type: Relevance
    ///
    /// Update requires: No interruption
    #[serde(rename = "Relevance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance: Option<Relevance>,

    ///
    /// Provides information about how the field is used during a search.
    ///
    /// Required: No
    ///
    /// Type: Search
    ///
    /// Update requires: No interruption
    #[serde(rename = "Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Search>,

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
    pub cfn_type: DocumentMetadataConfigurationTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DocumentMetadataConfigurationTypeEnum {
    /// DATE_VALUE
    #[serde(rename = "DATE_VALUE")]
    Datevalue,

    /// LONG_VALUE
    #[serde(rename = "LONG_VALUE")]
    Longvalue,

    /// STRING_LIST_VALUE
    #[serde(rename = "STRING_LIST_VALUE")]
    Stringlistvalue,

    /// STRING_VALUE
    #[serde(rename = "STRING_VALUE")]
    Stringvalue,
}

impl Default for DocumentMetadataConfigurationTypeEnum {
    fn default() -> Self {
        DocumentMetadataConfigurationTypeEnum::Datevalue
    }
}

impl cfn_resources::CfnResource for DocumentMetadataConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 30",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.relevance
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.search.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    pub group_attribute_field: cfn_resources::StrVal,

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
    pub user_name_attribute_field: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for JsonTokenTypeConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.group_attribute_field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'group_attribute_field'. {} is greater than 2048", s.len()));
            }
        }

        let the_val = &self.group_attribute_field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'group_attribute_field'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_name_attribute_field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'user_name_attribute_field'. {} is greater than 2048", s.len()));
            }
        }

        let the_val = &self.user_name_attribute_field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_name_attribute_field'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_regex: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute_field: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<cfn_resources::StrVal>,

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
    pub key_location: JwtTokenTypeConfigurationKeyLocationEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_manager_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name_attribute_field: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JwtTokenTypeConfigurationKeyLocationEnum {
    /// SECRET_MANAGER
    #[serde(rename = "SECRET_MANAGER")]
    Secretmanager,

    /// URL
    #[serde(rename = "URL")]
    Url,
}

impl Default for JwtTokenTypeConfigurationKeyLocationEnum {
    fn default() -> Self {
        JwtTokenTypeConfigurationKeyLocationEnum::Secretmanager
    }
}

impl cfn_resources::CfnResource for JwtTokenTypeConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.claim_regex {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'claim_regex'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.claim_regex {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'claim_regex'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.group_attribute_field {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'group_attribute_field'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.group_attribute_field {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'group_attribute_field'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.issuer {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 65 as _ {
                    return Err(format!(
                        "Max validation failed on field 'issuer'. {} is greater than 65",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.issuer {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'issuer'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.secret_manager_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1284 as _ {
                    return Err(format!("Max validation failed on field 'secret_manager_arn'. {} is greater than 1284", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.secret_manager_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'secret_manager_arn'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'url'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'url'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_name_attribute_field {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'user_name_attribute_field'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.user_name_attribute_field {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'user_name_attribute_field'. {} is less than 1", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Provides information for tuning the relevance of a field in a search. When a query       includes terms that match the field, the results are given a boost in the response based       on these tuning parameters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Relevance {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<cfn_resources::StrVal>,

    ///
    /// Indicates that this field determines how "fresh" a document is. For example, if       document 1 was created on November 5, and document 2 was created on October 31, document       1 is "fresher" than document 2. You can only set the Freshness field on one         DATE type field. Only applies to DATE fields.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Freshness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_order: Option<RelevanceRankOrderEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_importance_items: Option<Vec<ValueImportanceItem>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RelevanceRankOrderEnum {
    /// ASCENDING
    #[serde(rename = "ASCENDING")]
    Ascending,

    /// DESCENDING
    #[serde(rename = "DESCENDING")]
    Descending,
}

impl Default for RelevanceRankOrderEnum {
    fn default() -> Self {
        RelevanceRankOrderEnum::Ascending
    }
}

impl cfn_resources::CfnResource for Relevance {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.duration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10 as _ {
                    return Err(format!(
                        "Max validation failed on field 'duration'. {} is greater than 10",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.duration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'duration'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.importance {
            if *the_val > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'importance'. {} is greater than 10",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.importance {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'importance'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayable: Option<bool>,

    ///
    /// Indicates that the field can be used to create search facets, a count of results for       each value in the field. The default is false .
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Facetable")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,

    ///
    /// Determines whether the field can be used to sort the results of a query. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sortable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
}

impl cfn_resources::CfnResource for Search {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ServerSideEncryptionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token_type_configuration: Option<JwtTokenTypeConfiguration>,
}

impl cfn_resources::CfnResource for UserTokenConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.json_token_type_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.jwt_token_type_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies a key-value pair of the search boost value       for a document when the key is part of the metadata of a       document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValueImportanceItem {
    ///
    /// The document metadata value used for the search boost.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The boost value for a document when the key is part of       the metadata of a document.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl cfn_resources::CfnResource for ValueImportanceItem {
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
