#!/bin/bash
rsync --exclude target -r ./ hack@192.168.50.33:/home/hack/
ssh hack@192.168.50.33
# http://192.168.50.33:3000/camera
# ssh-copy-id hack@192.168.50.33

# on the pi - cargo run --release
# pass breakup-reopen