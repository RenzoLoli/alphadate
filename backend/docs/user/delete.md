# Delete User

<!--toc:start-->

- [Delete User](#delete-user)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Bad Request Response](#bad-request-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL              | Method | Auth required | Description       |
| ---------------- | ------ | ------------- | ----------------- |
| /api/v1/user/:id | DELETE | YES           | Delete user by id |

## URL Params

| Param | Type   | Description                  |
| ----- | ------ | ---------------------------- |
| id    | String | The id of the user to delete |

## Success Response

**Code** : `200 OK`

**Content** :

```json
  {
    "id": "string"
    "username": "string",
    "email": "string",
    "couplename": "string",
    "anniversary": "string",
    "photo": "string"
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

## Bad Request Response

**Code** : `400 BAD REQUEST`

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
