global.xit = global.it;
global.xtest = global.test;
if (global.it && global.it.skip) {
  global.it.skip = global.it;
}
if (global.test && global.test.skip) {
  global.test.skip = global.test;
}
if (global.describe && global.describe.skip) {
  global.describe.skip = global.describe;
}
