# todo-actix

Todo apps using actix framework

# Setup

## Diesel ORM
https://diesel.rs/

First of all, install the diesel cli
``` 
cargo install diesel_cli 
```
<br>

Then, set your postgresql in .env file
```
DATABASE_URL=postgres://username:password@localhost/todo_list
```

<br>

Then run this command
```
diesel setup
```

<br>

To add a table migrations, use this command. (For instance we use todos table)
```
diesel migration generate create_todos
```

<br>

Now look for `up.sql` & `down.sql` in migration directory.
<u>The ones with create_todos</u>

Use up.sql for migrations, and use down,sql for redo-ing.

Then apply migration with
```
diesel migration run
```

Or redo-ing with
```
diesel migration redo
```

<br><br>

![Jeddi's Profile Views](https://api.visitorbadge.io/api/visitors?path=https%3A%2F%2Fgithub.com%2FJeddi212&countColor=%23fce775&style=flat-square)
