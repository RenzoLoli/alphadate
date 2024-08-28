# Update User

| URL                   | Method | Auth required | Description       |
| --------------------- | ------ | ------------- | ----------------- |
| /api/v1/auth/user/:id | PUT    | YES           | Update user by id |

## URL Params

| Param | Type   | Required |
| ----- | ------ | -------- |
| id    | String | YES      |

## Request Body

```json
{
  "username?": "string",
  "email?": "string",
  "couplename?": "string",
  "anniversary?": "string",
  "photo?": "string"
}
```

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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
