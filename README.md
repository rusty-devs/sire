# Sire

[![Coverage Status](https://coveralls.io/repos/github/rusty-devs/sire/badge.svg?branch=main)](https://coveralls.io/github/rusty-devs/sire?branch=main) ![sire](https://github.com/rusty-devs/sire/actions/workflows/rust.yml/badge.svg)

## Summary

**Sire** is a command-line tool that simplifies creating and maintaining [Jinja](https://jinja.palletsprojects.com/en/3.1.x/templates/) style templates. The primary goal is to save users time when creating new source code projects by re-using consistent patterns (ie: folder structures, build tools, libraries and frameworks), and making it easy to keep those projects up to date when changes are made to the templates. The tool is intended to to be language-agnostic allowing projects written in any programming language or toolchain.

## Goals

* Provide a tool which is packaged as a single, small binary with no external dependencies.
* Make it easy for new users to create their own custom templates, and share templates with others.
* Provide advanced functionality to accommodate the most complex project structures.
* Define a framework to make it as easy as possible to reflect changes to a template in source projects that are based on it.

## To-Do

- [x] Command-line tool that supports Jinja style for creating projects from templates.
- [ ] Support updates to templates for existing project files.