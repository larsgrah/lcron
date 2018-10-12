# lcron
larscron - a rudimentary cron-like daemon, written in rust, made for userlevel and userland execution

## About lcron
lcron is a simple, efficient and fast cron-like daemon.

It was writting specificially targeting users of Linux Distributions like Solus that lack decent cron packages.

lcron does not require sudo to edit as it operates as the user that started it, this has the advantage that it makes it very easy to use for scripting and automating processes like for example getting email and notifying the user

lcron executes a script exactly the way it would be run by you in the terminal, meaning: it supports sending notifications to X (looking at you systemd-timers)

Its featureset is currently very limited, create issues if something is missing for your workflow.

## Installing lcron
You will need Rust and Cargo, I recommend installing it from rustup.

create a ".larscrontab" file in your home directory and begin adding your cronjobs in this format:

| Cron time format  | Program to execute  | One Argument  |
|---|---|---|
| 0 * * * * *  | bash  | /home/lars/myscript.sh  |

This example crontab would execute myscript.sh in the first second of every minute of every hour of every day and so on.

Seconds are not honored right now and should always be set to 0.

## Future development
- Take n Arguments
- clean up cron creation, maybe a "crontab -e" like functionality
- check crontab for validity on save
- list currently running crontabs on command
