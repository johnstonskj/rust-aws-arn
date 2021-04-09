/*!
Provides enums that represent known values for ARN partition, region, and service identifiers.
*/

use crate::Identifier;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

impl Default for Partition {
    fn default() -> Self {
        Self::Aws
    }
}

///
/// A list of known partition identifiers from
/// [docs.aws](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Partition {
    /// Corresponds to the partition "aws": AWS regions
    Aws,

    /// Corresponds to the partition "aws-cn": AWS China regions
    AwsChina,

    /// Corresponds to the partition "aws-us-gov": AWS GovCloud (US) regions
    AwsUsGov,
}

///
/// A list of known region identifiers from
/// [docs.aws](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    /// Corresponds to the region "af-south-1": Africa (Cape Town)
    AfSouth1,

    /// Corresponds to the region "ap-east-1": Asia Pacific (Hong Kong)
    ApEast1,

    /// Corresponds to the region "ap-northeast-1": Asia Pacific (Tokyo)
    ApNortheast1,

    /// Corresponds to the region "ap-northeast-2": Asia Pacific (Seoul)
    ApNortheast2,

    /// Corresponds to the region "ap-northeast-3": Asia Pacific (Osaka)
    ApNortheast3,

    /// Corresponds to the region "ap-southeast-1": Asia Pacific (Singapore)
    ApSoutheast1,

    /// Corresponds to the region "ap-southeast-2": Asia Pacific (Sydney)
    ApSoutheast2,

    /// Corresponds to the region "ap-south-1": Asia Pacific (Mumbai)
    ApSouth1,

    /// Corresponds to the region "ca-central-1": Canada (Central)
    CaCentral1,

    /// Corresponds to the region "eu-central-1": Europe (Frankfurt)
    EuCentral1,

    /// Corresponds to the region "eu-north-1": Europe (Stockholm)
    EuNorth1,

    /// Corresponds to the region "eu-south-1": Europe (Milan)
    EuSouth1,

    /// Corresponds to the region "eu-west-1": Europe (Ireland)
    EuWest1,

    /// Corresponds to the region "eu-west-2": Europe (London)
    EuWest2,

    /// Corresponds to the region "eu-west-3": Europe (Paris)
    EuWest3,

    /// Corresponds to the region "me-south-1": Europe (Bahrain)
    MeSouth1,

    /// Corresponds to the region "sa-east-1": South America (São Paulo)
    SaEast1,

    /// Corresponds to the region "us-east-1": US East (N. Virginia)
    UsEast1,

    /// Corresponds to the region "us-east-2": US East (Ohio)
    UsEast2,

    /// Corresponds to the region "us-west-1": US West (N. California)
    UsWest1,

    /// Corresponds to the region "us-west-2": US West (Oregon)
    UsWest2,
}

///
/// A list of known service identifiers.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Service {
    /// Corresponds to the service "accessanalyzer"
    AccessAnalyzer,

    /// Corresponds to the service "acm"
    CertificateManager,

    /// Corresponds to the service "acm-pca"
    CertificateManagerPrivateCa,

    /// Corresponds to the service "alexaforbusiness"
    AlexaForBusiness,

    /// Corresponds to the service "amp"
    Prometheus,

    /// Corresponds to the service "amplify"
    Amplify,

    /// Corresponds to the service "amplifybackend"
    AmplifyBackend,

    /// Corresponds to the service "apigateway"
    ApiGateway,

    /// Corresponds to the service "apigatewaymanagementapi"
    ApiGatewayManagementApi,

    /// Corresponds to the service "apigatewayv2"
    ApiGatewayV2,

    /// Corresponds to the service "appconfig"
    AppConfig,

    /// Corresponds to the service "appflow"
    AppFlow,

    /// Corresponds to the service "appintegrations"
    AppIntegrations,

    /// Corresponds to the service "application-autoscaling"
    ApplicationAutoscaling,

    /// Corresponds to the service "application-insights"
    ApplicationInsights,

    /// Corresponds to the service "appmesh"
    AppMesh,

    /// Corresponds to the service "appstream"
    AppStream,

    /// Corresponds to the service "appsync"
    AppSync,

    /// Corresponds to the service "athena"
    Athena,

    /// Corresponds to the service "auditmanager"
    AuditManager,

    /// Corresponds to the service "autoscaling"
    AutoScaling,

    /// Corresponds to the service "autoscaling-plans"
    AutoScalingPlans,

    /// Corresponds to the service "backup"
    Backup,

    /// Corresponds to the service "batch"
    Batch,

    /// Corresponds to the service "braket"
    Braket,

    /// Corresponds to the service "budgets"
    Budgets,

    /// Corresponds to the service "ce"
    CostExplorer,

    /// Corresponds to the service "chime"
    Chime,

    /// Corresponds to the service "cloud9"
    Cloud9,

    /// Corresponds to the service "clouddirectory"
    CloudDirectory,

    /// Corresponds to the service "cloudformation"
    CloudFormation,

    /// Corresponds to the service "cloudhsm"
    CloudHsm,

    /// Corresponds to the service "cloudhsmv2"
    CloudHsmV2,

    /// Corresponds to the service "cloudsearch"
    CloudSearch,

    /// Corresponds to the service "cloudsearchdomain"
    CloudSearchDomain,

    /// Corresponds to the service "cloudtrail"
    CloudTrail,

    /// Corresponds to the service "cloudwatch"
    CloudWatch,

    /// Corresponds to the service "codeartifact"
    CodeArtifact,

    /// Corresponds to the service "codebuild"
    CodeBuild,

    /// Corresponds to the service "codecommit"
    CodeCommit,

    /// Corresponds to the service "codedeploy"
    CodeDeploy,

    /// Corresponds to the service "codeguru-reviewer"
    CodeGuruReviewer,

    /// Corresponds to the service "codeguruprofiler"
    CodeGuruProfiler,

    /// Corresponds to the service "codepipeline"
    CodePipeline,

    /// Corresponds to the service "codestar"
    CodeStar,

    /// Corresponds to the service "codestar-connections"
    CodeStarConnections,

    /// Corresponds to the service "codestar-notifications"
    CodeStarNotifications,

    /// Corresponds to the service "cognito-identity"
    CognitoIdentity,

    /// Corresponds to the service "cognito-idp"
    CognitoIdentityProvider,

    /// Corresponds to the service "cognito-sync"
    CognitoSync,

    /// Corresponds to the service "comprehend"
    Comprehend,

    /// Corresponds to the service "comprehendmedical"
    ComprehendMedical,

    /// Corresponds to the service "compute-optimizer"
    ComputeOptimizer,

    /// Corresponds to the service "config"
    Config,

    /// Corresponds to the service "connect"
    Connect,

    /// Corresponds to the service "connect-contact-lens"
    ConnectContactLens,

    /// Corresponds to the service "connectparticipant"
    ConnectParticipant,

    /// Corresponds to the service "cur"
    CostUsageReport,

    /// Corresponds to the service "customer-profiles"
    CustomerProfiles,

    /// Corresponds to the service "databrew"
    GlueDataBrew,

    /// Corresponds to the service "dataexchange"
    DataExchange,

    /// Corresponds to the service "datapipeline"
    DataPipeline,

    /// Corresponds to the service "datasync"
    DataSync,

    /// Corresponds to the service "dax"
    DynamoDbAccelerator,

    /// Corresponds to the service "detective"
    Detective,

    /// Corresponds to the service "devicefarm"
    DeviceFarm,

    /// Corresponds to the service "devops-guru"
    DevOpsGuru,

    /// Corresponds to the service "directconnect"
    DirectConnect,

    /// Corresponds to the service "discovery"
    Discovery,

    /// Corresponds to the service "dlm"
    DataLifecycleManager,

    /// Corresponds to the service "dms"
    DatabaseMigration,

    /// Corresponds to the service "docdb"
    DocumentDb,

    /// Corresponds to the service "dynamodb"
    DynamoDb,

    /// Corresponds to the service "dynamodbstreams"
    DynamoDbStreams,

    /// Corresponds to the service "ebs"
    ElasticBlockStore,

    /// Corresponds to the service "ec2"
    Ec2,

    /// Corresponds to the service "ec2-instance-connect"
    Ec2InstanceConnect,

    /// Corresponds to the service "ecr"
    Ec2ContainerRegistry,

    /// Corresponds to the service "ecr-public"
    Ec2containerRegistryPublic,

    /// Corresponds to the service "ecs"
    Ec2ContainerService,

    /// Corresponds to the service "efs"
    ElasticFileSystem,

    /// Corresponds to the service "eks"
    ElasticKubernetes,

    /// Corresponds to the service "elastic-inference"
    ElasticInference,

    /// Corresponds to the service "elasticache"
    Elasticache,

    /// Corresponds to the service "elasticbeanstalk"
    ElasticBeanstalk,

    /// Corresponds to the service "elastictranscoder"
    ElasticTranscoder,

    /// Corresponds to the service "elb"
    ElasticLoadBalancing,

    /// Corresponds to the service "elbv2"
    ElasticLoadBalancingV2,

    /// Corresponds to the service "emr"
    ElasticMapReduce,

    /// Corresponds to the service "emr-containers"
    ElasticMapReduceContainers,

    /// Corresponds to the service "es"
    ElasticsearchService,

    /// Corresponds to the service "events"
    EventBridge,

    /// Corresponds to the service "firehose"
    Firehose,

    /// Corresponds to the service "fis"
    FaultInjectionSimulator,

    /// Corresponds to the service "fms"
    FirewallManagementService,

    /// Corresponds to the service "forecast"
    ForecastService,

    /// Corresponds to the service "forecastquery"
    ForecastQueryService,

    /// Corresponds to the service "frauddetector"
    FraudDetector,

    /// Corresponds to the service "fsx"
    Fsx,

    /// Corresponds to the service "gamelift"
    GameLift,

    /// Corresponds to the service "glacier"
    Glacier,

    /// Corresponds to the service "globalaccelerator"
    GlobalAccelerator,

    /// Corresponds to the service "glue"
    Glue,

    /// Corresponds to the service "greengrass"
    Greengrass,

    /// Corresponds to the service "greengrassv2"
    GreengrassV2,

    /// Corresponds to the service "groundstation"
    GroundStation,

    /// Corresponds to the service "guardduty"
    GuardDuty,

    /// Corresponds to the service "health"
    Health,

    /// Corresponds to the service "healthlake"
    HealthLake,

    /// Corresponds to the service "honeycode"
    Honeycode,

    /// Corresponds to the service "iam"
    IdentityAccessManagement,

    /// Corresponds to the service "identitystore"
    IdentityStore,

    /// Corresponds to the service "imagebuilder"
    ImageBuilder,

    /// Corresponds to the service "importexport"
    ImportExport,

    /// Corresponds to the service "inspector"
    Inspector,

    /// Corresponds to the service "iot"
    IoT,

    /// Corresponds to the service "iot-data"
    IoTData,

    /// Corresponds to the service "iot-jobs-data"
    IoTJobsData,

    /// Corresponds to the service "iot1click-devices"
    IoT1clickDevices,

    /// Corresponds to the service "iot1click-projects"
    IoT1clickProjects,

    /// Corresponds to the service "iotanalytics"
    IoTAnalytics,

    /// Corresponds to the service "iotdeviceadvisor"
    IoTDeviceAdvisor,

    /// Corresponds to the service "iotevents"
    IoTEvents,

    /// Corresponds to the service "iotevents-data"
    IoTEventsData,

    /// Corresponds to the service "iotfleethub"
    IoTFleetHub,

    /// Corresponds to the service "iotsecuretunneling"
    IoTSecureTunneling,

    /// Corresponds to the service "iotsitewise"
    IoTSitewise,

    /// Corresponds to the service "iotthingsgraph"
    IoTThingsGraph,

    /// Corresponds to the service "iotwireless"
    IoTWireless,

    /// Corresponds to the service "ivs"
    InteractiveVideo,

    /// Corresponds to the service "kafka"
    Kafka,

    /// Corresponds to the service "kendra"
    Kendra,

    /// Corresponds to the service "kinesis"
    Kinesis,

    /// Corresponds to the service "kinesis-video-archived-media"
    KinesisVideoArchivedMedia,

    /// Corresponds to the service "kinesis-video-media"
    KinesisVideoMedia,

    /// Corresponds to the service "kinesis-video-signaling"
    KinesisVideoSignaling,

    /// Corresponds to the service "kinesisanalytics"
    KinesisAnalytics,

    /// Corresponds to the service "kinesisanalyticsv2"
    KinesisAnalyticsV2,

    /// Corresponds to the service "kinesisvideo"
    KinesisVideo,

    /// Corresponds to the service "kms"
    KeyManagement,

    /// Corresponds to the service "lakeformation"
    LakeFormation,

    /// Corresponds to the service "lambda"
    Lambda,

    /// Corresponds to the service "lex-models"
    LexModels,

    /// Corresponds to the service "lex-runtime"
    LexRuntime,

    /// Corresponds to the service "lexv2-models"
    LexV2Models,

    /// Corresponds to the service "lexv2-runtime"
    LexV2Runtime,

    /// Corresponds to the service "license-manager"
    LicenseManager,

    /// Corresponds to the service "lightsail"
    Lightsail,

    /// Corresponds to the service "location"
    Location,

    /// Corresponds to the service "logs"
    CloudWatchLogs,

    /// Corresponds to the service "lookoutequipment"
    LookoutEquipment,

    /// Corresponds to the service "lookoutmetrics"
    LookoutMetrics,

    /// Corresponds to the service "lookoutvision"
    LookoutVision,

    /// Corresponds to the service "machinelearning"
    MachineLearning,

    /// Corresponds to the service "macie"
    Macie,

    /// Corresponds to the service "macie2"
    Macie2,

    /// Corresponds to the service "managedblockchain"
    ManagedBlockchain,

    /// Corresponds to the service "marketplace-catalog"
    MarketplaceCatalog,

    /// Corresponds to the service "marketplace-entitlement"
    MarketplaceEntitlement,

    /// Corresponds to the service "marketplacecommerceanalytics"
    MarketplaceCommerceAnalytics,

    /// Corresponds to the service "mediaconnect"
    MediaConnect,

    /// Corresponds to the service "mediaconvert"
    MediaConvert,

    /// Corresponds to the service "medialive"
    MediaLive,

    /// Corresponds to the service "mediapackage"
    MediaPackage,

    /// Corresponds to the service "mediapackage-vod"
    MediaPackageVod,

    /// Corresponds to the service "mediastore"
    MediaStore,

    /// Corresponds to the service "mediastore-data"
    MediaStoreData,

    /// Corresponds to the service "mediatailor"
    MediaTailor,

    /// Corresponds to the service "meteringmarketplace"
    MarketplaceMetering,

    /// Corresponds to the service "mgh"
    MigrationHub,

    /// Corresponds to the service "mgn"
    ApplicationMigration,

    /// Corresponds to the service "migrationhub-config"
    MigrationHubConfig,

    /// Corresponds to the service "mobile"
    Mobile,

    /// Corresponds to the service "mq"
    Mq,

    /// Corresponds to the service "mturk"
    MechanicalTurk,

    /// Corresponds to the service "mwaa"
    ManagedWorkflowsForApacheAirflow,

    /// Corresponds to the service "neptune"
    Neptune,

    /// Corresponds to the service "network-firewall"
    NetworkFirewall,

    /// Corresponds to the service "networkmanager"
    NetworkManager,

    /// Corresponds to the service "opsworks"
    OpsWorks,

    /// Corresponds to the service "opsworkscm"
    OpsWorksCm,

    /// Corresponds to the service "organizations"
    Organizations,

    /// Corresponds to the service "outposts"
    Outposts,

    /// Corresponds to the service "personalize"
    Personalize,

    /// Corresponds to the service "personalize-events"
    PersonalizeEvents,

    /// Corresponds to the service "personalize-runtime"
    PersonalizeRuntime,

    /// Corresponds to the service "pi"
    PerformanceInsights,

    /// Corresponds to the service "pinpoint"
    Pinpoint,

    /// Corresponds to the service "pinpoint-email"
    PinpointEmail,

    /// Corresponds to the service "pinpoint-sms-voice"
    PinpointSmsVoice,

    /// Corresponds to the service "polly"
    Polly,

    /// Corresponds to the service "pricing"
    Pricing,

    /// Corresponds to the service "qldb"
    Qldb,

    /// Corresponds to the service "qldb-session"
    QldbSession,

    /// Corresponds to the service "quicksight"
    QuickSight,

    /// Corresponds to the service "ram"
    ResourceAccessManager,

    /// Corresponds to the service "rds"
    RelationalDatabaseService,

    /// Corresponds to the service "rds-data"
    RdsDataService,

    /// Corresponds to the service "redshift"
    Redshift,

    /// Corresponds to the service "redshift-data"
    RedshiftDataApiService,

    /// Corresponds to the service "rekognition"
    Rekognition,

    /// Corresponds to the service "resource-groups"
    ResourceGroups,

    /// Corresponds to the service "resourcegroupstaggingapi"
    ResourceGroupsTaggingApi,

    /// Corresponds to the service "robomaker"
    RoboMaker,

    /// Corresponds to the service "route53"
    Route53,

    /// Corresponds to the service "route53domains"
    Route53Domains,

    /// Corresponds to the service "route53resolver"
    Route53Resolver,

    /// Corresponds to the service "s3"
    S3,

    /// Corresponds to the service "s3control"
    S3Control,

    /// Corresponds to the service "s3outposts"
    S3Outposts,

    /// Corresponds to the service "sagemaker"
    SageMaker,

    /// Corresponds to the service "sagemaker-a2i-runtime"
    AugmentedAiRuntime,

    /// Corresponds to the service "sagemaker-edge"
    SagemakerEdgeManager,

    /// Corresponds to the service "sagemaker-featurestore-runtime"
    SageMakerFeatureStoreRuntime,

    /// Corresponds to the service "sagemaker-runtime"
    SageMakerRuntime,

    /// Corresponds to the service "savingsplans"
    SavingsPlans,

    /// Corresponds to the service "schemas"
    EventBridgeSchemaRegistry,

    /// Corresponds to the service "sdb"
    SimpleDb,

    /// Corresponds to the service "secretsmanager"
    SecretsManager,

    /// Corresponds to the service "securityhub"
    SecurityHub,

    /// Corresponds to the service "serverlessrepo"
    ServerlessApplicationRepository,

    /// Corresponds to the service "service-quotas"
    ServiceQuotas,

    /// Corresponds to the service "servicecatalog"
    ServiceCatalog,

    /// Corresponds to the service "servicecatalog-appregistry"
    ServiceCatalogAppRegistry,

    /// Corresponds to the service "servicediscovery"
    ServiceDiscovery,

    /// Corresponds to the service "ses"
    SimpleEmail,

    /// Corresponds to the service "sesv2"
    SimpleEmailV2,

    /// Corresponds to the service "shield"
    Shield,

    /// Corresponds to the service "signer"
    Signer,

    /// Corresponds to the service "sms"
    ServerMigration,

    /// Corresponds to the service "snowball"
    Snowball,

    /// Corresponds to the service "sns"
    SimpleNotification,

    /// Corresponds to the service "sqs"
    SimpleQueue,

    /// Corresponds to the service "ssm"
    SimpleSystemsManager,

    /// Corresponds to the service "sso"
    SingleSignOn,

    /// Corresponds to the service "sso-admin"
    SingleSignOnAdmin,

    /// Corresponds to the service "sso-oidc"
    SingleSignOnOpenIdConnect,

    /// Corresponds to the service "stepfunctions"
    StepFunctions,

    /// Corresponds to the service "storagegateway"
    StorageGateway,

    /// Corresponds to the service "sts"
    SecurityToken,

    /// Corresponds to the service "support"
    Support,

    /// Corresponds to the service "swf"
    SimpleWorkflow,

    /// Corresponds to the service "synthetics"
    CloudWatchSynthetics,

    /// Corresponds to the service "textract"
    Textract,

    /// Corresponds to the service "timestream-query"
    TimestreamQuery,

    /// Corresponds to the service "timestream-write"
    TimestreamWrite,

    /// Corresponds to the service "transcribe"
    Transcribe,

    /// Corresponds to the service "transfer"
    Transfer,

    /// Corresponds to the service "translate"
    Translate,

    /// Corresponds to the service "waf"
    WebApplicationFirewall,

    /// Corresponds to the service "waf-regional"
    WebApplicationFirewallRegional,

    /// Corresponds to the service "wafv2"
    WebApplicationFirewallV2,

    /// Corresponds to the service "wellarchitected"
    WellArchitected,

    /// Corresponds to the service "workdocs"
    WorkDocs,

    /// Corresponds to the service "worklink"
    WorkLink,

    /// Corresponds to the service "workmail"
    WorkMail,

    /// Corresponds to the service "workmailmessageflow"
    WorkMailMessageFlow,

    /// Corresponds to the service "workspaces"
    WorkSpaces,

    /// Corresponds to the service "xray"
    XRay,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Partition> for Identifier {
    fn from(p: Partition) -> Self {
        match p {
            Partition::Aws => Identifier::new_unchecked("aws"),
            Partition::AwsChina => Identifier::new_unchecked("aws-cn"),
            Partition::AwsUsGov => Identifier::new_unchecked("aws-us-gov"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Region> for Identifier {
    fn from(r: Region) -> Self {
        match r {
            Region::AfSouth1 => Identifier::new_unchecked("af-south-1"),
            Region::ApEast1 => Identifier::new_unchecked("ap-east-1"),
            Region::ApNortheast1 => Identifier::new_unchecked("ap-northeast-1"),
            Region::ApNortheast2 => Identifier::new_unchecked("ap-northeast-2"),
            Region::ApNortheast3 => Identifier::new_unchecked("ap-northeast-3"),
            Region::ApSoutheast1 => Identifier::new_unchecked("ap-southeast-1"),
            Region::ApSoutheast2 => Identifier::new_unchecked("ap-southeast-2"),
            Region::ApSouth1 => Identifier::new_unchecked("ap-south-1"),
            Region::CaCentral1 => Identifier::new_unchecked("ca-central-1"),
            Region::EuCentral1 => Identifier::new_unchecked("eu-central-1"),
            Region::EuNorth1 => Identifier::new_unchecked("eu-north-1"),
            Region::EuSouth1 => Identifier::new_unchecked("eu-south-1"),
            Region::EuWest1 => Identifier::new_unchecked("eu-west-1"),
            Region::EuWest2 => Identifier::new_unchecked("eu-west-2"),
            Region::EuWest3 => Identifier::new_unchecked("eu-west-3"),
            Region::MeSouth1 => Identifier::new_unchecked("me-south-1"),
            Region::SaEast1 => Identifier::new_unchecked("sa-east-1"),
            Region::UsEast1 => Identifier::new_unchecked("us-east-1"),
            Region::UsEast2 => Identifier::new_unchecked("us-east-2"),
            Region::UsWest1 => Identifier::new_unchecked("us-west-1"),
            Region::UsWest2 => Identifier::new_unchecked("us-west-2"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Service> for Identifier {
    fn from(s: Service) -> Self {
        match s {
            Service::AccessAnalyzer => Identifier::new_unchecked("accessanalyzer"),
            Service::CertificateManager => Identifier::new_unchecked("acm"),
            Service::CertificateManagerPrivateCa => Identifier::new_unchecked("acm-pca"),
            Service::AlexaForBusiness => Identifier::new_unchecked("alexaforbusiness"),
            Service::Prometheus => Identifier::new_unchecked("amp"),
            Service::Amplify => Identifier::new_unchecked("amplify"),
            Service::AmplifyBackend => Identifier::new_unchecked("amplifybackend"),
            Service::ApiGateway => Identifier::new_unchecked("apigateway"),
            Service::ApiGatewayManagementApi => {
                Identifier::new_unchecked("apigatewaymanagementapi")
            }
            Service::ApiGatewayV2 => Identifier::new_unchecked("apigatewayv2"),
            Service::AppConfig => Identifier::new_unchecked("appconfig"),
            Service::AppFlow => Identifier::new_unchecked("appflow"),
            Service::AppIntegrations => Identifier::new_unchecked("appintegrations"),
            Service::ApplicationAutoscaling => Identifier::new_unchecked("application-autoscaling"),
            Service::ApplicationInsights => Identifier::new_unchecked("application-insights"),
            Service::AppMesh => Identifier::new_unchecked("appmesh"),
            Service::AppStream => Identifier::new_unchecked("appstream"),
            Service::AppSync => Identifier::new_unchecked("appsync"),
            Service::Athena => Identifier::new_unchecked("athena"),
            Service::AuditManager => Identifier::new_unchecked("auditmanager"),
            Service::AutoScaling => Identifier::new_unchecked("autoscaling"),
            Service::AutoScalingPlans => Identifier::new_unchecked("autoscaling-plans"),
            Service::Backup => Identifier::new_unchecked("backup"),
            Service::Batch => Identifier::new_unchecked("batch"),
            Service::Braket => Identifier::new_unchecked("braket"),
            Service::Budgets => Identifier::new_unchecked("budgets"),
            Service::CostExplorer => Identifier::new_unchecked("ce"),
            Service::Chime => Identifier::new_unchecked("chime"),
            Service::Cloud9 => Identifier::new_unchecked("cloud9"),
            Service::CloudDirectory => Identifier::new_unchecked("clouddirectory"),
            Service::CloudFormation => Identifier::new_unchecked("cloudformation"),
            Service::CloudHsm => Identifier::new_unchecked("cloudhsm"),
            Service::CloudHsmV2 => Identifier::new_unchecked("cloudhsmv2"),
            Service::CloudSearch => Identifier::new_unchecked("cloudsearch"),
            Service::CloudSearchDomain => Identifier::new_unchecked("cloudsearchdomain"),
            Service::CloudTrail => Identifier::new_unchecked("cloudtrail"),
            Service::CloudWatch => Identifier::new_unchecked("cloudwatch"),
            Service::CodeArtifact => Identifier::new_unchecked("codeartifact"),
            Service::CodeBuild => Identifier::new_unchecked("codebuild"),
            Service::CodeCommit => Identifier::new_unchecked("codecommit"),
            Service::CodeDeploy => Identifier::new_unchecked("codedeploy"),
            Service::CodeGuruReviewer => Identifier::new_unchecked("codeguru-reviewer"),
            Service::CodeGuruProfiler => Identifier::new_unchecked("codeguruprofiler"),
            Service::CodePipeline => Identifier::new_unchecked("codepipeline"),
            Service::CodeStar => Identifier::new_unchecked("codestar"),
            Service::CodeStarConnections => Identifier::new_unchecked("codestar-connections"),
            Service::CodeStarNotifications => Identifier::new_unchecked("codestar-notifications"),
            Service::CognitoIdentity => Identifier::new_unchecked("cognito-identity"),
            Service::CognitoIdentityProvider => Identifier::new_unchecked("cognito-idp"),
            Service::CognitoSync => Identifier::new_unchecked("cognito-sync"),
            Service::Comprehend => Identifier::new_unchecked("comprehend"),
            Service::ComprehendMedical => Identifier::new_unchecked("comprehendmedical"),
            Service::ComputeOptimizer => Identifier::new_unchecked("compute-optimizer"),
            Service::Config => Identifier::new_unchecked("config"),
            Service::Connect => Identifier::new_unchecked("connect"),
            Service::ConnectContactLens => Identifier::new_unchecked("connect-contact-lens"),
            Service::ConnectParticipant => Identifier::new_unchecked("connectparticipant"),
            Service::CostUsageReport => Identifier::new_unchecked("cur"),
            Service::CustomerProfiles => Identifier::new_unchecked("customer-profiles"),
            Service::GlueDataBrew => Identifier::new_unchecked("databrew"),
            Service::DataExchange => Identifier::new_unchecked("dataexchange"),
            Service::DataPipeline => Identifier::new_unchecked("datapipeline"),
            Service::DataSync => Identifier::new_unchecked("datasync"),
            Service::DynamoDbAccelerator => Identifier::new_unchecked("dax"),
            Service::Detective => Identifier::new_unchecked("detective"),
            Service::DeviceFarm => Identifier::new_unchecked("devicefarm"),
            Service::DevOpsGuru => Identifier::new_unchecked("devops-guru"),
            Service::DirectConnect => Identifier::new_unchecked("directconnect"),
            Service::Discovery => Identifier::new_unchecked("discovery"),
            Service::DataLifecycleManager => Identifier::new_unchecked("dlm"),
            Service::DatabaseMigration => Identifier::new_unchecked("dms"),
            Service::DocumentDb => Identifier::new_unchecked("docdb"),
            Service::DynamoDb => Identifier::new_unchecked("dynamodb"),
            Service::DynamoDbStreams => Identifier::new_unchecked("dynamodbstreams"),
            Service::ElasticBlockStore => Identifier::new_unchecked("ebs"),
            Service::Ec2 => Identifier::new_unchecked("ec2"),
            Service::Ec2InstanceConnect => Identifier::new_unchecked("ec2-instance-connect"),
            Service::Ec2ContainerRegistry => Identifier::new_unchecked("ecr"),
            Service::Ec2containerRegistryPublic => Identifier::new_unchecked("ecr-public"),
            Service::Ec2ContainerService => Identifier::new_unchecked("ecs"),
            Service::ElasticFileSystem => Identifier::new_unchecked("efs"),
            Service::ElasticKubernetes => Identifier::new_unchecked("eks"),
            Service::ElasticInference => Identifier::new_unchecked("elastic-inference"),
            Service::Elasticache => Identifier::new_unchecked("elasticache"),
            Service::ElasticBeanstalk => Identifier::new_unchecked("elasticbeanstalk"),
            Service::ElasticTranscoder => Identifier::new_unchecked("elastictranscoder"),
            Service::ElasticLoadBalancing => Identifier::new_unchecked("elb"),
            Service::ElasticLoadBalancingV2 => Identifier::new_unchecked("elbv2"),
            Service::ElasticMapReduce => Identifier::new_unchecked("emr"),
            Service::ElasticMapReduceContainers => Identifier::new_unchecked("emr-containers"),
            Service::ElasticsearchService => Identifier::new_unchecked("es"),
            Service::EventBridge => Identifier::new_unchecked("events"),
            Service::Firehose => Identifier::new_unchecked("firehose"),
            Service::FaultInjectionSimulator => Identifier::new_unchecked("fis"),
            Service::FirewallManagementService => Identifier::new_unchecked("fms"),
            Service::ForecastService => Identifier::new_unchecked("forecast"),
            Service::ForecastQueryService => Identifier::new_unchecked("forecastquery"),
            Service::FraudDetector => Identifier::new_unchecked("frauddetector"),
            Service::Fsx => Identifier::new_unchecked("fsx"),
            Service::GameLift => Identifier::new_unchecked("gamelift"),
            Service::Glacier => Identifier::new_unchecked("glacier"),
            Service::GlobalAccelerator => Identifier::new_unchecked("globalaccelerator"),
            Service::Glue => Identifier::new_unchecked("glue"),
            Service::Greengrass => Identifier::new_unchecked("greengrass"),
            Service::GreengrassV2 => Identifier::new_unchecked("greengrassv2"),
            Service::GroundStation => Identifier::new_unchecked("groundstation"),
            Service::GuardDuty => Identifier::new_unchecked("guardduty"),
            Service::Health => Identifier::new_unchecked("health"),
            Service::HealthLake => Identifier::new_unchecked("healthlake"),
            Service::Honeycode => Identifier::new_unchecked("honeycode"),
            Service::IdentityAccessManagement => Identifier::new_unchecked("iam"),
            Service::IdentityStore => Identifier::new_unchecked("identitystore"),
            Service::ImageBuilder => Identifier::new_unchecked("imagebuilder"),
            Service::ImportExport => Identifier::new_unchecked("importexport"),
            Service::Inspector => Identifier::new_unchecked("inspector"),
            Service::IoT => Identifier::new_unchecked("iot"),
            Service::IoTData => Identifier::new_unchecked("iot-data"),
            Service::IoTJobsData => Identifier::new_unchecked("iot-jobs-data"),
            Service::IoT1clickDevices => Identifier::new_unchecked("iot1click-devices"),
            Service::IoT1clickProjects => Identifier::new_unchecked("iot1click-projects"),
            Service::IoTAnalytics => Identifier::new_unchecked("iotanalytics"),
            Service::IoTDeviceAdvisor => Identifier::new_unchecked("iotdeviceadvisor"),
            Service::IoTEvents => Identifier::new_unchecked("iotevents"),
            Service::IoTEventsData => Identifier::new_unchecked("iotevents-data"),
            Service::IoTFleetHub => Identifier::new_unchecked("iotfleethub"),
            Service::IoTSecureTunneling => Identifier::new_unchecked("iotsecuretunneling"),
            Service::IoTSitewise => Identifier::new_unchecked("iotsitewise"),
            Service::IoTThingsGraph => Identifier::new_unchecked("iotthingsgraph"),
            Service::IoTWireless => Identifier::new_unchecked("iotwireless"),
            Service::InteractiveVideo => Identifier::new_unchecked("ivs"),
            Service::Kafka => Identifier::new_unchecked("kafka"),
            Service::Kendra => Identifier::new_unchecked("kendra"),
            Service::Kinesis => Identifier::new_unchecked("kinesis"),
            Service::KinesisVideoArchivedMedia => {
                Identifier::new_unchecked("kinesis-video-archived-media")
            }
            Service::KinesisVideoMedia => Identifier::new_unchecked("kinesis-video-media"),
            Service::KinesisVideoSignaling => Identifier::new_unchecked("kinesis-video-signaling"),
            Service::KinesisAnalytics => Identifier::new_unchecked("kinesisanalytics"),
            Service::KinesisAnalyticsV2 => Identifier::new_unchecked("kinesisanalyticsv2"),
            Service::KinesisVideo => Identifier::new_unchecked("kinesisvideo"),
            Service::KeyManagement => Identifier::new_unchecked("kms"),
            Service::LakeFormation => Identifier::new_unchecked("lakeformation"),
            Service::Lambda => Identifier::new_unchecked("lambda"),
            Service::LexModels => Identifier::new_unchecked("lex-models"),
            Service::LexRuntime => Identifier::new_unchecked("lex-runtime"),
            Service::LexV2Models => Identifier::new_unchecked("lexv2-models"),
            Service::LexV2Runtime => Identifier::new_unchecked("lexv2-runtime"),
            Service::LicenseManager => Identifier::new_unchecked("license-manager"),
            Service::Lightsail => Identifier::new_unchecked("lightsail"),
            Service::Location => Identifier::new_unchecked("location"),
            Service::CloudWatchLogs => Identifier::new_unchecked("logs"),
            Service::LookoutEquipment => Identifier::new_unchecked("lookoutequipment"),
            Service::LookoutMetrics => Identifier::new_unchecked("lookoutmetrics"),
            Service::LookoutVision => Identifier::new_unchecked("lookoutvision"),
            Service::MachineLearning => Identifier::new_unchecked("machinelearning"),
            Service::Macie => Identifier::new_unchecked("macie"),
            Service::Macie2 => Identifier::new_unchecked("macie2"),
            Service::ManagedBlockchain => Identifier::new_unchecked("managedblockchain"),
            Service::MarketplaceCatalog => Identifier::new_unchecked("marketplace-catalog"),
            Service::MarketplaceEntitlement => Identifier::new_unchecked("marketplace-entitlement"),
            Service::MarketplaceCommerceAnalytics => {
                Identifier::new_unchecked("marketplacecommerceanalytics")
            }
            Service::MediaConnect => Identifier::new_unchecked("mediaconnect"),
            Service::MediaConvert => Identifier::new_unchecked("mediaconvert"),
            Service::MediaLive => Identifier::new_unchecked("medialive"),
            Service::MediaPackage => Identifier::new_unchecked("mediapackage"),
            Service::MediaPackageVod => Identifier::new_unchecked("mediapackage-vod"),
            Service::MediaStore => Identifier::new_unchecked("mediastore"),
            Service::MediaStoreData => Identifier::new_unchecked("mediastore-data"),
            Service::MediaTailor => Identifier::new_unchecked("mediatailor"),
            Service::MarketplaceMetering => Identifier::new_unchecked("meteringmarketplace"),
            Service::MigrationHub => Identifier::new_unchecked("mgh"),
            Service::ApplicationMigration => Identifier::new_unchecked("mgn"),
            Service::MigrationHubConfig => Identifier::new_unchecked("migrationhub-config"),
            Service::Mobile => Identifier::new_unchecked("mobile"),
            Service::Mq => Identifier::new_unchecked("mq"),
            Service::MechanicalTurk => Identifier::new_unchecked("mturk"),
            Service::ManagedWorkflowsForApacheAirflow => Identifier::new_unchecked("mwaa"),
            Service::Neptune => Identifier::new_unchecked("neptune"),
            Service::NetworkFirewall => Identifier::new_unchecked("network-firewall"),
            Service::NetworkManager => Identifier::new_unchecked("networkmanager"),
            Service::OpsWorks => Identifier::new_unchecked("opsworks"),
            Service::OpsWorksCm => Identifier::new_unchecked("opsworkscm"),
            Service::Organizations => Identifier::new_unchecked("organizations"),
            Service::Outposts => Identifier::new_unchecked("outposts"),
            Service::Personalize => Identifier::new_unchecked("personalize"),
            Service::PersonalizeEvents => Identifier::new_unchecked("personalize-events"),
            Service::PersonalizeRuntime => Identifier::new_unchecked("personalize-runtime"),
            Service::PerformanceInsights => Identifier::new_unchecked("pi"),
            Service::Pinpoint => Identifier::new_unchecked("pinpoint"),
            Service::PinpointEmail => Identifier::new_unchecked("pinpoint-email"),
            Service::PinpointSmsVoice => Identifier::new_unchecked("pinpoint-sms-voice"),
            Service::Polly => Identifier::new_unchecked("polly"),
            Service::Pricing => Identifier::new_unchecked("pricing"),
            Service::Qldb => Identifier::new_unchecked("qldb"),
            Service::QldbSession => Identifier::new_unchecked("qldb-session"),
            Service::QuickSight => Identifier::new_unchecked("quicksight"),
            Service::ResourceAccessManager => Identifier::new_unchecked("ram"),
            Service::RelationalDatabaseService => Identifier::new_unchecked("rds"),
            Service::RdsDataService => Identifier::new_unchecked("rds-data"),
            Service::Redshift => Identifier::new_unchecked("redshift"),
            Service::RedshiftDataApiService => Identifier::new_unchecked("redshift-data"),
            Service::Rekognition => Identifier::new_unchecked("rekognition"),
            Service::ResourceGroups => Identifier::new_unchecked("resource-groups"),
            Service::ResourceGroupsTaggingApi => {
                Identifier::new_unchecked("resourcegroupstaggingapi")
            }
            Service::RoboMaker => Identifier::new_unchecked("robomaker"),
            Service::Route53 => Identifier::new_unchecked("route53"),
            Service::Route53Domains => Identifier::new_unchecked("route53domains"),
            Service::Route53Resolver => Identifier::new_unchecked("route53resolver"),
            Service::S3 => Identifier::new_unchecked("s3"),
            Service::S3Control => Identifier::new_unchecked("s3control"),
            Service::S3Outposts => Identifier::new_unchecked("s3outposts"),
            Service::SageMaker => Identifier::new_unchecked("sagemaker"),
            Service::AugmentedAiRuntime => Identifier::new_unchecked("sagemaker-a2i-runtime"),
            Service::SagemakerEdgeManager => Identifier::new_unchecked("sagemaker-edge"),
            Service::SageMakerFeatureStoreRuntime => {
                Identifier::new_unchecked("sagemaker-featurestore-runtime")
            }
            Service::SageMakerRuntime => Identifier::new_unchecked("sagemaker-runtime"),
            Service::SavingsPlans => Identifier::new_unchecked("savingsplans"),
            Service::EventBridgeSchemaRegistry => Identifier::new_unchecked("schemas"),
            Service::SimpleDb => Identifier::new_unchecked("sdb"),
            Service::SecretsManager => Identifier::new_unchecked("secretsmanager"),
            Service::SecurityHub => Identifier::new_unchecked("securityhub"),
            Service::ServerlessApplicationRepository => Identifier::new_unchecked("serverlessrepo"),
            Service::ServiceQuotas => Identifier::new_unchecked("service-quotas"),
            Service::ServiceCatalog => Identifier::new_unchecked("servicecatalog"),
            Service::ServiceCatalogAppRegistry => {
                Identifier::new_unchecked("servicecatalog-appregistry")
            }
            Service::ServiceDiscovery => Identifier::new_unchecked("servicediscovery"),
            Service::SimpleEmail => Identifier::new_unchecked("ses"),
            Service::SimpleEmailV2 => Identifier::new_unchecked("sesv2"),
            Service::Shield => Identifier::new_unchecked("shield"),
            Service::Signer => Identifier::new_unchecked("signer"),
            Service::ServerMigration => Identifier::new_unchecked("sms"),
            Service::Snowball => Identifier::new_unchecked("snowball"),
            Service::SimpleNotification => Identifier::new_unchecked("sns"),
            Service::SimpleQueue => Identifier::new_unchecked("sqs"),
            Service::SimpleSystemsManager => Identifier::new_unchecked("ssm"),
            Service::SingleSignOn => Identifier::new_unchecked("sso"),
            Service::SingleSignOnAdmin => Identifier::new_unchecked("sso-admin"),
            Service::SingleSignOnOpenIdConnect => Identifier::new_unchecked("sso-oidc"),
            Service::StepFunctions => Identifier::new_unchecked("stepfunctions"),
            Service::StorageGateway => Identifier::new_unchecked("storagegateway"),
            Service::SecurityToken => Identifier::new_unchecked("sts"),
            Service::Support => Identifier::new_unchecked("support"),
            Service::SimpleWorkflow => Identifier::new_unchecked("swf"),
            Service::CloudWatchSynthetics => Identifier::new_unchecked("synthetics"),
            Service::Textract => Identifier::new_unchecked("textract"),
            Service::TimestreamQuery => Identifier::new_unchecked("timestream-query"),
            Service::TimestreamWrite => Identifier::new_unchecked("timestream-write"),
            Service::Transcribe => Identifier::new_unchecked("transcribe"),
            Service::Transfer => Identifier::new_unchecked("transfer"),
            Service::Translate => Identifier::new_unchecked("translate"),
            Service::WebApplicationFirewall => Identifier::new_unchecked("waf"),
            Service::WebApplicationFirewallRegional => Identifier::new_unchecked("waf-regional"),
            Service::WebApplicationFirewallV2 => Identifier::new_unchecked("wafv2"),
            Service::WellArchitected => Identifier::new_unchecked("wellarchitected"),
            Service::WorkDocs => Identifier::new_unchecked("workdocs"),
            Service::WorkLink => Identifier::new_unchecked("worklink"),
            Service::WorkMail => Identifier::new_unchecked("workmail"),
            Service::WorkMailMessageFlow => Identifier::new_unchecked("workmailmessageflow"),
            Service::WorkSpaces => Identifier::new_unchecked("workspaces"),
            Service::XRay => Identifier::new_unchecked("xray"),
        }
    }
}
