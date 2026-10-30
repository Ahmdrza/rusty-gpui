# Review on your Mac

Run `bash scripts/run-macos.sh`. Use the native window, not screenshots, to assess interaction and performance.

## Visual review

- Compare light and dark appearances, then try Cobalt, Forest, Clay, Iris and Graphite.
- Enter a custom six-digit hex seed, then test generated and explicit secondary colors.
- Compare compact, comfortable and spacious density.
- Change fonts and font size; verify fields, buttons and labels remain readable.
- Change spacing and radius. Confirm related components move together rather than drifting independently.

## Interaction review

- Type, select, copy/paste, undo, and use a non-Latin IME in text inputs.
- Enter a malformed email, then correct it.
- Toggle password reveal. GPUI Component's reveal button is press-and-hold.
- Open the searchable select; try typing and arrow/Enter/Escape navigation.
- Choose and clear a date.
- Toggle selections and switch tabs; check the resulting event history.
- Open a modal, submit an empty name, enter a name, confirm, and reopen. Test Escape and focus return.
- Open a drawer and notification; close them with their provided controls.
- Tab to buttons and activate with Enter or Space. Disabled/loading actions should not fire.

## Performance review

- Choose Performance lab and 10,000 rows; scroll, select rows and resize columns.
- Run the frame sampler while scrolling and switching theme.
- Compare mean and p95 callback intervals on your actual Mac. A 60 Hz display commonly has roughly 16.7 ms between frames, but that is not a pass/fail threshold.
- Check CPU/GPU and memory with Activity Monitor/Instruments if your intended app workload warrants it.

## Validation boundary

Cloud verification uses Linux, X11 and software Vulkan. It can catch layout failures and exercise real native callbacks, but cannot establish Metal performance, macOS text metrics, VoiceOver behavior, battery use, or Retina scaling. Those checks must be performed on the laptop.
