# autoenv-rs

Automatically switch environments when moving through the file system.

I love [autoenv](https://github.com/hyperupcall/autoenv) but it's been running very slowly on my system.\
`autoenv-rs` is a faster alternative for bash.

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
![omg](https://github.com/user-attachments/assets/b8dc97da-27c1-4932-a9e0-1f934e4965d1)

<details>
  <summary>Automatically switch python environments</summary>
  
  ```bash
  echo "source venv/bin/activate" > .env
  echo "deactivate" > .env.leave
  ```
</details>

<details>
  <summary>Show which environments are sourced</summary>
  
  ```bash
  cd -v /path/to/another/dir/
  ```
</details>


> [!Caution]
>  no authorization is asked before sourcing env files

# Installation

Add the following to your .bashrc:
```bash
source ~/.autoenv-rs/bash_autocd
alias cd=__autoenv_rs_cd__
```

## Latest release
Download [latest release](https://github.com/gaheldev/autoenv-rs/releases/latest) and run `install` script.

## From source
Requires a working rust environement to build the project
```bash
git clone git@github.com:gaheldev/autoenv-rs.git
cd autoenv-rs
./install
```
