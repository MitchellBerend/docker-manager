# Quick start

1. Download the latests [release](https://github.com/MitchellBerend/docker-manager/releases).
2. Check if you need sudo rights to access docker commands
    If you do you can supply the `-s` [flag](#Flags)
3. Run `docker-manager completion --help` to set up auto completion for your
shell
4. Run `docker-manager ps -a`


# Goals
This project aims to have an easier way to interact with docker containers on
remote nodes from the comfort of your own terminal. The aim for this project is
to support all docker commands on remote nodes.


# Current commands

| Command  | Notes                         |
|----------|-------------------------------|
| EXEC     | --tty is not implemented      |
| IMAGES   |                               |
| LOGS     |                               |
| PS       |                               |
| RESTART  | Multiple containers supported |
| RM       |                               |
| START    |                               |
| STOP     |                               |
| SYSTEM   |                               |


# Flags

`-s`/`--sudo` Enables sudo on the remote node. This might be needed depending on
how the remote node and it's user is set up.

`-r`/`--regex` Lets the user supply a regex pattern that filters the nodes after
they are read in from `~/.ssh/config`.

`-i`/`--identity-file` Passes an identity file to the underlying ssh connection.
