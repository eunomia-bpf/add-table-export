# add-table-export

A tool to add an export of the table defined in the module.

For example, for this wasm module (displayed in text format):

```wat
(module
  (type (;0;) (func))
  (func $__wasm_call_ctors (;0;) (type 0))
  (func $_initialize (;1;) (type 0)
    call $__wasm_call_ctors
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global $__stack_pointer (;0;) (mut i32) i32.const 66560)
  (export "memory" (memory 0))
  (export "_initialize" (func $_initialize))
)
```

After running `add-table-export test.wasm -o test-added.wasm`, It will comes to:

```wat
(module
  (type (;0;) (func))
  (func $__wasm_call_ctors (;0;) (type 0))
  (func $_initialize (;1;) (type 0)
    call $__wasm_call_ctors
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global $__stack_pointer (;0;) (mut i32) i32.const 66560)
  (export "memory" (memory 0))
  (export "_initialize" (func $_initialize))
  (export "__indirect_function_table" (table 0))
)
```
