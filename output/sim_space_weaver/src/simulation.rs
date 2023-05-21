

/// Use the AWS::SimSpaceWeaver::Simulation resource to specify     a simulation that AWS CloudFormation starts in the AWS Cloud, in     your AWS account. In the resource properties section of your template,     provide the name of an existing IAM role     configured with the proper permissions, and the name of an existing Amazon S3 bucket.     Your account must have permissions to read the Amazon S3 bucket.     The Amazon S3 bucket must contain a valid schema. The schema must refer to     simulation assets that are already uploaded to the AWS Cloud. For more information,     see the       detailed tutorial in the AWSSimSpace Weaver User Guide.
///
/// Specify a SnapshotS3Location to start a simulation from a snapshot instead of from     a schema. When you start a simulation from a snapshot, SimSpace Weaver initializes the entity     data in the State Fabric with data saved in the snapshot, starts the spatial and service apps that     were running when the snapshot was created, and restores the clock to the appropriate tick. Your app     zip files must be in the same location in Amazon S3 as they were in for the original simulation.     You must start any custom apps separately. For more information about snapshots, see     Snapshots     in the AWSSimSpace Weaver User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSimulation {


    /// 
    /// The maximum running time of the simulation,    specified as a number of minutes (m or M), hours (h or H), or days (d or D). The simulation    stops when it reaches this limit. The maximum value is 14D, or its equivalent in the    other units. The default value is 14D. A value equivalent to 0 makes the    simulation immediately transition to STOPPING as soon as it reaches STARTED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaximumDuration")]
    pub maximum_duration: Option<String>,


    /// 
    /// The name of the simulation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role     that the simulation assumes to perform actions.      For more information about ARNs, see Amazon Resource Names (ARNs)     in the AWS General Reference.     For more information about IAM roles,     see IAM       roles in the AWS Identity and Access Management User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The location of the simulation schema in Amazon Simple Storage Service (Amazon S3).     For more information about Amazon S3, see the     Amazon Simple Storage Service User Guide.
    /// 
    /// Provide a SchemaS3Location to start your simulation from a schema.
    /// 
    /// If you provide a SchemaS3Location then you can't provide a SnapshotS3Location.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaS3Location")]
    pub schema_s3_location: Option<S3Location>,


    /// 
    /// The location of the snapshot in Amazon Simple Storage Service (Amazon S3).    For more information about Amazon S3, see the    Amazon Simple Storage Service User Guide.
    /// 
    /// Provide a SnapshotS3Location to start your simulation from a snapshot.
    /// 
    /// If you provide a SnapshotS3Location then you can't provide a SchemaS3Location.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotS3Location")]
    pub snapshot_s3_location: Option<S3Location>,

}



impl cfn_resources::CfnResource for CfnSimulation {
    fn type_string() -> &'static str {
        "AWS::SimSpaceWeaver::Simulation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.schema_s3_location.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.snapshot_s3_location.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A location in Amazon Simple Storage Service (Amazon S3) where SimSpace Weaver stores simulation data, such as your app .zip     files and schema file. For more information about Amazon S3, see the Amazon Simple Storage Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The name of an Amazon S3 bucket. For more information about buckets, see Creating,       configuring, and working with Amazon S3 buckets in the Amazon Simple Storage Service User       Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The key name of an object in Amazon S3. For more information about Amazon S3 objects and object     keys, see Uploading,       downloading, and working with objects in Amazon S3 in the Amazon Simple Storage Service User       Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectKey")]
    pub object_key: String,

}



impl cfn_resources::CfnResource for S3Location {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}