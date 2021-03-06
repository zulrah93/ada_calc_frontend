# About Staking Calculator Frontend

This is a Rust web application using the Actix Web Framework. Using the ada_calc CLI tool as a backend. Goal is to be performant and provide a nice web interface to estimate staking gains. 

# Supported Platforms

Supports all operating systems that Rust supports unless a dependency is specific to a platform although I will try my best as maintainer to not do that. If you feel a dependency is locking you out of using it please open a ticket in GitHub and I will see if we can find an alternative.

# Disclaimer

Like the command line tool, this is not investment advice. All forms of investment carries risk. This software and its maintainer are not responsible for any possible losses. Due to the nature of floating point math there will be errors in the calculation. So please use this tool for estimations; feel free to fork or contribute.

# Requirements

Rust must be installed on the target system. Please refer to https://www.rust-lang.org/tools/install

# Install via Powershell (Never Execute Script Without Understanding Risk)

```
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/zulrah93/ada_calc_frontend/master/install.ps1'))
```

# Install via Bash (Never Execute Script Without Understanding Risk)

```
sh -c "$(curl -fsSL https://raw.githubusercontent.com/zulrah93/ada_calc_frontend/master/install.sh)"
```

# How to Build and Run From Source

```
    git clone https://github.com/zulrah93/ada_calc_frontend.git
    cd ada_calc_frontend
    cargo run --release
```

