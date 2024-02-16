# napi-leak

Reproduction steps:
- yarn build
- yarn napi-leak

Then find the process, and attach something to check leaks on it. It would probably work with valgrind on Linux, here are my steps on MacOS:
- ps -a | grep node
- leaks pid | less