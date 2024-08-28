# Get Date Idea by ID

<!--toc:start-->

- [Get Date Idea by ID](#get-date-idea-by-id)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description         |
| --------------------- | ------ | ------------- | ------------------- |
| /api/v1/date-idea/:id | GET    | YES           | Get date idea by ID |

## URL Params

| Param | Type   | Description                    |
| ----- | ------ | ------------------------------ |
| id    | String | The id of the date idea to get |

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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
