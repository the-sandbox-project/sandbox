<img src="https://user-images.githubusercontent.com/98240335/234269982-b28309a1-7ad6-4074-be70-6b260c8d625d.png" align="right" />

# Sandbox 
> Welcome to the Sandbox Project, a way to test different programming languages and projects

[![asciicast](https://asciinema.org/a/kFOxcFwQDuUXJbHQAayYfoQeQ.svg)](https://asciinema.org/a/kFOxcFwQDuUXJbHQAayYfoQeQ)

Table of Contents
========
 * [Installation](#installation)
 * [Usage](#usage)
 * [Configuration](#configuration)


Installation
========
### Windows
Head to the [releases](https://github.com/the-sandbox-project/sandbox/releases) page and download the [MSI build](https://github.com/the-sandbox-project/sandbox/releases/download/v1.0.1/sandbox-0.1.1-x86_64.msi). From there, install it like any other program!

### MacOS / Linux

> **Note**
> Plans have been made to add sandbox to [Homebrew](https://brew.sh) and the [AUR](https://aur.archlinux.org) 
```
$ wget https://github.com/the-sandbox-project/sandbox/releases/download/v0.2.1/sandbox -O ~/.local/bin
```


Usage
========
### Help
> sandbox --help
This is the default Help command, running will produce:
```
Usage: sandbox [OPTIONS]

Options:
  -n, --new <NEW>              Create a New Environment [default: ]
  -S, --search <SEARCH>        Search for Environment [default: ]
  -I, --install <INSTALL>      Search for Environment [default: ]
  -U, --uninstall <UNINSTALL>  Create a New Environment [default: ]
  -R, --reinstall <REINSTALL>  Reinstall an Environment [default: ]
  -C, --clearcache             Clear the Install Cache
  -h, --help                   Print help
  -V, --version                Print version
```

### New
> sandbox --new `<ENVIRONMENT>`

This is the New command, it will start a new session with a specified environment and go back to a previously edited environment if one has been created before.

Ex:
```
$ sandbox --new rust-min
```
Output:
```
	                                   __                
              ___     ___    ___   __  __ /\_\    ___ ___    
             / _ `\  / __`\ / __`\/\ \/\ \\/\ \  / __` __`\  
            /\ \/\ \/\  __//\ \_\ \ \ \_/ |\ \ \/\ \/\ \/\ \ 
            \ \_\ \_\ \____\ \____/\ \___/  \ \_\ \_\ \_\ \_\
             \/_/\/_/\/____/\/___/  \/__/    \/_/\/_/\/_/\/_/


                                (rust-min)

                                📂 Find File
                                📄 Recents
                                🔎 Find Word
                                🔖 Bookmarks
                                ↗️  Last Session
```

### Search
> sandbox --search `<ENVIRONMENT>`

This is the Search command, you will use this to search the [templates repository](https://github.com/the-sandbox-project/sandbox-templates) and find a template that interests you!

Ex:  
```
$ sandbox --search minimal
```
Output:
```bash
    ❌ Default Rust Project (rust-min) - Minimal Rust Project, Created with Cargo new
    ✅ Default Golang Project (go-min) - Minimal Golang Project

    Environments That Match Query minimal
    Install any of these with sandbox install <ENVIRONMENT>
```


### Install
> sandbox --install `<ENVIRONMENT>`

This is the install command! You will use this to install the environments you find in the search field.

Ex:
```
$ sandbox --install rust-min
```

Output:
```
  [00:00:00] [#######################################] 340B/340B (0.0s)
  Installed rust-min! Test it out with: sandbox --new rust-min
```

Configuration
========
Sandbox has a configuration file

### Windows
Windows's Sandbox Configuration file is stored in the Appdata:
> %APPDATA%/sandbox/sandbox.yml

### MacOS / Linux
MacOS and Linux share the same config file location! You are able to find it here:
> ~/.config/sandbox/sandbox.yml


```yml
# sandbox.yml
editor:
  editorName: "code"
```

> **Note:** the `editorName` field is the editor command for any editor, for example: Neovim would be `nvim`
