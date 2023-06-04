/// Applies an Amazon S3 access policy to an Amazon S3 Multi-Region Access Point.
///
/// It is not possible to delete an access policy for a Multi-Region Access Point from the    CloudFormation template. When you attempt to delete the policy, CloudFormation updates the    policy using DeletionPolicy:Retain and UpdateReplacePolicy:Retain.    CloudFormation updates the policy to only allow access to the account that created the    bucket.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMultiRegionAccessPointPolicy {
    ///
    /// The name of the Multi-Region Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MrapName")]
    pub mrap_name: cfn_resources::StrVal,

    ///
    /// The access policy associated with the Multi-Region Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,

    #[serde(skip_serializing)]
    pub att_policy_status_is_public: CfnMultiRegionAccessPointPolicypolicystatusispublic,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMultiRegionAccessPointPolicypolicystatusispublic;
impl CfnMultiRegionAccessPointPolicypolicystatusispublic {
    pub fn att_name(&self) -> &'static str {
        r#"PolicyStatus.IsPublic"#
    }
}

impl cfn_resources::CfnResource for CfnMultiRegionAccessPointPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::S3::MultiRegionAccessPointPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The container element for a bucket's policy status.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PolicyStatus {
    ///
    /// The policy status for this bucket. TRUE indicates that this bucket is     public. FALSE indicates that the bucket is not public.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsPublic")]
    pub is_public: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PolicyStatus {
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
