const yaml = require('yaml')
const fs = require('fs').promises
const path = require('path')
module.exports = async () => {
  const filepath = path.join(__dirname, 'mobs.yaml')
  const file = (await fs.readFile(filepath)).toString()
  const data = yaml.parse(file)
  return data
}
