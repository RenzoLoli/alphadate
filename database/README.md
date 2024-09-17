# Database

## Info

Database initialization using _SurrealDB_

## Setup

- needed surreal-cli from official [Installation Guide](https://surrealdb.com/docs/surrealdb/installation/linux)

- validate version

      surreal version

- env properties (Not implemented yet)

      # authentication
      S_USER=
      S_PASS=

      # server binding
      S_PORT=
      S_HOST=

      # surreal properties
      S_METHOD=
      S_DBPATH=
      S_LOGLEVEL=

      # One liner
      S_USER= S_PASS= S_PORT= S_HOST= S_METHOD= S_DBPATH= S_LOGLEVEL=

- initialize engine

      chmod +x ./init_server
      ./init_server
