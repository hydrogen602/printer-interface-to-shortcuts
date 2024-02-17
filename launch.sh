#!/bin/sh
# runs as root from systemd

# Runs launch2.sh as user jonathan instead of root (aka drop privileges)
setpriv --reuid=jonathan --regid=jonathan --clear-groups --inh-caps=-all /home/jonathan/printer-actions/launch2.sh
