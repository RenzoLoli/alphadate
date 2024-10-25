# Get Alphabet By Id

<!--toc:start-->

- [Get Alphabet By Id](#get-alphabet-by-id)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                  | Method | Auth required | Description        |
| -------------------- | ------ | ------------- | ------------------ |
| /api/v1/alphabet/:id | GET    | YES           | Get alphabet by id |

## URL Params

| Param | Type   | Description                   |
| ----- | ------ | ----------------------------- |
| id    | String | The id of the alphabet to get |

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
