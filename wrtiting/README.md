<!-- @format -->

# Development

### Tailwind CSS

```
npx @tailwindcss/cli -i ./input.css -o ./desktop/assets/tailwind.css --watch
```

### Docker

```
docker compose up
```

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, a `ui` crate for shared components and a `api` crate for shared backend logic:

#### Connects to the DB?

```
psql -h localhost -p 5432 -d tester - U tester

```

#### SQLX database create

```
sqlx database create --database-url postgres://tester:tester@localhost:5432/tester

```

```
sqlx migrate add posts
```

```
sqlx migrate add profile_table
```

```
sqlx migrate add message_table
```

```
sqlx migrate run --database-url postgres://tester:tester@localhost:5432/tester
```

-- Add migration script here

CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL
);

sqlx migrate run --database-url="postgres://myuser:mypassword@localhost:5435/mydatabase"

CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL
);

dx bundle --platform desktop \
 --package-types "macos" \
 --package-types "dmg"

```

```
