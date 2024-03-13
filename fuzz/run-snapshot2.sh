
fuzz() {
    cargo +nightly fuzz run -j $(nproc) $1 -- -max_total_time=$2 -timeout=2 -max_len=36 -len_control=1
}


fuzz snapshot2 300
