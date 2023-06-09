/// The Virtual Deliverability Manager (VDM) attributes that apply to your Amazon SES account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian_attributes: Option<GuardianAttributes>,

    #[serde(skip_serializing)]
    pub att_vdm_attributes_resource_id: CfnVdmAttributesvdmattributesresourceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVdmAttributesvdmattributesresourceid;
impl CfnVdmAttributesvdmattributesresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"VdmAttributesResourceId"#
    }
}

impl cfn_resources::CfnResource for CfnVdmAttributes {
    fn type_string(&self) -> &'static str {
        "AWS::SES::VdmAttributes"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dashboard_attributes
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.guardian_attributes
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for your VDM configuration as applicable to the Dashboard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement_metrics: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DashboardAttributes {
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

/// Settings for your VDM configuration as applicable to the Guardian.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_shared_delivery: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GuardianAttributes {
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
