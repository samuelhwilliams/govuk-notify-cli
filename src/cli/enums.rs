#[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
pub enum NotifyEnvironment {
    DEV,
    PREVIEW,
    STAGING,
    PRODUCTION,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display)]
pub enum DbTarget {
    PAAS,
    AWS,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display)]
pub enum DeploymentEnvironment {
    PREVIEW,
    STAGING,
    PRODUCTION,
}

// #[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
// pub enum AwsEnvironment {
//     DEV,
//     PREVIEW,
//     STAGING,
//     PRODUCTION,
// }
