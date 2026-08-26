/**
 * The build placeholder: stands in for the user's CSS placeholder between the
 * load hook and the bundle, during builds only.
 *
 * A statement at-rule is the one form that survives CSS minification in place:
 * esbuild and Lightning CSS both drop comments, including legal ones, but must
 * keep `@layer` because it declares layer order. Using it for every marker
 * style also keeps Lightning CSS from ever parsing the default `@stylex;`
 * marker, which it reports as an unknown at-rule.
 */
export const BUILD_CSS_PLACEHOLDER = '@layer __stylex_build_placeholder__;';

/**
 * Hands the stylesheet to the bundler with the build placeholder in place of
 * every marker occurrence, so a stray second marker cannot survive into the
 * output: the bundle step fills the first and removes the rest.
 */
export function toBuildPlaceholder(css: string, marker: string): string {
  return css.split(marker).join(BUILD_CSS_PLACEHOLDER);
}

/**
 * Removes every marker occurrence, for the stylesheets that did not receive the
 * rules.
 */
export function stripMarkers(source: string, markers: string[]): string {
  return markers.reduce((stripped, marker) => stripped.split(marker).join(''), source);
}

/**
 * Replaces the first marker occurrence and drops every later one: repeating the
 * whole rule set per marker would only duplicate it. Splitting rather than
 * `String#replace` also keeps `$&`-like sequences in the CSS literal intact.
 */
export function replaceFirstMarker(source: string, marker: string, replacement: string): string {
  const start = source.indexOf(marker);

  if (start === -1) return source;

  const tail = source.slice(start + marker.length);

  return source.slice(0, start) + replacement + stripMarkers(tail, [marker]);
}

/**
 * Picks a stable stylesheet to append the rules to when no marker reached the
 * output. Preference: `index.css`, then `style.css`, then `main.css`, then the
 * first one given.
 *
 * Names may be bare asset names or full paths, so the well-known names are
 * anchored to a path boundary rather than matched anywhere in the string.
 */
export function pickCssAsset(cssAssets: string[]): string | null {
  if (cssAssets.length === 0) return null;

  const preferred =
    cssAssets.find(f => /(^|\/)index\.css$/.test(f)) ||
    cssAssets.find(f => /(^|\/)style\.css$/.test(f)) ||
    cssAssets.find(f => /(^|\/)main\.css$/.test(f));

  return preferred || cssAssets[0] || null;
}

/**
 * One stylesheet the rules can go into, named so `transformCss` and the asset
 * preference have something to work with. Each host stores its stylesheets
 * differently -- a bundle asset, a webpack asset, a file already on disk -- and
 * this is the only part of that difference the injection needs to know about.
 */
export type CssInjectionTarget = {
  name: string;
  read(): string | Promise<string>;
  write(source: string): void | Promise<void>;
};

/** Runs the collected rules through whatever the host does to CSS. */
export type FinalizeCss = (css: string, targetName: string) => Promise<string>;

/**
 * Puts the collected rules where the marker is and takes every marker they did
 * not replace back out, falling back to a preferred stylesheet when no marker
 * survived into the output.
 *
 * Returns whether the caller has nothing left to report: either the rules were
 * placed, or there were none to place and the markers are gone.
 */
export default async function injectIntoCssTargets(
  targets: CssInjectionTarget[],
  markers: string[],
  collectedCSS: string | null,
  finalizeCss: FinalizeCss
): Promise<boolean> {
  // Read once per target: the fallback below needs the same contents, and a
  // second read of a file on disk would be wasted work.
  const sources = new Map<CssInjectionTarget, string>();

  for (const target of targets) {
    sources.set(target, (await target.read()).toString());
  }

  let injected = false;

  for (const target of targets) {
    const source = sources.get(target) ?? '';
    const marker = markers.find(candidate => source.includes(candidate));

    if (!marker) continue;

    let next = source;

    if (!injected) {
      // An empty rule set still has to take the marker back out, otherwise it
      // ships to the browser.
      const finalCSS = collectedCSS ? await finalizeCss(collectedCSS, target.name) : '';

      next = replaceFirstMarker(next, marker, finalCSS);
      injected = true;
    }

    // Whatever is left over -- a second marker here, or a marker in another
    // stylesheet -- would repeat the rules, so it is only removed.
    await target.write(stripMarkers(next, markers));
  }

  if (injected || !collectedCSS) return true;

  // No marker reached the output, so append to a preferred stylesheet instead.
  const targetName = pickCssAsset(targets.map(target => target.name));
  const fallback = targets.find(target => target.name === targetName);

  if (!fallback) return false;

  const existing = sources.get(fallback) ?? '';
  const finalCSS = await finalizeCss(collectedCSS, fallback.name);

  await fallback.write(existing ? existing + '\n' + finalCSS : finalCSS);

  return true;
}
