# RFC 0042: Slider Thumb Position And Vertical Orientation

- Status: Accepted
- Created: 2026-10-05

## Summary

Make the Slider thumb follow the value and add a vertical slider. The thumb
is positioned absolutely along the root, and `Slider` takes an
`orientation`. A vertical slider fills from the bottom, reads pointer
positions along its height, and renders `aria-orientation="vertical"`.

## Current State

As of M166:

- The thumb style sets `left: <percent>%` and `translateX(-50%)`, but
  neither the thumb class nor its style positions it, so `left` has no effect.
  The thumb sits after the track at the end of the root for every value,
  half outside it; only the range shows the value.
- `Slider` always renders `aria-orientation="horizontal"`, sizes the track
  along its width, and maps pointer positions along `clientX`. There is no
  vertical slider.

## Decision

- The thumb style becomes `position: absolute` with `left: <percent>%`,
  `top: 50%`, and `transform: translate(-50%, -50%)`, so its center sits on
  the value along the root. The position is inline, so it does not depend on
  Tailwind classes being compiled.
- `Slider` gains `orientation: SliderOrientation` (`Horizontal` by
  default, or `Vertical`). It renders `aria-orientation` and
  `data-orientation`.
- A vertical slider:
  - gives the root, track, and range classes that size them along the height
    (`h-full w-auto flex-col` on the root, `h-full w-2` on the track,
    `w-full` on the range);
  - renders the range as `bottom: 0%; height: <percent>%`;
  - places the thumb at `bottom: <percent>%`, `left: 50%`, with
    `translate(-50%, 50%)`;
  - maps the pointer with `(rect.bottom - clientY) / rect.height` in the
    page script, which reads `data-orientation` on each event.
- Keys stay as they are: ArrowUp and ArrowRight increase and ArrowDown and
  ArrowLeft decrease, as the APG slider pattern asks for both orientations.

## Scope

In scope:

- the thumb position, the orientation prop, the vertical styles, and the
  pointer mapping in the crate and the template
- the Slider docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Right-to-left horizontal sliders | Keys and the pointer would need the computed direction, which the Rust key handler cannot read | A consumer ships a right-to-left Slider |
| Multiple thumbs | Needs per-thumb focus, values, and collision rules | A consumer needs a range slider |
| Inverted sliders | No consumer has asked for a slider that grows from the end | A consumer asks for one |
| Desktop and Mobile self-test scenarios | The pointer script already runs in the WebViews | A WebView reports different pointer behavior |

## Verification

- Unit tests cover the range and thumb styles for both orientations.
- The CLI parity test keeps the template pointer script identical to the
  crate script, and the generated fixture smoke keeps the template compiling.
- `npm run verify:runtime-interactions` asserts that the horizontal thumb
  center moves with the value, and that a vertical slider in the Web preview
  renders `aria-orientation="vertical"`, puts its thumb center at the value
  measured from the bottom, and changes value with a press near its top and
  with ArrowUp.
- Reverse checks: not positioning the thumb, mapping the vertical pointer
  along `clientX`, or keeping a horizontal `aria-orientation` each make the
  verifier fail.
