

/// This resource associates the specified application with the specified fleet. This is only supported for Elastic fleets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationFleetAssociation {


    /// The ARN of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationArn")]
    pub application_arn: String,


    /// The name of the fleet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "FleetName")]
    pub fleet_name: String,

}

impl cfn_resources::CfnResource for CfnApplicationFleetAssociation {
    fn type_string() -> &'static str {
        "AWS::AppStream::ApplicationFleetAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
