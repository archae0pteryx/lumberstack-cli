use crate::{
    cli::{app_paths::{AppPaths, TemplatePaths}, progress::AppProgress},
    system::{error::AppError, utils},
};

pub(super) fn create_schema(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("💎 Initializing Prisma");
    utils::copy_file(
        TemplatePaths::root(Some("prisma/schema.prisma.template")),
        AppPaths::api(Some("db/schema.prisma")),
    )
}

pub(super) fn copy_seed(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("💎 Creating seed");
    utils::copy_file(
        TemplatePaths::root(Some("prisma/seed.ts.template")),
        AppPaths::root(Some("scripts/seed.ts")),
    )
}
