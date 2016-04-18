#!/usr/local/bin/bash
cargo build --release
for ratio in {1..100..10};
do
    echo "Running ratio: $ratio"
    ipython generate_graph.py -- --connect-ratio $ratio test.txt
    for i in {1..5}
    do
        target/release/max_flow dfs dicaps test.txt >> data/performance/experiments.txt
    done

    for i in {1..5}
    do
        target/release/max_flow bfs dicaps test.txt >> data/performance/experiments.txt
    done
done
