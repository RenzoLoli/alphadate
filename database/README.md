# Database

<!--toc:start-->

- [Database](#database)
  - [Info](#info)
  - [Setup](#setup)
  <!--toc:end-->

## Info

Database initialization using _SurrealDB_

## Setup

- needed surreal-cli from official [Installation Guide](https://surrealdb.com/docs/surrealdb/installation/linux)

- validate version

      surreal version

- env properties (Not implemented yet)

      # authentication
      DATABASE_USER=
      DATABASE_PASS=

      # server binding
      DATABASE_PORT=
      DATABASE_HOST=

      # surreal properties
      DATABASE_METHOD=
      DATABASE_DBPATH=
      DATABASE_LOGLEVEL=

      # One liner
      DATABASE_USER=root DATABASE_PASS=root DATABASE_PORT=4700 DATABASE_HOST=0.0.0.0 DATABASE_METHOD= DATABASE_DBPATH=/var/lib/surrealdb DATABASE_LOGLEVEL=debug surreal version

- initialize engine

      chmod +x ./init_server
      ./init_server
