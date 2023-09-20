# FAFFinder
A tool that searches through your disc using a set of Regex strings. Can either look for file titles or look through file content.

## Example
* `faffinder Cargo\.toml` Find all files in the current directory and subdirectories that fullfills the regex `Cargo\.toml`.
* `faffinder -c Cargo\.toml` Find all files in the current directory and subdirectories whose content fullfills the regex `Cargo\.toml`.
* `faffinder -p <Path> ...`  Search from the Path.