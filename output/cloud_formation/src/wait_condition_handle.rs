

/// The AWS::CloudFormation::WaitConditionHandle type has no properties. When you reference the   WaitConditionHandle resource by using the Ref function, AWS CloudFormation returns a  presigned URL. You pass this URL to applications or scripts that are running on your Amazon EC2 instances to  send signals to that URL. An associated AWS::CloudFormation::WaitCondition resource checks the URL for  the required number of success signals or for a failure signal.
#[derive(Default, serde::Serialize)]
pub struct CfnWaitConditionHandle {

}
