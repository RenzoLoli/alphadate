# Database

## Info

Database initialization using _SurrealDB_

## Setup

- needed surreal-cli from official [Installation Guide](https://surrealdb.com/docs/surrealdb/installation/linux)

- validate version

      surreal version

- env properties (Not implemented yet)

      # authentication
      USER=
      PASS=

      # server binding
      PORT=
      HOST=

      # surreal properties
      METHOD=
      LOGLEVEL=

      # One liner
      USER= PASS= PORT= HOST= METHOD= LOGLEVEL=

- initialize engine

      chmod +x ./init_server
      ./init_server
