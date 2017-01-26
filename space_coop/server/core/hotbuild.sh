#/usr/bin/env bash

while inotifywait -r -e modify,close_write,move,create,delete "./space_coop" "./third_party"
do
bazel build //space_coop/server/core
done
