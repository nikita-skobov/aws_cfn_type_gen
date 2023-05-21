/// The AWS::Rekognition::StreamProcessor type creates a stream processor used to detect      and recognize faces or to detect connected home     labels in a streaming video. Amazon Rekognition Video is a consumer of live video from     Amazon Kinesis Video Streams. There are two different settings for stream processors in     Amazon Rekognition, one for detecting faces and one for connected home features.
///
/// If you are creating a stream processor for detecting faces, you provide a     Kinesis video stream (input) and a Kinesis data stream (output). You also specify the face     recognition criteria in FaceSearchSettings. For example, the collection containing faces     that you want to recognize.
///
/// If you are creating a stream processor for detection of connected home labels, you     provide a Kinesis video stream for input, and for output an Amazon S3 bucket and an Amazon SNS topic. You     can also provide a KMS key ID to encrypt the data sent to your Amazon S3 bucket. You     specify what you want to detect in ConnectedHomeSettings, such as people, packages, and     pets.
///
/// You can also specify where in the frame you want Amazon Rekognition to monitor with     BoundingBoxRegionsOfInterest and PolygonRegionsOfInterest. The Name is used to manage the     stream processor and it is the identifier for the stream processor. The       AWS::Rekognition::StreamProcessor resource creates a stream processor in     the same Region where you create the Amazon CloudFormation stack.
///
/// For more information, see CreateStreamProcessor.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStreamProcessor {
    ///
    /// List of BoundingBox objects, each of which denotes a region of interest on screen.     For more information, see the BoundingBox field of RegionOfInterest.
    ///
    /// Required: No
    ///
    /// Type: List of BoundingBox
    ///
    /// Update requires: Replacement
    #[serde(rename = "BoundingBoxRegionsOfInterest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box_regions_of_interest: Option<Vec<BoundingBox>>,

    ///
    /// Connected home settings to use on a streaming video. You can use a stream processor for connected home features and select      what you want the stream processor to detect, such as people or pets. When the stream processor has started, one notification is sent for      each object class specified. For more information,      see the ConnectedHome section of StreamProcessorSettings.
    ///
    /// Required: No
    ///
    /// Type: ConnectedHomeSettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectedHomeSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_home_settings: Option<ConnectedHomeSettings>,

    ///
    /// Allows you to opt in or opt out to share data with Rekognition to improve model performance.      You can choose this option at the account level or on a per-stream basis. Note that if you opt out at the account level this setting is ignored on individual streams.     For more information, see StreamProcessorDataSharingPreference.
    ///
    /// Required: No
    ///
    /// Type: DataSharingPreference
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSharingPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sharing_preference: Option<DataSharingPreference>,

    ///
    /// The input parameters used to recognize faces in a streaming video analyzed by an Amazon Rekognition stream processor.      For more information regarding the contents of the parameters, see FaceSearchSettings.
    ///
    /// Required: No
    ///
    /// Type: FaceSearchSettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "FaceSearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_search_settings: Option<FaceSearchSettings>,

    ///
    /// Amazon Rekognition's Video Stream Processor takes a Kinesis video stream as input. This is the Amazon Kinesis Data Streams instance      to which the Amazon Rekognition stream processor streams the analysis results.     This must be created within the constraints specified at      KinesisDataStream.
    ///
    /// Required: No
    ///
    /// Type: KinesisDataStream
    ///
    /// Update requires: Replacement
    #[serde(rename = "KinesisDataStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream: Option<KinesisDataStream>,

    ///
    /// The Kinesis video stream that provides the source of the streaming video for an Amazon Rekognition Video stream processor. For more information,      see KinesisVideoStream.
    ///
    /// Required: Yes
    ///
    /// Type: KinesisVideoStream
    ///
    /// Update requires: Replacement
    #[serde(rename = "KinesisVideoStream")]
    pub kinesis_video_stream: KinesisVideoStream,

    ///
    /// The identifier for your Amazon Key Management Service key (Amazon KMS key). Optional parameter for connected home stream processors      used to encrypt results and data published to your Amazon S3 bucket.      For more information, see the KMSKeyId section of CreateStreamProcessor.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The Name attribute specifies the name of the stream processor and it must be within the     constraints described in the Name section of StreamProcessor.     If you don't specify a name, Amazon CloudFormation generates a unique ID and uses that ID for the stream processor name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the object detection results and completion status of a video analysis operation.     Amazon Rekognition publishes a notification the first time an object of interest or a person is detected in the video stream.      Amazon Rekognition also publishes an end-of-session notification with a summary when the stream processing session is complete.     For more information, see StreamProcessorNotificationChannel.
    ///
    /// Required: No
    ///
    /// Type: NotificationChannel
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,

    ///
    /// A set of ordered lists of Point objects.      Each entry of the set contains a polygon denoting a region of interest on the screen. Each polygon is an ordered      list of Point objects.     For more information, see the Polygon field of RegionOfInterest.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolygonRegionsOfInterest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_regions_of_interest: Option<serde_json::Value>,

    ///
    /// The ARN of the IAM role that allows access to the stream processor. The IAM role provides Rekognition read permissions to the Kinesis stream.      It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a connected home stream processor.      This is required for both face search and connected home stream processors.      For information about constraints, see the RoleArn section of CreateStreamProcessor.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The Amazon S3 bucket location to which Amazon Rekognition publishes the detailed inference results of a video analysis operation.      For more information, see the S3Destination section of StreamProcessorOutput.
    ///
    /// Required: No
    ///
    /// Type: S3Destination
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,

    ///
    /// A set of tags (key-value pairs) that you want to attach to the stream processor.     For more information, see the Tags section of CreateStreamProcessor.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnStreamProcessor {
    fn type_string(&self) -> &'static str {
        "AWS::Rekognition::StreamProcessor"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connected_home_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.data_sharing_preference
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.face_search_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_data_stream
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_video_stream.validate()?;

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.notification_channel
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Identifies the bounding box around the label, face, text, or personal protective equipment.    The left (x-coordinate) and top (y-coordinate) are coordinates representing the top and    left sides of the bounding box. Note that the upper-left corner of the image is the origin    (0,0).
///
/// The top and left values returned are ratios of the overall    image size. For example, if the input image is 700x200 pixels, and the top-left coordinate of    the bounding box is 350x50 pixels, the API returns a left value of 0.5 (350/700)    and a top value of 0.25 (50/200).
///
/// The width and height values represent the dimensions of the    bounding box as a ratio of the overall image dimension. For example, if the input image is    700x200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. For more information, see       BoundingBox.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BoundingBox {
    ///
    /// Height of the bounding box as a ratio of the overall image height.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Height")]
    pub height: f64,

    ///
    /// Left coordinate of the bounding box as a ratio of overall image width.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Left")]
    pub left: f64,

    ///
    /// Top coordinate of the bounding box as a ratio of overall image height.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Top")]
    pub top: f64,

    ///
    /// Width of the bounding box as a ratio of the overall image width.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Width")]
    pub width: f64,
}

impl cfn_resources::CfnResource for BoundingBox {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Connected home settings to use on a streaming video. Defining the settings is required in the request parameter for CreateStreamProcessor.      Including this setting in the CreateStreamProcessor request lets you use the stream processor for connected home features. You can then select      what you want the stream processor to detect, such as people or pets.
///
/// When the stream processor has started, one notification is sent      for each object class specified. For example, if packages and pets are selected, one SNS notification is published the first time a package is      detected and one SNS notification is published the first time a pet is detected. An end-of-session summary is also published.      For more information, see the ConnectedHome section of StreamProcessorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectedHomeSettings {
    ///
    /// Specifies what you want to detect in the video, such as people, packages, or pets.      The current valid labels you can include in this list are: "PERSON", "PET", "PACKAGE", and "ALL".
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,

    ///
    /// The minimum confidence required to label an object in the video.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f64>,
}

impl cfn_resources::CfnResource for ConnectedHomeSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Allows you to opt in or opt out to share data with Rekognition to improve model performance.      You can choose this option at the account level or on a per-stream basis. Note that if you opt out at the account level, this setting is ignored on individual streams.     For more information, see StreamProcessorDataSharingPreference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSharingPreference {
    ///
    /// Describes the opt-in status applied to a stream processor's data sharing policy.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "OptIn")]
    pub opt_in: bool,
}

impl cfn_resources::CfnResource for DataSharingPreference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The input parameters used to recognize faces in a streaming video analyzed by a Amazon Rekognition stream processor. FaceSearchSettings is a request      parameter for CreateStreamProcessor.       For more information, see FaceSearchSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FaceSearchSettings {
    ///
    /// The ID of a collection that contains faces that you want to search for.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "CollectionId")]
    pub collection_id: cfn_resources::StrVal,

    ///
    /// Minimum face match confidence score that must be met to return a result for a recognized face. The default is 80.     0 is the lowest confidence. 100 is the highest confidence. Values between 0 and 100 are accepted, and values lower than 80 are set to 80.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f64>,
}

impl cfn_resources::CfnResource for FaceSearchSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.collection_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'collection_id'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.collection_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'collection_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Amazon Rekognition Video Stream Processor take as input a Kinesis video stream (Input) and a Kinesis data stream (Output).      This is the Amazon Kinesis Data Streams instance to which the Amazon Rekognition stream processor streams the analysis results.      This must be created within the constraints specified at      KinesisDataStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisDataStream {
    ///
    /// ARN of the output Amazon Kinesis Data Streams stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: (^arn:([a-z\d-]+):kinesis:([a-z\d-]+):\d{12}:.+$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KinesisDataStream {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Kinesis video stream that provides the source of the streaming video for an Amazon Rekognition Video stream processor. For more information, see     KinesisVideoStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisVideoStream {
    ///
    /// ARN of the Kinesis video stream stream that streams the source video.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: (^arn:([a-z\d-]+):kinesisvideo:([a-z\d-]+):\d{12}:.+$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KinesisVideoStream {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the object detection results and completion status of a video analysis operation.      Amazon Rekognition publishes a notification the first time an object of interest or a person is detected in the video stream.       Amazon Rekognition also publishes an an end-of-session notification with a summary when the stream processing session is complete.      For more information, see StreamProcessorNotificationChannel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationChannel {
    ///
    /// The ARN of the SNS topic that receives notifications.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NotificationChannel {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Amazon S3 bucket location to which Amazon Rekognition publishes the detailed inference results of a video analysis operation.      These results include the name of the stream processor resource, the session ID of the stream processing session,      and labeled timestamps and bounding boxes for detected labels. For more information, see      S3Destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Destination {
    ///
    /// Describes the destination Amazon Simple Storage Service (Amazon S3) bucket name of a stream processor's exports.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: cfn_resources::StrVal,

    ///
    /// Describes the destination Amazon Simple Storage Service (Amazon S3) object keys of a stream processor's exports.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Destination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
