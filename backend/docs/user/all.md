# All Users

<!--toc:start-->

- [All Users](#all-users)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL              | Method | Auth required | Description   |
| ---------------- | ------ | ------------- | ------------- |
| /api/v1/user/all | GET    | YES           | Get all users |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "username": "string",
    "password": "string",
    "email": "string",
    "couplename": "string",
    "anniversary": "string",
    "photo": "string"
  }
]
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
