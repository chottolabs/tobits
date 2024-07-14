# tobits
instead of building things from scratch, we'll do it top-down... it's
abstractions all the way down

## TODO

Not actually sure where I want to start tho... the interesting thing about
top-down is that I need to take responsibility for my own search heuristics
(i.e. beam width + cost)

- VLLM inference
  - Ray (distributed compute orchestration?)
  - continuous batching
  - some custom kernels (is it worth?)
  - specualtive decoding (not yet stable)
- vector search (ANN)
  - HNSW
  - spotify/voyager
- whisper pipeline
  - smarter speculative decoding
