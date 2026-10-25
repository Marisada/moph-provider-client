# MOPH ProviderID Client tester

## Preparing
1. Copy `/backend/src/credentials-empty` to `/backend/src/credentials`
2. Fill `/backend/src/credentials/health-id-client.txt` with Health ID's `client-id`
3. Fill `/backend/src/credentials/health-id-secret.txt` with Health ID's `secret-key` 
4. Fill `/backend/src/credentials/provider-id-client.txt` with Provider ID's `client-id`
5. Fill `/backend/src/credentials/provider-id-secret.txt` with Provider ID's `secret-key` 
6. (Optional) change parsing mode at `/backend/src/main.rs:47`
    - `fast`: find only `access_token` and `hash_cid` from response
    - `full`: parse all response to structure 

## Setup
```bash
cargo install wasm-bindgen-cli
```

## How to run
1. Build client
    * Windows
        ```cmd
        client
        ```
    * Linux
        ```bash
        ./client.sh
        ```

2. Serve
    * Windows
        ```cmd
        serve
        ```
    * Linux
        ```bash
        ./serve.sh
        ```

3. Open [http://localhost:8088](http://localhost:8088)

4. Reload client (after editing frontend)
    * Windows
        ```cmd
        reload
        ```
    * Linux
        ```bash
        ./reload.sh
        ```
