#/usr/bin/env bash

# run like './hotbuild "bazel build //lib:lib"'

FULL_CMD="$1 --action_env=LD_LIBRARY_PATH=\"\$LD_LIBRARY_PATH\", --action_env=PATH=\"\$PATH\" --test_output=all --verbose_failures"

eval "$FULL_CMD"

while inotifywait -r -e modify,close_write,move,create,delete "./space_coop" "./third_party" "./common" "./network" "./tools" "./graphics"
do
  eval "$FULL_CMD"
done
