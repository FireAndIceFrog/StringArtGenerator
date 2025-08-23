var fs = require('fs');
var chalk = require('chalk');
const typescriptTransform = require('i18next-scanner-typescript');

module.exports = {
  output: './public/locales',
  options: {
    func: {
      // don't pass ts or tsx here!
      extensions: ['.js', '.jsx'],
    },
    trans: {
      // don't pass ts or tsx here!
      extensions: ['.js', '.jsx'],
    },
    
  },
  // your i18next-scanner config
  // ...
  transform: typescriptTransform(
    // options
    {
      // default value for extensions
      extensions: [".ts", ".tsx"],
      // optional ts configuration
      tsOptions: {
        target: "es2017",
      },
    },

    // optional custom transform function
    function customTransform(outputText, file, enc, done) {
      // do something custom with the transpiled `outputText`
      this.parser.parseTransFromString(outputText);
      this.parser.parseFuncFromString(outputText);

      done();
    },
  ),
};