# Update User

<!--toc:start-->

- [Update User](#update-user)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

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