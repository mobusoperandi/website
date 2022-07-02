const path = require('path')
const rootPath = __dirname
module.exports = {
  paths: {
    input: path.resolve(rootPath, 'input'),
    tailwind: path.resolve(rootPath, 'tailwind.css'),
    output: path.resolve(rootPath, 'output')
  }
}
