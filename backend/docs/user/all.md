# All Users

<!--toc:start-->

- [All Users](#all-users)
  - [Description](#description)
  - [Success Response](#success-response)
  <!--toc:end-->

## Description

| URL                 | Method | Auth required | Description   |
| ------------------- | ------ | ------------- | ------------- |
| /api/v1/auth/users/ | GET    | YES           | Get all users |

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

## Unauthorized Response

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "message": "string"
}
```
