/// The AWS::Batch::JobQueue resource specifies the parameters for an AWS Batch job queue  definition. For more information, see Job   Queues in the AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnJobQueue {
    ///
    /// The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler  uses this parameter to determine which compute environment runs a specific job. Compute environments must be in  the VALID state before you can associate them with a job queue. You can associate up to three compute  environments with a job queue. All of the compute environments must be either EC2 (EC2 or   SPOT) or Fargate (FARGATE or FARGATE_SPOT); EC2 and Fargate compute  environments can't be mixed.
    ///
    /// NoteAll compute environments that are associated with a job queue must share the same architecture. AWS Batch doesn't   support mixing compute environment architecture types in a single job queue.
    ///
    /// Required: Yes
    ///
    /// Type: List of ComputeEnvironmentOrder
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,

    ///
    /// The name of the job queue. It can be up to 128 letters long. It can contain uppercase and lowercase letters,  numbers, hyphens (-), and underscores (_).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobQueueName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub job_queue_name: Option<cfn_resources::StrVal>,

    ///
    /// The priority of the job queue. Job queues with a higher priority (or a higher integer value for the   priority parameter) are evaluated first when associated with the same compute environment. Priority is  determined in descending order. For example, a job queue with a priority value of 10 is given scheduling  preference over a job queue with a priority value of 1. All of the compute environments must be either  EC2 (EC2 or SPOT) or Fargate (FARGATE or FARGATE_SPOT); EC2 and  Fargate compute environments can't be mixed.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,

    ///
    /// The Amazon Resource Name (ARN) of the scheduling policy. The format is   aws:Partition:batch:Region:Account:scheduling-policy/Name       .  For example,   aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchedulingPolicyArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub scheduling_policy_arn: Option<cfn_resources::StrVal>,

    ///
    /// The state of the job queue. If the job queue state is ENABLED, it is able to accept jobs. If the  job queue state is DISABLED, new jobs can't be added to the queue, but jobs already in the queue can  finish.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub state: Option<JobQueueStateEnum>,

    ///
    /// The tags that are applied to the job queue. For more information, see Tagging your AWS Batch resources in           AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_job_queue_arn: CfnJobQueuejobqueuearn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum JobQueueStateEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for JobQueueStateEnum {
    fn default() -> Self {
        JobQueueStateEnum::Disabled
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnJobQueuejobqueuearn;
impl CfnJobQueuejobqueuearn {
    pub fn att_name(&self) -> &'static str {
        r#"JobQueueArn"#
    }
}

impl cfn_resources::CfnResource for CfnJobQueue {
    fn type_string(&self) -> &'static str {
        "AWS::Batch::JobQueue"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The order that compute environments are tried in for job placement within a queue. Compute  environments are tried in ascending order. For example, if two compute environments are  associated with a job queue, the compute environment with a lower order integer value is tried  for job placement first. Compute environments must be in the VALID state before you  can associate them with a job queue. All of the compute environments must be either EC2   (EC2 or SPOT) or Fargate (FARGATE or   FARGATE_SPOT); EC2 and Fargate compute environments can't be mixed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComputeEnvironmentOrder {
    ///
    /// The Amazon Resource Name (ARN) of the compute environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeEnvironment")]
    pub compute_environment: cfn_resources::StrVal,

    ///
    /// The order of the compute environment. Compute environments are tried in ascending order. For  example, if two compute environments are associated with a job queue, the compute environment  with a lower order integer value is tried for job placement first.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Order")]
    pub order: i64,
}

impl cfn_resources::CfnResource for ComputeEnvironmentOrder {
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
