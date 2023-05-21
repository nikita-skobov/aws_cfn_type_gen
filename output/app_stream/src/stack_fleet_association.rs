/// The AWS::AppStream::StackFleetAssociation resource associates the specified fleet with the specified stack for Amazon AppStream 2.0.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStackFleetAssociation {
    ///
    /// The name of the fleet.
    ///
    /// To associate a fleet with a stack, you must specify a dependency on the fleet resource. For more information, see DependsOn Attribute.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "FleetName")]
    pub fleet_name: String,

    ///
    /// The name of the stack.
    ///
    /// To associate a fleet with a stack, you must specify a dependency on the stack resource. For more information, see DependsOn Attribute.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

impl cfn_resources::CfnResource for CfnStackFleetAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::StackFleetAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.fleet_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'fleet_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.stack_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'stack_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
