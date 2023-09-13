#!/bin/sh

set -eu

su jonathan -c 'cd /home/jonathan/printer-actions && cargo run > run.log 2>&1'

#/home/jonathan/.cargo/bin/
