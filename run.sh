#!/bin/bash
cargo objcopy --release -- -O binary target.bin

pushd ~/vc/jq_flasher
# update the firmware path to match yours
python3 main.py --chipname bl702 --port /dev/ttyUSB0 --firmware ~/vc/rust/bl702/bl702-test/target.bin
popd
