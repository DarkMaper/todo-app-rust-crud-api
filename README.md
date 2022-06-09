# TODO App Backend
App written in Rust using Rocket for api server and Diesel MySQL.

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