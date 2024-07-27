// @ts-check

/** @type {import("syncpack").RcFile} */
const config = {
  dependencyTypes: ['dev', 'overrides', 'peer', 'pnpmOverrides', 'prod', 'resolutions'],
  filter: '.',
  indent: '  ',
  semverGroups: [],
  semverRange: '',
  sortAz: [
    'contributors',
    "private",
    'dependencies',
    'devDependencies',
    'keywords',
    'peerDependencies',
    'resolutions',
    'scripts',
  ],
  sortFirst: [
    'name',
    'description',
    'version',
    'private',
    'author',
    'license',
    'sideEffects',
    'module',
    'types',
    'exports',
    'imports',
    'files',
    'publishConfig',
    'scripts',
    'volta',
    'config',
    'dependencies',
    'devDependencies',
    'peerDependencies',
  ],
  source: ['package.json', 'pkgs/*/package.json'],
  versionGroups: [],
};

module.exports = config;
