# Login

Used to collect a Token for a registered User.

**URL** : `/api/v1/auth/login/`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
  "email": "[valid email address]",
  "password": "[password in plain text]"
}
```

**Data example**

```json
{
  "username": "usermail@domain.com",
  "password": "P4ssw0rd"
}
```

## Success Response

**Code** : `200 OK`

**Content**

```json
{
  "token": "[user token]"
}
```

**Content Example**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJyZW56b2xvbGkxQGhvdG1haWwuY29tIiwiZXhwIjoxNzIzMTA1MjU1fQ.R8WslwDOrvGDiqB-CyPPF-m7W_L1WYFWcWKhW-xEY38"
}
```

## Bad Request Response

**Condition** : If 'username' and 'password' combination is wrong.

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "Needed email and password"
}
```

## Not Found Response

**Condition** : If user is not found.

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "[Cause of the error]" 
}
