/// The AWS::QLDB::Stream resource specifies a journal stream for a given Amazon     Quantum Ledger Database (Amazon QLDB) ledger. The stream captures every document revision     that is committed to the ledger's journal and delivers the data to a specified Amazon     Kinesis Data Streams resource.
///
/// For more information, see StreamJournalToKinesis in the Amazon QLDB API     Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnStream {
    ///
    /// The exclusive date and time that specifies when the stream ends. If you don't define     this parameter, the stream runs indefinitely until you cancel it.
    ///
    /// The ExclusiveEndTime must be in ISO 8601 date and time format     and in Universal Coordinated Time (UTC). For example:     2019-06-13T21:36:34Z.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExclusiveEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_end_time: Option<cfn_resources::StrVal>,

    ///
    /// The inclusive start date and time from which to start streaming journal data. This     parameter must be in ISO 8601 date and time format and in Universal     Coordinated Time (UTC). For example: 2019-06-13T21:36:34Z.
    ///
    /// The InclusiveStartTime cannot be in the future and must be before       ExclusiveEndTime.
    ///
    /// If you provide an InclusiveStartTime that is before the ledger's       CreationDateTime, QLDB effectively defaults it to the ledger's       CreationDateTime.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InclusiveStartTime")]
    pub inclusive_start_time: cfn_resources::StrVal,

    ///
    /// The configuration settings of the Kinesis Data Streams destination for your stream request.
    ///
    /// Required: Yes
    ///
    /// Type: KinesisConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "KinesisConfiguration")]
    pub kinesis_configuration: KinesisConfiguration,

    ///
    /// The name of the ledger.
    ///
    /// Required: Yes
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
    #[serde(rename = "LedgerName")]
    pub ledger_name: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a     journal stream to write data records to a Kinesis Data Streams resource.
    ///
    /// To pass a role to QLDB when requesting a journal stream, you must have permissions to     perform the iam:PassRole action on the IAM role resource. This is required for     all journal stream requests.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 1600
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The name that you want to assign to the QLDB journal stream. User-defined names can     help identify and indicate the purpose of a stream.
    ///
    /// Your stream name must be unique among other active streams for a     given ledger. Stream names have the same naming constraints as ledger names, as defined in       Quotas in Amazon QLDB in the Amazon QLDB Developer     Guide.
    ///
    /// Required: Yes
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
    #[serde(rename = "StreamName")]
    pub stream_name: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_arn: CfnStreamarn,

    #[serde(skip_serializing)]
    pub att_id: CfnStreamid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStreamarn;
impl CfnStreamarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStreamid;
impl CfnStreamid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnStream {
    fn type_string(&self) -> &'static str {
        "AWS::QLDB::Stream"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.kinesis_configuration.validate()?;

        let the_val = &self.ledger_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'ledger_name'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.ledger_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'ledger_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1600 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 1600",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        let the_val = &self.stream_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'stream_name'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.stream_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'stream_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The configuration settings of the Amazon Kinesis Data Streams destination for an Amazon QLDB journal     stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct KinesisConfiguration {
    ///
    /// Enables QLDB to publish multiple data records in a single Kinesis Data Streams record, increasing the     number of records sent per API call.
    ///
    /// Default: True
    ///
    /// ImportantRecord aggregation has important implications for processing records and requires       de-aggregation in your stream consumer. To learn more, see KPL Key Concepts and        Consumer        De-aggregation in the Amazon Kinesis Data Streams Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AggregationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_enabled: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) of the Kinesis Data Streams resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 1600
    ///
    /// Update requires: Replacement
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for KinesisConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.stream_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1600 as _ {
                    return Err(format!(
                        "Max validation failed on field 'stream_arn'. {} is greater than 1600",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.stream_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'stream_arn'. {} is less than 20",
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
