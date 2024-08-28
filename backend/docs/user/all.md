# All Users

| URL                 | Method | Auth required | Description   |
| ------------------- | ------ | ------------- | ------------- |
| /api/v1/auth/users/ | GET    | YES           | Get all users |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "username": "string",
    "password": "string",
    "email": "string",
    "couplename": "string",
    "anniversary": "string",
    "photo": "string"
  }
]
```
