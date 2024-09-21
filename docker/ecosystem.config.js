module.exports = {
  apps: [
    {
      name: "frontend",
      script: "npm",
      args: "run start",
      cwd: "/usr/src/alphadate-frontend",
      interpreter: "/bin/sh",
      watch: ".",
    },
    {
      name: "database",
      script: "./init-server",
      cwd: "/usr/bin/alphadate-database",
      interpreter: "/bin/sh",
      watch: ".",
      env: {
        S_HOST: "127.0.0.1",
        S_PORT: "4700",
      },
    },
    {
      name: "backend",
      script: "./backend",
      cwd: "/usr/bin/alphadate-backend",
      interpreter: "/bin/sh",
      watch: ".",
      env: {
        DB_HOST: "127.0.0.1",
        DB_PORT: "4700",

        HOST: "127.0.0.1",
        PORT: "3000",
        CORS_ORIGINS: "http://127.0.0.1:4200",
      },
    },
  ],
};
