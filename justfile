dev:
  pebble install --emulator emery --logs & EMULATOR_PID=$! ; \
  trap "kill $EMULATOR_PID 2>/dev/null" EXIT INT TERM; \
  watchexec -p -i build -e c,h "pebble build && pebble install"
