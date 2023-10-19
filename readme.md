## Prism API

### Connect to Apideck

Apideck is our core provider for consumers for now. We're using it on par with direct connections toward interested APIs (Redtail CRM e.g.).

For authorization, apideck requires to pass 3 headers: token, app id and consumer id. token and app id are static and can't change during session. This is an env vars. As for consumer id, we have different ids for each of our partner.

#### API Key

```js
 const headers = {
    // ...
    'Authorization': `Bearer ${API_KEY}`,
    // ...
 };
 ```

#### App ID

```js
 const headers = {
    // ...
    'x-apideck-app-id': APP_ID,
    // ...
 };
 ```

#### Consumer ID

```js
 const headers = {
    // ...
    'x-apideck-consumer-id': consumerId,
    // ...
 };
 ```
