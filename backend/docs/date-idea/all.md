# All Date Ideas

<!--toc:start-->

- [All Date Ideas](#all-date-ideas)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description        |
| --------------------- | ------ | ------------- | ------------------ |
| /api/v1/date-idea/all | GET    | YES           | Get all date ideas |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
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
