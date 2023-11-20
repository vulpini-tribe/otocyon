## Prism API

### Connect to Apideck

Apideck is our core provider for consumers for now. We're using it on par with direct connections toward interested APIs (Redtail CRM e.g.).

For authorization, apideck requires to pass 3 headers: token, app id and consumer id. token and app id are static and can't change during session. This is an env vars. As for consumer id, we have different ids for each of our partner.

#### 

```rust
let headers = Headers {
    // API Key
    "Authorization": format!("Bearer {}", API_KEY),
    // App ID
    "x-apideck-app-id": APP_ID,
    // Consumer ID
    "x-apideck-consumer-id": consumer_id,

    // Consumer ID
    "x-apideck-service-id": service_id, // "hubspot"
 };
 ```

Notes, pipelines

Users, 
