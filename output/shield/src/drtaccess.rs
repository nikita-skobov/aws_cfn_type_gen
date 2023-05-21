

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html
#[derive(Default, serde::Serialize)]
pub struct CfnDRTAccess {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-rolearn
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-logbucketlist
    #[serde(rename = "LogBucketList")]
    pub log_bucket_list: Option<Vec<String>>,

}
