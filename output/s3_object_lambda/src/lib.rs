
pub mod cfn_access_point {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPoint {
    /// The Object lambda Access Point Configuration that configures transformations to be applied on the objects on specified S3 Actions
    #[serde(rename = "ObjectLambdaConfiguration")]
    pub object_lambda_configuration: ObjectLambdaConfiguration,
    /// The name you want to assign to this Object lambda Access Point.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AwsLambda {
    #[serde(rename = "FunctionPayload")]
    pub function_payload: Option<String>,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ObjectLambdaConfiguration {
    #[serde(rename = "TransformationConfigurations")]
    pub transformation_configurations: Vec<TransformationConfiguration>,
    #[serde(rename = "SupportingAccessPoint")]
    pub supporting_access_point: String,
    #[serde(rename = "AllowedFeatures")]
    pub allowed_features: Option<Vec<String>>,
    #[serde(rename = "CloudWatchMetricsEnabled")]
    pub cloud_watch_metrics_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TransformationConfiguration {
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "ContentTransformation")]
    pub content_transformation: (),

}
pub type Action = String;
#[derive(serde::Serialize, Default)]
pub struct Alias {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    pub is_public: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,

}


}

pub mod cfn_access_point_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPointPolicy {
    /// A policy document containing permissions to add to the specified ObjectLambdaAccessPoint. For more information, see Access Policy Language Overview (https://docs.aws.amazon.com/AmazonS3/latest/dev/access-policy-language-overview.html) in the Amazon Simple Storage Service Developer Guide.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    /// The name of the Amazon S3 ObjectLambdaAccessPoint to which the policy applies.
    #[serde(rename = "ObjectLambdaAccessPoint")]
    pub object_lambda_access_point: String,

}



}
