# Update Date Idea

<!--toc:start-->

- [Update Date Idea](#update-date-idea)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                   | Method | Auth required | Description      |
| --------------------- | ------ | ------------- | ---------------- |
| /api/v1/date-idea/:id | PUT    | YES           | Update date idea |

## URL Params

| Param | Type   | Description                       |
| ----- | ------ | --------------------------------- |
| id    | String | The id of the date idea to update |

## Request Body

```json
{
  "idea": "string",
  "description": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "idea": "string",
  "description": "string"
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
