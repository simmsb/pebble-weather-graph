dev:
  set -euo pipefail; \
  pebble build; \
  pebble install --emulator emery --logs -v --platform emery --pypkjs & EMULATOR_PID=$! ; \
  trap "kill $EMULATOR_PID 2>/dev/null" EXIT INT TERM; \
  watchexec -p -i build -e rs,c,h,ts,json "pebble build && pebble install"


build:
    pebble build


build_release:
    env EXTRA_CARGO_ARGS="--no-default-features" pebble build

push: build_release
    pebble install --cloudpebble --logs
