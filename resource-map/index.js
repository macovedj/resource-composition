import { resourceMap } from "./resource_map/resource_map.js"
import { _setPreopens } from '@bytecodealliance/preview2-shim/filesystem';
_setPreopens({"./foo": "./foo"})
const inst = new resourceMap.ResourceMap()

inst.insert("Danny", "founder")
inst.insert("Oscar", "evangelist")
console.log(inst.get("Danny"))
console.log(inst.keys())