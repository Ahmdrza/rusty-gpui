# Support and limitations

Rusty GPUI is an early pre-1.0 library. Evaluate the examples and gallery with the workflows your application needs.

| Area | Current status |
| --- | --- |
| macOS | Native editing, keyboard, search, overlays and table interactions reviewed on Apple Silicon with a Retina display. |
| Linux | CI compilation, focused tests and packaged-consumer builds pass. Native visual and keyboard review remains pending. |
| Windows | Experimental; not release-tested. |
| Accessibility | Custom controls lack a semantic accessibility tree and screen-reader integration. Applications requiring that support should wait for it. |
| Displays | Other display scales and multi-monitor configurations remain unverified. |
| Themes | One app-wide theme; independent themes per subtree are unsupported. |

The component catalog includes specialized widgets that have not all received dedicated visual and interaction review. The gallery's frame sampler measures render callbacks, not GPU time.

## Dependencies

The default registry consumer passed the release security audit with zero vulnerability and unsoundness findings; dependency maintenance warnings remain. Optional webview and test-support dependency graphs contain additional advisories and are not covered by that result. Audit results are time-sensitive: run the current checks described in [Contributing](../CONTRIBUTING.md).

The optional webview feature remains experimental. See [dependency architecture](dependencies.md) for package roles and maintenance responsibilities.

## Compatibility

Before 1.0, breaking public API changes increment the minor version; compatible fixes increment the patch version. Current stable Rust is the development baseline; no minimum supported Rust version is promised.
