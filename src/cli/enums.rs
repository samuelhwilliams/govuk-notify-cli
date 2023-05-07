#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display, Copy)]
pub enum NotifyEnvironment {
    DEV,
    PREVIEW,
    STAGING,
    PRODUCTION,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::Display, Copy)]
pub enum InfrastructureTarget {
    PAAS,
    AWS,
}
