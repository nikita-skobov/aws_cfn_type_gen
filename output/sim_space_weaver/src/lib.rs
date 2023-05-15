
pub mod cfn_simulation {

#[derive(serde::Serialize, Default)]
pub struct CfnSimulation {
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotS3Location")]
    pub snapshot_s3_location: Option<S3Location>,
    /// The name of the simulation.
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SchemaS3Location")]
    pub schema_s3_location: Option<S3Location>,
    /// Role ARN.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The maximum running time of the simulation.
    #[serde(rename = "MaximumDuration")]
    pub maximum_duration: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "ObjectKey")]
    pub object_key: String,
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}


}
