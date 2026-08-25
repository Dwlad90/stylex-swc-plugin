const path = require('node:path');
const EV = '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/.scratch/fix_media-query-order-wrapping/evidence';
const { run, runRust } = require(path.join(EV, 'ref.cjs'));

const CASES = {
  'vendor prefixed feature': { color: { default: 'black', '@media (-webkit-min-device-pixel-ratio: 2)': 'red', '@media (max-width: 50px)': 'blue' } },
  'moz prefixed feature': { color: { default: 'black', '@media (-moz-device-pixel-ratio: 2)': 'red', '@media (max-width: 50px)': 'blue' } },
  'prefixed beside a width ladder': { color: { default: 'black', '@media (-webkit-min-device-pixel-ratio: 2) and (min-width: 200px)': 'red', '@media (min-width: 100px)': 'blue' } },
  'emoji in a feature name': { color: { default: 'black', '@media (\u{1F600}: 1)': 'red', '@media (max-width: 50px)': 'blue' } },
  'combining marks in a feature name': { color: { default: 'black', '@media (mín-width: 100px)': 'red', '@media (max-width: 50px)': 'blue' } },
  'escaped at sign': { color: { default: 'black', '@media (min-width: 100px) and (\\@foo: 1)': 'red', '@media (max-width: 50px)': 'blue' } },
  'trailing whitespace in the key': { color: { default: 'black', '@media (min-width: 100px) ': 'red', '@media (max-width: 50px)': 'blue' } },
  'bare @media': { color: { default: 'black', '@media ': 'red' } },
  'only a default': { color: { default: 'black' } },
  'a media key with no default': { color: { '@media (min-width: 200px)': 'red', '@media (min-width: 100px)': 'blue' } },
  'comma separated disjuncts': { color: { default: 'black', '@media (min-width: 200px), (max-width: 100px)': 'red', '@media (min-width: 100px)': 'blue' } },
  'media type beside a width': { color: { default: 'black', '@media screen and (min-width: 200px)': 'red', '@media (min-width: 100px)': 'blue' } },
  'huge length': { color: { default: 'black', '@media (min-width: 1e308px)': 'red', '@media (max-width: 50px)': 'blue' } },
  'nan-ish length': { color: { default: 'black', '@media (min-width: 0.0000000001px)': 'red', '@media (max-width: 50px)': 'blue' } },
};

// A conditional map nested many levels below the style object.
function deepNest(levels) {
  let value = { default: 'black', '@media (min-width: 200px)': 'red', '@media (min-width: 100px)': 'blue' };
  for (let i = 0; i < levels; i++) value = { default: 'black', ':hover': value };
  return { color: value };
}
CASES['media keys eight levels deep'] = deepNest(8);

for (const [label, props] of Object.entries(CASES)) {
  const code = `import * as stylex from '@stylexjs/stylex';\nexport const styles = stylex.create({ x: ${JSON.stringify(props)} });\n`;
  const outs = {};
  for (const [name, fn] of [['babel', run], ['rust', runRust]]) {
    try { outs[name] = fn(code).rules.map(r => r[1].ltr); }
    catch (e) { outs[name] = 'REJECTED'; }
  }
  const agree = JSON.stringify(outs.babel) === JSON.stringify(outs.rust);
  console.log(`\n${agree ? 'AGREE ' : 'DIFFER'} ${label}`);
  console.log('  babel', JSON.stringify(outs.babel).slice(0, 400));
  if (!agree) console.log('  rust ', JSON.stringify(outs.rust).slice(0, 400));
}
