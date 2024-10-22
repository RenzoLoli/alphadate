module.exports = {
  apps: [
    {
      name: "frontend",
      script: "./entry.frontend.sh",
      cwd: "/app/frontend",
      interpreter: "/bin/sh",
      out_file: "/data/pm2/frontend.out.log",
      error_file: "/data/pm2/frontend.err.log",
      log_file: "/data/pm2/frontend.log",
    },
    {
      name: "database",
      script: "./init_server",
      cwd: "/app/database",
      interpreter: "/bin/sh",
      out_file: "/data/pm2/database.out.log",
      error_file: "/data/pm2/database.err.log",
      log_file: "/data/pm2/database.log",
    },
    {
      name: "backend",
      script: "./entry.backend.sh",
      cwd: "/app/backend",
      interpreter: "/bin/sh",
      out_file: "/data/pm2/backend.out.log",
      error_file: "/data/pm2/backend.err.log",
      log_file: "/data/pm2/backend.log",
    },
  ],
};
