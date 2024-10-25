# Random Date Idea

<!--toc:start-->

- [Random Date Idea](#random-date-idea)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                                   | Method | Auth required | Description          |
| ------------------------------------- | ------ | ------------- | -------------------- |
| /api/v1/date-idea/random/:alphabet-id | GET    | YES           | Get random date idea |

## URL Params

| Param       | Type   | Description                             |
| ----------- | ------ | --------------------------------------- |
| alphabet-id | String | The id of the alphabet to get date idea |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "idea": "string",
  "description": "string",
  "tags": [
    {
      "id": "string",
      "name": "string"
    }
  ]
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
