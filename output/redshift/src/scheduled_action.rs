

/// Creates a scheduled action. A scheduled action contains a schedule and an Amazon Redshift API action.       For example, you can create a schedule of when to run the ResizeCluster API operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnScheduledAction {


    /// 
    /// If true, the schedule is enabled. If false, the scheduled action does not trigger.       For more information about state of the scheduled action, see AWS::Redshift::ScheduledAction.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,


    /// 
    /// The end time in UTC when the schedule is no longer active. After this time, the scheduled action does not trigger.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,


    /// 
    /// The IAM role to assume to run the scheduled action.       This IAM role must have permission to run the Amazon Redshift API operation in the scheduled action.       This IAM role must allow the Amazon Redshift scheduler (Principal scheduler.redshift.amazonaws.com) to assume permissions on your behalf.         For more information about the IAM role to use with the Amazon Redshift scheduler, see Using Identity-Based Policies for Amazon Redshift in the Amazon Redshift Cluster Management Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRole")]
    pub iam_role: Option<String>,


    /// 
    /// The schedule for a one-time (at format) or recurring (cron format) scheduled action.       Schedule invocations must be separated by at least one hour.
    /// 
    /// Format of at expressions is "at(yyyy-mm-ddThh:mm:ss)". For example, "at(2016-03-04T17:27:00)".
    /// 
    /// Format of cron expressions is "cron(Minutes Hours Day-of-month Month Day-of-week Year)".       For example, "cron(0 10 ? * MON *)". For more information, see       Cron Expressions       in the Amazon CloudWatch Events User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,


    /// 
    /// The description of the scheduled action.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduledActionDescription")]
    pub scheduled_action_description: Option<String>,


    /// 
    /// The name of the scheduled action.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,


    /// 
    /// The start time in UTC when the schedule is active. Before this time, the scheduled action does not trigger.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,


    /// 
    /// A JSON format string of the Amazon Redshift API operation with input parameters.
    /// 
    /// "{\"ResizeCluster\":{\"NodeType\":\"ds2.8xlarge\",\"ClusterIdentifier\":\"my-test-cluster\",\"NumberOfNodes\":3}}".
    /// 
    /// Required: No
    ///
    /// Type: ScheduledActionType
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAction")]
    pub target_action: Option<ScheduledActionType>,

}



impl cfn_resources::CfnResource for CfnScheduledAction {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::ScheduledAction"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.iam_role {

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'iam_role'. {} is greater than 2147483647", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.schedule {

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'schedule'. {} is greater than 2147483647", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.scheduled_action_description {

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'scheduled_action_description'. {} is greater than 2147483647", the_val.len()));
        }

        }
        
        let the_val = &self.scheduled_action_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'scheduled_action_name'. {} is greater than 2147483647", the_val.len()));
        }

        
        self.target_action.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a pause cluster operation. For example, a scheduled action to run the PauseCluster API operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PauseClusterMessage {


    /// 
    /// The identifier of the cluster to be paused.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}



impl cfn_resources::CfnResource for PauseClusterMessage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cluster_identifier;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", the_val.len()));
        }

        
        Ok(())
    }
}

/// Describes a resize cluster operation. For example, a scheduled action to run the ResizeCluster API operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResizeClusterMessage {


    /// 
    /// A boolean value indicating whether the resize operation is using the classic resize       process. If you don't provide this parameter or set the value to       false, the resize type is elastic.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classic")]
    pub classic: Option<bool>,


    /// 
    /// The unique identifier for the cluster to resize.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,


    /// 
    /// The new cluster type for the specified cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterType")]
    pub cluster_type: Option<String>,


    /// 
    /// The new node type for the nodes you are adding. If not specified, the cluster's current node type is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeType")]
    pub node_type: Option<String>,


    /// 
    /// The new number of nodes for the cluster. If not specified, the cluster's current number of nodes is used.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<i64>,

}



impl cfn_resources::CfnResource for ResizeClusterMessage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cluster_identifier;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", the_val.len()));
        }

        
        if let Some(the_val) = &self.cluster_type {

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'cluster_type'. {} is greater than 2147483647", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.node_type {

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'node_type'. {} is greater than 2147483647", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Describes a resume cluster operation. For example, a scheduled action to run the ResumeCluster API operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResumeClusterMessage {


    /// 
    /// The identifier of the cluster to be resumed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}



impl cfn_resources::CfnResource for ResumeClusterMessage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cluster_identifier;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", the_val.len()));
        }

        
        Ok(())
    }
}

/// The action type that specifies an Amazon Redshift API operation that is supported by the Amazon Redshift scheduler.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScheduledActionType {


    /// 
    /// An action that runs a PauseCluster API operation.
    /// 
    /// Required: No
    ///
    /// Type: PauseClusterMessage
    ///
    /// Update requires: No interruption
    #[serde(rename = "PauseCluster")]
    pub pause_cluster: Option<PauseClusterMessage>,


    /// 
    /// An action that runs a ResizeCluster API operation.
    /// 
    /// Required: No
    ///
    /// Type: ResizeClusterMessage
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResizeCluster")]
    pub resize_cluster: Option<ResizeClusterMessage>,


    /// 
    /// An action that runs a ResumeCluster API operation.
    /// 
    /// Required: No
    ///
    /// Type: ResumeClusterMessage
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResumeCluster")]
    pub resume_cluster: Option<ResumeClusterMessage>,

}



impl cfn_resources::CfnResource for ScheduledActionType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.pause_cluster.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.resize_cluster.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.resume_cluster.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}