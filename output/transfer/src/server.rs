

/// Instantiates an auto-scaling virtual server based on the selected file transfer protocol    in AWS. When you make updates to your file transfer protocol-enabled server or when you work    with users, use the service-generated ServerId property that is assigned to the    newly created server.
#[derive(Default, serde::Serialize)]
pub struct CfnServer {


    /// 
    /// Key-value pairs that can be used to group and search for servers.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the workflow ID for the workflow to assign and the execution role that's used for executing the workflow.
    /// 
    /// In addition to a workflow to execute when a file is uploaded completely, WorkflowDetails can also contain a   workflow ID (and execution role) for a workflow to execute on partial upload. A partial upload occurs when a file is open when   the session disconnects.
    /// 
    /// Required: No
    ///
    /// Type: WorkflowDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkflowDetails")]
    pub workflow_details: Option<WorkflowDetails>,


    /// 
    /// Required when IdentityProviderType is set to     AWS_DIRECTORY_SERVICE,         AWS_LAMBDA or API_GATEWAY. Accepts an array containing    all of the information required to use a directory in AWS_DIRECTORY_SERVICE or    invoke a customer-supplied authentication API, including the API Gateway URL. Not required    when IdentityProviderType is set to SERVICE_MANAGED.
    /// 
    /// Required: No
    ///
    /// Type: IdentityProviderDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityProviderDetails")]
    pub identity_provider_details: Option<IdentityProviderDetails>,


    /// 
    /// Specifies the name of the security policy that is attached to the server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Pattern: TransferSecurityPolicy-.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityPolicyName")]
    pub security_policy_name: Option<String>,


    /// 
    /// Specifies the domain of the storage system that is used for file transfers.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EFS | S3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// 
    /// The virtual private cloud (VPC) endpoint settings that are configured for your server.    When you host your endpoint within your VPC, you can make your endpoint accessible only to resources    within your VPC, or you can attach Elastic IP addresses and make your endpoint accessible to clients over    the internet. Your VPC's default security groups are automatically assigned to your    endpoint.
    /// 
    /// Required: No
    ///
    /// Type: EndpointDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointDetails")]
    pub endpoint_details: Option<EndpointDetails>,


    /// 
    /// The mode of authentication for a server. The default value is     SERVICE_MANAGED, which allows you to store and access user credentials within    the AWS Transfer Family service.
    /// 
    /// Use AWS_DIRECTORY_SERVICE to provide access to    Active Directory groups in AWS Directory Service for Microsoft Active Directory or Microsoft Active Directory in your    on-premises environment or in AWS using AD Connector. This option also requires you to    provide a Directory ID by using the IdentityProviderDetails parameter.
    /// 
    /// Use the API_GATEWAY value to integrate with an identity provider of your choosing. The    API_GATEWAY setting requires you to provide an Amazon API Gateway endpoint URL to call    for authentication by using the IdentityProviderDetails parameter.
    /// 
    /// Use the AWS_LAMBDA value to directly use an AWS Lambda function as your identity provider.    If you choose this value, you must specify the ARN for the Lambda function in the Function parameter    for the IdentityProviderDetails data type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: API_GATEWAY | AWS_DIRECTORY_SERVICE | AWS_LAMBDA | SERVICE_MANAGED
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityProviderType")]
    pub identity_provider_type: Option<String>,


    /// 
    /// Specifies a string to display when users connect to a server. This string is displayed before the user authenticates.   For example, the following banner displays details about using the system:
    /// 
    /// This system is for the use of authorized users only. Individuals using this computer system without authority,   or in excess of their authority, are subject to having all of their activities on this system monitored and recorded by   system personnel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\x09-\x0D\x20-\x7E]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreAuthenticationLoginBanner")]
    pub pre_authentication_login_banner: Option<String>,


    /// 
    /// Specifies a string to display when users connect to a server. This string is displayed after the user authenticates.
    /// 
    /// NoteThe SFTP protocol does not support post-authentication display banners.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\x09-\x0D\x20-\x7E]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostAuthenticationLoginBanner")]
    pub post_authentication_login_banner: Option<String>,


    /// 
    /// Specifies the file transfer protocol or protocols over which your file transfer protocol    client can connect to your server's endpoint. The available protocols are:
    /// 
    /// SFTP (Secure Shell (SSH) File Transfer Protocol): File transfer over      SSH                        FTPS (File Transfer Protocol Secure): File transfer with TLS      encryption                        FTP (File Transfer Protocol): Unencrypted file transfer                        AS2 (Applicability Statement 2): used for transporting structured business-to-business data
    /// 
    /// Note                                                           If you select FTPS, you must choose a certificate stored in AWS Certificate Manager (ACM)       which is used to identify your server when clients connect to it over       FTPS.                  If Protocol includes either FTP or FTPS, then the       EndpointType must be VPC and the       IdentityProviderType must be either AWS_DIRECTORY_SERVICE, AWS_LAMBDA, or API_GATEWAY.                  If Protocol includes FTP, then      AddressAllocationIds cannot be associated.                  If Protocol is set only to SFTP, the EndpointType       can be set to PUBLIC and the IdentityProviderType can be set any of the supported identity types:       SERVICE_MANAGED, AWS_DIRECTORY_SERVICE, AWS_LAMBDA, or API_GATEWAY.                  If Protocol includes AS2, then the        EndpointType must be VPC, and domain must be Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: List of Protocol
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<Protocol>>,


    /// 
    /// The type of endpoint that you want your server to use. You can choose to make your server's endpoint publicly accessible (PUBLIC)    or host it inside your VPC. With an endpoint that is hosted in a VPC, you can restrict access to your server and    resources only within your VPC or choose to make it internet facing by attaching Elastic IP addresses directly to it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PUBLIC | VPC | VPC_ENDPOINT
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Certificate Manager (ACM) certificate. Required    when Protocols is set to FTPS.
    /// 
    /// To request a new public certificate, see Request a public certificate    in the         AWS Certificate Manager User Guide.
    /// 
    /// To import an existing certificate into ACM, see Importing certificates into ACM    in the         AWS Certificate Manager User Guide.
    /// 
    /// To request a private certificate to use FTPS through private IP addresses, see Request a     private certificate in the         AWS Certificate Manager User    Guide.
    /// 
    /// Certificates with the following cryptographic algorithms and key sizes are    supported:
    /// 
    /// 2048-bit RSA (RSA_2048)               4096-bit RSA (RSA_4096)               Elliptic Prime Curve 256 bit (EC_prime256v1)               Elliptic Prime Curve 384 bit (EC_secp384r1)               Elliptic Prime Curve 521 bit (EC_secp521r1)
    /// 
    /// NoteThe certificate must be a valid SSL/TLS X.509 version 3 certificate with FQDN or IP     address specified and information about the issuer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1600
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that allows a server to turn    on Amazon CloudWatch logging for Amazon S3 or Amazon EFSevents. When set, you can view user activity in    your CloudWatch logs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingRole")]
    pub logging_role: Option<String>,


    /// 
    /// The protocol settings that are configured for your server.
    /// 
    /// To indicate passive mode (for FTP and FTPS protocols), use the PassiveIp parameter.      Enter a single dotted-quad IPv4 address, such as the external IP address of a firewall, router, or load balancer.                   To ignore the error that is generated when the client attempts to use the SETSTAT command on a file that you are     uploading to an Amazon S3 bucket, use the SetStatOption parameter. To have the AWS Transfer Family server ignore the     SETSTAT command and upload files without needing to make any changes to your SFTP client, set the value to     ENABLE_NO_OP. If you set the SetStatOption parameter to ENABLE_NO_OP, Transfer Family     generates a log entry to Amazon CloudWatch Logs, so that you can determine when the client is making a SETSTAT     call.               To determine whether your AWS Transfer Family server resumes recent, negotiated sessions through a unique session ID, use the     TlsSessionResumptionMode parameter.                        As2Transports indicates the transport method for the AS2 messages. Currently, only HTTP is supported.
    /// 
    /// Required: No
    ///
    /// Type: ProtocolDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProtocolDetails")]
    pub protocol_details: Option<ProtocolDetails>,

}


/// Specifies the file transfer protocol or protocols over which your file transfer protocol    client can connect to your server's endpoint. The available protocols are:
#[derive(Default, serde::Serialize)]
pub struct Protocol {

}


/// Required when IdentityProviderType is set to     AWS_DIRECTORY_SERVICE,         AWS_LAMBDA or API_GATEWAY. Accepts an array containing    all of the information required to use a directory in AWS_DIRECTORY_SERVICE or    invoke a customer-supplied authentication API, including the API Gateway URL. Not required    when IdentityProviderType is set to SERVICE_MANAGED.
#[derive(Default, serde::Serialize)]
pub struct IdentityProviderDetails {


    /// 
    /// Provides the location of the service endpoint used to authenticate users.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: Option<String>,


    /// 
    /// The identifier of the AWS Directory Service directory that you want to stop sharing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^d-[0-9a-f]{10}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectoryId")]
    pub directory_id: Option<String>,


    /// 
    /// The ARN for a Lambda function to use for the Identity provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 170
    ///
    /// Pattern: ^arn:[a-z-]+:lambda:.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Function")]
    pub function: Option<String>,


    /// 
    /// For SFTP-enabled servers, and for custom identity providers only, you    can specify whether to authenticate using a password, SSH key pair, or both.
    /// 
    /// PASSWORD - users must provide their password to connect.                        PUBLIC_KEY - users must provide their private key to connect.                        PUBLIC_KEY_OR_PASSWORD - users can authenticate with either their password or their key. This is the default value.                        PUBLIC_KEY_AND_PASSWORD - users must provide both their private key and their password to connect.      The server checks the key first, and then if the key is valid, the system prompts for a password.      If the private key provided does not match the public key that is stored, authentication fails.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PASSWORD | PUBLIC_KEY | PUBLIC_KEY_AND_PASSWORD | PUBLIC_KEY_OR_PASSWORD
    ///
    /// Update requires: No interruption
    #[serde(rename = "SftpAuthenticationMethods")]
    pub sftp_authentication_methods: Option<String>,


    /// 
    /// This parameter is only applicable if your IdentityProviderType is API_GATEWAY. Provides the type of InvocationRole used to authenticate the user    account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationRole")]
    pub invocation_role: Option<String>,

}


/// Indicates the transport method for the AS2 messages. Currently, only HTTP is supported.
#[derive(Default, serde::Serialize)]
pub struct As2Transport {

}


/// Specifies the workflow ID for the workflow to assign and the execution role that's used for executing the workflow.
///
/// In addition to a workflow to execute when a file is uploaded completely, WorkflowDetails can also contain a   workflow ID (and execution role) for a workflow to execute on partial upload. A partial upload occurs when a file is open when   the session disconnects.
#[derive(Default, serde::Serialize)]
pub struct WorkflowDetail {


    /// 
    /// A unique identifier for the workflow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^w-([a-z0-9]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkflowId")]
    pub workflow_id: String,


    /// 
    /// Includes the necessary permissions for S3, EFS, and Lambda operations that Transfer can    assume, so that all workflow steps can operate on the required resources
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,

}


/// The protocol settings that are configured for your server.
#[derive(Default, serde::Serialize)]
pub struct ProtocolDetails {


    /// 
    /// Use the SetStatOption to ignore the error that is generated when the client attempts to use SETSTAT on a file you are uploading to an S3 bucket.
    /// 
    /// Some SFTP file transfer clients can attempt to change the attributes of remote files, including timestamp and permissions, using commands, such as SETSTAT when uploading the file.     However, these commands are not compatible with object storage systems, such as Amazon S3. Due to this incompatibility, file uploads from these clients can result in errors even when     the file is otherwise successfully uploaded.
    /// 
    /// Set the value to ENABLE_NO_OP to have the Transfer Family server ignore the SETSTAT command, and upload files without needing to make any changes to your SFTP client.     While the SetStatOption       ENABLE_NO_OP setting ignores the error, it does generate a log entry in Amazon CloudWatch Logs, so you can determine when the client is making a SETSTAT call.
    /// 
    /// NoteIf you want to preserve the original timestamp for your file, and modify other file attributes using SETSTAT, you can use Amazon EFS as backend storage with Transfer Family.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT | ENABLE_NO_OP
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetStatOption")]
    pub set_stat_option: Option<String>,


    /// 
    /// List of As2Transport objects.
    /// 
    /// Required: No
    ///
    /// Type: List of As2Transport
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "As2Transports")]
    pub as2_transports: Option<Vec<As2Transport>>,


    /// 
    /// Indicates passive mode, for FTP and FTPS protocols.    Enter a single IPv4 address, such as the public IP address of a firewall, router, or load balancer.    For example:
    /// 
    /// aws transfer update-server --protocol-details PassiveIp=0.0.0.0
    /// 
    /// Replace 0.0.0.0 in the example above with the actual IP address you want to use.
    /// 
    /// Note     If you change the PassiveIp value, you must stop and then restart your Transfer Family server for the change to take effect. For details on using passive mode (PASV) in a NAT environment, see Configuring your FTPS server behind a firewall or NAT with AWS Transfer Family.
    /// 
    /// Special values
    /// 
    /// The AUTO and 0.0.0.0 are special values for the PassiveIp parameter. The value PassiveIp=AUTO    is assigned by default to FTP and FTPS type servers. In this case, the server automatically responds with one of the endpoint IPs within the PASV response.    PassiveIp=0.0.0.0 has a more unique application for its usage. For example, if you have a High Availability (HA) Network Load Balancer (NLB) environment,    where you have 3 subnets, you can only specify a single IP address using the PassiveIp parameter. This reduces the effectiveness of having High Availability.    In this case, you can specify PassiveIp=0.0.0.0. This tells the client to use the same IP address as the Control connection and utilize all AZs for their    connections. Note, however, that not all FTP clients support the PassiveIp=0.0.0.0 response. FileZilla and WinSCP do support it. If you are using other    clients, check to see if your client supports the PassiveIp=0.0.0.0 response.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "PassiveIp")]
    pub passive_ip: Option<String>,


    /// 
    /// A property used with Transfer Family servers that use the FTPS protocol. TLS Session Resumption provides a mechanism to resume or share a negotiated secret    key between the control and data connection for an FTPS session. TlsSessionResumptionMode determines whether or not the server resumes recent,    negotiated sessions through a unique session ID. This property is available during CreateServer and UpdateServer calls.    If a TlsSessionResumptionMode value is not specified during CreateServer, it is set to ENFORCED by default.
    /// 
    /// DISABLED: the server does not process TLS session resumption client requests and creates a new TLS session for each request.                         ENABLED: the server processes and accepts clients that are performing TLS session resumption.       The server doesn't reject client data connections that do not perform the TLS session resumption client processing.                        ENFORCED: the server processes and accepts clients that are performing TLS session resumption.       The server rejects client data connections that do not perform the TLS session resumption client processing.       Before you set the value to ENFORCED, test your clients.        NoteNot all FTPS clients perform TLS session resumption. So, if you choose to enforce        TLS session resumption, you prevent any connections from FTPS clients that don't perform        the protocol negotiation. To determine whether or not you can use the        ENFORCED value, you need to test your clients.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED | ENFORCED
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsSessionResumptionMode")]
    pub tls_session_resumption_mode: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// The virtual private cloud (VPC) endpoint settings that are configured for your server.    When you host your endpoint within your VPC, you can make your endpoint accessible only to resources    within your VPC, or you can attach Elastic IP addresses and make your endpoint accessible to clients over    the internet. Your VPC's default security groups are automatically assigned to your    endpoint.
#[derive(Default, serde::Serialize)]
pub struct EndpointDetails {


    /// 
    /// A list of subnet IDs that are required to host your server endpoint in your VPC.
    /// 
    /// NoteThis property can only be set when EndpointType is set to        VPC.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The VPC ID of the virtual private cloud in which the server's endpoint will be     hosted.
    /// 
    /// NoteThis property can only be set when EndpointType is set to        VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,


    /// 
    /// The ID of the VPC endpoint.
    /// 
    /// NoteThis property can only be set when EndpointType is set to        VPC_ENDPOINT.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 22
    ///
    /// Maximum: 22
    ///
    /// Pattern: ^vpce-[0-9a-f]{17}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,


    /// 
    /// A list of address allocation IDs that are required to attach an Elastic IP address to your    server's endpoint.
    /// 
    /// NoteThis property can only be set when EndpointType is set to VPC     and it is only valid in the UpdateServer API.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AddressAllocationIds")]
    pub address_allocation_ids: Option<Vec<String>>,


    /// 
    /// A list of security groups IDs that are available to attach to your server's    endpoint.
    /// 
    /// NoteThis property can only be set when EndpointType is set to     VPC.You can edit the SecurityGroupIds property in the UpdateServer API only if you are changing the EndpointType from      PUBLIC or VPC_ENDPOINT to VPC. To change security     groups associated with your server's VPC endpoint after creation, use the Amazon EC2      ModifyVpcEndpoint API.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}


/// Container for the WorkflowDetail data type.    It is used by actions that trigger a workflow to begin execution.
#[derive(Default, serde::Serialize)]
pub struct WorkflowDetails {


    /// 
    /// A trigger that starts a workflow if a file is only partially uploaded. You can attach a workflow to a server  that executes whenever there is a partial upload.
    /// 
    /// A partial upload occurs when a file is open when the session disconnects.
    /// 
    /// Required: No
    ///
    /// Type: List of WorkflowDetail
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPartialUpload")]
    pub on_partial_upload: Option<Vec<WorkflowDetail>>,


    /// 
    /// A trigger that starts a workflow: the workflow begins to execute after a file is uploaded.
    /// 
    /// To remove an associated workflow from a server, you can provide an empty OnUpload object, as in the following example.
    /// 
    /// aws transfer update-server --server-id s-01234567890abcdef --workflow-details '{"OnUpload":[]}'
    /// 
    /// Required: No
    ///
    /// Type: List of WorkflowDetail
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnUpload")]
    pub on_upload: Option<Vec<WorkflowDetail>>,

}