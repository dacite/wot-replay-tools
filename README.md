# [Wot Replay Tools](https://dacite.github.io/wot-replay-tools)

This application aims to provide some functionalities to visualize data from `.wotreplay` files. Thanks to WebAssembly and Rust, replays are ready to visualize almost instantly. The application runs entirely in your browser and is a essentially a static website. 

Currently, it does not provide much utility as it is a **work in progress**.

**Use here**: https://dacite.github.io/wot-replay-tools
## Features
- Very basic display of tank positions

If you are looking to visualize the packet information of a replay see [Wot Packet Analyzer](https://dacite.github.io/wot-packet-analyzer)
# Credits
- https://github.com/Gabouchet/wot-game-reader
    - Map bounding boxes were retrieved using this library
- https://github.com/evido/wotreplay-parser
    - Initial algorithm for converting cartesian coordinates to HTML5 Canvas coordinates comes from here
