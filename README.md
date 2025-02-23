# autoenv-rs

Automatically switch environments when moving in file tree

I love [autoenv](https://github.com/hyperupcall/autoenv), but it's been running very slowly on my system, so I wrote a faster alternative with its main features only.

# Usage
Like the original autoenv:

```bash
$ echo "echo 'whoa'" > ./project/.env
$ cd ./project
whoa
$ echo "echo ':O'" > .env.leave
$ cd ..
:O
```

# Installation
## Latest release
Download [latest release](https://github.com/gaheldev/autoenv-rs/releases/latest) and run `install` script.
Finally add the following to your .bashrc:
```
source ~/.autoenv-rs/bash_autocd"
alias cd=__autoenv_rs_cd__"
```

## From source
```
git clone git@github.com:gaheldev/autoenv-rs.git
cd autoenv-rs
./install
```

Add to your .bashrc:
```
source ~/.autoenv-rs/bash_autocd"
alias cd=__autoenv_rs_cd__"
```
