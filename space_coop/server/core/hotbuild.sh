#/usr/bin/env bash

# A small script to autobuild //space_coop/server/core
#
# If running with ./bazel-bin/space_coop/server/launcher/launcher, the code will be automatically loaded once compilation completes

while inotifywait -r -e modify,close_write,move,create,delete "./space_coop" "./third_party"
do
bazel build //space_coop/server/core
done
