

/// The AWS::Location::GeofenceCollection resource specifies the ability to       detect and act when a tracked device enters or exits a defined geographical boundary       known as a geofence.
#[derive(Default, serde::Serialize)]
pub struct CfnGeofenceCollection {


    /// 
    /// An optional description for the geofence collection.
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
    /// A custom name for the geofence collection.
    /// 
    /// Requirements:
    /// 
    /// Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods           (.), and underscores (_).                Must be a unique geofence collection name.               No spaces allowed. For example, ExampleGeofenceCollection.
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
    #[serde(rename = "CollectionName")]
    pub collection_name: String,

}