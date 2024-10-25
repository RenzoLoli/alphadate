# Add Idea to Alphabet

<!--toc:start-->

- [Add Idea to Alphabet](#add-idea-to-alphabet)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                                              | Method | Auth required | Description          |
| ------------------------------------------------ | ------ | ------------- | -------------------- |
| /api/v1/alphabet/:alphabet-id/date-idea/:idea-id | POST   | YES           | Add idea to alphabet |

## URL Params

| Param       | Type   | Description                            |
| ----------- | ------ | -------------------------------------- |
| alphabet-id | String | The id of the alphabet to add idea to  |
| idea-id     | String | The id of the date idea to add idea to |

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
