/**
 * Build the animated README hero from deterministic Playwright screenshots.
 *
 * Run `node scripts/screenshots/capture.mjs` first so every source frame is
 * rendered from the current Vue frontends. Requires ffmpeg and img2webp on
 * PATH; no extra npm package is needed.
 */
import { spawnSync } from 'node:child_process'
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  rmSync,
  statSync,
} from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const HERE = dirname(fileURLToPath(import.meta.url))
const ROOT = resolve(HERE, '../..')
const SCREENSHOT_DIR = resolve(ROOT, 'docs/screenshots')
const OUTPUT = resolve(SCREENSHOT_DIR, 'iec104-simulator-demo.webp')
const TEMP_ROOT = resolve(ROOT, 'output/playwright')

const scenes = [
  'tut-1-slave-current-main.png',
  'slave-point-csv-log-analysis.png',
  'slave-random-simulation.png',
  'master-multi-ca-comm-log.png',
].map((name) => resolve(SCREENSHOT_DIR, name))

for (const scene of scenes) {
  if (!existsSync(scene)) {
    throw new Error(`Missing hero source frame: ${scene}\nRun node scripts/screenshots/capture.mjs first.`)
  }
}

const width = 960
const height = 540
const outputFps = 8
const frameDurationMs = Math.round(1000 / outputFps)
const sceneSeconds = 1.4
const fadeSeconds = 0.25

function run(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8' })
  if (result.error?.code === 'ENOENT') {
    throw new Error(`${command} was not found on PATH.`)
  }
  if (result.status !== 0) {
    throw new Error(`${command} failed (${result.status}):\n${result.stderr || result.stdout}`)
  }
}

mkdirSync(TEMP_ROOT, { recursive: true })
const tempDir = mkdtempSync(resolve(TEMP_ROOT, 'hero-frames-'))

try {
  // Returning to the opening overview before the file ends keeps the infinite
  // loop smooth instead of cutting directly from Master back to Slave.
  const inputs = [...scenes, scenes[0]]
  const ffmpegArgs = ['-hide_banner', '-loglevel', 'error', '-y']
  for (const input of inputs) {
    ffmpegArgs.push('-loop', '1', '-t', String(sceneSeconds), '-i', input)
  }

  const filters = inputs.map((_, index) =>
    `[${index}:v]scale=${width}:${height}:force_original_aspect_ratio=increase:flags=lanczos,`
    + `crop=${width}:${height},setsar=1,fps=25,format=rgba,setpts=PTS-STARTPTS[v${index}]`,
  )

  let current = 'v0'
  for (let index = 1; index < inputs.length; index++) {
    const next = `x${index}`
    const offset = ((sceneSeconds - fadeSeconds) * index).toFixed(2)
    filters.push(
      `[${current}][v${index}]xfade=transition=fade:duration=${fadeSeconds}:offset=${offset}[${next}]`,
    )
    current = next
  }
  filters.push(`[${current}]fps=${outputFps}[out]`)

  ffmpegArgs.push(
    '-filter_complex', filters.join(';'),
    '-map', '[out]',
    '-an',
    resolve(tempDir, 'frame-%03d.png'),
  )
  run('ffmpeg', ffmpegArgs)

  const frames = readdirSync(tempDir)
    .filter((name) => name.endsWith('.png'))
    .sort()
  if (frames.length === 0) {
    throw new Error('ffmpeg did not generate any animation frames.')
  }

  const webpArgs = ['-min_size', '-sharp_yuv', '-loop', '0']
  for (const frame of frames) {
    webpArgs.push(
      '-d', String(frameDurationMs),
      '-lossy', '-q', '76', '-m', '6',
      resolve(tempDir, frame),
    )
  }
  webpArgs.push('-o', OUTPUT)
  run('img2webp', webpArgs)

  const sizeMiB = statSync(OUTPUT).size / 1024 / 1024
  console.log(
    `✓ ${OUTPUT} (${sizeMiB.toFixed(2)} MiB, ${width}×${height}, ${frames.length} frames)`,
  )
} finally {
  rmSync(tempDir, { recursive: true, force: true })
}
