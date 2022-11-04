use crate::{
    app_config::AppConfig,
    lumberstack::Lumberstack,
    task_definitions::{
        heroku::create::Heroku,
        prisma::create::Prisma,
        redwood::{
            auth::RedwoodAuth, create::RedwoodApp, generate::RedwoodGenerate,
            playwright::Playwright, tailwind::Tailwind,
        },
        templates::{copy::TemplateCopy, github::GithubTemplates, tags::TaskTag},
    },
};
use anyhow::Result;

pub fn execute_tasks(app_config: &AppConfig) -> Result<()> {
    let mut app = Lumberstack::new();

    let clone_task = GithubTemplates::clone_templates(TaskTag::Clone, app_config);

    let create_task = RedwoodApp::create_redwood_app(TaskTag::Create, app_config);

    let playwright_task = Playwright::create_playwright(TaskTag::Playwright, app_config);

    let generate_pages_task = RedwoodGenerate::generate_pages(TaskTag::Generate, app_config);

    let auth_task = RedwoodAuth::generate_auth(TaskTag::Auth, app_config);

    let tailwind_task = Tailwind::create_tailwind(TaskTag::Tailwind, app_config);

    let template_copy_task = TemplateCopy::inject_templates(TaskTag::Templates, app_config);

    let prisma_task = Prisma::setup_prisma(TaskTag::Prisma, app_config);

    let heroku_task = Heroku::create_heroku(TaskTag::Heroku, app_config);

    app.queue(clone_task);
    app.queue(create_task);
    app.queue(tailwind_task);
    app.queue(playwright_task);
    app.queue(generate_pages_task);
    app.queue(auth_task);
    app.queue(template_copy_task);
    app.queue(prisma_task);
    app.queue(heroku_task);

    app.process();
    Ok(())
}
