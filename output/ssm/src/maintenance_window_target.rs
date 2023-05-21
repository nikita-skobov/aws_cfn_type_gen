/// The AWS::SSM::MaintenanceWindowTarget resource registers a target with a    maintenance window for AWS Systems Manager. For more information, see     RegisterTargetWithMaintenanceWindow in the AWS Systems Manager API     Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMaintenanceWindowTarget {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,

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
    pub resource_type: MaintenanceWindowTargetResourceTypeEnum,

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
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MaintenanceWindowTargetResourceTypeEnum {
    /// INSTANCE
    #[serde(rename = "INSTANCE")]
    Instance,

    /// RESOURCE_GROUP
    #[serde(rename = "RESOURCE_GROUP")]
    Resourcegroup,
}

impl Default for MaintenanceWindowTargetResourceTypeEnum {
    fn default() -> Self {
        MaintenanceWindowTargetResourceTypeEnum::Instance
    }
}

impl cfn_resources::CfnResource for CfnMaintenanceWindowTarget {
    fn type_string(&self) -> &'static str {
        "AWS::SSM::MaintenanceWindowTarget"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.owner_information {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'owner_information'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.owner_information {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'owner_information'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.targets;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'targets'. {} is greater than 5",
                the_val.len()
            ));
        }

        let the_val = &self.window_id;

        if the_val.len() > 20 as _ {
            return Err(format!(
                "Max validation failed on field 'window_id'. {} is greater than 20",
                the_val.len()
            ));
        }

        let the_val = &self.window_id;

        if the_val.len() < 20 as _ {
            return Err(format!(
                "Min validation failed on field 'window_id'. {} is less than 20",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The Targets property type specifies adding a target to a maintenance window    target in AWS Systems Manager.
///
/// Targets is a property of the AWS::SSM::MaintenanceWindowTarget resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for Targets {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if the_val.len() > 163 as _ {
            return Err(format!(
                "Max validation failed on field 'key'. {} is greater than 163",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'key'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.values;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'values'. {} is greater than 50",
                the_val.len()
            ));
        }

        Ok(())
    }
}
