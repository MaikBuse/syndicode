# Changelog

## [0.9.9](https://github.com/MaikBuse/syndicode/compare/v0.9.8...v0.9.9) (2025-06-13)


### Bug Fixes

* **client:** change log path to the binary directory ([38b607d](https://github.com/MaikBuse/syndicode/commit/38b607d692bcf53b9cbf4b45cc3d158af299ad75))

## [0.9.8](https://github.com/MaikBuse/syndicode/compare/v0.9.7...v0.9.8) (2025-06-13)


### Bug Fixes

* **ci:** add linker for the aarch build ([b79e35d](https://github.com/MaikBuse/syndicode/commit/b79e35d229a97d99bc6f50deab34412337e2cc28))

## [0.9.7](https://github.com/MaikBuse/syndicode/compare/v0.9.6...v0.9.7) (2025-06-13)


### Bug Fixes

* **ci:** switch linux to musl to increase compatability ([1218ee3](https://github.com/MaikBuse/syndicode/commit/1218ee3a5e4e1924ba5865174339dde5de9cb8ba))

## [0.9.6](https://github.com/MaikBuse/syndicode/compare/v0.9.5...v0.9.6) (2025-06-13)


### Bug Fixes

* **ci:** pin the ubuntu version in order to improve compatibility ([93df2ef](https://github.com/MaikBuse/syndicode/commit/93df2ef8fb88b9f6c3faacd139e0d6c21a59806c))

## [0.9.5](https://github.com/MaikBuse/syndicode/compare/v0.9.4...v0.9.5) (2025-06-13)


### Bug Fixes

* **ci:** release asset path ([f969f74](https://github.com/MaikBuse/syndicode/commit/f969f74d93ba643fb0d19796783f15e13d96900e))

## [0.9.4](https://github.com/MaikBuse/syndicode/compare/v0.9.3...v0.9.4) (2025-06-13)


### Bug Fixes

* (ci): stop explicitly setting the protoc path ([0fcefa7](https://github.com/MaikBuse/syndicode/commit/0fcefa7dd1e9c8ae2581acf7784dea2c369b1dcd))

## [0.9.3](https://github.com/MaikBuse/syndicode/compare/v0.9.2...v0.9.3) (2025-06-13)


### Bug Fixes

* **ci:** set explicit protoc path ([371a723](https://github.com/MaikBuse/syndicode/commit/371a723fd96d61773e25a0ff149b5510f5a41353))

## [0.9.2](https://github.com/MaikBuse/syndicode/compare/v0.9.1...v0.9.2) (2025-06-13)


### Bug Fixes

* **ci:** cache cargo dependencies only ([d4a68a6](https://github.com/MaikBuse/syndicode/commit/d4a68a6b761a908ae811084a0d67344d9c3c0bd2))

## [0.9.1](https://github.com/MaikBuse/syndicode/compare/v0.9.0...v0.9.1) (2025-06-13)


### Bug Fixes

* **ci:** set token for protoc setup ([81cda9c](https://github.com/MaikBuse/syndicode/commit/81cda9cbbc0f9d11c96a4166499eb73688c01c0e))

## [0.9.0](https://github.com/MaikBuse/syndicode/compare/v0.8.1...v0.9.0) (2025-06-13)


### Features

* add checks for fmt and clippy ([7c8da50](https://github.com/MaikBuse/syndicode/commit/7c8da50c16e4a012a51ce0e5518f69d9f8afa7f7))
* add instructions to resend verification mail and login ([f6b87a9](https://github.com/MaikBuse/syndicode/commit/f6b87a99a411db6e1866c551a1076d47862b3df3))
* change licensing to cc ([5ee0a87](https://github.com/MaikBuse/syndicode/commit/5ee0a871706a743f8511a56b3ddf43a3781397fa))
* **ci:** add recipe to format the workspace ([6d0e3b1](https://github.com/MaikBuse/syndicode/commit/6d0e3b1809bbce5d7764ddb17831fd82553760dc))
* **ci:** build and publish the syndicode-client on release ([99d02b9](https://github.com/MaikBuse/syndicode/commit/99d02b9a7fc3488fe4ba82fec9cbfe9c4683a450))
* **ci:** improve caching ([c96645f](https://github.com/MaikBuse/syndicode/commit/c96645f7242059e0454a58511f25bc3b585e7f1e))
* **client:** Acquire business listings ([70763e8](https://github.com/MaikBuse/syndicode/commit/70763e8f94f28ccb3598c810491e488bb28fcb10))
* **client:** add get corporation ([3c5d742](https://github.com/MaikBuse/syndicode/commit/3c5d7429eb973c916157752a3ed7f13bcd10e7ea))
* **client:** display in stream errors ([6639c32](https://github.com/MaikBuse/syndicode/commit/6639c32204fb2d61dae8a85c20703c809fb79f1a))
* **client:** improve error handling on connection failure ([c411969](https://github.com/MaikBuse/syndicode/commit/c411969f6b0f069a629b55cebfcb0928353e7180))
* **client:** improve logging ([6b27aef](https://github.com/MaikBuse/syndicode/commit/6b27aefa1d95dcc8889015ced22b79fd9658b684))
* **client:** improve shutdown mechanics ([8d0e117](https://github.com/MaikBuse/syndicode/commit/8d0e11707a83d9d918bcba6929fdecd8068e60fe))
* **client:** introducing admin requests ([efd4617](https://github.com/MaikBuse/syndicode/commit/efd4617c28fe8aa6139ae36560dee2cea5940be2))
* **client:** introducing response detail view ([bb62168](https://github.com/MaikBuse/syndicode/commit/bb621682cec88c1291f5bd6c43c295b8717a5b36))
* **client:** mask password inputs ([2b0241f](https://github.com/MaikBuse/syndicode/commit/2b0241f3d3fc1ff9ae351d55f6ccc663d641f642))
* handle user deltion during game tick ([fecb1ae](https://github.com/MaikBuse/syndicode/commit/fecb1aef1ac99ca02881bbdb2b9b363bf77cdd37))
* introduce integration tests ([9dc4ab2](https://github.com/MaikBuse/syndicode/commit/9dc4ab2a36b2a1b408cf3554834436d333c1477f))
* introducing basic economy gameloop ([7faf385](https://github.com/MaikBuse/syndicode/commit/7faf38511769202bfe71ffe11d7517b1f0f16e9a))
* introducing get user service ([d7dd5bb](https://github.com/MaikBuse/syndicode/commit/d7dd5bbb2af64e52388f3afb0d2aa92aef719ae2))
* introducing grpcs tui client ([19dcb83](https://github.com/MaikBuse/syndicode/commit/19dcb837b88d2d4643ee373058d5a767d4301b13))
* query business listings by multiple parameters ([a7cfc30](https://github.com/MaikBuse/syndicode/commit/a7cfc3074597f8421c8414824f0dc77a46090055))
* send game tick notifications to clients ([46ad925](https://github.com/MaikBuse/syndicode/commit/46ad92530863903fdcda4a1b87a1b3ebbafb0c39))
* **server:** improved error handling by introducing a PresentationError ([5385004](https://github.com/MaikBuse/syndicode/commit/5385004f435efd317908a5c38efd82d2d5a44bb9))
* **server:** introducing business income calculation ([7f2407d](https://github.com/MaikBuse/syndicode/commit/7f2407db1eeb042705688560b378419d73454664))
* **server:** return stream errors as game updates ([6b38f21](https://github.com/MaikBuse/syndicode/commit/6b38f216c4a64608aaae51ff1dfc5d693467404d))


### Bug Fixes

* **client:** box enum variants with large content ([ebfa3d5](https://github.com/MaikBuse/syndicode/commit/ebfa3d5362053b1c422989ade3cc880db73e348a))
* **client:** remove active line underscore ([ad2643b](https://github.com/MaikBuse/syndicode/commit/ad2643bff74ed9fec344ea5b6acd50d9617f8b9a))
* **client:** return empty uuids on request ([d08c3a2](https://github.com/MaikBuse/syndicode/commit/d08c3a2759d357c9b10945d7a33cf204f8b4f58d))
* **client:** setup crossterm dependency to improve shutdown ([aee3ee6](https://github.com/MaikBuse/syndicode/commit/aee3ee64f645a4f885a88ab170fd3732a835d861))
* create user unit tests ([e0c2506](https://github.com/MaikBuse/syndicode/commit/e0c2506ec4e88bd7927ad40b70bd946cf1e4af2d))
* port and protocoll of grpcurl call ([32987a0](https://github.com/MaikBuse/syndicode/commit/32987a0bd70c4519a27edfb7d9ddbd6a956842d7))
* **server:** enable all query parameters on business listings ([8123f73](https://github.com/MaikBuse/syndicode/commit/8123f732de1a68191cb365bb4bb774047cb672e1))
* spelling mistake ([a8e9a76](https://github.com/MaikBuse/syndicode/commit/a8e9a7662a55df88bdb9d6a9c4d8b815df136715))

## [0.8.1](https://github.com/MaikBuse/syndicode/compare/v0.8.0...v0.8.1) (2025-04-20)


### Bug Fixes

* update database preparation ([8cfcefd](https://github.com/MaikBuse/syndicode/commit/8cfcefd7262a559280fc60bc4dc47350906495ee))

## [0.8.0](https://github.com/MaikBuse/syndicode/compare/v0.7.0...v0.8.0) (2025-04-20)


### Features

* introducing complex responses ([87ae101](https://github.com/MaikBuse/syndicode/commit/87ae1013b0b1661aa6d180a313f86e3dad86bc8a))
* introducing user verification on registration ([5626683](https://github.com/MaikBuse/syndicode/commit/56266837a53f0ff934c2029be71c5f0869939b34))
* rate limit by category ([8377c96](https://github.com/MaikBuse/syndicode/commit/8377c9666a0c0bb0ff23124724d46ffb07f73f61))


### Bug Fixes

* github rust ci ([1d191c5](https://github.com/MaikBuse/syndicode/commit/1d191c513f93b0f0b32d88ca68a9a27dee96baa2))
* stop rate limits for health checks ([3674f1c](https://github.com/MaikBuse/syndicode/commit/3674f1c104c8ba28258cf31a83f8333e5e861452))

## [0.7.0](https://github.com/MaikBuse/syndicode/compare/v0.6.0...v0.7.0) (2025-04-13)


### Features

* update sqlx preparation ([18ab05e](https://github.com/MaikBuse/syndicode/commit/18ab05ea391e8713830d4e7a00831153b305bd3b))

## [0.6.0](https://github.com/MaikBuse/syndicode/compare/v0.5.0...v0.6.0) (2025-04-09)


### Features

* add test for create user uc ([da60707](https://github.com/MaikBuse/syndicode/commit/da60707563a3e1711ad91dff39490441ca1e23a9))
* consume actions from valkey stream ([e6c9301](https://github.com/MaikBuse/syndicode/commit/e6c930154b682465b83a4046ea20d5f858eb725d))
* implement fundamental processing logic ([6f35157](https://github.com/MaikBuse/syndicode/commit/6f35157f50fd1ab7784a0404121ad8a540b0a1d7))
* implement leader election with multiple server instances ([62f5ab4](https://github.com/MaikBuse/syndicode/commit/62f5ab4ad70a0e993f9a1d29bdac6218ac719e24))
* improve password verification ([9b65a5e](https://github.com/MaikBuse/syndicode/commit/9b65a5e78550b3a797f365846b2105c18f9575e2))
* introducing rate limiting for requests ([9f48d30](https://github.com/MaikBuse/syndicode/commit/9f48d30cb0e35285d6c72d14eb2afbbb6a536a83))
* introducing sleeping logic to establish fixed game tick intervals ([a2bb6de](https://github.com/MaikBuse/syndicode/commit/a2bb6de065b797843347b8c37d40f9a163caf020))
* introducing valkey based rate limiting ([a78bc02](https://github.com/MaikBuse/syndicode/commit/a78bc0235464a6ba3d0426697c06ea2e39219257))
* moved logic from action handler to use cases ([7b5b963](https://github.com/MaikBuse/syndicode/commit/7b5b96314704462cbfabbc8f4a5ff71b3735b90d))
* push actions to a valkey stream in order to enable decoupled handling ([2fdea02](https://github.com/MaikBuse/syndicode/commit/2fdea02b7de881cd589c84633b77dcf8ae487821))

## [0.5.0](https://github.com/MaikBuse/syndicode/compare/v0.4.2...v0.5.0) (2025-03-30)


### Features

* add authentication checks to delete- and get user ([cd5942d](https://github.com/MaikBuse/syndicode/commit/cd5942dffcd567351f9921e493faf875307ead7c))
* add checks for username and password ([a4caf97](https://github.com/MaikBuse/syndicode/commit/a4caf973d2ebfe4e9e0d80dac8cc18e7a199be85))
* create a corporation with every new user ([5b89d7d](https://github.com/MaikBuse/syndicode/commit/5b89d7dec541202dbb66929d6055c0f49d59f0f6))


### Bug Fixes

* remove unneeded request attributes ([f9431c9](https://github.com/MaikBuse/syndicode/commit/f9431c90e1780918708ecae698fa6f80816443ac))

## [0.4.2](https://github.com/MaikBuse/syndicode/compare/v0.4.1...v0.4.2) (2025-03-30)


### Bug Fixes

* stop asking for authentication on registration ([81ceeda](https://github.com/MaikBuse/syndicode/commit/81ceedae8272cca94711550369baf4388f4b873e))
* wrong comparison on skip logging check ([dbf54df](https://github.com/MaikBuse/syndicode/commit/dbf54df618fc937c9a107bca914f4e11822a0f77))

## [0.4.1](https://github.com/MaikBuse/syndicode/compare/v0.4.0...v0.4.1) (2025-03-30)


### Bug Fixes

* exclude registration from authentication middleware ([59e0951](https://github.com/MaikBuse/syndicode/commit/59e09512b73848f2c5944f58f16be588e93e880e))

## [0.4.0](https://github.com/MaikBuse/syndicode/compare/v0.3.2...v0.4.0) (2025-03-30)


### Features

* check authorization on user creation ([8d2e42f](https://github.com/MaikBuse/syndicode/commit/8d2e42f7f7463bbc0d65ffc7c1dd23a2bad8f3f5))


### Bug Fixes

* directory issue ([2075cfb](https://github.com/MaikBuse/syndicode/commit/2075cfbd9e97c37a5935861db26755b66b98a7e3))

## [0.3.2](https://github.com/MaikBuse/syndicode/compare/v0.3.1...v0.3.2) (2025-03-30)


### Bug Fixes

* directory of syndicode server ([c21d4d6](https://github.com/MaikBuse/syndicode/commit/c21d4d60226e05c19bf989b7c83038a669f54336))

## [0.3.1](https://github.com/MaikBuse/syndicode/compare/v0.3.0...v0.3.1) (2025-03-30)


### Bug Fixes

* application name according to new directory ([bc0c6f9](https://github.com/MaikBuse/syndicode/commit/bc0c6f956f101754cc54b05dce7b7eb75000794e))

## [0.3.0](https://github.com/MaikBuse/syndicode/compare/v0.2.0...v0.3.0) (2025-03-30)


### Features

* enable automated builds after releases ([5ae81a2](https://github.com/MaikBuse/syndicode/commit/5ae81a2197ce5a48e1aec8007d0cf2421d6696b6))
* remove sessions in favor of on global game world ([dbc95d2](https://github.com/MaikBuse/syndicode/commit/dbc95d2c63633a38fd37885077f56f6df68c136a))

## [0.2.0](https://github.com/MaikBuse/syndicode/compare/0.1.4...v0.2.0) (2025-03-29)


### Features

* add a table of contents ([c76566d](https://github.com/MaikBuse/syndicode/commit/c76566dd36836701fa9181497a7c148b61cffc72))
* add more detail to the getting-started section ([9ea51ce](https://github.com/MaikBuse/syndicode/commit/9ea51ceec65f5aae98a462bd27fa3090a60f54a1))
* add release please ([78d3e80](https://github.com/MaikBuse/syndicode/commit/78d3e80724912c967ee56a657ed0d4d0deac4cb1))
* stop logging of health checks ([55eba3d](https://github.com/MaikBuse/syndicode/commit/55eba3d017fa288b8ce712f11d1c841b8982e6b6))


### Bug Fixes

* switch to GITHUB_TOKEN ([de4bc78](https://github.com/MaikBuse/syndicode/commit/de4bc78355f75850833e664d6a836100cfa66448))
