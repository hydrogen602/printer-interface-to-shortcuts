# Project

Provides a simple interface to complex common actions via the octoprint API

### Makefile

`copy` - Copies the rust source to the octoprint server

`remote-run` - Runs the code on the octoprint server interactively, i.e. can be quit with ^C

`run` - Connects to the octoprint server, copies over the code, kills the current process and spawns the new one

`clippy` - Runs the clippy linter on the code with `__CARGO_FIX_YOLO`

`read-log` - Reads the log file from the process spawned by `run`

`kill-remote` - Kills the existing process on the octoprint server