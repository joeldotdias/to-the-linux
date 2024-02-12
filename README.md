# ttl (To The Linux)

ttl is a lightweight command line utility that lets you execute linux commands on windows. The syntax and output have been tweaked a little to be more understandable.<br>
This will (hopefully) be my last project on Windows.

## Syntax
Every command is prefixed with `ttl`.<br>
A basic skeleton of each command would look like `ttl <command> <opts>`

- **cat**
    - Reading a file<br>
    `ttl cat <filename>` displays the contents of the file
    - Writing to a file<br>
    `ttl cat to <filename>` takes input from stdin and overwrite the contents of a file<br>
    `ttl cat on <filename>` takes input from stdin and appends it to the exisiting file contents<br>
    Type `:q` to exit stdin and write to the file.

    - **Examples**
    <img alt="ttl cat example" src="/images/cat.png" />

