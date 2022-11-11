# Sinkmap [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/lavafroth/sinkmap)](https://rust-reportcard.xuri.me/report/github.com/lavafroth/sinkmap) ![build](https://github.com/lavafroth/sinkmap/actions/workflows/rust.yml/badge.svg)
Rust tool to extract JavaScript source code from sourcemap files 

### Getting started

Use the `-u` flag for specifying a URL or a local file and the `-o` flag for a directory to output the source files to. For example:

```sh
sinkmap -o npm_home -u https://static.npmjs.com/homepage/homepage.1e2201dead1e1f3672df.js.map

wrote 81 bytes to "npm_home/shared/components/head/images/favicon-230x230.png"
wrote 81 bytes to "npm_home/shared/components/head/images/favicon-96x96.png"
wrote 266 bytes to "npm_home/shared/pages/homepage/homepage.js"
wrote 299 bytes to "npm_home/shared/components/homepage/homepage.js"
wrote 3783 bytes to "npm_home/shared/components/homepage/pane-homepage-content.js"
wrote 674 bytes to "npm_home/shared/components/button/button.js"
wrote 81 bytes to "npm_home/shared/components/head/images/android-chrome-192x192.png"
wrote 81 bytes to "npm_home/shared/components/head/images/coast-228x228.png"
wrote 81 bytes to "npm_home/shared/components/head/images/favicon-16x16.png"
wrote 81 bytes to "npm_home/shared/components/head/images/open-graph.png"
wrote 81 bytes to "npm_home/shared/components/head/images/mstile-144x144.png"
wrote 81 bytes to "npm_home/shared/components/head/images/browserconfig.xml"
wrote 81 bytes to "npm_home/design-system/avatar/wombat-no-avatar.svg"
wrote 81 bytes to "npm_home/shared/components/head/images/apple-touch-icon-120x120.png"
wrote 81 bytes to "npm_home/shared/components/head/images/apple-touch-icon-144x144.png"
wrote 81 bytes to "npm_home/shared/components/head/images/apple-touch-icon-152x152.png"
wrote 81 bytes to "npm_home/shared/components/head/images/apple-touch-icon-180x180.png"
wrote 81 bytes to "npm_home/shared/components/head/images/favicon-32x32.png"
```
```sh
tree -L 2 npm_home
npm_home
├── design-system
│   └── avatar
└── shared
    ├── components
    └── pages


5 directories, 0 files
```
