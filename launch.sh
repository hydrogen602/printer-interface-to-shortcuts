#!/bin/sh
# runs as root from systemd

# set -eu

setpriv --reuid=jonathan --regid=jonathan --clear-groups --inh-caps=-all /home/jonathan/printer-actions/launch2.sh

# su jonathan -c 'cd /home/jonathan/printer-actions && cargo run > run.log 2>&1'

#/home/jonathan/.cargo/bin/
