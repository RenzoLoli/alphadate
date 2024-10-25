# Get User By Id

<!--toc:start-->

- [Get User By Id](#get-user-by-id)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL              | Method | Auth required | Description    |
| ---------------- | ------ | ------------- | -------------- |
| /api/v1/user/:id | GET    | YES           | Get user by id |

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

## Internal Server Error Response

**Code** : `500 INTERNAL SERVER ERROR`

**Content** :

```json
{
  "message": "string"
}
```
