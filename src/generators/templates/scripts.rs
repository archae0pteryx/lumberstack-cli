use crate::{
    cli::app_paths::AppPaths,
    system::{utils::{load_json, merge_json}, error::AppError},
};

use serde_json::Value;
use std::fs::File;

pub struct Scripts;

impl Scripts {
    pub fn copy_template() -> Result<(), AppError> {
        let destination = AppPaths::root(Some("package.json"));
        let mut package_json = load_json(&destination).unwrap();
        let script = Self::custom_template();
        merge_json(&mut package_json, &script);
        let new_package = File::create(&destination)?;
        serde_json::to_writer_pretty(new_package, &package_json)?;
        Ok(())
    }

    fn custom_template() -> Value {
        return serde_json::json!({
            "scripts": {
                "start": "node index.js",
                "build": "yarn rw build",
                "lint": "yarn eslint && yarn pretty",
                "eslint": "npx eslint .",
                "pretty": "npx prettier -c .",
                "dev": "yarn db:daemon && yarn rw dev",
                "test": "yarn rw test --no-watch",
                "test:watch": "yarn rw test",
                "test:e2e": "npx playwright test -c web/playwright.config.ts",
                "testdb": "yarn testdb:down && docker compose up testdb",
                "testdb:daemon": "yarn testdb:down && docker compose up -d testdb",
                "testdb:down": "docker compose rm -sf testdb",
                "db": "yarn db:down && docker compose up db",
                "db:daemon": "yarn db:down && docker compose up -d db",
                "db:down": "docker compose down",
                "db:migrate": "yarn rw prisma migrate",
                "db:migrate:dev": "yarn db:migrate dev",
                "db:migrate:deploy": "yarn db:migrate deploy",
                "db:seed": "yarn rw prisma db seed",
                "db:setup": "yarn db:migrate:dev && yarn db:seed",
                "db:reset": "yarn rw prisma migrate reset",
                "db:deploy": "yarn db:migrate:deploy"
            }
        });
    }
}
