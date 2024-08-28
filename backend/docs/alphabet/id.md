# Get Alphabet By Id

<!--toc:start-->

- [Get Alphabet By Id](#get-alphabet-by-id)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description        |
| --------------------- | ------ | ------------- | ------------------ |
| /api/v1/alphabets/:id | GET    | YES           | Get alphabet by id |

## URL Params

| Param | Type   | Description                   |
| ----- | ------ | ----------------------------- |
| id    | String | The id of the alphabet to get |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "title": "string",
  "user_dates": [
    {
      "id": "string",
      "letter": "string",
      "completed": "string",
      "date_idea_id": "string"
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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
