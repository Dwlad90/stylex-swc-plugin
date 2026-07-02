import path from 'path';

export function isAllowlistedPackage(resourcePath: string, stylexPackages: string[]) {
  const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;
  const nodeModulesEntries = path.normalize(resourcePath).split(nodeModulesSegment).slice(1);

  return stylexPackages.some(packageName => {
    const normalizedPackageName = path.normalize(packageName).replace(/[\\/]$/, '');

    return nodeModulesEntries.some(
      entry =>
        entry === normalizedPackageName || entry.startsWith(`${normalizedPackageName}${path.sep}`)
    );
  });
}
