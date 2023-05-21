

/// The AWS::CloudWatch::Dashboard resource specifies an Amazon CloudWatch dashboard. A dashboard is a       customizable home page in the CloudWatch console that you can use to monitor your AWS resources in a single view.
///
/// All dashboards in your account are global, not region-specific.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDashboard {


    /// 
    /// The detailed information about the dashboard in JSON format, including the widgets to include and their location 			on the dashboard. This parameter is required.
    /// 
    /// For more information about the syntax, 		    	see Dashboard Body Structure and Syntax.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardBody")]
    pub dashboard_body: String,


    /// 
    /// The name of the dashboard. The name must be between 1 and 255 characters. If you do not specify a name, one will be generated automatically.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DashboardName")]
    pub dashboard_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnDashboard {
    fn type_string() -> &'static str {
        "AWS::CloudWatch::Dashboard"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
