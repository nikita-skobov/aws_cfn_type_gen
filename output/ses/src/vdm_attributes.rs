

/// The Virtual Deliverability Manager (VDM) attributes that apply to your Amazon SES account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVdmAttributes {


    /// 
    /// Specifies additional settings for your VDM configuration as applicable to the       Dashboard.
    /// 
    /// Required: No
    ///
    /// Type: DashboardAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardAttributes")]
    pub dashboard_attributes: Option<DashboardAttributes>,


    /// 
    /// Specifies additional settings for your VDM configuration as applicable to the       Guardian.
    /// 
    /// Required: No
    ///
    /// Type: GuardianAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "GuardianAttributes")]
    pub guardian_attributes: Option<GuardianAttributes>,

}



impl cfn_resources::CfnResource for CfnVdmAttributes {
    fn type_string() -> &'static str {
        "AWS::SES::VdmAttributes"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.dashboard_attributes.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.guardian_attributes.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for your VDM configuration as applicable to the Dashboard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DashboardAttributes {


    /// 
    /// Specifies the status of your VDM engagement metrics collection. Can be one of the       following:
    /// 
    /// ENABLED – Amazon SES enables engagement metrics for           your account.               DISABLED – Amazon SES disables engagement metrics for           your account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngagementMetrics")]
    pub engagement_metrics: Option<String>,

}



impl cfn_resources::CfnResource for DashboardAttributes {
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

/// Settings for your VDM configuration as applicable to the Guardian.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GuardianAttributes {


    /// 
    /// Specifies the status of your VDM optimized shared delivery. Can be one of the       following:
    /// 
    /// ENABLED – Amazon SES enables optimized shared delivery           for your account.               DISABLED – Amazon SES disables optimized shared           delivery for your account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptimizedSharedDelivery")]
    pub optimized_shared_delivery: Option<String>,

}



impl cfn_resources::CfnResource for GuardianAttributes {
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