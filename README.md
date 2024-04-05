# DAILI, an easy-to-use, flexible, and minimalist to-do list CLI ✍️

This started with the idea of being used by myself as a daily to-do app because I didn't find the ones available interesting or accessible enough; it's still true by now.

## The problem 🐌

I have always found the apps I was trying boring, and to be honest, I wanted something straight to the point. I wanted a daily to-do app, but that didn't seem to be the main idea of a lot of to-do list apps I've tried, and I've found it hard to find such features in those apps. I also wished to have it on my terminal: home sweet home, it's all about convenience. Everything I use is there: my editor, my fast automations, my tmux. I thought a little bit, and I remembered I'm a programmer, so why not create my own app?

## The idea 💡

DAILI is designed to be a simple CLI for your daily organization using _tasks_. Whether a task is done or not, the _today.toml_ file handles it, and you can personalize your daily template by editing your base.toml.

Both files are located in _$HOME/.daili/_.

## Base File 📓
> this is what you should see when using `daili -h base`

**Your base.toml file should be located at $HOME/.daili/ and look something like this:**

    [essential]
    1 = "Paint a picture"
    2 = "Study for school"
    3 = "Feed the cat"

    [optional]
    4 = "Watch anime"
    5 = "Try something new"

*Dont forget that the labels, the numbers for the tasks and the quantity are your choice!*

This is the base, the things you would like to do everyday
The program fetches your preferences from this file to create a today.toml file (know more about it with daili -h today)

## Today File ☀️
> this is what you should see when using `daili -h today`

You dont need to worry about this file, as it's handled and created automatically by the cli

*Your today.toml file should not ideally be touched as you can update it through the cli,
but if you will, it'll be automatically created, being located at $HOME/.daili/*

This file contains temporary data that is overwriten after a span of 24 hours from midnight (24:00 PM).
It retains information on wether the tasks are done or not, being read and modified by the cli
to show you and update the tasks properly

## How to use it ❓

Use `daili m` followed by the number of a task you want to mark, or `daili um` for unmarking tasks. You can also use the longer versions `mark` and `unmark`.

`daili -h` can be used to see the usage and documentation. `daili` alone will simply show you your list for today.

### Good luck organizing your day with DAILI 👋
