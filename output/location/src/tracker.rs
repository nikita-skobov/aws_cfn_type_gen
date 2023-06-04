/// Specifies a tracker resource in your AWS account, which lets you       receive current and historical location of devices.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTracker {
    ///
    /// An optional description for the tracker resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A key identifier for an       AWS         KMS customer managed key. Enter a key ID, key ARN, alias name, or alias ARN.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the position filtering for the tracker resource.
    ///
    /// Valid values:
    ///
    /// TimeBased - Location updates are evaluated against linked geofence collections,           but not every location update is stored. If your update frequency is more often than 30 seconds,           only one update per 30 seconds is stored for each unique device ID.                                  DistanceBased - If the device has moved less than 30 m (98.4 ft), location updates are           ignored. Location updates within this area are neither evaluated against linked geofence collections, nor stored.           This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through.           Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map.                                  AccuracyBased - If the device has moved less than the measured accuracy,           location updates are ignored. For example, if two consecutive updates from a device           have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device           has moved less than 15 m. Ignored location updates are neither evaluated against           linked geofence collections, nor stored. This can reduce the effects of GPS noise           when displaying device trajectories on a map, and can help control your costs by reducing the           number of geofence evaluations.
    ///
    /// This field is optional. If not specified, the default value is TimeBased.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AccuracyBased | DistanceBased | TimeBased
    ///
    /// Update requires: Replacement
    #[serde(rename = "PositionFiltering")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub position_filtering: Option<TrackerPositionFilteringEnum>,

    ///
    /// The name for the tracker resource.
    ///
    /// Requirements:
    ///
    /// Contain only alphanumeric characters (A-Z, a-z, 0-9) , hyphens (-), periods (.), and underscores (_).Must be a unique tracker resource name.         No spaces allowed. For example, ExampleTracker.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[-._\w]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrackerName")]
    pub tracker_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnTrackerarn,

    #[serde(skip_serializing)]
    pub att_create_time: CfnTrackercreatetime,

    #[serde(skip_serializing)]
    pub att_tracker_arn: CfnTrackertrackerarn,

    #[serde(skip_serializing)]
    pub att_update_time: CfnTrackerupdatetime,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TrackerPositionFilteringEnum {
    /// AccuracyBased
    #[serde(rename = "AccuracyBased")]
    Accuracybased,

    /// DistanceBased
    #[serde(rename = "DistanceBased")]
    Distancebased,

    /// TimeBased
    #[serde(rename = "TimeBased")]
    Timebased,
}

impl Default for TrackerPositionFilteringEnum {
    fn default() -> Self {
        TrackerPositionFilteringEnum::Accuracybased
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTrackerarn;
impl CfnTrackerarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTrackercreatetime;
impl CfnTrackercreatetime {
    pub fn att_name(&self) -> &'static str {
        r#"CreateTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTrackertrackerarn;
impl CfnTrackertrackerarn {
    pub fn att_name(&self) -> &'static str {
        r#"TrackerArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTrackerupdatetime;
impl CfnTrackerupdatetime {
    pub fn att_name(&self) -> &'static str {
        r#"UpdateTime"#
    }
}

impl cfn_resources::CfnResource for CfnTracker {
    fn type_string(&self) -> &'static str {
        "AWS::Location::Tracker"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

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

        let the_val = &self.tracker_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'tracker_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.tracker_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'tracker_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
