# Alphadate REST API Documentation

<!--toc:start-->
- [Alphadate REST API Documentation](#alphadate-rest-api-documentation)
  - [Open Endpoints](#open-endpoints)
    - [Authentication Related](#authentication-related)
    - [Playground Endpoints](#playground-endpoints)
  - [Required Authentication Endpoints](#required-authentication-endpoints)
    - [User Related](#user-related)
<!--toc:end-->

## Open Endpoints

### Authentication Related

- [Login](docs/auth/login.md) : `POST /api/v1/auth/login/`
- [Register](docs/auth/register.md) : `POST /api/v1/auth/register/`
- [Validate](docs/auth/validate.md) : `POST /api/v1/auth/validate/`

### Playground Endpoints

- Hello World : `GET /` (Just a "hello world" endpoint)

## Required Authentication Endpoints

- !!For now, a database is not used, so an endpoint is used for the users that are stored in memory

### User Related

- [Users](docs/user/all.md) : `GET /api/v1/user/all`
- [User](docs/user/id.md) : `GET /api/v1/user?id={id}`
