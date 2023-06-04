/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDRTAccess {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-logbucketlist
    #[serde(rename = "LogBucketList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_bucket_list: Option<Vec<String>>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-rolearn
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_account_id: CfnDRTAccessaccountid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDRTAccessaccountid;
impl CfnDRTAccessaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"AccountId"#
    }
}

impl cfn_resources::CfnResource for CfnDRTAccess {
    fn type_string(&self) -> &'static str {
        "AWS::Shield::DRTAccess"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
