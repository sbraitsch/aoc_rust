# Elf - Advent of Code CLI

## FAQ

### What can this do?
- Create boilerplate for new AoC Solutions
- Load your puzzle input (requires a session cookie)
- Submit solutions (**NYI**)
- Bootstrap a fully new Rust project with predefined structure and utility functions (**NYI**)
- Add boilerplate for a new year of solutions inside a previously bootstrapped project (**NYI**)

### How do I use it?
Simple Example: 
```
> cd dir/to/put/project
> elf bootstrap
> cd aoc/
> elf new -d=1
```

`elf bootstrap`:

| arg      | alt  | effect                                                                                                                                               |
|----------|------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| `--full` | `-f` | Flag to signal if a completely new project should be created. <br> If missing, a submodule for the year will be created inside the existing project. |
| `--year` | `-y` | The year to create a module for. <br> Defaults to current year if missing.                                                                           |

`elf new`:

| arg          | alt  | effect                                                                            |
|--------------|------|-----------------------------------------------------------------------------------|
| `--year`     | `-y` | The year to create a solution stub for. <br> Defaults to current year if missing. |
| `--day`      | `-d` | The day to create a solution stub for. <br> Defaults to current day if missing.   |
| `--template` | `-t` | Path to an optional template file to base the stub on.                            |
| `--root`     | `-r` | Path to the project root. <br> Required, if elf is called from somewhere else.    |

`elf submit`:

| arg      | alt  | effect                                                                        |
|----------|------|-------------------------------------------------------------------------------|
| `--year` | `-y` | The year of the solution to submit. <br> Defaults to current year if missing. |
| `--day`  | `-d` | The day of the solution to submit. <br> Defaults to current day if missing.   |
| `--part` | `-p` | The part of the solution to submit. <br> Defaults to 1 if missing.            |


