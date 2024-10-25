# Renew token

<!--toc:start-->

- [Renew token](#renew-token)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL                 | Method | Auth required | Description |
| ------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/renew/ | POST   | YES           | Renew token |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "token": "string"
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
