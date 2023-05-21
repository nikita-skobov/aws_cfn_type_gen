

/// The AWS::AppStream::StackFleetAssociation resource associates the specified fleet with the specified stack for Amazon AppStream 2.0.
#[derive(Default, serde::Serialize)]
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
