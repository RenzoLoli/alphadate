# Create Tag

<!--toc:start-->

- [Create Tag](#create-tag)
  - [Description](#description)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL         | Method | Auth required | Description |
| ----------- | ------ | ------------- | ----------- |
| /api/v1/tag | POST   | YES           | Create tag  |

## Request Body

```json
{
  "name": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "name": "string"
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
