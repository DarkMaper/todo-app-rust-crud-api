# TODO App Backend
[![License: MIT](https://img.shields.io/github/license/DarkMaper/todo-app-rust-crud-api)](LICENSE)

App written in Rust using Rocket for API server and Diesel MySQL.

---
## Rocket config
```toml
# Rocket.toml
[global.databases.todoapp]
url = "mysql://username:password@host/database"

[default]
port=3000
```
## Todo list

- [x] Connection to Database
- [x] Create Signin and Signup routes and controllers
    - [x] Refresh Token System
- [x] Add auth guard for JWT
- [x] Notes
    - [x] Show notes from logged user
    - [x] Create notes
    - [x] Edit notes
    - [x] Delete notes

## Usage
### Building app

**Note:** For Windows, you need install MySQL Connector C (included in MySQL Server) and set environment variable MYSQLCLIENT_LIB_DIR to the path of the folder containing the mysqlclient.dll file. For Linux, you need libmysqlclient-dev in Debian-based distros.

```bash
cargo build --release
```

### Docker

You can build a image of the app with:
```bash
docker build -t todoapp .
```

And run a container with:
```bash
docker run -p 3000:3000 \
-e ROCKET_DATABASES={todoapp={url="mysql://user:password@database_host/database_name"}} \
-e ROCKET_PORT=3000 \
-e ROCKET_ADDRESS=0.0.0.0 \
-e ALLOWED_ORIGINS=http://localhost,http://localhost:8080 \
--name todoapp-container todoapp
```

### Docker Compose

```yaml
version: '3.9'
services:
  app:
    build: .
    restart: always
    environment:
      - ROCKET_DATABASES={todoapp={url="mysql://todoapp:todoapp@db/todoapp"}}
      - ROCKET_PORT=3000
      - ROCKET_ADDRESS=0.0.0.0
      - ALLOWED_ORIGINS=http://localhost,http://localhost:8080
    depends_on:
      - db
    ports:
      - "3000:3000"
  db:
    image: mariadb:10.7.4
    restart: always
    environment:
      - MYSQL_ROOT_PASSWORD=root
      - MYSQL_DATABASE=todoapp
      - MYSQL_USER=todoapp
      - MYSQL_PASSWORD=todoapp
    volumes:
      - todoapp-db:/var/lib/mysql
volumes:
  todoapp-db:

```

---
MIT License