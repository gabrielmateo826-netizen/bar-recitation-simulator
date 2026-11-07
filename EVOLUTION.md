# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))

* **(sync)** Add timeout guard and actionable network error handling to PIN pairing ([dfda7dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dfda7ddcf5ed1b6d2ff0efd8d3cd4b74fd8f1fb4))

* **(polish)** Close executeDeckRegeneration brace and add defensive error handling to openDeckRegenerateModal ([4e0ff63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4e0ff638c7db5d13026450bc8b34538ea5b9e778))

* **(polish)** Add explicit touch event and window bindings for Polish modal ([57f408c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/57f408c52876f5de7938f6d516204343640e1f25))

* **(sync)** Optimize generateCompleteMasterPayload to exclude heavy source text for lightweight PIN sync ([14367c4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/14367c4ec74c7db42085dc354bdaa9af17a41820))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))

* Auto-update EVOLUTION.md [skip ci] ([8250179](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8250179a3b403c022b2e3cf678f17d72101027a2))

* Auto-update EVOLUTION.md [skip ci] ([7a6c3dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7a6c3dd14c075c9141f16b4869f1e24793ecf5bd))

* Auto-update EVOLUTION.md [skip ci] ([69f1f63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/69f1f63341840f3659d325b8e76e0fef2ecc74d2))

* Auto-update EVOLUTION.md [skip ci] ([8d7f65b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8d7f65ba5fb64d96f270691c32376a444a3a53ed))

* Auto-update EVOLUTION.md [skip ci] ([49def04](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/49def04c54504f1c424fba3a281e1b5b9918f94b))


### Features & New Capabilities

* **(sync)** Add automatic JSON file fallback when cloud relay fails ([03d29eb](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03d29ebd3abdf7727ce29da09dc770051a19683b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))

* **(sync)** Add timeout guard and actionable network error handling to PIN pairing ([dfda7dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dfda7ddcf5ed1b6d2ff0efd8d3cd4b74fd8f1fb4))

* **(polish)** Close executeDeckRegeneration brace and add defensive error handling to openDeckRegenerateModal ([4e0ff63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4e0ff638c7db5d13026450bc8b34538ea5b9e778))

* **(polish)** Add explicit touch event and window bindings for Polish modal ([57f408c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/57f408c52876f5de7938f6d516204343640e1f25))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))

* Auto-update EVOLUTION.md [skip ci] ([8250179](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8250179a3b403c022b2e3cf678f17d72101027a2))

* Auto-update EVOLUTION.md [skip ci] ([7a6c3dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7a6c3dd14c075c9141f16b4869f1e24793ecf5bd))

* Auto-update EVOLUTION.md [skip ci] ([69f1f63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/69f1f63341840f3659d325b8e76e0fef2ecc74d2))

* Auto-update EVOLUTION.md [skip ci] ([8d7f65b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8d7f65ba5fb64d96f270691c32376a444a3a53ed))


### Features & New Capabilities

* **(sync)** Add automatic JSON file fallback when cloud relay fails ([03d29eb](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03d29ebd3abdf7727ce29da09dc770051a19683b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))

* **(sync)** Add timeout guard and actionable network error handling to PIN pairing ([dfda7dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dfda7ddcf5ed1b6d2ff0efd8d3cd4b74fd8f1fb4))

* **(polish)** Close executeDeckRegeneration brace and add defensive error handling to openDeckRegenerateModal ([4e0ff63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4e0ff638c7db5d13026450bc8b34538ea5b9e778))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))

* Auto-update EVOLUTION.md [skip ci] ([8250179](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8250179a3b403c022b2e3cf678f17d72101027a2))

* Auto-update EVOLUTION.md [skip ci] ([7a6c3dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7a6c3dd14c075c9141f16b4869f1e24793ecf5bd))

* Auto-update EVOLUTION.md [skip ci] ([69f1f63](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/69f1f63341840f3659d325b8e76e0fef2ecc74d2))


### Features & New Capabilities

* **(sync)** Add automatic JSON file fallback when cloud relay fails ([03d29eb](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03d29ebd3abdf7727ce29da09dc770051a19683b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))

* **(sync)** Add timeout guard and actionable network error handling to PIN pairing ([dfda7dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dfda7ddcf5ed1b6d2ff0efd8d3cd4b74fd8f1fb4))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))

* Auto-update EVOLUTION.md [skip ci] ([8250179](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8250179a3b403c022b2e3cf678f17d72101027a2))

* Auto-update EVOLUTION.md [skip ci] ([7a6c3dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7a6c3dd14c075c9141f16b4869f1e24793ecf5bd))


### Features & New Capabilities

* **(sync)** Add automatic JSON file fallback when cloud relay fails ([03d29eb](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03d29ebd3abdf7727ce29da09dc770051a19683b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))

* **(sync)** Add timeout guard and actionable network error handling to PIN pairing ([dfda7dd](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dfda7ddcf5ed1b6d2ff0efd8d3cd4b74fd8f1fb4))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))

* Auto-update EVOLUTION.md [skip ci] ([8250179](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8250179a3b403c022b2e3cf678f17d72101027a2))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))

* **(syntax)** Fix bracket balance in expandDeterministicScopeItems and support special law ranges ([1dc4c0e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/1dc4c0ed4ec160f14d24f8176aecf96e0034f552))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))

* Auto-update EVOLUTION.md [skip ci] ([42d4e3e](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/42d4e3e264dafa97c69a3d7188039a68c7818c45))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))

* **(parser)** Priority check for special law circulars over default codal rules ([edd9051](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/edd9051ab84b2c97a21b1ae96db959ad58f60f15))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))

* Auto-update EVOLUTION.md [skip ci] ([a785da3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a785da3909457403a622fd3f5db8746b7e1f0ddf))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))

* **(syntax)** Close expandDeterministicScopeItems brace and fix custom rule parser ([279f636](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/279f63618f78b7e562c90ee2a9727bec15c933a4))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))

* Auto-update EVOLUTION.md [skip ci] ([93a7de9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/93a7de970b76d4a23b04d017c42e08481e6f217b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))

* **(parser)** Remove regex boundary trap on A.M. and repoint remote to ph-bar-recit ([9a12935](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/9a129359a9bec4ad60ba16f89acb5030de5c9757))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))

* Auto-update EVOLUTION.md [skip ci] ([6559852](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6559852f2906b43b981ac3a99a2f5fd1862ea73a))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))

* **(parser)** Support universal special laws and circular section ranges ([5d4ffea](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5d4ffeae356772ff709b7de6a98467a9f5b23db9))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))

* Auto-update EVOLUTION.md [skip ci] ([101b8c5](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/101b8c525a47983b5584370187f70f5a6fb16030))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))

* **(parser)** Prevent special circular section ranges from defaulting to Rule 130 ([d12572c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d12572cca3dfa53e58f0afb5c097ee7bd723f2d0))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))

* Auto-update EVOLUTION.md [skip ci] ([8535312](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8535312b4d0aae9a51a941f0b73eab23ddea8d93))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))

* Update polish engine and fix file path sync ([91d24c2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/91d24c21dd88c6f951a86958816f3c4f81b1750a))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))

* Auto-update EVOLUTION.md [skip ci] ([8057f13](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/8057f1361d5d49374e06292ed76caf0c8eb1a4b0))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))

* **(polish)** Add multi-layer preset auto-detection in polish modal ([2ff4fad](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2ff4fad510a9030db28b376e749fc294e4fd9baa))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))

* Auto-update EVOLUTION.md [skip ci] ([6439427](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6439427667c5da35590f1c6c5065e679f1c9af8a))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))

* Auto-update EVOLUTION.md [skip ci] ([32d7fe4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32d7fe4b09bb02f8b484ee32fc54d6e7d8357748))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))

* Auto-update EVOLUTION.md [skip ci] ([6e54511](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6e54511d4a3bf5ffe343530f6c11505a6f4c4066))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))

* **(polish)** Add dynamic preset auto-detection and adjustable card count sizing in polish modal ([5cd3cc9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5cd3cc9fc7d44affe22d278f1c1df92f697b030a))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))

* Auto-update EVOLUTION.md [skip ci] ([b884a5d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b884a5d0e1edee0b29addfad318ff3c15b27937b))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))

* Auto-update EVOLUTION.md [skip ci] ([ad254ef](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/ad254ef889eb1a842fe82eda222eb0ace1927869))

* Update README with direct download links and web simulator hub ([7ca4149](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7ca4149db82c30656b58eeea5e73ebce890d9741))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite.

---
## [Unreleased]

### Bug Fixes & Engine Corrections

* **(tour)** Eliminate page reload on subject selector during guided walkthrough ([328a1d0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/328a1d085950df35f20c1e66bad464ce754a46f2))

* **(changelog)** Prevent null timestamp error in git-cliff template ([0ebf1f4](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0ebf1f43b708360c9d906d1addc1bb8d47b948ac))


### Documentation & Guides

* Auto-update EVOLUTION.md [skip ci] ([788c576](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/788c576f80e390202273daf3f0e32b2d82d51d4c))


### Other Engineering Updates

* Initialize Bar Recitation Simulator with universal macOS builder ([b3723c6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b3723c6a06f3a5106dd5a3c057344efc2e122b0a))

* Fix workflow folder structure ([dd55fd6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/dd55fd6fed12ca53a7d7fb6928e6fd61a6cf0faa))

* Add Android build workflow ([1427046](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/142704616dc046bd6be3f78bac7787cd4e5bbf8a))

* Fix Android SDK setup using native runner toolchains ([d1445d6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/d1445d6a9bc7add8b93e8b5561558838942b0213))

* Add GitHub Pages deployment from src folder ([4fad80a](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/4fad80a2e1e7978d92c23df594cc9e1c97cd6b45))

* Fix Tauri Android target alias to aarch64 ([cdfc193](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cdfc193e74413c8e21216ac695f6cf695e406770))

* Fix YAML indentation and whitespace syntax on line 47 ([292e7ff](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/292e7ffc0d5b39b2669e8d6b6d357d23a22dd210))

* Add export toolbar and switch frontend to live GitHub Pages URL ([765f0f6](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/765f0f675b577131fd80c89e59603d606d8b886a))

* Build signed debug APK for direct sideloading ([c44e8e7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/c44e8e73a870655964dc4c43e89a15a76e761ffb))

* Fix YAML line 50 whitespace and build signed debug APK ([7603e6b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7603e6bf22bdac661a681be0669cf611755b0dab))

* Fix syntax error at bottom of index.html ([74a181b](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/74a181ba415d10bc3b6242fb9208f061fd84764c))

* Fix Tauri asset path to ../src and deploy updated hybrid simulator ([2a4462f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/2a4462ffcc8a477fefb3f74ac23b73e9bed11f52))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([a1bd641](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a1bd641cef8aa63e511a714de8fb8287a37a1aae))

* Production release v0.1.0: finalized 10-step tour, smart video container, and elevated compiler modal ([297d7d1](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/297d7d15b084b10b12ceacbc8cacad3736e2cf09))

* Upgrade macOS runner image from macos-14 to macos-15 ([7fd89a0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/7fd89a0cfd5dea275f1c8ea3200f44c9fce94da4))

* Add universal Save As file picker for backups, decks, and study sheet exports ([a91c7b2](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a91c7b25a5dd9d70bd20badb36b79c5c06526fd3))

* Fix Special Rules routing, save-first deck compiler, and amendatory statute audit ([084f846](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/084f8460c4a5e9d642072cde0c187dc185232112))

* Update Tauri remote URL and permissions ([90ecd62](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/90ecd623d6329773e46e55bf4b8d0168f3ffc972))

* Update Tauri remote URL and permissions ([bc3a5d3](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/bc3a5d3997da3f10f726eb996650f5b14d78c08a))

* Add non-destructive deck merge and duplication engine ([6bf0f40](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/6bf0f4020b3767ac801cd4b85928a3a1826092d8))

* Fix syntax braces and add visual deck merge modal ([79b79c0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/79b79c0b0618c744f9b47deb61d331270bcb2115))

* Update remote endpoint to ph-bar-recit organization ([32fe81f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/32fe81f66cfe26aeba907f6d9dd303ec79447905))

* Convert app icon to 32-bit RGBA for Android ([5f9e9c9](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/5f9e9c9eb3c1d63776a6e86472ee011991b0013a))

* Add valid Windows ICO icon and adjust runner configuration ([0f6ba06](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/0f6ba0639ec471ae6b921aa065afa1be8d716d43))

* Remove duplicate merge controller block ([f41086c](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f41086c9c0105ffa388541fc3821d7c058299151))

* Switch macOS runner to macos-13 to clear queue ([428f36d](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/428f36d46d6d1dc6fb3e79515e51be88916e443c))

* Update tauri config, bundle icons, and mac build workflow ([cd73d54](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/cd73d5459589dfde69a7f98004473ee3a30945a3))

* Switch to active macos-14 runner ([f22ca44](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/f22ca448daf9c81d4a6acf88e5a88c1ba200418e))

* Add artifact upload and restrict builds to manual dispatch ([abbc8e0](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/abbc8e09f839050382e25ebb34ef55e69f36ba6e))

* Configure automatic GitHub Pages deployment ([a5b868f](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/a5b868fdb04abbfbba87754528715520800dd84e))

* Fix async/await syntax guards and synchronize complete workspace build ([03c72fe](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/03c72fe335066edd5c0996b21e623a75d0b7d0f7))

* Update to 6-digit PIN pair sync ([b647682](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b647682357b8a559684c0082263187f2246c99cd))

* Complete Parts 1-5: Session auto-resume, multi-paragraph scope parser, polish deck tool, and 6-digit PIN docs ([e2c70a7](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/e2c70a7f3d6b7e3db0541d6807c7d0206e91b5a1))

* Fix scope parser regex and add card count selector to polish deck feature ([b2e7e53](https://github.com/ph-bar-recit/bar-recitation-simulator/commit/b2e7e53ba01124be45e6e454d0a72a686edee3d5))

# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite from its initial multi-variant inception through its evolution into a cross-platform Tauri application with serverless 6-digit PIN pairing sync, session auto-resume, and advanced deck regeneration capabilities.

---

## 1. Core Engine & AI Interrogation Architecture

* **Dual-Option & Multi-Variant Architecture (Options 2a, 2b, 3a, 3b):**
  * *Codal & Statutory Variants (2a/3a):* Tailored AI interrogation prompts, extraction parsers, and evaluation rubrics targeting statutory provisions, Rules/Articles/Sections, essential elements/requisites, and express exceptions.
  * *Case FIR Variants (2b/3b):* Re-engineered professor mandates, agenda extraction, and grading schemas to execute formal Supreme Court oral case recitations using the Facts-Issue-Ruling (FIR) methodology.

* **Turn-by-Turn On-Demand Generation Architecture:**
  * Replaced legacy multi-question batch generation models with a two-phase pipeline:
    1. *Boot/Setup Phase:* Lightweight agenda extraction (`extractStructuredAgenda`) parses syllabus files or scope constraints into clean arrays of titles/citations (~200 tokens), preventing output cap failures on boot.
    2. *Per-Turn Phase:* Questions and case calls are generated individually on demand (~120–150 tokens) strictly rotating through structured typologies.

* **Decoupled Single-Task Evaluation (ALAC & FIR):**
  * Purged next-question/next-case payload generation from evaluation requests (`submitStudentAnswer`). Gemini grades student submissions exclusively under substantial compliance standards, halving output token consumption per turn and eliminating rate-limit spikes.

* **Asymmetric Jurisprudential & Statutory Token Excerpt Sizing:**
  * Optimized token payloads by separating task-specific retrieval budgets:
    * *Question/Case Call Generation:* Capped at $4,000$ characters for fast 1.5–2.5 second turnaround.
    * *Diagnostic Evaluation / Grading:* Capped at $8,000$ to $10,000$ characters, anchored by priority section/case-title weighting multipliers ($\times 50$).

* **Recitation-Filter Mitigation (`temperature: 0.1`) & Explicit Safety Overrides (`BLOCK_NONE`):**
  * Configured generation temperature to $0.1$ and structured benchmark schemas into operative rule summaries, essential elements, and exceptions to bypass Google's automated pre-training memorization `RECITATION` filters.
  * Configured explicit `safetySettings` with `BLOCK_NONE` across all harmful content categories (`HARASSMENT`, `HATE_SPEECH`, `SEXUALLY_EXPLICIT`, `DANGEROUS_CONTENT`) to prevent true-to-life legal fact patterns involving crimes or injuries from triggering false-positive AI blocks.

* **Multi-Key & Multi-Model Failover Cascade:**
  * Integrated automated API key hot-swapping across comma-separated multi-project keys (`AIzaSyKey1..., AIzaSyKey2...`) and a robust finite linear model cascade (`gemini-3.8-flash` $\rightarrow$ `gemini-3.6-flash` $\rightarrow$ `gemini-3.7-flash` $\rightarrow$ `gemini-3.5-flash`) with 2.5-second pacing delays on HTTP 429/503 responses and a 40-second watchdog timer.

---

## 2. UI/UX Interrogation Interface & Modals

* **Spoiler-Free Question Cards:**
  * Hidden active topic badges and typology headers from live question screens. Questions display strictly as **`Question X of Y`**, forcing students to perform pure issue-spotting without premature hints. Contextual topic disclosure is restricted to diagnostic evaluations and debrief screens.

* **Polymorphic Diagnostic Evaluation & Debrief Modals:**
  * Standardized setup, diagnostic evaluation, transcript, and debrief modals across all variants, enabling users to review professor critiques, missed statutory elements, and authoritative benchmark breakdowns. Filter options include *All* vs. *Deficient Only*.

* **Rich Markdown Study Sheet Exporter (`.md`) & Word Ready (`.doc`):**
  * Integrated study sheet export utilities formatting transcripts into numbered bold questions followed by clean structured paragraphs for seamless integration into Obsidian, Notion, or Microsoft Word with proper UTF-8 BOM encoding.

* **Custom Flashcard Studio, Bulk Ingestion & Cloze Deletions:**
  * Provided authoring tools supporting 5 distinct legal archetypes (Codal, Case FIR, Bar ALAC, Procedural, and Simple Q&A) with RemNote-style Cloze Deletions (`{{c1::...}}` with optional hints), bulk CSV/TSV/Anki import, and live in-place card editing (`Ctrl+E` or `E`) without losing active session state.

---

## 3. Storage & Cross-Device Synchronization

* **Isolated Subject-Specific IndexedDB Databases:**
  * Implemented dynamic database sharding tied to the active subject workspace and version (e.g., `BarRecitUniversalMasterDB_v14_1`), ensuring complete isolation of mastery ledgers, syllabi, and reference sources across different law subjects without cross-contamination.

* **Adaptive Mastery Ledger & Spaced Repetition:**
  * Maintained local tracking of student performance (`EXCELLENT`, `SATISFACTORY`, `DEFICIENT`), categorizing provisions and cases into *Critical Drill*, *Practicing*, and *Mastered* ledgers stored inside IndexedDB with automated weak-spot resurfacing and priority re-queuing (Leitner queue).

* **Serverless 6-Digit Device Pairing Sync:**
  * Replaced browser-blocked Google OAuth AppData flows with an ephemeral, zero-login 6-digit device pairing snapshot relay (`KVdb.io`).
  * *Workflow:* The primary device packages decks, sources, and mastery into a compressed JSON snapshot tied to a temporary 6-digit PIN (valid for 15 minutes). Entering this PIN on a secondary device (Android APK, tablet, or laptop) pulls and non-destructively merges all state in under 2 seconds.

* **Local Native Folder Backup & Master JSON Migration:**
  * Configured automated JSON subject auto-saving to local disk (`Documents\BarRecitationSync\`) for background synchronization via OneDrive, Google Drive Desktop, or Syncthing wrappers.
  * Maintained master workspace `.json` export and import utilities for manual cross-device migration.

---

## 4. Advanced Engine Enhancements & New Additions (Parts 4 & 5)

* **Session Auto-Resume (Part 4):**
  * Implemented automatic session state saving to `localStorage` (`bar_active_session_[Subject]`) on every card advance.
  * Added a startup check on app boot across all platforms (Web, macOS DMG, Android APK, and Windows EXE) that detects unfinished recitation runs and prompts: *"Found an active recitation session... Would you like to resume where you left off?"* Automatically clears when the batch concludes.

* **Multi-Paragraph & Sub-Clause Scope Parser Upgrade (Part 5A):**
  * Enhanced `expandDeterministicScopeItems` to automatically scan statutory sections for internal sub-letters `(a), (b), (c)` and multi-paragraph structures (such as Article 315 Estafa paragraphs 1, 2, and 3), unbundling them into distinct atomic cards rather than skipping or lumping them together into monolithic blocks.

* **Smart "Regenerate & Polish Deck" Tool (Part 5B):**
  * Added a **"✨ Polish"** button to every saved deck in the Library.
  * Built an interactive configuration modal displaying the deck's original settings (e.g., active dimensions and presets), allowing users to keep original configurations or modify presets (*Codal*, *Case FIR*, *Reviewer*, *Full Bar Sim*), strip low-value boilerplate (effectivity clauses), and inject active-recall Cloze Deletions while preserving Leitner mastery scores.

---

## 5. Cross-Platform Deployment & Tauri Distribution

* **Standalone Portability & Remote Webview Updates:**
  * Configured all simulators as completely self-contained single-file HTML applications incorporating Tailwind CSS and PDF.js via secure CDNs, requiring zero backend server installation or cloud configuration.
  * Maintained remote webview routing via GitHub Pages (`https://ph-bar-recit.github.io/bar-recitation-simulator/`) and Tauri GitHub Actions, ensuring that every `git push origin main` instantly propagates updates to all installed Tauri desktop and mobile client wrappers without requiring re-installations.