#!/bin/bash

cargo build --release

# hyperfine --warmup 3 --export-markdown performance.md \
#     --prepare 'bash test_data_gen.sh && sleep 2' \
#     'rm -rf ./test_data' \
#     './target/release/frm ./test_data'

hyperfine --export-markdown performance.md \
    --prepare 'bash test_data_gen.sh && sleep 2' \
    'rm -rf ./test_data' \
    './target/release/frm ./test_data'

rm -rf ./test_data
