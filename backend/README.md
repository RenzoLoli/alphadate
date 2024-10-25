# Alphadate REST API Documentation

<!--toc:start-->

- [Alphadate REST API Documentation](#alphadate-rest-api-documentation)
  1. [Authentication](#authentication)
  2. [Open Endpoints](#open-endpoints)

      - [Authentication Related](#authentication-related)
      - [Playground Endpoints](#playground-endpoints)
  3. [Required Authentication Endpoints](#required-authentication-endpoints)

      - [User Related](#user-related)
      - [Token Related](#token-related)
      - [Tag Related](#tag-related)
      - [Date Idea Related](#date-idea-related)
      - [Alphabet Related](#alphabet-related)
  <!--toc:end-->

## Authentication

- [Bearer](docs/auth/bearer.md): "Header Authorization Bearer"

## Open Endpoints

### Authentication Related

- [Login](docs/auth/login.md) : `POST /api/v1/auth/login/`
- [Register](docs/auth/register.md) : `POST /api/v1/auth/register/`

### Playground Endpoints

- Hello World : `GET /` (Just a "hello world" endpoint)

## Required Authentication Endpoints

### User Related

- [GetAll](docs/user/all.md) : `GET /api/v1/user/all`
- [GetById](docs/user/id.md) : `GET /api/v1/user/{id}`
- [Update](docs/user/update.md) : `PUT /api/v1/user/{id}`
- [Delete](docs/user/delete.md) : `DELETE /api/v1/user/{id}`

### Token Related

- [Renew](docs/token/renew.md) : `POST /api/v1/token/renew`

### Tag Related

- [GetAll](docs/tag/all.md) : `GET /api/v1/tag/all`
- [GetById](docs/tag/id.md) : `GET /api/v1/tag/{id}`
- [Update](docs/tag/update.md) : `PUT /api/v1/tag/{id}`
- [Delete](docs/tag/delete.md) : `DELETE /api/v1/tag/{id}`
- [Create](docs/tag/create.md) : `POST /api/v1/tag`

### Date Idea Related

- [GetAll](docs/date-idea/all.md) : `GET /api/v1/date-idea/all`
- [GetById](docs/date-idea/id.md) : `GET /api/v1/date-idea/{id}`
- [Create](docs/date-idea/create.md) : `POST /api/v1/date-idea/{id}`
- [Update](docs/date-idea/update.md) : `PUT /api/v1/date-idea/{id}`
- [Delete](docs/date-idea/delete.md) : `DELETE /api/v1/date-idea/{id}`
- [Random](docs/date-idea/random.md): `GET /api/v1/date-idea/random/{alphabet-id}`
- [AddTag](docs/date-idea/add_tag.md) : `POST /api/v1/date-idea/{date-idea-id}`
- [RemoveTag](docs/date-idea/remove_tag.md) : `DELETE /api/v1/date-idea/{date-idea-id`

### Alphabet Related

- [GetAll](docs/alphabet/all.md) : `GET /api/v1/alphabet/all/{user_id}`
- [GetAllBase](docs/alphabet/all_base.md) : `GET /api/v1/alphabet/all/{user_id}/base`
- [GetById](docs/alphabet/id.md) : `GET /api/v1/alphabet/{id}`
- [Create](docs/alphabet/create.md) : `POST /api/v1/alphabet/{id}`
- [Update](docs/alphabet/update.md) : `PUT /api/v1/alphabet/{id}`
- [Delete](docs/alphabet/delete.md) : `DELETE /api/v1/alphabet/{id}`
- [AddIdea](docs/alphabet/add_idea.md) : `POST /api/v1/alphabet/{alphabet-id}`
- [RemoveIdea](docs/alphabet/remove_idea.md) : `DELETE /api/v1/alphabet/{alphabet-id}`
- [ToggleComplete](docs/alphabet/toggle_complete.md) : `GET /api/v1/alphabet/{alphabet-id}/complete/date-id`
