# Autoenv rewrite in rust

wraps `cd` to automatically run `.env` file when moving in directory and `.env.leave` when leaving directory

## BUGS


## REFACTOR
- type for env files + merge get_env functions
- encapsulate logic for sourcing env files
- implementation of rev() for ancestors() ?


## Installation
-[ ] automatically create ~/.autoenv-rs/bash_autocd
-[ ] add source bash_autocd to bashrc
-[ ] optionally add alias to cd


## `autoenv` functionalities

-[x] `cd` wrapper
-[x] source `.env`
-[x] source `.env.leave`
-[ ] manual authorization of `.env` and `.env.leave` files
-[ ] dialog on first run to authorize files
-[ ] show file content on authorization

-[ ] custom cd
-[ ] custom env file names
-[x] walk through dirs to activate upper folders env (properly source `.env` from its folder on the way, unlike autoenv)
-[x] walk through dirs to deactivate non common ancestors `.env.leave` (properly source `.env.leave` from its folder on the way, unlike autoenv)


## additional functionalities?
-[ ] .env.exclusive file that's only active for the exact folder
-[ ] .env.leave.exclusive file that deactivate when leaving the exact folder

-[ ] .env.override to not apply ancestors env ?

-[ ] add `cd -v` to display sourced envs


## Tests
### framework
-[ ] makefile or cargo test can run bash tests?
-[ ] bash test suite

### regressions
-[ ] in and out of env
-[ ] nested folders of env
-[ ] in and out of env from/to nested folder
-[ ] on load in all places
-[ ] time cd from deeply nested folder to deeply nested folder with many uncommon ancestors with many envs
-[ ] source `.env` from ancestors to children
-[ ] source `.env.leave` from children to ancestors
-[ ] `cd -` is coherent (last folder before running autocd)

### to implement
-[ ] authorize new env files
-[ ] remove env should remove authorization

-[ ] don't run parent env if already done ? (autoenv does, autoenv-rs too for now)


## `cd` wrapper

-[x] `cd [dir]` + error code 0 if changed dir, 1 otherwise
-[x] `cd` -> `$HOME`
-[x] `-L`
-[x] `-P` + error 0 if changed and `$PWD` successfully set
-[x] `-e`
-[x] `-@`

doc
```
cd: cd [-L|[-P [-e]] [-@]] [dir]
    Change the shell working directory.
    
    Change the current directory to DIR.  The default DIR is the value of the
    HOME shell variable. If DIR is "-", it is converted to $OLDPWD.
    
    The variable CDPATH defines the search path for the directory containing
    DIR.  Alternative directory names in CDPATH are separated by a colon (:).
    A null directory name is the same as the current directory.  If DIR begins
    with a slash (/), then CDPATH is not used.
    
    If the directory is not found, and the shell option `cdable_vars' is set,
    the word is assumed to be  a variable name.  If that variable has a value,
    its value is used for DIR.
    
    Options:
      -L	force symbolic links to be followed: resolve symbolic
    		links in DIR after processing instances of `..'
      -P	use the physical directory structure without following
    		symbolic links: resolve symbolic links in DIR before
    		processing instances of `..'
      -e	if the -P option is supplied, and the current working
    		directory cannot be determined successfully, exit with
    		a non-zero status
      -@	on systems that support it, present a file with extended
    		attributes as a directory containing the file attributes
    
    The default is to follow symbolic links, as if `-L' were specified.
    `..' is processed by removing the immediately previous pathname component
    back to a slash or the beginning of DIR.
    
    Exit Status:
    Returns 0 if the directory is changed, and if $PWD is set successfully when
    -P is used; non-zero otherwise.
```
