import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    // Аналог testEnvironment: 'node'
    environment: 'node',
    // Аналог testPathIgnorePatterns: ['/__fixtures__/']
    exclude: ['**/node_modules/**', '**/dist/**', '**/__fixtures__/**'],
    // Если вы хотите использовать глобальные функции типа describe, it, expect без импорта
    globals: true,
  },
});
