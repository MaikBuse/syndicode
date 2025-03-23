# Vectory Server

## Setup

1. Declare environment variables
    ```
    export DATABASE_URL="sqlite:syndicode.db"
    export JWT_SECRET="some-super-secret-string"
    ```

2. Create the database
    ```
    sqlx db create
    ```

3. Create sql migrations
    ```
    sqlx migrate add <name>
    ```

4. Run sql migrations
    ```
    sqlx migrate run
    ```
