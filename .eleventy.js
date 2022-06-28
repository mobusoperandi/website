module.exports = eleventyConfig => {
  eleventyConfig.addPassthroughCopy('./index.js')
  eleventyConfig.setNunjucksEnvironmentOptions({
    throwOnUndefined: true,
    trimBlocks: true,
    lstripBlock: true,
  })
}
