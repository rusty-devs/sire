# Ideas

## Phase 1: Initialization
* automation:
  * setup Github Actions for CI
  * publish a basic users guide to Github Pages on every build
  * publish API docs to docs.rs
  * publish crate to rust.io
  * publish binaries for MacOS, Linux and Windows to Github Releases
  * run unit tests on every build
  * run code formatter on every build
* implementation:
  * create basic structure for the project (tests, docs, code, etc.)
  * select initial set of dependencies (ie: YAML parser, templating engine, CLI interface lib, etc.)
  * provide basic CLI interface with simple set of parameters (ie: source folder for template, output folder)
  * parse a list of key-value pairs from a YAML formatted config file in the root of the source folder and use those as substitution variables for the templates
  * iterate over all files in the source folder, copying them to the target folder and applying the properties loaded from the YAML file to the contents of every file - using the templating engine chosed from a third party library
  * write up a simple README for the landing page on Github (ie: just a brief summary of what the tool is for and maybe an example of how it is to be used)
* need to add a license to the project (Apache 2.0)
* create a contributors guide for the project, summarizing basic coding standards and such


## Future Phases
* enhance substitution / templating logic to apply to files and folders in the template source in addition to the contents of those files
* add support for versioned templates (ie: to update a project from one version of a template to another)
* define a structured way to log debug output for the program
* define a structured way for handling errors in the templates that are loaded to help users debug problems
* add support for conditional operations to templates (ie: ability to turn on and off certain parts of a template, like selecting which CI tool to use when generating CI scripts from a template)
* add support for applying a template to an existing folder / project
* find some way for users to extend the functionality of sire so they can customize the behavior of the template engine and/or customize the logic needed to upgrade from one version of the template to another
* add support for pulling template source files from Git repositories (and maybe other locations like FTP sites, HTTP sites, etc.)