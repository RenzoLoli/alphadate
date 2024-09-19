# Delete User

<!--toc:start-->

- [Delete User](#delete-user)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Bad Request Response](#bad-request-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description       |
| --------------------- | ------ | ------------- | ----------------- |
| /api/v1/auth/user/:id | DELETE | YES           | Delete user by id |

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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
