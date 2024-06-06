### Draftlang

Draftlang is a functional programming languaue that allow users to expose custom system calls via a `json` interface. Example

```
[draft]
import utils::{mkfile, rmFile}

func createFile(){
    mkfile("app.txt")
}

func renameFile(){
    rmFile("app.txt", "new_name.txt")
}

[/draft]

[exec]

{
    "makeFile":"createFile()",
    "renameFile":"renameFile()",
    "parallelRun":"createFile() || renameFile()"
}

[/exec]

```

To execute functions we can then call the command that exposes the `draftlang` function\

`draftlang exec makeFile parallelRun`

This repository would contain
- `asft-util`: A custom wrapper for the [ASDF CLI] (https://asdf-vm.com/) tool which would be vital for runtime managment

- `draftlang-parser`: This workspace should contain the grammer parser and lexical analysis of the token.

- `draftlang-executor` : This should contain the executor that is responsible for interpreting the language. 