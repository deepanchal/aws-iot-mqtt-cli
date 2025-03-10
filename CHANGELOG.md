# Changelog

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
