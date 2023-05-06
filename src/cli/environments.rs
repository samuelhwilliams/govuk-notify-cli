#[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
pub enum NotifyEnvironment {
    DEV,
    PREVIEW,
    STAGING,
    PRODUCTION,
}

// #[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
// pub enum InfrastructureEnvironment {
//     PREVIEW,
//     STAGING,
//     PRODUCTION,
// }

// #[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
// pub enum AwsEnvironment {
//     DEV,
//     PREVIEW,
//     STAGING,
//     PRODUCTION,
// }
