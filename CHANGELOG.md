# Changelog

## [0.4.0](https://github.com/deepanchal/aws-iot-mqtt-cli/compare/v0.3.0...v0.4.0) (2025-06-19)


### Features

* **app:** add colors for clap cli ([16dff27](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/16dff274d705a84b8747830e95d67f6cabdcf568))


### Bug Fixes

* **app:** increase mqtt max_packet_size option ([2b905ca](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/2b905ca09108c4dde2f1aada0e8896fade649433))
* **deps:** update rust crate chrono to v0.4.41 ([f5dbb45](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/f5dbb450448237758154fc84e87a7042beaf50cc))
* **deps:** update rust crate clap to v4.5.34 ([424cd12](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/424cd123ea81dcccb2b3b53d50bcd689f85ac225))
* **deps:** update rust crate clap to v4.5.40 ([7fc14b3](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/7fc14b306c8ee300dea5b0219a7f84b09d6d8e4b))
* **deps:** update rust crate env_logger to v0.11.8 ([9865ab9](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/9865ab9319d803626e5a46295c0d02e494ac6766))
* **deps:** update rust crate log to v0.4.27 ([4035ebc](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/4035ebc0f037b58b4ad574a830f1e5b55fdbc4fb))
* **deps:** update rust crate tokio to v1.45.1 ([f54c5ad](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/f54c5adabf5f2f37fe7a1b1f343d6b87d9fe0738))

## [0.3.0](https://github.com/deepanchal/aws-iot-mqtt-cli/compare/v0.2.0...v0.3.0) (2025-03-17)


### Features

* **app:** upgrade project edition to 2024 following docs, format code ([9e9b574](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/9e9b5743b8a2773280e8245ebbed6bc1a843bca4))
* **app:** use pretty compact formatter to format json payloads ([d13e814](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/d13e814a0010df6df7f66852c35241951f81ee50))


### Bug Fixes

* **deps:** update rust crate clap to v4.5.32 ([1defde7](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/1defde70a1bcda313bd4cb4e0da497cc72ab47bf))
* **deps:** update rust crate tokio to v1.44.1 ([31c1eda](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/31c1edac60aa03c57ddf89a6b65291728847defe))
* **pkg:** format + sort deps in Cargo.toml ([856cfcc](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/856cfcc0c971296876f86864adb9579f85b278eb))
* **pkg:** update deps with cargo update ([adddc0e](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/adddc0e859d8d19ce1d113dcb4f548ef5910c1ef))

## [0.2.0](https://github.com/deepanchal/aws-iot-mqtt-cli/compare/v0.1.0...v0.2.0) (2025-03-10)


### Features

* **app:** add mqtt port flag, use rumqttc client directly for sub/pub, enable clean session ([d4351b1](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/d4351b175e7509b252332dfa782f9b6c5f6bdb3b))
* **app:** handle graceful shutdown w/ ctrl+c, unsubscribe from mqtt topics on shutdown ([bd1a01e](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/bd1a01e519aed3662af9b9d1eca49b4dc51a90cd))


### Bug Fixes

* **app:** print long help if no subcommands are provided by user ([d1d22e0](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/d1d22e0b7f230e875d1fcb690d704627873fee6e))
* **app:** update cli path flags, add setup_logging, refactor code ([1074087](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/10740873ddef2bf22e351cf0c0c6cc1c4fa7b7b8))
* **deps:** update rust crate env_logger to v0.11.7 ([5108021](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/51080214daf5cc497ca0811ae6265c5c5886f4f1))


### Code Refactoring

* **app:** refactor mqtt log formatting into format.rs module, update code usages ([b4ea7dc](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/b4ea7dcbecc9dd6b4a889d5770a1ec3ea79649e0))

## 0.1.0 (2025-03-09)


### Features

* **app:** add cli script w/ mqtt sub & pub functionality ([2ea2f14](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/2ea2f14723ff250d11f2e512f6cb085e29a67f8e))
* **app:** allow setting aws iot settings w/ env vars & cli flags, add verbose logging ([f64d175](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/f64d1753baced6302a5ecfbf47b279b3db1407d1))
* **init:** init repo w/ cargo init ([d53ae2e](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/d53ae2e3adfc6c6d74da2c63e4fb5bd7b567a0cc))


### Bug Fixes

* **config:** setup config for auto release from release-please ([42be533](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/42be53302043a9288b1eed34adcc0781b6d1e3c8))
* **lint:** fix cargo clippy warnings ([80b355c](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/80b355c6be0b971caafd70ebcfd838140c6aa79b))


### Documentation

* add README.md ([5077362](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/5077362d62b8a7829bbd5bf8c1b0dd954fb8bb23))


### Continuous Integration

* add cd.yml ([85a3acc](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/85a3acce75cbcd4b10bf259f0a61422a2956cc19))
* add ci.yml ([705314c](https://github.com/deepanchal/aws-iot-mqtt-cli/commit/705314c4d256a990e64a354e27b38064e6f3c302))
