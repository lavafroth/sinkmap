# sinkmap
Rust tool to extract JavaScript source code from sourcemap files 

# Quick start

Use the `-u` flag for specifying a URL or a local file and the `-o` flag for a directory to output the source files to. For example:

```sh
./sinkmap -u https://reactjs.org/$(curl -fsL "https://reactjs.org/" | grep -Po "app-.*?\.js" | head -n 1).map -o reactjssrc
```
```
wrote 1510 bytes to "reactjssrc/node_modules/gatsby/node_modules/core-js/internals/redefine.js"
wrote 847 bytes to "reactjssrc/node_modules/fbjs/lib/hyphenateStyleName.js"
wrote 10703 bytes to "reactjssrc/node_modules/gatsby-link/index.js"
wrote 673 bytes to "reactjssrc/node_modules/fbjs/lib/hyphenate.js"
wrote 4663 bytes to "reactjssrc/node_modules/gatsby/node_modules/core-js/internals/fix-regexp-well-known-symbol-logic.js"
{snip}
rote 7172 bytes to "reactjssrc/.cache/navigation.js"
wrote 1009 bytes to "reactjssrc/node_modules/gatsby/node_modules/core-js/internals/classof.js"
wrote 998 bytes to "reactjssrc/node_modules/inline-style-prefixer/static/plugins/flexboxOld.js"
wrote 9640 bytes to "reactjssrc/src/theme.js"
wrote 604 bytes to "reactjssrc/node_modules/gatsby/node_modules/core-js/internals/regexp-sticky-helpers.js"
```
```
tree -L 1 reactjssrc
```
```
reactjssrc
├── gatsby-browser.js
├── node_modules
├── src
└── (webpack)

3 directories, 1 file
```
