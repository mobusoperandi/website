const embedEverything = require("eleventy-plugin-embed-everything");

module.exports = eleventyConfig => {
  eleventyConfig.addPassthroughCopy('./index.js')
  eleventyConfig.setNunjucksEnvironmentOptions({
    throwOnUndefined: true,
    trimBlocks: true,
    lstripBlock: true,
  })
  eleventyConfig.addPlugin(embedEverything);
  return {
    markdownTemplateEngine: "njk"
  }
}
