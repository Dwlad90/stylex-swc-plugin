import esbuild from 'esbuild'
import { config } from './config.mjs'

export const context = await esbuild.context(config)