# Ruetta
[crates.io](https://crates.io/crates/ruetta)

Rust cli tool to replace [hygen](https://github.com/jondot/hygen) templates, which is unmaintaned.

## Usage
```sh
ruetta <command> [arguments]
```

### init
Create the template folder.

`ruetta init`


### clean
Remove the template folder.

*Aliases: cl*

`ruetta clean`


### info
Show information about a template.

*Aliases: i*

`ruetta info svelte component`


### create
Create a new template definition.

*Aliases: c*

`ruetta create`


### make
Generate files from a template.

*Aliases: m, mk*

`ruetta make svelte component Button ./src/components`

Optional flags:

`--dry-run` - just showcase what files would be generated without actually generating them.

`--force` - force override if file exists.

`--vars="some:var,another:1"` - defines some variables that can be injected into the template


### help
Show this help message.

*Aliases: h, -h*

`ruetta help`


### version
Show the application version.

*Aliases: v, -v, --version*

`ruetta version`

## Config File
The config file will be generate on `init` and will look like this:
```json
{
  "folder": "/home/you/some/folder",
  "aliases": {}
}
```

**Folder** contains the templates that you will define.
**Aliases** is just a simple helper to define custom aliases, for example if you want to make a `ruetta mk svelte comp` behave like a `ruetta mk sv c` you can define those as alias.
```json
"aliases": {"sv": "svelte", "c": "comp"}
```



## Template Definitions
Example template for a svelte component in [./examples/svelte/c](./examples/svelte/c)

They will need to be defined in the folder specified in the config and ruetta will look them up like so:
`[LANGUAGE] [TEMPLATE]`

```
└── svelte
    ├── c
    │   ├── index.ruetta
    │   └── types.ruetta
    └── store
        └── index.ruetta
```
In this example we have got `svelte` as *Language* and `c` or `store` as *templates*.

There MUST always need to be an `index.ruetta` and if there are multiple files for that template you need to specify them inside the index.

This is the index file
```ejs
---
description: a simple Svelte Component
to: <%- folder %>/<%= Name %>.svelte
append_after: something
append: true
exclude_if: something
files:
    - types
---
<script lang="ts">
  import { type <%= Name %> } from "./types.ts";
  type Props = {
    <%= name %>: <%= Name %>
  };
  const {<%= name %>}: Props = $props();
</script>

<h3><%= Name %></h3>
<% if (some_var) { %>
<h1>Stuff</h1>
<% } %>
```
Each template folder can have multiple files but always needs an `index.ruetta` file.
Each file has a [Frontmatter](https://jekyllrb.com/docs/front-matter/) part for the template configuration and a **body**, that will be rendered in the file.

On the **Frontmatter**:

**description**: will be shown on **info** command.

**to**: will be computed to set the destination of this file.

**files**: will only be read if it is an index file and specified if any other files in this template and maintain the order whilst generating them.

**append** (**append_after**): if the file exists, it will render then append the content, if *append_after* is defined will try to append the content after a particular string you defined.

**exclude_if**: will prevent a file to be written if a certain variable is specified.

On the body, you can use [EJS](https://ejs.co/) syntax.
