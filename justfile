mod server
mod db

set dotenv-required := true
set dotenv-load := true

default:
  just --list server
  just --list db
