
pub mod cfn_collection {

#[derive(serde::Serialize, Default)]
pub struct CfnCollection {
    /// The name of the collection
    #[serde(rename = "CollectionId")]
    pub collection_id: CollectionId,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

pub type CollectionId = String;pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// The name of the project
    #[serde(rename = "ProjectName")]
    pub project_name: ProjectName,

}

pub type ProjectName = String;pub type Arn = String;

}

pub mod cfn_stream_processor {

#[derive(serde::Serialize, Default)]
pub struct CfnStreamProcessor {
    /// ARN of the IAM role that allows access to the stream processor, and provides Rekognition read permissions for KVS stream and write permissions to S3 bucket and SNS topic.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Name of the stream processor. It's an identifier you assign to the stream processor. You can use it to manage the stream processor.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Indicates whether Rekognition is allowed to store the video stream data for model-training.
    #[serde(rename = "DataSharingPreference")]
    pub data_sharing_preference: Option<DataSharingPreference>,
    /// The BoundingBoxRegionsOfInterest specifies an array of bounding boxes of interest in the video frames to analyze, as part of connected home feature. If an object is partially in a region of interest, Rekognition will tag it as detected if the overlap of the object with the region-of-interest is greater than 20%.
    #[serde(rename = "BoundingBoxRegionsOfInterest")]
    pub bounding_box_regions_of_interest: Option<Vec<BoundingBox>>,
    /// The ARN of the SNS notification channel where events of interests are published, as part of connected home feature.
    #[serde(rename = "NotificationChannel")]
    pub notification_channel: Option<NotificationChannel>,
    /// Connected home settings to use on a streaming video. Note that either ConnectedHomeSettings or FaceSearchSettings should be set. Not both
    #[serde(rename = "ConnectedHomeSettings")]
    pub connected_home_settings: Option<ConnectedHomeSettings>,
    /// The S3 location in customer's account where inference output & artifacts are stored, as part of connected home feature.
    #[serde(rename = "S3Destination")]
    pub s3_destination: Option<S3Destination>,
    /// The Kinesis Video Stream that streams the source video.
    #[serde(rename = "KinesisVideoStream")]
    pub kinesis_video_stream: KinesisVideoStream,
    /// The Amazon Kinesis Data Stream stream to which the Amazon Rekognition stream processor streams the analysis results, as part of face search feature.
    #[serde(rename = "KinesisDataStream")]
    pub kinesis_data_stream: Option<KinesisDataStream>,
    /// The KMS key that is used by Rekognition to encrypt any intermediate customer metadata and store in the customer's S3 bucket.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Face search settings to use on a streaming video. Note that either FaceSearchSettings or ConnectedHomeSettings should be set. Not both
    #[serde(rename = "FaceSearchSettings")]
    pub face_search_settings: Option<FaceSearchSettings>,
    /// The PolygonRegionsOfInterest specifies a set of polygon areas of interest in the video frames to analyze, as part of connected home feature. Each polygon is in turn, an ordered list of Point
    #[serde(rename = "PolygonRegionsOfInterest")]
    pub polygon_regions_of_interest: Option<Vec<Polygon>>,

}


#[derive(serde::Serialize, Default)]
pub struct DataSharingPreference {
    #[serde(rename = "OptIn")]
    pub opt_in: bool,

}
pub type Labels = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct ConnectedHomeSettings {
    #[serde(rename = "Labels")]
    pub labels: Labels,
    #[serde(rename = "MinConfidence")]
    pub min_confidence: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationChannel {
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Destination {
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "ObjectKeyPrefix")]
    pub object_key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FaceSearchSettings {
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    #[serde(rename = "FaceMatchThreshold")]
    pub face_match_threshold: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct KinesisVideoStream {
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Point {
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "X")]
    pub x: f64,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisDataStream {
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Polygon {

}

#[derive(serde::Serialize, Default)]
pub struct BoundingBox {
    #[serde(rename = "Top")]
    pub top: f64,
    #[serde(rename = "Width")]
    pub width: f64,
    #[serde(rename = "Height")]
    pub height: f64,
    #[serde(rename = "Left")]
    pub left: f64,

}


}
