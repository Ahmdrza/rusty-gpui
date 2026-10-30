# Rusty GPUI Theme

Renderer-independent, dependency-free design tokens for Rusty GPUI. Generates semantic light/dark colors, OKLCH scales, contrast-adjusted text, typography, density and spacing from ThemeConfig.

```rust
use rusty_gpui_theme::{Color, ThemeConfig};
let config = ThemeConfig { primary: Color::hex("#4567D5").unwrap(), ..Default::default() };
let tokens = config.generate();
assert!(tokens.control_height > tokens.font_size);
```

Use rusty-gpui to apply these tokens to native controls. This crate itself does not depend on GPUI. MIT licensed.
