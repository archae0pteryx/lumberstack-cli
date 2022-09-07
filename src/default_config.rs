pub fn generate_default_config() -> &'static str {
    return r#"
{
  "app_name": "myapp",
  "builder": [
    {
      "tag": "redwood",
      "feedback": "Creating Redwood App",
      "commands": [
        {
          "command": "yarn create redwood-app {{app_name}} --typescript --overwrite",
          "context": "."
        },
        {
          "command": "yarn rw generate page home / --force",
          "context": "{{app_name}}"
        }
      ]
    },
    {
      "tag": "templates",
      "feedback": "Copying redwood templates",
      "templates": [
        {
          "source": "templates/.gitignore.template",
          "dest": "{{app_name}}/.gitignore"
        },
        {
          "source": "templates/.prettierignore.template",
          "dest": "{{app_name}}/.prettierignore"
        },
        {
          "source": "templates/web/src/pages/HomePage",
          "dest": "{{app_name}}/web/src/pages/"
        },
        {
          "source": "templates/web/src/layouts",
          "dest": "{{app_name}}/web/src/"
        },
        {
          "source": "templates/web/src/components",
          "dest": "{{app_name}}/web/src/"
        },
        {
          "source": "templates/.github",
          "dest": "{{app_name}}/"
        },
        {
          "source": "templates/CONTRIBUTING.md.template",
          "dest": "{{app_name}}/CONTRIBUTING.md"
        },
        {
          "source": "templates/LICENSE.md.template",
          "dest": "{{app_name}}/LICENSE.md"
        },
        {
          "source": "templates/README.md.template",
          "dest": "{{app_name}}/README.md"
        },
        {
          "source": "templates/web/src/scaffold.css.template",
          "dest": "{{app_name}}/web/src/scaffold.css"
        },
        {
          "source": "templates/heroku",
          "dest": "{{app_name}}/"
        }
      ]
    },
    {
      "tag": "prisma",
      "feedback": "Setting up prisma",
      "templates": [
        {
          "source": "templates/.env.template",
          "dest": "{{app_name}}/.env"
        },
        {
          "source": "templates/docker-compose.template",
          "dest": "{{app_name}}/docker-compose.yml"
        },
        {
          "source": "templates/api/db/schema.prisma.template",
          "dest": "{{app_name}}/api/db/schema.prisma"
        },
        {
          "source": "templates/api/db/seed.ts.template",
          "dest": "{{app_name}}/api/db/seed.ts"
        }
      ],
      "commands": [
        {
          "command": "docker compose -f {{app_name}}/docker-compose.yml stop db"
        },
        {
          "feedback": "Starting docker postgres",
          "command": "docker compose -f {{app_name}}/docker-compose.yml up db -d"
        },
        {
          "feedback": "Creating migration",
          "command": "yarn rw prisma migrate dev --name init",
          "context": "{{app_name}}"
        }
      ]
    },
    {
      "tag": "auth",
      "feedback": "Setting up redwood auth",
      "templates": [
        {
          "source": "templates/api/src/directives/requireAuth/requireAuth.test.ts.template",
          "dest": "{{app_name}}/api/src/directives/requireAuth/requireAuth.test.ts"
        }
      ],
      "commands": [
        {
          "command": "yarn rw setup auth dbAuth --force",
          "context": "{{app_name}}"
        },
        {
          "command": "yarn rw generate dbAuth --force",
          "context": "{{app_name}}"
        }
      ]
    },
    {
      "tag": "tailwind",
      "feedback": "Setting up tailwind",
      "commands": [
        {
          "command": "yarn rw setup ui tailwind",
          "context": "{{app_name}}"
        }
      ]
    },
    {
      "tag": "playwright",
      "feedback": "Setting up playwright",
      "commands": [
        {
          "command": "yarn create playwright --quiet --lang=ts",
          "context": "{{app_name}}/web/"
        }
      ]
    }
  ]
}
"#;
}
