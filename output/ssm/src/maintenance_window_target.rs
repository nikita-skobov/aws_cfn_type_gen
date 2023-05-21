

/// The AWS::SSM::MaintenanceWindowTarget resource registers a target with a    maintenance window for AWS Systems Manager. For more information, see     RegisterTargetWithMaintenanceWindow in the AWS Systems Manager API     Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnMaintenanceWindowTarget {


    /// 
    /// The name for the maintenance window target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The targets to register with the maintenance window. In other words, the instances to run    commands on when the maintenance window runs.
    /// 
    /// You must specify targets by using the WindowTargetIds parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Targets
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Vec<Targets>,


    /// 
    /// A description for the target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The type of target that is being registered with the maintenance window.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: INSTANCE | RESOURCE_GROUP
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: String,


    /// 
    /// The ID of the maintenance window to register the target with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^mw-[0-9a-f]{17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "WindowId")]
    pub window_id: String,


    /// 
    /// A user-provided value that will be included in any Amazon CloudWatch Events events that are  raised while running tasks for these targets in this maintenance window.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "OwnerInformation")]
    pub owner_information: Option<String>,

}


/// The Targets property type specifies adding a target to a maintenance window    target in AWS Systems Manager.
///
/// Targets is a property of the AWS::SSM::MaintenanceWindowTarget resource.
#[derive(Default, serde::Serialize)]
pub struct Targets {


    /// 
    /// User-defined criteria for sending commands that target managed nodes that meet the  criteria.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 163
    ///
    /// Pattern: ^[\p{L}\p{Z}\p{N}_.:/=\-@]*$|resource-groups:ResourceTypeFilters|resource-groups:Name
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// User-defined criteria that maps to Key. For example, if you specified   tag:ServerRole, you could specify value:WebServer to run a command on  instances that include EC2 tags of ServerRole,WebServer.
    /// 
    /// Depending on the type of target, the maximum number of values for a key might be lower than  the global maximum of 50.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}
