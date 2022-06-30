const embedEverything = require("eleventy-plugin-embed-everything");

module.exports = eleventyConfig => {
  eleventyConfig.setTemplateFormats(['njk', 'md', 'js'])
  eleventyConfig.setNunjucksEnvironmentOptions({
    throwOnUndefined: true,
    trimBlocks: true,
    lstripBlock: true,
  })
  eleventyConfig.addPlugin(embedEverything);
  return {
    dir: {
      input: 'input',
      output: 'output',
    },
    markdownTemplateEngine: "njk"
  }
}
