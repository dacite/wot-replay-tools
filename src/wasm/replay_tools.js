import * as wasm from "./replay_tools_bg.wasm";
import { __wbg_set_wasm } from "./replay_tools_bg.js";
__wbg_set_wasm(wasm);
export * from "./replay_tools_bg.js";
