const embedEverything = require("eleventy-plugin-embed-everything");
const path = require('path')
const util = require('node:util');
const execFile = util.promisify(require('node:child_process').execFile);
const common = require('./common')
const now = String(Date.now())

module.exports = eleventyConfig => {
  eleventyConfig.setTemplateFormats(['njk', 'md', 'js'])
  eleventyConfig.addShortcode('version', () => now)
  eleventyConfig.addWatchTarget(common.paths.tailwind)
  eleventyConfig.on(
    'eleventy.after',
    async () => {
      const { stdout, stderr } = await execFile(
        'npx',
        [
          'tailwind',
          '--input', common.paths.tailwind,
          '--output', path.resolve(common.paths.output, 'style.css'),
          '--content', path.resolve(common.paths.output, 'index.html')
        ]
      )
      console.log(stdout);
      console.error(stderr);
    }
  )
  eleventyConfig.setNunjucksEnvironmentOptions({
    throwOnUndefined: true,
    trimBlocks: true,
    lstripBlock: true,
  })
  eleventyConfig.addPlugin(embedEverything);
  return {
    dir: {
      input: common.paths.input,
      output: common.paths.output,
    },
    markdownTemplateEngine: "njk"
  }
}
