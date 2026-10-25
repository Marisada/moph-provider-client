## Token structure
- Both Health ID and Provider ID access token is JWT RS256.
- Payload is signed but not encrypted — anyone can read the claims.
- All tokens in this file have been modified to redact certain sensitive information, they cannot be verified using a public key.
- Last tested at 2026-10-25

### HealthID access token
```
eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIwMTk2YTU0MC02NDY1LTczN2YtODU4OC1kNTNiMDc3Nzc3NzciLCJqdGkiOiJkYmZmNjliMzQwNmIwNTgyNTg4ZjQ5NzcyZjYxZjRiN2UwYjk1NmUzNDhmMzJhZDJhNjNjYThkMjI4YmY5NTBiZTAwNjlkMWI1OTc5ZTg2MCIsImlhdCI6MTc5MjczNjUzMi44MTUxMzIsIm5iZiI6MTc5MjczNjUzMi44MTUxMzgsImV4cCI6MTgwMDUxMjUzMi43NzcxNTYsInN1YiI6ImRmNWM1ZDQzLTY0M2QtNDNjZi1hMWM3LWNjY2NjY2NjY2NjYyIsInNjb3BlcyI6WyJuYW1lIiwibmFtZV9wcmVmaXgiLCJzdXJuYW1lIiwiaWRfY2FyZCIsImJpcnRoZGF0ZSIsIm5hdGlvbmFsaXR5IiwibW9iaWxlX25vIiwiaWFsIiwiYWFsIiwiZW1haWwiXSwic2NvcGVzX2RldGFpbCI6eyJpZF9jYXJkIjoiMzFYWFhYWFhYWFg3MSIsImhhc2hfaWRfY2FyZCI6IjBhZDljZWY0M2QxMjAxMjY4OGFhYWFhYWFjY2NjY2NjY2NjY2NjY2NjY2NjYzg1OTk3NGFjMDE2MWExMDk0NjIiLCJub25fdGhfaWRfY2FyZCI6IiIsImhhc2hfbm9uX3RoX2lkX2NhcmQiOiIiLCJiaW9faWQiOiIiLCJoYXNoX2Jpb19pZCI6IiIsIm5hbWVfcHJlZml4IjoiXHUwZTE5XHUwZTMyXHUwZTIyIiwibmFtZSI6Ilx1MGUyMVx1MGUzMlx1MGUyM1x1MGUzNFx1MGUyOVx1MGUwZVx1MGUzMiIsInN1cm5hbWUiOiJcdTBlMWVcdTBlMzRcdTBlMTdcdTBlMzFcdTBlMDFcdTBlMjlcdTBlNGNcdTBlMThcdTBlMjNcdTBlMjNcdTBlMjEiLCJiaXJ0aGRhdGUiOiIxODg4LTA4LTA4IiwibmF0aW9uYWxpdHkiOiJUaGFpIiwibW9iaWxlX25vIjoiMDgxMDAwMDAwMCIsImlhbCI6Mi4zfX0.Ha9rFGhUfmR0D4hVAVIW1kGJqBBo7--gg65mNzQ6d0u1oeYlHkekQmOVGINKi4jNfjK2bxQoD0jkOdQfo0Cg_Ndfspbh6XkZaHbAX-w9ZpAamelL8ZXbvIiHeuWbKNQcGvbdCQQ80QmQK5aYG_rQH0pOSLmoe3MjFb32t3rULZeWuiIibDNPLkRojxbyo8tUWC30_YQjOf1SbgQ2IjUsim3pe2LxYI4OK3zEDt8bpBgWLvAYhet4Mzuhlra9XboTs8vbG-xopQc1fuAtqNL8L2vTLd363aSX-a2W-D1VoKMC_dCIvM8vp9Es1jjWwX0pIAQ7v53n_xNdpPXfVh9fqhcHpULbVUYWS3K0dz0XIeTJ1C-MWT_lNR2Y75HviXb_brBfyVUncInD5WAX8U2mQgOABxXvXfMThytt6S-wCt7SWwGUIiKCCTyS5kEmUQTrGTS2g_2F2UJOX0KH3J-MQObTdcFg7eVgy1I3r9unqBAj0ttyUxfZgaQGQqKb_T3AplnBhncDmSHCyt4iJyoh-FT72ZZD8A8Zkxy28zdFRh5BnXOu9SBAVDjBOT0iIeKd9fPCgEJVBU_-6ng6p_V67xLD5Ey8-TaTnjXuAAWfXVHwppkyUYjcXQMHTGPfVlPUtw-622GEYRW30xteQqum-fYCnD_8xjgbSFfIe93QAUc
```

can decode to header
```json
{
  "typ": "JWT",
  "alg": "RS256"
}
```
and payload
```json
{
  "aud": "0196a540-6465-737f-8588-d53b07777777",
  "jti": "dbff69b3406b0582588f49772f61f4b7e0b956e348f32ad2a63ca8d228bf950be0069d1b5979e860",
  "iat": 1792736532.815132,
  "nbf": 1792736532.815138,
  "exp": 1800512532.777156,
  "sub": "df5c5d43-643d-43cf-a1c7-cccccccccccc",
  "scopes": [
    "name",
    "name_prefix",
    "surname",
    "id_card",
    "birthdate",
    "nationality",
    "mobile_no",
    "ial",
    "aal",
    "email"
  ],
  "scopes_detail": {
    "id_card": "31XXXXXXXXX71",
    "hash_id_card": "0ad9cef43d12012688aaaaaaacccccccccccccccccccc859974ac0161a109462",
    "non_th_id_card": "",
    "hash_non_th_id_card": "",
    "bio_id": "",
    "hash_bio_id": "",
    "name_prefix": "นาย",
    "name": "มาริษฎา",
    "surname": "พิทักษ์ธรรม",
    "birthdate": "1888-08-08",
    "nationality": "Thai",
    "mobile_no": "0810000000",
    "ial": 2.3
  }
}
```

### ProviderID access token
```
eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6IjQ3NDdmNTE3M2E0MjhjN2YyNDczMGVhNTA2ZTY3Y2M2NTJhMzk1MDgwOWJhNzVmMWI1OTc5ZjExMmZjN2UxM2JjZDkzZjQyMTg3OGE5ODAyIn0.eyJhdWQiOiI1MDAiLCJqdGkiOiI0NzQ3ZjUxNzNhNDI4YzdmMjQ3MzBlYTUwNmU2N2NjNjUyYTM5NTA4MDliYTc1ZjFiNTk3OWYxMTJmYzdlMTNiY2Q5M2Y0MjE4NzhhOTgwMiIsImlhdCI6MTc5MjczNjUzMywibmJmIjoxNzkyNzM2NTMzLCJleHAiOjE3OTI4MjI5MzMsInN1YiI6IjQyMTE3MTQxOTc3Nzc3NzciLCJzY29wZXMiOltdfQ.ayyu3c0S2oAauGa_Gjm4tEjud7SQp73TJH2swx4IRtdz5gHzafOJ3Nw-Oxuz53RB7cpM0e3zvbeNlMH52Vosf9DBNAe9aWI6lTX812Z6PGTCNJc0RwFfoeYogBUGfpWTzLSs0rIpgZDhXt9k7AeXS7Ijd4XQKiFr4MgFFTkfd73bQeid8NncwezAWXfFuoma0eUBCPh0OM3bOyoXSz0FVsRe1mKqcJeqNU1VwyRwaWeA-V-HJs9u6YV_IOEsEXDGXW-PyQL7xs2YQw2aGxZ-XOpw7NLSbDBiFtGJtR-C6m7IsMBGF1o5pevFD0t81hbNbcooPEmJ7jZUESxEPkk7hYZt_TknT3Ny0h70LOKiLPr42QzHwf1L-aZoYKsvtYgTN6KMmadKX0CkXStHZWRBOGbWp5xt9cb_UMTd7fqmSZ1nB5ND7-Pl_O0n8KPQ9PSJYV9CxhS5_XM6RG5-XlLEpP5wg6SwJRQV2_IhvxNrHvYLRvw0tHod4ZDK5xMPzJNKmAapn8ap4szaF4ig1Nn_kGOGJ9TqGolC8vRHVsRn80cJkP0rqyCtggUgTClElANlWZULzKP6sCmndcynYJqeM2Fw-h6GWD6LWhjwTeKipnf1XkRQxf9uTembKI3LvIRoxQFZzmgeiYm2EI9FulLv2aHYfzu6pP2muNphZcxq8FI
```

can decode to header
```json
{
  "typ": "JWT",
  "alg": "RS256",
  "jti": "4747f5173a428c7f24730ea506e67cc652a3950809ba75f1b5979f112fc7e13bcd93f421878a9802"
}
```
and payload
```json
{
  "aud": "500",
  "jti": "4747f5173a428c7f24730ea506e67cc652a3950809ba75f1b5979f112fc7e13bcd93f421878a9802",
  "iat": 1792736533,
  "nbf": 1792736533,
  "exp": 1792822933,
  "sub": "4211714197777777",
  "scopes": []
}
```
> NOTE: `sub` in JWT token is `account_id` of response's `data`
