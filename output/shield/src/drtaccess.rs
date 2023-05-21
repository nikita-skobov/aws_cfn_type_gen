

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDRTAccess {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-logbucketlist
    #[serde(rename = "LogBucketList")]
    pub log_bucket_list: Option<Vec<String>>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-rolearn
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



impl cfn_resources::CfnResource for CfnDRTAccess {
    fn type_string(&self) -> &'static str {
        "AWS::Shield::DRTAccess"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}