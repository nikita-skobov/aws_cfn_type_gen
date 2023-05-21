

/// The AWS::Location::TrackerConsumer resource specifies an association       between a geofence collection and a tracker resource. The geofence collection is       referred to as the consumer of the tracker. This allows the tracker resource       to communicate location data to the linked geofence collection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrackerConsumer {


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
    /// The Amazon Resource Name (ARN) for the geofence collection to be associated to tracker       resource. Used when you need to specify a resource across all AWS.
    /// 
    /// Format example:             arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollectionConsumer
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1600
    ///
    /// Pattern: ^arn(:[a-z0-9]+([.-][a-z0-9]+)*){2}(:([a-z0-9]+([.-][a-z0-9]+)*)?){2}:([^/].*)?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConsumerArn")]
    pub consumer_arn: String,

}

impl cfn_resources::CfnResource for CfnTrackerConsumer {
    fn type_string() -> &'static str {
        "AWS::Location::TrackerConsumer"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
