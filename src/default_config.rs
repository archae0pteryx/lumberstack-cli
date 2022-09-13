use log::info;

pub fn generate_default_config() -> &'static str {
    info!("⚙️ Using default configuration");
    return r#"{
  "app_name": "myapp",
  "keep_templates": false,
  "builder": [
    {
      "tag": "download",
      "feedback": "fetching templates",
      "commands": [
        {
          "command": "curl -L https://github.com/codingzeal/lumberstack-templates/archive/main.tar.gz -o /tmp/lumberstack-templates.tar.gz"
        },
        {
          "command": "tar -C /tmp -xzf /tmp/lumberstack-templates.tar.gz",
          "feedback": "Extracting templates"
        },
        {
          "command": "mv /tmp/lumberstack-templates-main/templates ./templates"
        },
        {
          "command": "rm -rf /tmp/lumberstack-templates-main && rm -f /tmp/lumberstack-templates.tar.gz",
          "feedback": "Cleaning up"
        }
      ]
    },
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
      "feedback": "Copying default templates",
      "templates": [
        {
          "source": "templates",
          "dest": "{{app_name}}"
        }
      ]
    },
    {
      "tag": "prisma",
      "feedback": "Setting up prisma",
      "commands": [
        {
          "command": "docker compose -f docker-compose.yml stop db",
          "context": "{{app_name}}"
        },
        {
          "feedback": "Starting docker postgres",
          "command": "docker compose -f docker-compose.yml up db -d",
          "context": "{{app_name}}"
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
          "source": "templates/api/src/directives/requireAuth/requireAuth.test.ts",
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
    },
    {
      "tag": "heroku",
      "feedback": "Setting up heroku",
      "commands": [
        {
          "command": "yarn add pm2",
          "context": "{{app_name}}"
        },
        {
          "command": "echo 'TODO Setup Heroku!'"
        }
      ]
    }
  ]
}
"#;
}
