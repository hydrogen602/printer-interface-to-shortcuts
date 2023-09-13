.PHONY: clippy remote-run

copy:
	rsync -r src Cargo.toml Cargo.lock .env printer-actions.service launch.sh octopi:~/printer-actions

# remote-run:
# 	ssh -t octopi "cd ~/printer-actions && RUST_BACKTRACE=1 cargo run"

# kill-remote:
# 	ssh octopi 'kill $$(pidof printer-actions)'

# spawn:
# 	ssh octopi 'cd ~/printer-actions && nohup cargo run > run.log 2>&1 &'

clippy:
	__CARGO_FIX_YOLO=1 cargo clippy --fix --allow-staged 

# run:
# 	$(MAKE) copy 
# 	$(MAKE) kill-remote || echo "No process to kill"
# 	$(MAKE) spawn

read-log:
	ssh -t octopi 'tail -n 30 -f ~/printer-actions/run.log'

# see readme on how to run