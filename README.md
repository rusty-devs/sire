# Sire

[![Coverage Status](https://coveralls.io/repos/github/rusty-devs/sire/badge.svg?branch=main)](https://coveralls.io/github/rusty-devs/sire?branch=main) ![sire](https://github.com/rusty-devs/sire/actions/workflows/rust.yml/badge.svg)

Welcome!

**sire** is a command line tool that simplifies creating and maintaining new source code projects by leveraging patterns from user defined "templates". The primary purpose for the tool is to save developers and teams time when creating new source code projects by re-using consistent patterns (ie: folder structures, build tools, libraries and frameworks), and making it easy to keep those projects up to date when changes are made to the templates. The tool is intended to to be language-agnostic allowing projects written in  programming language or toolchain to be managed in a similar way.

To be as easy to use as possible we aim to achieve the following goals:

* provide a tool which is packaged as a single, small binary with no external dependencies
* make it easy for new users to create their own custom templates, and share templates with others
* provide advanced functionality to accommodate the most complex project structures
* define a framework to make it as easy as possible to reflect changes to a template in source projects that are based on it

We hope to provide a tool that saves people time and energy by avoiding manual steps creating build scripts, defining folder structures, creating configuration files, and other such monotonous / boilerplate work while giving them a powerful framework to orchestrate more complex project setups with the most minimal effort possible.