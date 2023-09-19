[package]
name = "aoc-autobench"
version = "0.4.0"
authors = ["Grégory Obanos <gregory.obanos@gmail.com>"]

[dependencies]
{CRATE_NAME} = { path = "../../.." }
aoc-runner = { git = "https://github.com/pedantic79/cargo-aoc.git", branch = "new-criterion", package="aoc-runner" }

[dev-dependencies]
criterion = "0.5"

{PROFILE}

[[bench]]
name = "aoc_benchmark"
harness = false
