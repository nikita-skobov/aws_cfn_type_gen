

/// Specifies a tracker resource in your AWS account, which lets you       receive current and historical location of devices.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTracker {


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
    pub kms_key_id: Option<String>,


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
    pub position_filtering: Option<String>,


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
    pub tracker_name: String,


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
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnTracker {
    fn type_string() -> &'static str {
        "AWS::Location::Tracker"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
