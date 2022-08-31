use crate::{
    cli::{app_paths::*, logger::Logger, progress::*},
    system::{error::AppError, utils},
};

pub(super) fn copy_components(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("📄 Copying components");
    let source = TemplatePaths::web(Some("src/components"));
    let dest = AppPaths::web(Some("src/components"));

    Logger::loud_info(format!("{} -> {}", source, dest));

    utils::copy_directory(
        TemplatePaths::web(Some("src/components")),
        AppPaths::web(Some("src/components")),
    )
}

pub(super) fn copy_layouts() -> Result<(), AppError> {
    utils::copy_directory(
        TemplatePaths::web(Some("src/layouts")),
        AppPaths::web(Some("src/layouts")),
    )
}

pub(super) fn copy_home_page() -> Result<(), AppError> {
    utils::copy_directory(
        TemplatePaths::web(Some("src/pages")),
        AppPaths::web(Some("src/pages")),
    )
}

pub(super) fn copy_auth_unit_test() -> Result<(), AppError> {
    utils::copy_file(
        TemplatePaths::api(Some("src/directives/requireAuth/requireAuth.test.ts")),
        AppPaths::api(Some("src/directives/requireAuth/requireAuth.test.ts")),
    )
}
