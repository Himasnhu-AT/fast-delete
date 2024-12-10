### Potential Improvements

1. **Reduce System Time**: Investigate ways to reduce the time spent in the kernel. This could involve optimizing file deletion operations and minimizing synchronization overhead.
2. **Optimize Thread Management**: Ensure that the thread pool is efficiently managing threads and not creating unnecessary overhead.
3. **Batch Deletion**: Consider batching file deletions to reduce the number of system calls. This could help reduce the system time.
4. **Profile the Code**: Use profiling tools to identify bottlenecks in the code and optimize them. Tools like `perf`, `valgrind`, or Rust-specific profilers can be helpful.
5. **Error Handling**: Ensure that error handling is efficient and does not introduce unnecessary delays.
