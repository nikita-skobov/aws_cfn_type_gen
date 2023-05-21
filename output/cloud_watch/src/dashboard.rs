

/// The AWS::CloudWatch::Dashboard resource specifies an Amazon CloudWatch dashboard. A dashboard is a       customizable home page in the CloudWatch console that you can use to monitor your AWS resources in a single view.
///
/// All dashboards in your account are global, not region-specific.
#[derive(Default, serde::Serialize)]
pub struct CfnDashboard {


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

}
