/// Creates an archive of events with the specified settings. When you create an archive,    incoming events might not immediately start being sent to the archive. Allow a short period of    time for changes to take effect. If you do not specify a pattern to filter events sent to the    archive, all events are sent to the archive except replayed events. Replayed events are not    sent to an archive.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnArchive {
    ///
    /// The name for the archive to create.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 48
    ///
    /// Pattern: [\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ArchiveName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_name: Option<String>,

    ///
    /// A description for the archive.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    /// An event pattern to use to filter events sent to the archive.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<serde_json::Value>,

    ///
    /// The number of days to retain events for. Default value is 0. If set to 0, events are    retained indefinitely
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,

    ///
    /// The ARN of the event bus that sends events to the archive.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
}

impl cfn_resources::CfnResource for CfnArchive {
    fn type_string(&self) -> &'static str {
        "AWS::Events::Archive"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.archive_name {
            if the_val.len() > 48 as _ {
                return Err(format!(
                    "Max validation failed on field 'archive_name'. {} is greater than 48",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.archive_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'archive_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.retention_days {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'retention_days'. {} is less than 0",
                    the_val
                ));
            }
        }

        let the_val = &self.source_arn;

        if the_val.len() > 1600 as _ {
            return Err(format!(
                "Max validation failed on field 'source_arn'. {} is greater than 1600",
                the_val.len()
            ));
        }

        let the_val = &self.source_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'source_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
