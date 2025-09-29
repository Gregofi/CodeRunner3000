# Evaluator

Can be run with `cargo build` and `cargo run`. Do note that it currently
requires root priviledges, since it edits cgroups.

## Language nuances

Some languages (their compilers or interpreters) have some nuances that need to be taken into account.

### Go

Go needs `GOCACHE` to be set to a folder.
The compiler then caches some files there.

For each Go version, we create a folder in `/opt/evaluator/compilers/go/<version>/cache` and run `go build std` for each version. The /opt/evaluator folder is mounted as RW volume, which is enough for the cache.
