# Changelog

## [0.7.1](https://github.com/MaikBuse/syndicode/compare/v0.7.0...v0.7.1) (2025-04-13)


### Bug Fixes

* github rust ci ([cd7c62a](https://github.com/MaikBuse/syndicode/commit/cd7c62aee4bf781b1980c482ef279fddab022264))
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
