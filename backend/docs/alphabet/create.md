# Create Alphabet

<!--toc:start-->

- [Create Alphabet](#create-alphabet)
  - [Description](#description)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL              | Method | Auth required | Description     |
| ---------------- | ------ | ------------- | --------------- |
| /api/v1/alphabet | POST   | YES           | Create alphabet |

## Request Body

```json
{
  "title": "string"
  "user_id": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "title": "string"
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

## Internal Server Error Response

**Code** : `500 INTERNAL SERVER ERROR`

**Content** :

```json
{
  "message": "string"
}
```
