use super::enums::NotifyEnvironment;

pub fn get_account_name_from_environment(
    environment: NotifyEnvironment,
    admin: bool,
) -> &'static str {
    match admin {
        true => match environment {
            NotifyEnvironment::DEV => return "notify-tools-admin",
            NotifyEnvironment::PREVIEW => return "notify-preview-admin",
            NotifyEnvironment::STAGING => return "notify-staging-admin",
            NotifyEnvironment::PRODUCTION => return "notify-production-admin",
        },
        false => match environment {
            NotifyEnvironment::DEV => return "notify-tools",
            NotifyEnvironment::PREVIEW => return "notify-preview",
            NotifyEnvironment::STAGING => return "notify-staging",
            NotifyEnvironment::PRODUCTION => return "notify-production",
        },
    };
}
