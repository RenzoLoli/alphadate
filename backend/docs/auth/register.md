# Register

<!--toc:start-->

- [Register](#register)
  - [Description](#description)
  - [Data constraints](#data-constraints)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

Used to register a new User.

| URL                    | Method | Auth required | Description |
| ---------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/register/ | POST   | NO            | Register    |

## Data constraints

```json
{
  "username": "[username in plain text]",
  "password": "[password in plain text]",
  "email": "[valid email address]",
  "couplename": "[couplename in plain text]",
  "anniversary": "[date in format 'yyyy/mm/dd']",
  "photo": "[valid url pic]"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "message": "Registered Succesfully"
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
