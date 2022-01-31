- [Intro](#intro)
- [Usage](#usage)
  - [Install](#install)
  - [Uninstall](#uninstall)
  - [Configuration](#configuration)


# Intro
This program serves as an interface that provides docker information on remote nodes.
It is tested on rust version 1.58.1 stable.


# Usage
## Install
You can install this by compiling it manually or running the install.sh script.
It will automatically install the program in your /usr/bin and does require sudo
priviledges. 

## Uninstall
You can run the uninstall.sh script, or manually delete the /usr/bin/docker-manager
program.

## Configuration
The program will use the current users ~/.ssh/config file to look for hostnames.
This means if you run this program with `sudo docker-manager ps` it will check 
`/home/root/.ssh/config`. There is a regex flag in some of the commands that lets
you filter on hostname by pattern. This is important if you have nodes that do not
have docker installed on them.
