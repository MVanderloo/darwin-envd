# darwin-envd

This was designed as a declarative way to define environment variables in a shell initialization script. This is my way of replacing environment.d feature set that most Linux systems have.

## Usage

envd supports the following variable expansion syntaxes:

- $VAR - Simple variable expansion
- ${VAR} - Braced variable expansion
- ${VAR:-fallback} - Variable with fallback if unset or empty
- $(command) or \`command\` - Command substitution

Put that file in `~/.config/environment.d/*.conf`, or any of the directories specified on your path. All files in this directory will be read in alphabetical order and printed for your shell to interpret.

Since child processes cannot set environment variables for parent processes, you must source the result of this script. This is the syntax for supported shells.

### Bash
source $(envd)

### Zsh
source $(envd)

### Fish
envd | source



