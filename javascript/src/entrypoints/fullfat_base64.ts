import { UseApi } from "../low_level.js"
import * as api from "../wasm_bindgen_output/web/index.js"
//@ts-ignore
UseApi(api)

export * from "../stable.js"
export * as next from "../next_slim.js"
