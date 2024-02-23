# ttl (To The Linux)

ttl is a lightweight command line utility that lets you execute Linux (or Unix based) commands on Windows. The syntax and output have been tweaked a little to be more understandable.<br>
The intention behind making this project is simply to build a cli tool. This README is being written just in case anyone actually ends up using it. This will (hopefully) be my last project on Windows.

## Syntax
Every command is prefixed with `ttl`<br>
A basic skeleton of each command would look like `ttl <command> <opts>`

- **cat**
    - Reading a file<br>
    `ttl cat <filename>` displays the contents of the file.
    - Writing to a file<br>
    `ttl cat to <filename>` takes input from stdin and overwrite the contents of a file.<br>
    `ttl cat on <filename>` takes input from stdin and appends it to the exisiting file contents.<br>
    \> and \>> are not used simply because I have overwritten the file contents quite a few times when I intended to append. On the other hand, *to* and *on* make for a slightly more understandable syntax and might limit mistakes to some extent.<br>
    Type `:q` to exit stdin and write to the file because Ctrl + D does not signify EOF on Windows.

    - **Examples** <br>
      <img alt="ttl cat example" height="270" src="/images/cat.png" />

- **wc**
    - wc without any flags will return the number of lines, words and bytes in the file.<br>
      `wc <filename>` displays lines, wordd and bytes in the flag
    - You can add \-l, \-w or \-b to only get the info you need. Flags can be stacked to get multiple sets of info.<br>
      `wc -<flags>` display only the info specified in the flags.
    - You can get info for multiple files simply by writing multiple files.<br>
      `wc -<flags> <filename 1> <filename 2> ... <filename n>`
    - **Examples** <br>
      <img alt="ttl cat example" height="170" src="/images/wc.png" />
      
      
      
      
