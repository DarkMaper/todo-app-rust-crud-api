# TODO App Backend
App written in Rust using Rocket for api server and Diesel MySQL.

## Rocker config
```toml
[global.databases.todoapp]
url = "mysql://username:password@host/database"

[default]
port=5000
```
## Todo list

- [x] Connection to Database
- [x] Create Signin and Signup routes and controllers
- [x] Add auth guard
- [ ] Notes
    - [x] Show notes from logged user
    - [x] Create notes
    - [ ] Edit notes
    - [ ] Delete notes