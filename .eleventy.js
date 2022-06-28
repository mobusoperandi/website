module.exports = eleventyConfig => {
  eleventyConfig.setNunjucksEnvironmentOptions({
    throwOnUndefined: true,
    trimBlocks: true,
    lstripBlock: true,
  })
}
