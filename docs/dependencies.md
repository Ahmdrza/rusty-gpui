# Dependency choices and compatibility distributions

Rusty GPUI 0.4.0 uses a distinct maintained rusty-gpui-components package derived from GPUI Component 0.5.1, plus rusty-gpui-runtime/rusty-gpui-http derived from GPUI/gpui_http_client 0.2.2. Every package preserves its upstream Apache-2.0 license and provenance. These are not official upstream releases.

The component fork carries Rusty GPUI's shared presentation and interaction fixes. The runtime fork changes its HTTP dependency to the Rusty GPUI compatibility helper; the helper disables zed-async-tar's unused default xattr feature. The helper's archive extraction never enables xattr restoration. This removes xattr 0.2.3's invalid libc::ENOATTR reference on Linux without pinning libc backward. There are no patch tables in the repository or in consumer manifests.

The public facade reexports the runtime as rusty_gpui::gpui. An app importing a crate named gpui directly must alias package rusty-gpui-runtime at the matching Rusty GPUI version. Original registry GPUI types and Rusty GPUI runtime types have separate identities and must not be mixed.

The isolated package check compiles a registry-only consumer from staged crate archives, checking that the original gpui/gpui_http_client/gpui-component packages and xattr are absent. See RELEASING.md for checks, package names and publication order.

Shared presentation is in crates/rusty-gpui-components/src/foundation.rs. The behavior fixes and upstream maintenance responsibilities are recorded in that crate's UPSTREAM.md. Font/icon notices remain in rusty-gpui/assets. The earlier 0.3.0 dependency audit is historical; use scripts/dependency-report.py for the current graph.
