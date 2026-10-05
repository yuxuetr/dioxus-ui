// OKLCH parsing, formatting, and WCAG contrast for the theme presets
// (RFC 0057). Colors are [lightness 0-1, chroma, hue in degrees].

export const parseOklch = (value) => {
  const match = value.match(/^oklch\(\s*([\d.]+)(%?)\s+([\d.]+)\s+([\d.]+)\s*\)$/);
  if (match === null) {
    throw new Error(`not an opaque oklch() color: ${value}`);
  }
  const lightness = Number(match[1]) / (match[2] === "%" ? 100 : 1);
  return [lightness, Number(match[3]), Number(match[4])];
};

const round = (value, digits) => Number(value.toFixed(digits));

export const formatOklch = ([lightness, chroma, hue]) => {
  return `oklch(${round(lightness, 3)} ${round(chroma, 3)} ${round(hue, 3)})`;
};

const linearSrgb = ([lightness, chroma, hue]) => {
  const a = chroma * Math.cos((hue * Math.PI) / 180);
  const b = chroma * Math.sin((hue * Math.PI) / 180);
  const l = (lightness + 0.3963377774 * a + 0.2158037573 * b) ** 3;
  const m = (lightness - 0.1055613458 * a - 0.0638541728 * b) ** 3;
  const s = (lightness - 0.0894841775 * a - 1.291485548 * b) ** 3;
  return [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ].map((channel) => Math.min(1, Math.max(0, channel)));
};

const encode = (channel) => (channel <= 0.0031308 ? 12.92 * channel : 1.055 * channel ** (1 / 2.4) - 0.055);
const decode = (channel) => (channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);

// Browsers paint 8-bit sRGB, so contrast is measured on the quantized color.
const quantize = (encoded) => Math.round(encoded * 255) / 255;

// Relative luminance of a color, or of `overlay` at `alpha` over `color`,
// blended in gamma-encoded 8-bit sRGB as browsers paint it.
export const luminance = (color, overlay = null, alpha = 0) => {
  let encoded = linearSrgb(color).map((channel) => quantize(encode(channel)));
  if (overlay !== null) {
    const top = linearSrgb(overlay).map((channel) => quantize(encode(channel)));
    encoded = encoded.map((channel, index) => quantize(top[index] * alpha + channel * (1 - alpha)));
  }
  const [red, green, blue] = encoded.map(decode);
  return 0.2126 * red + 0.7152 * green + 0.0722 * blue;
};

const ratioOf = (first, second) => {
  const [light, dark] = [first, second].sort((x, y) => y - x);
  return (light + 0.05) / (dark + 0.05);
};

export const contrastRatio = (first, second) => ratioOf(luminance(first), luminance(second));

// Contrast of `foreground` on `tint` at `alpha` over `background`, such as
// `text-destructive` on `bg-destructive/10` over a popover.
export const tintedContrastRatio = (foreground, background, tint, alpha) => {
  return ratioOf(luminance(foreground), luminance(background, tint, alpha));
};

export const AA_TEXT_RATIO = 4.5;

// The text pairs components render, as [foreground token, background token],
// with an optional [tint token, alpha] laid over the background.
export const TEXT_PAIRS = [
  ["foreground", "background"],
  ["foreground", "muted"],
  ["card-foreground", "card"],
  ["popover-foreground", "popover"],
  ["accent-foreground", "accent"],
  ["muted-foreground", "muted"],
  ["muted-foreground", "background"],
  ["primary-foreground", "primary"],
  ["secondary-foreground", "secondary"],
  ["destructive-foreground", "destructive"],
  ["destructive", "background"],
  ["destructive", "card"],
  ["destructive", "popover"],
  ["destructive", "popover", ["destructive", 0.1]],
  ["sidebar-foreground", "sidebar"],
  ["sidebar-accent-foreground", "sidebar-accent"],
  ["sidebar-primary-foreground", "sidebar-primary"],
  ["success-foreground", "success"],
  ["warning-foreground", "warning"],
  ["info-foreground", "info"],
  ["foreground", "card", ["success", 0.1]],
  ["foreground", "card", ["warning", 0.1]],
  ["foreground", "card", ["info", 0.1]],
];
