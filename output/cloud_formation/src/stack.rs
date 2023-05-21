/// The AWS::CloudFormation::Stack resource nests a stack as a resource in a top-level template.
///
/// You can add output values from a nested stack within the containing template. You use the GetAtt function with the nested stack's logical name and the name of the output value in the nested stack  in the format Outputs.NestedStackOutputName.
///
/// When you apply template changes to update a top-level stack, CloudFormation updates the top-level stack  and initiates an update to its nested stacks. CloudFormation updates the resources of modified nested  stacks, but doesn't update the resources of unmodified nested stacks. For more information, see CloudFormation   stack updates.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStack {
    ///
    /// The Amazon Simple Notification Service (Amazon SNS) topic ARNs to publish stack related events. You can find your   Amazon SNS topic ARNs using the Amazon SNS console or your Command Line Interface (CLI).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,

    ///
    /// The set value pairs that represent the parameters passed to CloudFormation when this nested stack is  created. Each parameter has a name corresponding to a parameter defined in the embedded template and a value  representing the value that you want to set for the parameter.
    ///
    /// NoteIf you use the Ref function to pass a parameter value to a nested stack, comma-delimited list   parameters must be of type String. In other words, you can't pass values that are of type   CommaDelimitedList to nested stacks.
    ///
    /// Conditional. Required if the nested stack requires input parameters.
    ///
    /// Whether an update causes interruptions depends on the resources that are being updated. An update never causes a  nested stack to be replaced.
    ///
    /// Required: Conditional
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,

    ///
    /// Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to the resources  created in the stack. A maximum number of 50 tags can be specified.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that's  located in an Amazon S3 bucket. For more information, see Template anatomy.
    ///
    /// Whether an update causes interruptions depends on the resources that are being updated. An update never causes a  nested stack to be replaced.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateURL")]
    pub template_url: String,

    ///
    /// The length of time, in minutes, that CloudFormation waits for the nested stack to reach the   CREATE_COMPLETE state. The default is no timeout. When CloudFormation detects that the nested  stack has reached the CREATE_COMPLETE state, it marks the nested stack resource as   CREATE_COMPLETE in the parent stack and resumes creating the parent stack. If the timeout period  expires before the nested stack reaches CREATE_COMPLETE, CloudFormation marks the nested stack  as failed and rolls back both the nested stack and parent stack.
    ///
    /// Updates aren't supported.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i64>,
}

impl cfn_resources::CfnResource for CfnStack {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::Stack"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.notification_arns {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'notification_arns'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.template_url;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'template_url'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.template_url;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'template_url'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.timeout_in_minutes {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'timeout_in_minutes'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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
