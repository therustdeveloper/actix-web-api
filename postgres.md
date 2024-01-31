# Create Postgres Database

## Connect to Postgres Database

```shell
psql postgres                                                                                                                                                                                         ─╯
psql (14.10 (Homebrew))
Type "help" for help.

postgres=#
```

## Create Postgres Database

```shell
postgres=# create database actixwebapi;
CREATE DATABASE
postgres=#
```

## Create dev User

```shell
postgres=# create user dev with encrypted password 'metallica';
CREATE ROLE
postgres=# 
```

## Grant Permissions to User on the Database

```shell
postgres=# grant all privileges on database actixwebapi to dev;
GRANT
```

## Grant createdb role to dev user

```shell
postgres=# alter user dev createdb;
ALTER ROLE
postgres=# 
```

## Grant All on Public to actixwebapi Database

### Exit the root session

```shell
postgres=# exit
```

### Connect to actixwebapi Database

```shell
psql actixwebapi                                                                                                                                                                                      ─╯
psql (14.10 (Homebrew))
Type "help" for help.

actixwebapi=#
```

### Grant all on schema public to dev user in actixwebapi database

```shell
actixwebapi=# grant all on schema public to dev;
GRANT
actixwebapi=#
```
