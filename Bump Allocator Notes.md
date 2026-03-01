## Bump Allocator Notes.

A bump allocator is a type of memory allocator.  
In computing, a memory allocator is a program (or component of a runtime/system) that is in charge of managing memory resources in a computer system. The term _managing_ involves both allocating and deallocating memory. [1](https://www.sciencedirect.com/topics/computer-science/memory-allocator#:~:text=A%20memory%20allocator%20is%20a,%2C%20thread%2C%20and%20pipe%20structs.) 

An analogy that describes its role is a caterer that serves requests (measured in bytes) when an order is placed by a user manually or automatically by a system. There are different memory allocator designs; a bump allocator is one of the simplest.

### Definition:

A bump allocator is a type of allocator that uses a predetermined fixed memory region along with a maintained pointer such that, when memory is requested, the pointer is moved (incremented) by the requested memory size. It does not support individual deallocation of memory chunks (rather, it frees all allocations at once by resetting the pointer to the beginning of the region). [2](https://os.phil-opp.com/allocator-designs/) It is especially useful when the objects required are short-lived or exist in short phases.

### Unique Feature:

In a bump allocator, memory can be viewed as contiguous chunks placed sequentially in memory. 

When memory of a certain size is requested, if the requested memory size is not a multiple of the allocator's alignment (commonly 4, 8, or 16 bytes), it is rounded up to the next multiple of that alignment. For example, if a program requires 22 bytes and the alignment is 8 bytes, the bump allocator will allocate 24 bytes (not 22). If the alignment is 16 bytes, it may allocate 32 bytes. 

The small memory difference can be seen as wastage of memory (internal fragmentation), but the trade-off is that memory chunks are ordered side by side (sequentially) in memory, which allows for very fast allocation and excellent cache locality for the CPU. 

Padding is often used by the compiler internally to ensure structure members are aligned. The alignment number (usually a power of 2) is largely determined by the hardware architecture of the system. [5](https://en.wikipedia.org/wiki/Data_structure_alignment#:~:text=the%20host%20machine.-,Computing%20padding,original%20value%20is%20left%20unchanged)

### Use Cases:

Bump allocators are primarily used in programs that require high throughput (processing large volumes of data in a short time) or in short-lived programs.

- In graphics, a bump allocator will allocate a large chunk of memory for a frame, then free it before moving to the next frame, repeating the process for subsequent frames.
- In computing, short-lived memory within iterative loops can be efficiently managed by a bump allocator. When moving to the next iteration, the memory is freed/reset and made available for reuse.
- When handling an HTTP connection request, a bump allocator can be used such that once the connection is closed, the entire memory region is reset.

### Side Note:

- Bump allocators fall under the arena allocation technique, also known as region-based memory management. [3](https://en.wikipedia.org/wiki/Region-based_memory_management) [4](https://onlinelibrary.wiley.com/doi/epdf/10.1002/spe.4380200104)

### Real World Examples:

1. [A Fast, Bump-Allocated Virtual DOM with Rust and Wasm](https://hacks.mozilla.org/2019/03/fast-bump-allocated-virtual-doms-with-rust-and-wasm/#conclusion)
2. Game Engines.
3. Web Servers.

### References:

1. Research paper: https://www.sciencedirect.com/topics/computer-science/memory-allocator#:~:text=A%20memory%20allocator%20is%20a,%2C%20thread%2C%20and%20pipe%20structs.
2. OS written in Rust: https://os.phil-opp.com/allocator-designs/
3. Wikipedia: https://en.wikipedia.org/wiki/Region-based_memory_management
4. Research paper: https://onlinelibrary.wiley.com/doi/epdf/10.1002/spe.4380200104
5. Wikipedia: https://en.wikipedia.org/wiki/Data_structure_alignment#:~:text=the%20host%20machine.-,Computing%20padding,original%20value%20is%20left%20unchanged.