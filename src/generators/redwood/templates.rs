use log::info;

use crate::{
    cli::{app_paths::*, progress::*},
    system::{error::AppError, utils},
};

pub(super) fn copy_components(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("📄 Copying components");
    let source = TemplatePaths::web(Some("src/components"));
    let dest = AppPaths::web(Some("src/components"));
    utils::copy_dir(source, dest)?;
    Ok(())
}

pub(super) fn copy_layouts() -> Result<(), AppError> {
    utils::copy_dir(
        TemplatePaths::web(Some("src/layouts")),
        AppPaths::web(Some("src/layouts")),
    )?;
    Ok(())
}

pub(super) fn copy_home_page() -> Result<(), AppError> {
    utils::copy_dir(
        TemplatePaths::web(Some("src/pages")),
        AppPaths::web(Some("src/pages")),
    )?;
    Ok(())
}

pub(super) fn copy_auth_unit_test() -> Result<(), AppError> {
    utils::copy_file(
        TemplatePaths::api(Some("src/directives/requireAuth/requireAuth.test.ts")),
        AppPaths::api(Some("src/directives/requireAuth/requireAuth.test.ts")),
    )
}
