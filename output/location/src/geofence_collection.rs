/// The AWS::Location::GeofenceCollection resource specifies the ability to       detect and act when a tracked device enters or exits a defined geographical boundary       known as a geofence.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnGeofenceCollection {
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
    pub collection_name: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_arn: CfnGeofenceCollectionarn,

    #[serde(skip_serializing)]
    pub att_collection_arn: CfnGeofenceCollectioncollectionarn,

    #[serde(skip_serializing)]
    pub att_create_time: CfnGeofenceCollectioncreatetime,

    #[serde(skip_serializing)]
    pub att_update_time: CfnGeofenceCollectionupdatetime,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGeofenceCollectionarn;
impl CfnGeofenceCollectionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGeofenceCollectioncollectionarn;
impl CfnGeofenceCollectioncollectionarn {
    pub fn att_name(&self) -> &'static str {
        r#"CollectionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGeofenceCollectioncreatetime;
impl CfnGeofenceCollectioncreatetime {
    pub fn att_name(&self) -> &'static str {
        r#"CreateTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGeofenceCollectionupdatetime;
impl CfnGeofenceCollectionupdatetime {
    pub fn att_name(&self) -> &'static str {
        r#"UpdateTime"#
    }
}

impl cfn_resources::CfnResource for CfnGeofenceCollection {
    fn type_string(&self) -> &'static str {
        "AWS::Location::GeofenceCollection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.collection_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'collection_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.collection_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'collection_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

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

        Ok(())
    }
}
