# Quick start

1. Download the latests [release](https://github.com/MitchellBerend/docker-manager/releases).
2. Ensure the user you log in with has sudo rights.
3. Run `docker-manager completion --help` to set up auto completion for your
shell
4. Run `docker-manager ps`

# Goals
This project aims to have an easier way to interact with docker containers on
remote nodes from the comfort of your own terminal. The aim for this project is
to support all docker commands on remote nodes.


# Current commands


| Command  | Notes                    |
|----------|--------------------------|
| PS       |                          |
| STOP     |                          |
| LOGS     |                          |
| EXEC     | --tty is not implemented |


# Flags

`-s`/`--sudo` Enables sudo on the remote node. This might be needed depending on
how the remote node and it's user is set up.
