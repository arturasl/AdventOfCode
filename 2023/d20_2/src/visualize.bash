#!/bin/bash

cargo run main.rs < large.in | tee out.dot && dot out.dot -Tsvg > out.svg && chromium out.svg
