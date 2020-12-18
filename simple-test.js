const { sync } = require('./index')

console.assert(JSON.stringify([...sync()]) === '[4,2]', 'Simple test failed')

console.info('Simple test passed')

for (let i = 0; i < 1000000; i++) {
  sync()

  if (global.gc && i % 500 === 0) {
    global.gc()
  }
}
