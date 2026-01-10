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


### help
Show this help message.

*Aliases: h, -h*

`ruetta help`


### version
Show the application version.

*Aliases: v, -v, --version*

`ruetta version`


## Template Definitions
Example template for a svelte component in [./examples/svelte/c](./examples/svelte/c)

This is the index file
```ejs
---
description: a simple Svelte Component
to: <%- folder %>/<%= Name %>.svelte
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
```
Each template folder can have multiple files but always needs an `index.ruetta` file.
Each file has a [Frontmatter](https://jekyllrb.com/docs/front-matter/) part for the template configuration and a **body**, that will be rendered in the file.

On the **Frontmatter**:
**description**: will be shown on **info** command.
**to**: will be computed to set the destination of this file.
**files**: will only be read if it is an index file and specified if any other files in this template and maintain the order whilst generating them.

On the body, you can use [EJS](https://ejs.co/) syntax.
