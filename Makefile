.PHONY: clippy copy copy-service restart

copy:
	rsync -r src build.rs Cargo.toml Cargo.lock .env launch.sh launch2.sh octopi:~/printer-actions

copy-service:
	rsync -r printer-actions.service octopi:~/printer-actions

# remote-run:
# 	ssh -t octopi "cd ~/printer-actions && RUST_BACKTRACE=1 cargo run"

# kill-remote:
# 	ssh octopi 'kill $$(pidof printer-actions)'

# spawn:
# 	ssh octopi 'cd ~/printer-actions && nohup cargo run > run.log 2>&1 &'

clippy:
	__CARGO_FIX_YOLO=1 cargo clippy --fix --allow-staged 

restart:
	ssh -t octopi 'sudo systemctl restart printer-actions.service'

compile:
	ssh -t octopi 'cd ~/printer-actions && cargo build'

run:
	$(MAKE) copy 
	$(MAKE) compile
	$(MAKE) restart
# 	$(MAKE) kill-remote || echo "No process to kill"
# 	$(MAKE) spawn

read-log:
	ssh -t octopi 'tail -n 30 -f ~/printer-actions/run.log'

# see readme on how to run