

/// The AWS::RoboMaker::RobotApplication resource creates an AWS     RoboMaker robot application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRobotApplication {


    /// 
    /// The current revision id.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,


    /// 
    /// The environment of the robot application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<String>,


    /// 
    /// The name of the robot application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The robot software suite used by the robot application.
    /// 
    /// Required: Yes
    ///
    /// Type: RobotSoftwareSuite
    ///
    /// Update requires: No interruption
    #[serde(rename = "RobotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,


    /// 
    /// The sources of the robot application.
    /// 
    /// Required: No
    ///
    /// Type: List of SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<SourceConfig>>,


    /// 
    /// A map that contains tag keys and tag values that are attached to the robot     application.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}



impl cfn_resources::CfnResource for CfnRobotApplication {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::RobotApplication"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.name {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.robot_software_suite.validate()?;

        Ok(())
    }
}

/// Information about a robot software suite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RobotSoftwareSuite {


    /// 
    /// The name of the robot software suite. General is the only supported value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: General | ROS | ROS2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: RobotSoftwareSuiteNameEnum,


    /// 
    /// The version of the robot software suite. Not applicable for General software suite.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Dashing | Foxy | Kinetic | Melodic
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<RobotSoftwareSuiteVersionEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RobotSoftwareSuiteNameEnum {

    /// General
    #[serde(rename = "General")]
    General,

    /// ROS
    #[serde(rename = "ROS")]
    Ros,

    /// ROS2
    #[serde(rename = "ROS2")]
    Ros2,

}

impl Default for RobotSoftwareSuiteNameEnum {
    fn default() -> Self {
        RobotSoftwareSuiteNameEnum::General
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RobotSoftwareSuiteVersionEnum {

    /// Dashing
    #[serde(rename = "Dashing")]
    Dashing,

    /// Foxy
    #[serde(rename = "Foxy")]
    Foxy,

    /// Kinetic
    #[serde(rename = "Kinetic")]
    Kinetic,

    /// Melodic
    #[serde(rename = "Melodic")]
    Melodic,

}

impl Default for RobotSoftwareSuiteVersionEnum {
    fn default() -> Self {
        RobotSoftwareSuiteVersionEnum::Dashing
    }
}


impl cfn_resources::CfnResource for RobotSoftwareSuite {
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

/// Information about a source configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceConfig {


    /// 
    /// The target processor architecture for the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARM64 | ARMHF | X86_64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architecture")]
    pub architecture: SourceConfigArchitectureEnum,


    /// 
    /// The Amazon S3 bucket name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][a-z0-9.\-]*[a-z0-9]
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The s3 object key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceConfigArchitectureEnum {

    /// ARM64
    #[serde(rename = "ARM64")]
    Arm64,

    /// ARMHF
    #[serde(rename = "ARMHF")]
    Armhf,

    /// X86_64
    #[serde(rename = "X86_64")]
    X8664,

}

impl Default for SourceConfigArchitectureEnum {
    fn default() -> Self {
        SourceConfigArchitectureEnum::Arm64
    }
}


impl cfn_resources::CfnResource for SourceConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.s3_bucket;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 's3_bucket'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.s3_bucket;

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 's3_bucket'. {} is less than 3", the_val.len()));
        }

        
        let the_val = &self.s3_key;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 's3_key'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.s3_key;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 's3_key'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}