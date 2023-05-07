#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display)]
pub enum NotifyEnvironment {
    DEV,
    PREVIEW,
    STAGING,
    PRODUCTION,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display)]
pub enum InfrastructureTarget {
    PAAS,
    AWS,
}
