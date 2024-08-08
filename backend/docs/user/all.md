# All Users

Used to collect all users on memory.

**URL** : `/api/v1/auth/users/`

**Method** : `GET`

**Auth required** : YES

## Success Response

**Code** : `200 OK`

**Content**

```json
[
  {
    "username": "[username in plain text]",
    "password": "[password in plain text]",
    "email": "[valid email address]",
    "couplename": "[couplename in plain text]",
    "anniversary": "[date in format 'dd/mm/yyyy']",
    "photo": "[valid url pic]"
  }
]
```

**Content Example**

```json
[
  {
    "username": "user123",
    "password": "P4ssw0rd",
    "email": "usermail@domain.com",
    "couplename": "couple123",
    "anniversary": "17/04/1999",
    "photo": "http://www.freepic.com/love.png"
  }
]
```
