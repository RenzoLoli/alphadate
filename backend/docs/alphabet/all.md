# All Alphabets

<!--toc:start-->

- [All Alphabets](#all-alphabets)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                           | Method | Auth required | Description       |
| ----------------------------- | ------ | ------------- | ----------------- |
| /api/v1/alphabet/all/:user_id | GET    | YES           | Get all alphabets |

## URL Params

| Param   | Type   | Description                             |
| ------- | ------ | --------------------------------------- |
| user_id | String | The id of the user to get alphabets for |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "id": "string",
    "title": "string",
    "user_dates": [
      {
        "id": "string",
        "letter": "string",
        "completed": "boolean",
        "date_idea": [
          {
            "id": "string",
            "idea": "string",
            "description": "string"
            "tags": [
              {
                "id": "string",
                "name": "string"
              }
            ]
          }
        ]
      }
    ]
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

## Internal Server Error Response

**Code** : `500 INTERNAL SERVER ERROR`

**Content** :

```json
{
  "message": "string"
}
```
