# Libsql SIGSEGV Minimal Example

1. Build the extension.

     $ gcc vendor/libsql/libsql-sqlite3/ext/misc/uuid.c -shared -fPIC -o uuid.so

2. Run in debug mode
   
     $ cargo run 
     > done

3. Run in release mode

     $ cargo run release
     > [2]    2772584 segmentation fault  cargo run --release

3. Running in the debugger yields.

```
    Thread 1 "libsql_ext" received signal SIGSEGV, Segmentation fault.
    0x00005555558a2697 in sqlite3_db_config ()
    (gdb) bt
    #0  0x00005555558a2697 in sqlite3_db_config ()
    #1  0x0000555555708708 in <libsql::local::impls::LibsqlConnection as libsql::connection::Conn>::enable_load_extension::hc6f7c4e1278f41bb ()
    #2  0x00005555555e5876 in libsql::connection::Connection::load_extension_enable::h8612d57f81fc2354 ()
    #3  0x00005555555c5b0d in tokio::runtime::park::CachedParkThread::block_on::hac19db9a63fd35de ()
    #4  0x00005555555cc26b in tokio::runtime::context::runtime::enter_runtime::hc2f44aa6ccc4bc56 ()
    #5  0x00005555555c6644 in tokio::runtime::runtime::Runtime::block_on::he7bdfecffcc7c81e ()
    #6  0x00005555555cb852 in libsql_ext::main::h602b23daecb54968 ()
    #7  0x00005555555c77d3 in std::sys_common::backtrace::__rust_begin_short_backtrace::h293d6abbb561b7f2 ()
    #8  0x00005555555c77e9 in std::rt::lang_start::{{closure}}::h949cdbbb0275ee68 ()
    #9  0x00005555559e62cd in std::rt::lang_start_internal::h4838558278cab352 ()
    #10 0x00005555555cb935 in main ()
```
