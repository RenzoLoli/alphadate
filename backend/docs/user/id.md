# Get User By Id

<!--toc:start-->

- [Get User By Id](#get-user-by-id)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description    |
| --------------------- | ------ | ------------- | -------------- |
| /api/v1/auth/user/:id | GET    | YES           | Get user by id |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "username": "string",
  "password": "string",
  "email": "string",
  "couplename": "string",
  "anniversary": "string",
  "photo": "string"
}
```

## Bad Request Response

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "string"
}
```

## Unauthorized Response

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "message": "string"
}
```

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
