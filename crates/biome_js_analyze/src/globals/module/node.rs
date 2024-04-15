/// Sorted array of Node builtin modules
///
/// Source: <https://github.com/inspect-js/is-core-module/blob/8317b311856a61935d7257ad5f31f9b0cfd13b5f/core.json#L1-L158>
pub const BUILTIN_MODULES: &[&str; 156] = &[
    "_debug_agent",
    "_debugger",
    "_http_agent",
    "_http_client",
    "_http_common",
    "_http_incoming",
    "_http_outgoing",
    "_http_server",
    "_linklist",
    "_stream_duplex",
    "_stream_passthrough",
    "_stream_readable",
    "_stream_transform",
    "_stream_wrap",
    "_stream_writable",
    "_tls_common",
    "_tls_legacy",
    "_tls_wrap",
    "assert",
    "assert/strict",
    "async_hooks",
    "buffer",
    "buffer_ieee754",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "diagnostics_channel",
    "dns",
    "dns/promises",
    "domain",
    "events",
    "freelist",
    "fs",
    "fs/promises",
    "http",
    "http2",
    "https",
    "inspector",
    "inspector/promises",
    "module",
    "net",
    "node-inspect/lib/_inspect",
    "node-inspect/lib/internal/inspect_client",
    "node-inspect/lib/internal/inspect_repl",
    "node:_http_agent",
    "node:_http_client",
    "node:_http_common",
    "node:_http_incoming",
    "node:_http_outgoing",
    "node:_http_server",
    "node:_stream_duplex",
    "node:_stream_passthrough",
    "node:_stream_readable",
    "node:_stream_transform",
    "node:_stream_wrap",
    "node:_stream_writable",
    "node:_tls_common",
    "node:_tls_wrap",
    "node:assert",
    "node:assert/strict",
    "node:async_hooks",
    "node:buffer",
    "node:child_process",
    "node:cluster",
    "node:console",
    "node:constants",
    "node:crypto",
    "node:dgram",
    "node:diagnostics_channel",
    "node:dns",
    "node:dns/promises",
    "node:domain",
    "node:events",
    "node:fs",
    "node:fs/promises",
    "node:http",
    "node:http2",
    "node:https",
    "node:inspector",
    "node:inspector/promises",
    "node:module",
    "node:net",
    "node:os",
    "node:path",
    "node:path/posix",
    "node:path/win32",
    "node:perf_hooks",
    "node:process",
    "node:punycode",
    "node:querystring",
    "node:readline",
    "node:readline/promises",
    "node:repl",
    "node:stream",
    "node:stream/consumers",
    "node:stream/promises",
    "node:stream/web",
    "node:string_decoder",
    "node:sys",
    "node:test",
    "node:test/reporters",
    "node:timers",
    "node:timers/promises",
    "node:tls",
    "node:trace_events",
    "node:tty",
    "node:url",
    "node:util",
    "node:util/types",
    "node:v8",
    "node:vm",
    "node:wasi",
    "node:worker_threads",
    "node:zlib",
    "os",
    "path",
    "path/posix",
    "path/win32",
    "perf_hooks",
    "process",
    "punycode",
    "querystring",
    "readline",
    "readline/promises",
    "repl",
    "smalloc",
    "stream",
    "stream/consumers",
    "stream/promises",
    "stream/web",
    "string_decoder",
    "sys",
    "test/reporters",
    "timers",
    "timers/promises",
    "tls",
    "trace_events",
    "tty",
    "url",
    "util",
    "util/types",
    "v8",
    "v8/tools/arguments",
    "v8/tools/codemap",
    "v8/tools/consarray",
    "v8/tools/csvparser",
    "v8/tools/logreader",
    "v8/tools/profile_view",
    "v8/tools/splaytree",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
];

/// Returns `true` if `name` is a Node builtin module.
///
/// ```
/// use biome_js_analyze::globals::module::node::is_builtin_module;
///
/// assert!(is_builtin_module(&"fs"));
/// ```
pub fn is_builtin_module(name: &str) -> bool {
    BUILTIN_MODULES.binary_search(&name).is_ok()
}

#[test]
fn test_order() {
    for items in BUILTIN_MODULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
