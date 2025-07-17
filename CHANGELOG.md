# Changelog

## [0.14.0](https://github.com/MaikBuse/syndicode/compare/v0.13.0...v0.14.0) (2025-07-17)


### Features

* add visuals for headquarter display on map ([e4cd0f0](https://github.com/MaikBuse/syndicode/commit/e4cd0f07a896b0c01bc2367e856122c8b5457649))
* **server:** increase building layer performance by reducing calculations ([36b1fe8](https://github.com/MaikBuse/syndicode/commit/36b1fe8d5e4f95163fe42951f217004543be5e0f))
* **web:** add custom map ([92e8640](https://github.com/MaikBuse/syndicode/commit/92e86405f04c2be58768c9c56981996e4b5d286e))
* **web:** add sidebar ([d3f1979](https://github.com/MaikBuse/syndicode/commit/d3f19796bcfdde5efee6dcf9e62f82c868e18275))
* **web:** autofocus on forms ([bcf7652](https://github.com/MaikBuse/syndicode/commit/bcf765295ca1ccd1dc83f51e1ff1cfb91e60a122))
* **web:** send people to verification if user inactive ([b7283f5](https://github.com/MaikBuse/syndicode/commit/b7283f513155d2108fe825f384862ae521ece854))

## [0.13.0](https://github.com/MaikBuse/syndicode/compare/v0.12.2...v0.13.0) (2025-07-16)


### Features

* add just commands to build and upload pbf files ([4d794bf](https://github.com/MaikBuse/syndicode/commit/4d794bf116503ee2f92d7fcaa2d39cbc53487ab3))
* introducing headquarter buildings ([1fe2e6d](https://github.com/MaikBuse/syndicode/commit/1fe2e6de8cced1fd131d2a1a9f22ad38fd2f6bb5))
* **server:** add building volume to database ([f91c29b](https://github.com/MaikBuse/syndicode/commit/f91c29bf4346b0cc2a074c4b5a08c2103c3610ba))
* **server:** add gml id to query businesses request ([5d17018](https://github.com/MaikBuse/syndicode/commit/5d17018c7107807e1fe44224d824a4ef4e562099))
* **server:** add owning business to query building request ([95c810b](https://github.com/MaikBuse/syndicode/commit/95c810bf069321bf4e22bebd6c8b10cd1ef8dc17))
* **server:** introducing request to query businesses ([e2dbc85](https://github.com/MaikBuse/syndicode/commit/e2dbc856d6d3ac22bcb30773bfe4361b105de87f))
* update TILE_URL to cdn url ([0d36c40](https://github.com/MaikBuse/syndicode/commit/0d36c407325b322ab9405e95cebbe086a12b5a55))
* **web:** add boundary around the game area ([18de2c0](https://github.com/MaikBuse/syndicode/commit/18de2c00d01f066436cd65525478e6fad4ed2439))
* **web:** add visual boundary around tokyo ([cdbecc3](https://github.com/MaikBuse/syndicode/commit/cdbecc311e96e9e2b148d5bff75af56f8efc41a8))
* **web:** adjust zooms and starting camera position ([23f7960](https://github.com/MaikBuse/syndicode/commit/23f7960d230ee0b0035dadce7157d2d8e4f39753))
* **web:** highlight owned businesses on map ([f13d4e5](https://github.com/MaikBuse/syndicode/commit/f13d4e5f523e4eb31d57f2352f5df835f4110406))


### Bug Fixes

* make logging less verbose ([18f117f](https://github.com/MaikBuse/syndicode/commit/18f117f2802f12c4ab63f2c8c52d7cf6149da2b1))
* **server:** calculation of building volume ([24a5d98](https://github.com/MaikBuse/syndicode/commit/24a5d98a86793f2058d1087b7ae3d9a3787bcc79))
* **server:** command to insert buildings into db ([a10d29d](https://github.com/MaikBuse/syndicode/commit/a10d29d8f7745f1dbc841a96a2a2cd049e5e3ec7))
* **server:** error handling on login ([f0d02ba](https://github.com/MaikBuse/syndicode/commit/f0d02ba5cb9ae0cf0036a3f986166bd55725f799))
* **server:** parameters to query businesses ([cab7d82](https://github.com/MaikBuse/syndicode/commit/cab7d820a14968be90d74265f4132be435734bb8))
* **syndicode:** set cdn as tile url ([c23e075](https://github.com/MaikBuse/syndicode/commit/c23e0757ab127af7cdabe12ee2569fddfe609e85))
* **web:** allign zoom levels ([88abc86](https://github.com/MaikBuse/syndicode/commit/88abc8663477dc41279fb20bb56f6ab384ad92a7))
* **web:** regenerate proto files ([f1aacb6](https://github.com/MaikBuse/syndicode/commit/f1aacb6b155b69f9b7788a1fe68d0d9446f296b8))
* **web:** remove unused viewstate ([6271a66](https://github.com/MaikBuse/syndicode/commit/6271a661460a91290b6cf624324d07328776ff2b))
* **web:** type errors ([b842926](https://github.com/MaikBuse/syndicode/commit/b84292692e217f54f2f1f728567ac2e883c2cb2f))

## [0.12.2](https://github.com/MaikBuse/syndicode/compare/v0.12.1...v0.12.2) (2025-07-12)


### Bug Fixes

* only run migration on elected leader to prevent multiple migration attempts ([ae7fb7a](https://github.com/MaikBuse/syndicode/commit/ae7fb7ac2cd92e29b63e3d4098a1984ed6b2e2f7))
* set env vars for unit test ([3c66a7d](https://github.com/MaikBuse/syndicode/commit/3c66a7df8a482867adf8b3c01d5f0c4cdf54b823))

## [0.12.1](https://github.com/MaikBuse/syndicode/compare/v0.12.0...v0.12.1) (2025-07-09)


### Bug Fixes

* **ci:** set working directory to config permissions ([a55310b](https://github.com/MaikBuse/syndicode/commit/a55310b7cb8a0edae9810fdcaa13abf35d93c7e2))

## [0.12.0](https://github.com/MaikBuse/syndicode/compare/v0.11.0...v0.12.0) (2025-07-08)


### Features

* add logs ([61caf3a](https://github.com/MaikBuse/syndicode/commit/61caf3ac5aaf1438ffdee657fbd9406e17efcd58))
* introducing database restore mechanism ([7a72d53](https://github.com/MaikBuse/syndicode/commit/7a72d53fe0d4e03f66bd80d22791ecbfe6554d48))


### Bug Fixes

* **ci:** database env vars ([9544ad8](https://github.com/MaikBuse/syndicode/commit/9544ad8e875058de96f6b4f498c3cb91f0690252))
* **ci:** env names ([1e81365](https://github.com/MaikBuse/syndicode/commit/1e8136551bd1a8589963d96f5b8b7ed87a157197))
* **ci:** env var reading ([75848fe](https://github.com/MaikBuse/syndicode/commit/75848fe3c968df2ab843edbd5fee1acc356e655a))
* **ci:** explicitely set postgres version ([ec92902](https://github.com/MaikBuse/syndicode/commit/ec92902ea1fe42705027d585a798ff4975c9b72b))
* **ci:** improve logging ([76d3aec](https://github.com/MaikBuse/syndicode/commit/76d3aec0667caeb6645d3063507cdbf287d24acb))
* **ci:** server address on the client to localhost ([18865fb](https://github.com/MaikBuse/syndicode/commit/18865fbade381fd43a5cebeffab815d5be385608))
* **ci:** set valkey password env ([5072c3c](https://github.com/MaikBuse/syndicode/commit/5072c3ca8f805dc0c641d8f21d6daaa050e09c25))
* **server:** spelling ([7f61975](https://github.com/MaikBuse/syndicode/commit/7f61975baa60e5e673c361b17e66c871baa3754f))
* **server:** stackoverflow crash ([60952b2](https://github.com/MaikBuse/syndicode/commit/60952b2cfbe2b6029b01550c16ff8e7dc9caa0fd))
* **server:** use tracing instead of print ([5e1038e](https://github.com/MaikBuse/syndicode/commit/5e1038e0c74f42dd0eaa933a7c1aa6ede017a952))
* **web:** prevent failthrough ([ecd9070](https://github.com/MaikBuse/syndicode/commit/ecd9070d9d2292e85ff57daf95a03d09a8579081))

## [0.11.0](https://github.com/MaikBuse/syndicode/compare/v0.10.0...v0.11.0) (2025-07-06)


### Features

* add stricter parameters for clippy ([e9d602c](https://github.com/MaikBuse/syndicode/commit/e9d602cafad96349ef76c0532240f2a64e661303))
* change licensing to CC BY-NC 4.0 ([f91068b](https://github.com/MaikBuse/syndicode/commit/f91068b0655fd82adaac1aedf55d86e909629ccc))
* **client:** add clippy commands ([8e0b0e0](https://github.com/MaikBuse/syndicode/commit/8e0b0e0183825e15e13f93f41e9e1d451a2cf0ef))
* **client:** always place config file in workspace root ([9761050](https://github.com/MaikBuse/syndicode/commit/9761050ce46193de8b23c79bdeed06eee5e90154))
* color owned buildings on the map ([25fa01a](https://github.com/MaikBuse/syndicode/commit/25fa01a8f3cd9a6a2935860325ab98e83558f098))
* forward ip address from client to server ([286761c](https://github.com/MaikBuse/syndicode/commit/286761c5ca9b17983da242d82c307cbd90b60300))
* include more features in clippy fix ([f81b0dc](https://github.com/MaikBuse/syndicode/commit/f81b0dc7cf28dcdc995e06bd7f33354329b3d79c))
* **proto:** move proto files to the project root in order to make it available for all modules ([225b803](https://github.com/MaikBuse/syndicode/commit/225b803264a41ba1754fc371ecea3a50837ef47f))
* **server:** add fast path for health-checks in middleware ([2e22a32](https://github.com/MaikBuse/syndicode/commit/2e22a3250aaef9d80807daf77f755647675700fb))
* **server:** add more informative error messages on login ([8d07d99](https://github.com/MaikBuse/syndicode/commit/8d07d9922cf8224391f28d28623d30103d0817f2))
* **server:** add more initialization features ([23b2e9d](https://github.com/MaikBuse/syndicode/commit/23b2e9da6fb45ee2bfcf262bc7ea70496ca4a621))
* **server:** change the logo to an svg ([d98d40e](https://github.com/MaikBuse/syndicode/commit/d98d40e7f8630ac42e68a60498a26c5f5f13b3c6))
* **server:** get requesting ip address from alternative header when called by proxy ([3e35411](https://github.com/MaikBuse/syndicode/commit/3e35411f3d26325a6861cd35b790b7f251572ce2))
* **server:** introducing business ownership ([e999f62](https://github.com/MaikBuse/syndicode/commit/e999f62ff3b21429b7cde907e3bdf44d09968604))
* **server:** optimized processing by reducing round trips to the database ([2fe629b](https://github.com/MaikBuse/syndicode/commit/2fe629b4da67d9bf9df7a6b6bbbb0461dff8652d))
* **server:** parse buildings from parquet file during bootstrap ([53c13c5](https://github.com/MaikBuse/syndicode/commit/53c13c59714383929ef27bf128e6c58711a1f4f7))
* **server:** query building ownerships ([89a8807](https://github.com/MaikBuse/syndicode/commit/89a8807e19e41a625d252a8fe7d9842d82c4551f))
* **server:** reduce default business count ([96fb4fb](https://github.com/MaikBuse/syndicode/commit/96fb4fb09d394b783862d1df4f528252d7be6e48))
* **server:** return rate limit error as game update in stream ([ed578fb](https://github.com/MaikBuse/syndicode/commit/ed578fbcfe39c130e742ee3117c069d8bf764279))
* **server:** separate building parquet by wards ([e1dc21e](https://github.com/MaikBuse/syndicode/commit/e1dc21e2f87e4b27bd473f718770807bf2a378b7))
* **server:** use apache arrow for columnar import of parquet ([aa55de4](https://github.com/MaikBuse/syndicode/commit/aa55de43ceab87b4dac1d68d18987a9fde52e633))
* **web:** add login, register and verify flow ([5bdbf2a](https://github.com/MaikBuse/syndicode/commit/5bdbf2a1dd9e1e499d8a5e124f9f253ed3efce80))
* **web:** add stricter checks ([a432872](https://github.com/MaikBuse/syndicode/commit/a4328721c86e796fcaf6b521b06bca752f03b6a3))
* **web:** implement authentication flows ([fd1a956](https://github.com/MaikBuse/syndicode/commit/fd1a956201d99f0c378426605dd1053289420ad1))
* **web:** introducing syndicode-web ([e581e28](https://github.com/MaikBuse/syndicode/commit/e581e28255114002e59e17e61df48b87743b6d97))
* **web:** style with cyberpunk theme ([26f18ac](https://github.com/MaikBuse/syndicode/commit/26f18ac2aa45b009d2d20f214be47c59568e1c5d))


### Bug Fixes

* **client:** init crypto provider only once during runs and tests ([6c7a6ac](https://github.com/MaikBuse/syndicode/commit/6c7a6acf7628dc6edade82484fde8d3c12be2441))
* **client:** remove config unit tests ([9507f71](https://github.com/MaikBuse/syndicode/commit/9507f71bb8b848cd65bb00579580ce066aec7e5a))
* **project:** configuration with env vars ([4300195](https://github.com/MaikBuse/syndicode/commit/4300195e1c78b39c698b4368c4d6252fb07611d8))
* **server:** adjust data type to parquet schema ([2fd1daa](https://github.com/MaikBuse/syndicode/commit/2fd1daa76784fada440fece80ae760ba29d75f9c))
* **server:** remove catch-up-logic for when tick processing fails ([f13af88](https://github.com/MaikBuse/syndicode/commit/f13af88ceeabde0daccf7962ee792c8ca1288f5f))
* **server:** use postgis geometry types ([0aaa928](https://github.com/MaikBuse/syndicode/commit/0aaa928bd02cdf01ab980cb2a86ee9658f428d38))

## [0.10.0](https://github.com/MaikBuse/syndicode/compare/v0.9.9...v0.10.0) (2025-06-15)


### Features

* **client:** overwrite config with env vars if present ([7840995](https://github.com/MaikBuse/syndicode/commit/78409958d3d993e2b7dcbd299b0ebdf74226ac04))
* **client:** set default config to the official server ([ef32b11](https://github.com/MaikBuse/syndicode/commit/ef32b113aee244063bbdb4b5e4d688bb1fc35ee8))


### Bug Fixes

* **client:** check for press events ([4ff6067](https://github.com/MaikBuse/syndicode/commit/4ff606705fce3603f92f4746bfb4e234142adbc5))
* **client:** fix mayor tls handshake issue ([65443ec](https://github.com/MaikBuse/syndicode/commit/65443ece395bf49bbe61b9214c075d4982ce3a06))

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
