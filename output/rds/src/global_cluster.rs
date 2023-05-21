/// The AWS::RDS::GlobalCluster resource creates or updates an Amazon Aurora global database     spread across multiple AWS Regions.
///
/// The global database contains a single primary cluster with read-write capability,     and a read-only secondary cluster that receives     data from the primary cluster through high-speed replication     performed by the Aurora storage subsystem.
///
/// You can create a global database that is initially empty, and then     add a primary cluster and a secondary cluster to it.
///
/// For information about Aurora global databases, see       Working with Amazon Aurora Global Databases in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGlobalCluster {
    ///
    /// The deletion protection setting for the new global database.     The global database can't be deleted when deletion protection is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,

    ///
    /// The name of the database engine to be used for this DB cluster.
    ///
    /// If this property isn't specified, the database engine is derived from the source DB cluster specified        by the SourceDBClusterIdentifier property.
    ///
    /// NoteIf the SourceDBClusterIdentifier property isn't specified, this property is required.          If the SourceDBClusterIdentifier property is specified, make sure this property isn't specified.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<cfn_resources::StrVal>,

    ///
    /// The engine version of the Aurora global database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// The cluster identifier of the global database cluster.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The DB cluster identifier or Amazon Resource Name (ARN) to use as the primary cluster of the global database.
    ///
    /// NoteIf the Engine property isn't specified, this property is required. If the Engine property is specified,         make sure this property isn't specified.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The storage encryption setting for the global database cluster.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
}

impl cfn_resources::CfnResource for CfnGlobalCluster {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::GlobalCluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
