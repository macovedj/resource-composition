import {sorter} from "./fullsort/fullsort.js"
import { _setPreopens } from '@bytecodealliance/preview2-shim/filesystem';
_setPreopens({"./foo": "./foo"})

console.log(sorter.sort())