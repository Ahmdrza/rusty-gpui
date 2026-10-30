# Upstream provenance

Derived from gpui_http_client 0.2.2, distributed by the Zed/GPUI contributors under Apache-2.0. The original LICENSE-APACHE is preserved. This package is a Rusty GPUI compatibility distribution under a distinct name, not an official GPUI release.

The source remains upstream gpui_http_client 0.2.2. Its manifest disables the default xattr feature on zed-async-tar. This avoids xattr 0.2.3's removed libc::ENOATTR reference on Linux. GPUI's extraction helper uses Archive::new, whose unpack_xattrs setting is false, and does not request extended-attribute preservation. Archive contents are still unpacked normally; xattr restoration is not provided by this distribution.

Keep manifest changes narrow and document upstream upgrades. Release version follows the Rusty GPUI workspace; upstream version remains recorded separately.
