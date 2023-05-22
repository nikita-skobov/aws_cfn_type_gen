/// Creates a new event data store.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventDataStore {
    ///
    /// The advanced event selectors to use to select the events for the data store. You can     configure up to five advanced event selectors for each event data store.
    ///
    /// For more information about how to use advanced event selectors to log CloudTrail     events, see Log events by using advanced event selectors in the CloudTrail User Guide.
    ///
    /// For more information about how to use advanced event selectors to include AWS Config configuration items in your event data store, see Create an event data store for AWS Config configuration       items in the CloudTrail User Guide.
    ///
    /// For more information about how to use advanced event selectors to include non-AWS events in your event data store, see Create an integration to log events from outside AWS in the CloudTrail User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of AdvancedEventSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,

    ///
    /// Specifies the AWS KMS key ID to use to encrypt the events delivered by       CloudTrail. The value can be an alias name prefixed by alias/, a     fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique     identifier.
    ///
    /// ImportantDisabling or deleting the KMS key, or removing CloudTrail       permissions on the key, prevents CloudTrail from logging events to the event data       store, and prevents users from querying the data in the event data store that was       encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you       disable or delete a KMS key that you are using with an event data store,       delete or back up your event data store.
    ///
    /// CloudTrail also supports AWS KMS multi-Region keys. For more     information about multi-Region keys, see Using multi-Region       keys in the         AWS Key Management Service Developer Guide.
    ///
    /// Examples:
    ///
    /// alias/MyAliasName                                arn:aws:kms:us-east-2:123456789012:alias/MyAliasName                                arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012                                12345678-1234-1234-1234-123456789012
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 350
    ///
    /// Pattern: ^[a-zA-Z0-9._/\-:]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the event data store includes events from all regions, or only from     the region in which the event data store is created.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,

    ///
    /// The name of the event data store.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9._\-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether an event data store collects events logged for an organization in       AWS Organizations.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,

    ///
    /// The retention period of the event data store, in days. You can set a retention period of up to 2557 days,      the equivalent of seven years.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 7
    ///
    /// Maximum: 2557
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i64>,

    ///
    /// A list of tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Specifies whether termination protection is enabled for the event data store. If     termination protection is enabled, you cannot delete the event data store until termination     protection is disabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,

    #[serde(skip_serializing)]
    pub att_created_timestamp: CfnEventDataStorecreatedtimestamp,

    #[serde(skip_serializing)]
    pub att_event_data_store_arn: CfnEventDataStoreeventdatastorearn,

    #[serde(skip_serializing)]
    pub att_status: CfnEventDataStorestatus,

    #[serde(skip_serializing)]
    pub att_updated_timestamp: CfnEventDataStoreupdatedtimestamp,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventDataStorecreatedtimestamp;
impl CfnEventDataStorecreatedtimestamp {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedTimestamp"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventDataStoreeventdatastorearn;
impl CfnEventDataStoreeventdatastorearn {
    pub fn att_name(&self) -> &'static str {
        r#"EventDataStoreArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventDataStorestatus;
impl CfnEventDataStorestatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventDataStoreupdatedtimestamp;
impl CfnEventDataStoreupdatedtimestamp {
    pub fn att_name(&self) -> &'static str {
        r#"UpdatedTimestamp"#
    }
}

impl cfn_resources::CfnResource for CfnEventDataStore {
    fn type_string(&self) -> &'static str {
        "AWS::CloudTrail::EventDataStore"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 350 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 350",
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

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 3 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 3",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.retention_period {
            if *the_val > 2557 as _ {
                return Err(format!(
                    "Max validation failed on field 'retention_period'. {} is greater than 2557",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.retention_period {
            if *the_val < 7 as _ {
                return Err(format!(
                    "Min validation failed on field 'retention_period'. {} is less than 7",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Advanced event selectors let you create fine-grained selectors for the following AWS CloudTrail event record ﬁelds. They help you control costs by logging only those     events that are important to you. For more information about advanced event selectors, see       Logging data events in the         AWS CloudTrail User Guide.
///
/// You cannot apply both event selectors and advanced event selectors to a trail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdvancedEventSelector {
    ///
    /// Contains all selector statements in an advanced event selector.
    ///
    /// Required: Yes
    ///
    /// Type: List of AdvancedFieldSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldSelectors")]
    pub field_selectors: Vec<AdvancedFieldSelector>,

    ///
    /// An optional, descriptive name for an advanced event selector, such as "Log data events     for only two S3 buckets".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AdvancedEventSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A single selector statement in an advanced event selector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdvancedFieldSelector {
    ///
    /// An operator that includes events that match the last few characters of the event record     field specified as the value of Field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndsWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<Vec<String>>,

    ///
    /// An operator that includes events that match the exact value of the event record field     specified as the value of Field. This is the only valid operator that you can     use with the readOnly, eventCategory, and       resources.type fields.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Equals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,

    ///
    /// A field in a CloudTrail event record on which to filter events to be logged. For     event data stores for AWS Config configuration items, Audit Manager evidence, or non-AWS events, the field is used only for     selecting events as filtering is not supported.
    ///
    /// For CloudTrail event records, supported fields include readOnly,       eventCategory, eventSource (for management events),       eventName, resources.type, and resources.ARN.
    ///
    /// For event data stores for AWS Config configuration items, Audit Manager evidence, or non-AWS events, the only supported field is       eventCategory.
    ///
    /// readOnly          - Optional. Can be set to          Equals a value of true or false. If you do        not add this field, CloudTrail logs both read and          write events. A value of true logs only          read events. A value of false logs only          write events.                                   eventSource          - For filtering        management events only. This can be set only to NotEquals          kms.amazonaws.com.                                   eventName          - Can use any operator.        You can use it to ﬁlter in or ﬁlter out any data event logged to CloudTrail,        such as PutBucket or GetSnapshotBlock. You can have        multiple values for this ﬁeld, separated by commas.                                   eventCategory          - This is required and        must be set to Equals.                                                                                     For CloudTrail event records, the value           must be Management or Data.                                          For AWS Config           configuration items, the value must be ConfigurationItem.                                         For Audit Manager evidence, the value must be Evidence.                                         For non-AWS events, the value must be ActivityAuditLog.                                                      resources.type          - This ﬁeld is        required for CloudTrail data events. resources.type can only        use the Equals operator, and the value can be one of the        following:                                                                                                                                                                                                                  AWS::DynamoDB::Table                                            AWS::Lambda::Function                                            AWS::S3::Object                                            AWS::CloudTrail::Channel                                            AWS::Cognito::IdentityPool                                            AWS::DynamoDB::Stream                                            AWS::EC2::Snapshot                                            AWS::FinSpace::Environment                                            AWS::Glue::Table                                            AWS::GuardDuty::Detector                                            AWS::KendraRanking::ExecutionPlan                                            AWS::ManagedBlockchain::Node                                            AWS::SageMaker::ExperimentTrialComponent                                            AWS::SageMaker::FeatureGroup                                            AWS::S3::AccessPoint                                            AWS::S3ObjectLambda::AccessPoint                                            AWS::S3Outposts::Object                             You can have only one resources.type ﬁeld per selector. To log data        events on more than one resource type, add another selector.                                   resources.ARN          - You can use any        operator with resources.ARN, but if you use Equals or          NotEquals, the value must exactly match the ARN of a valid resource        of the type you've speciﬁed in the template as the value of resources.type. For        example, if resources.type equals AWS::S3::Object, the ARN must be in        one of the following formats. To log all data events for all objects in a specific S3        bucket, use the StartsWith operator, and include only the bucket ARN as        the matching value.        The trailing slash is intentional; do not exclude it. Replace the text between        less than and greater than symbols (<>) with resource-specific information.                                                            arn:<partition>:s3:::<bucket_name>/                                            arn:<partition>:s3:::<bucket_name>/<object_path>/                             When resources.type equals AWS::DynamoDB::Table, and the operator is        set to Equals or NotEquals, the ARN must be in the        following format:                                                  arn:<partition>:dynamodb:<region>:<account_ID>:table/<table_name>                             When resources.type equals AWS::Lambda::Function, and the operator is        set to Equals or NotEquals, the ARN must be in the        following format:                                                  arn:<partition>:lambda:<region>:<account_ID>:function:<function_name>                             When resources.type equals AWS::CloudTrail::Channel, and the operator is        set to Equals or NotEquals, the ARN must be in the        following format:                                                  arn:<partition>:cloudtrail:<region>:<account_ID>:channel/<channel_UUID>                             When resources.type equals AWS::Cognito::IdentityPool, and the operator is        set to Equals or NotEquals, the ARN must be in the        following format:                                                  arn:<partition>:cognito-identity:<region>:<account_ID>:identitypool/<identity_pool_ID>                             When resources.type equals AWS::DynamoDB::Stream, and        the operator is set to Equals or NotEquals, the ARN must be        in the following format:                                                  arn:<partition>:dynamodb:<region>:<account_ID>:table/<table_name>/stream/<date_time>                             When resources.type equals AWS::EC2::Snapshot, and the        operator is set to Equals or NotEquals, the ARN must be in        the following format:                                                  arn:<partition>:ec2:<region>::snapshot/<snapshot_ID>                             When resources.type equals AWS::FinSpace::Environment,        and the operator is set to Equals or NotEquals, the ARN        must be in the following format:                                                  arn:<partition>:finspace:<region>:<account_ID>:environment/<environment_ID>                             When resources.type equals AWS::Glue::Table, and the        operator is set to Equals or NotEquals, the ARN must be in        the following format:                                                  arn:<partition>:glue:<region>:<account_ID>:table/<database_name>/<table_name>                             When resources.type equals AWS::GuardDuty::Detector, and the        operator is set to Equals or NotEquals, the ARN must be in        the following format:                                                  arn:<partition>:guardduty:<region>:<account_ID>:detector/<detector_ID>                             When resources.type equals AWS::KendraRanking::ExecutionPlan, and the        operator is set to Equals or NotEquals, the ARN must be in        the following format:                                                  arn:<partition>:kendra-ranking:<region>:<account_ID>:rescore-execution-plan/<rescore_execution_plan_ID>                             When resources.type equals AWS::ManagedBlockchain::Node,        and the operator is set to Equals or NotEquals, the ARN        must be in the following format:                                                  arn:<partition>:managedblockchain:<region>:<account_ID>:nodes/<node_ID>                             When resources.type equals AWS::SageMaker::ExperimentTrialComponent, and the operator is set to        Equals or NotEquals, the ARN must be in the following format:                                                  arn:<partition>:sagemaker:<region>:<account_ID>:experiment-trial-component/<experiment_trial_component_name>                             When resources.type equals AWS::SageMaker::FeatureGroup, and the operator is set to        Equals or NotEquals, the ARN must be in the following format:                                                  arn:<partition>:sagemaker:<region>:<account_ID>:feature-group/<feature_group_name>                             When resources.type equals AWS::S3::AccessPoint, and the        operator is set to Equals or NotEquals, the ARN must be in        one of the following formats. To log events on all objects in an S3 access point, we        recommend that you use only the access point ARN, don’t include the object path, and        use the StartsWith or NotStartsWith operators.                                                            arn:<partition>:s3:<region>:<account_ID>:accesspoint/<access_point_name>                                            arn:<partition>:s3:<region>:<account_ID>:accesspoint/<access_point_name>/object/<object_path>                             When resources.type equals        AWS::S3ObjectLambda::AccessPoint, and the operator is set to        Equals or NotEquals, the ARN must be in the following        format:                                                  arn:<partition>:s3-object-lambda:<region>:<account_ID>:accesspoint/<access_point_name>                             When resources.type equals AWS::S3Outposts::Object, and        the operator is set to Equals or NotEquals, the ARN must be        in the following format:                                                  arn:<partition>:s3-outposts:<region>:<account_ID>:<object_path>
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [\w|\d|\.|_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: cfn_resources::StrVal,

    ///
    /// An operator that excludes events that match the last few characters of the event record     field specified as the value of Field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotEndsWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ends_with: Option<Vec<String>>,

    ///
    /// An operator that excludes events that match the exact value of the event record field     specified as the value of Field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,

    ///
    /// An operator that excludes events that match the first few characters of the event     record field specified as the value of Field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotStartsWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_starts_with: Option<Vec<String>>,

    ///
    /// An operator that includes events that match the first few characters of the event record     field specified as the value of Field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartsWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for AdvancedFieldSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'field'. {} is greater than 1000",
                    s.len()
                ));
            }
        }

        let the_val = &self.field;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'field'. {} is less than 1",
                    s.len()
                ));
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
