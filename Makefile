.PHONY: clippy remote-run

# EXE_FILE := target/arm-unknown-linux-gnueabihf/debug/printer-actions
# STAGE_FILE := target/arm-unknown-linux-gnueabihf/printer-actions

# ${EXE_FILE}: $(shell find src -type f)
# 	cross build --target=arm-unknown-linux-gnueabihf

# ${STAGE_FILE}: ${EXE_FILE}
# 	rm -f "${STAGE_FILE}"
# 	upx -o "${STAGE_FILE}" "${EXE_FILE}"

# install-exe: ${STAGE_FILE}
# 	scp "${STAGE_FILE}" octopi:
# 	touch install-exe

# # install-static: $(shell find static -type f)
# # 	scp -r static octopi:
# # 	touch install-static

# install: install-exe #install-static

# remote-run: install
# 	ssh -t octopi RUST_BACKTRACE=1 ./printer-actions

copy:
	rsync -r . octopi:~/printer-actions

clippy:
	__CARGO_FIX_YOLO=1 cargo clippy --fix --allow-staged 
