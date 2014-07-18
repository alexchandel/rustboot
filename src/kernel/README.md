## kernel
### Files
```
kernel/
├── elf.rs  ELF
├── int.rs  Integer
├── memory
│   ├── allocator.rs    Buddy memory allocator
│   ├── mod.rs
│   └── virtual.rs
├── mod.rs      Kernel
├── ptr.rs      Pointer (mut_offset)
├── README.md   this document
└── rt.rs       Runtime
```

### Memory allocator: `memory/allocator.rs`

The [buddy memory allocation[1]][1] system is implemented with the use of a binary tree.

1: http://en.wikipedia.org/wiki/Buddy_memory_allocation
