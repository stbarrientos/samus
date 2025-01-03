# Samus
Memory-only storage service inspired by Redis

### Goals


### Development
All you need is docker.
` docker compose run -it dev bash`

With the server running, run a simple test from another terminal with:
`cat sample_commands.txt | nc 127.0.0.1 6666`

Note: you must have `nc` installed on your system (should be there on most Unix systems)

