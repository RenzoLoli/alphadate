# Login

<!--toc:start-->

- [Login](#login)
  - [Description](#description)
  - [Data constraints](#data-constraints)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

Used to collect a Token for a registered User.

| URL                 | Method | Auth required | Description |
| ------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/login/ | POST   | NO            | Login       |

## Data constraints

```json
{
  "email": "[valid email address]",
  "password": "[password in plain text]"
}
```

## Success Response

**Description** : JWT Token as Response.

**Code** : `200 OK`

**Content**

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

## Internal Server Error Response

**Code** : `500 INTERNAL SERVER ERROR`

**Content** :

```json
{
  "message": "string"
}
```
