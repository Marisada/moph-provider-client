# OAUTH 2.0

## Terms
- Resource Owner: Entity that can grant access to the protected resources
- Client: Application used by Resource Owner
- User Agent: System with Client inside, ex. web browser 
- Resource Server: Server hosting the protected resources
- Authorization Server: Server that authenticates the Resource Owner
- Access Token: Token for getting protected resource from Resource Server
- ID Token: Token for getting Access Token from Authorization Server
- Credentials: Client ID + Client Secret or Username + Password granted by Authorized Server

## Security concern
- Authorization Code MUST can be used only once
- NOT send URL with Credentials or Access Token to prevent Browser History Attack
- NOT exposed Credentials at Client
- NOT exposed Authorization Code at Client
- NOT exposed Access Token at Client

## OAUTH 2.0 flow guide
- Client is the Resource Owner: use [Client Credentials Flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/client-credentials-flow)

- Client is a web app executing on the server: use [Authorization Code Flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow)

- Client absolutely trusted with user credentials: use [Resource Owner Password Flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/resource-owner-password-flow)

- Client is a Single-Page App: [Implicit Flow with Form Post](https://auth0.com/docs/get-started/authentication-and-authorization-flow/implicit-flow-with-form-post) or [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow-with-pkce)

- Client is a Native/Mobile App: use [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow-with-pkce)


## Client Credentials Flow (for Machine to Machine: M2M)
- Send Credentials
  => get Access Token
- No Resource Owner interaction
- Not input Credentials in Client
- Send at Server side only: Access Token + Credentials is NOT exposed

## Resource Owner Password Flow
- Fill Credentials in App
  => Client/Resource Server send Credentials to Authorized Server => get Access Token
- Input Credentials in Client
- Send at Client side: Access Token + Credentials is exposed!!!
- Send at Server side: Access Token + Credentials is NOT exposed

## Implicit Flow with Form Post
- Open authorization URI
  => Authorization prompt
  => Authenticate and Consent
  => Redirect to App with ID Token
- Redirect-based flows
- No Access Token, cannot get the protected resources

## Authorization Code Flow
- Open authorization URI
  => Authorization prompt
  => Authenticate and Consent
  => Redirect to App with Authorization Code
  => Client/Resource Server send Authorization Code + Credentials to Authorized Server
  => Client/Resource Server get ID Token and Access Token
- Redirect-based flows
- Send at Client side: Access Token + Authorization Code + Credentials is exposed!!!
- Send at Server side: Access Token + Authorization Code + Credentials is NOT exposed
- Can return Refresh Tokens
- Hardening Authorized Server by allowed only registered Redirect URI and check Redirect URI at Link and Send Authorization Code phase to prevent CSRF-like attack (change Redirect URI to attacker's URI)

## Authorization Code Flow with Proof Key for Code Exchange (PKCE)
- Link
  => App generate Code Verifier + Code Challenge
  => Send Code Challenge to Authorized Server
  => Redirect to Authorization prompt
  => Authenticate and Consent
  => Redirect to App with Authorization Code
  => Send Authorization Code + Code Verifier to Authorized Server
  => get ID Token and Access Token
- Same as Authorization Code Flow, but without Credentials
- Can return Refresh Tokens
