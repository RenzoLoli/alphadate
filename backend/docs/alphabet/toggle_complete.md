# Update Alphabet Complete Status

<!--toc:start-->

- [Update Alphabet Complete Status](#update-alphabet-complete-status)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                                            | Method | Auth required | Description     |
| ---------------------------------------------- | ------ | ------------- | --------------- |
| /api/v1/alphabet/:alphabet-id/complete/date-id | GET    | YES           | Update complete |

## URL Params

| Param       | Type   | Description                               |
| ----------- | ------ | ----------------------------------------- |
| alphabet-id | String | The id of the alphabet to update complete |
| date-id     | String | The id of the date to update complete     |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "complete": "boolean"
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
