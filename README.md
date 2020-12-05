# PassUp

Automatically update passwords from common Password Managers. Supported Password Managers are:
- pass
- KeePass(kdbx)
- KeePassX(kdbx)
- KeePassXC(kdbx)

## Getting Started
### Prerequisites

## Config file
Allows you to choose between pass and the kdbx variant.
### Example configurations:
#### kdbx
```
type_of_db = "kdbx"
path_to_kdbx = "path_to_kdbx_file"
kdbx_password = "password"
script_dir = "path_to_script_directory"
blacklist = ["url1", "url2", ...]
```
#### pass
```
type_of_db = "pass"
script_dir = "path_to_script_directory"
blacklist = ["url1", "url2", ...]
```
Store the config file as a ".toml".

## Program Arguments
The program takes one argument which is the path to the config file.