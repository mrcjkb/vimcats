# Changelog

## 0.1.0 (2024-08-26)


### ⚠ BREAKING CHANGES

* gpl2+ license
* migrate cargo project
* better type parsing ([#47](https://github.com/mrcjkb/vimcats/issues/47))
* separate vimdoc from parser ([#42](https://github.com/mrcjkb/vimcats/issues/42))
* better emmylua grammar support ([#37](https://github.com/mrcjkb/vimcats/issues/37))
* remove TagType::Empty variant
* add `--prefix-func` and `--prefix-type` options ([#29](https://github.com/mrcjkb/vimcats/issues/29))
* don't add newline needlessly in function desc
* description should appear first in enum ([#23](https://github.com/mrcjkb/vimcats/issues/23))
* tokenize file in one step ([#14](https://github.com/mrcjkb/vimcats/issues/14))
* proper types ([#11](https://github.com/mrcjkb/vimcats/issues/11))
* `opt-level` to `3`
* replace `clap` with `lexopt`
* replace `tabled` with `tabular` + `textwrap`
* make `cli` optional to crate user

### change

* `opt-level` to `3` ([5a87e73](https://github.com/mrcjkb/vimcats/commit/5a87e73c46b83c81b4efce8fa03939b12e0173fb))
* description should appear first in enum ([#23](https://github.com/mrcjkb/vimcats/issues/23)) ([b6fbede](https://github.com/mrcjkb/vimcats/commit/b6fbede0b36f4c3369fdafd3040e7a74fd2a5ce1))
* don't add newline needlessly in function desc ([d314f90](https://github.com/mrcjkb/vimcats/commit/d314f90b1f1178792155990df791ae63f57b360a))
* make `cli` optional to crate user ([896e95f](https://github.com/mrcjkb/vimcats/commit/896e95f01c441e7b63567bc0edb58682224daba1))
* remove TagType::Empty variant ([b1a494e](https://github.com/mrcjkb/vimcats/commit/b1a494e4813fbf08242f825e6029faa57dcd2f75))


### Features

* add `---[@divider](https://github.com/divider)` ([#8](https://github.com/mrcjkb/vimcats/issues/8)) ([da4aab1](https://github.com/mrcjkb/vimcats/commit/da4aab124d9e630fb6821d3df6a49852e7cea991))
* add `---[@export](https://github.com/export)` ([#30](https://github.com/mrcjkb/vimcats/issues/30)) ([f66ff82](https://github.com/mrcjkb/vimcats/commit/f66ff828f375d51f708fd31f61dc24e580571803))
* add `---[@mod](https://github.com/mod)` ([#7](https://github.com/mrcjkb/vimcats/issues/7)) ([6081501](https://github.com/mrcjkb/vimcats/commit/608150160d915b5dc2d8b1516605f34950e3cdfc))
* add `---[@private](https://github.com/private)` ([#10](https://github.com/mrcjkb/vimcats/issues/10)) ([4f00536](https://github.com/mrcjkb/vimcats/commit/4f0053653ff7cbad67cdcce983135305ac18ac8f))
* add `---[@see](https://github.com/see)` support in `---[@type](https://github.com/type)` ([2b36e50](https://github.com/mrcjkb/vimcats/commit/2b36e50cf2a589f8fbb98ba6eea8f15830697eec))
* add `---[@tag](https://github.com/tag)` node ([0d52dc0](https://github.com/mrcjkb/vimcats/commit/0d52dc07360b2e6842517f83f3c1cde568c0cc11))
* add `---[@toc](https://github.com/toc)` (Table of Contents) generation ([#25](https://github.com/mrcjkb/vimcats/issues/25)) ([f90293d](https://github.com/mrcjkb/vimcats/commit/f90293d29e611cc2657330e5b839d34531852bf1))
* add `--expand-opt` flag ([#56](https://github.com/mrcjkb/vimcats/issues/56)) ([3bd37b5](https://github.com/mrcjkb/vimcats/commit/3bd37b536f8c8b2a9018fa7541cf30738f6e11cf)), closes [#44](https://github.com/mrcjkb/vimcats/issues/44)
* add `--indent` option ([#75](https://github.com/mrcjkb/vimcats/issues/75)) ([388acf0](https://github.com/mrcjkb/vimcats/commit/388acf0080b54bce5abede88e8ea7d16a05deea7))
* add `--layout` option and `--layout=compact:n` ([#59](https://github.com/mrcjkb/vimcats/issues/59)) ([711ff54](https://github.com/mrcjkb/vimcats/commit/711ff541c5d5a979ab72700ff00aea11d6612f85)), closes [#40](https://github.com/mrcjkb/vimcats/issues/40)
* add `--layout=mini:n` option ([#60](https://github.com/mrcjkb/vimcats/issues/60)) ([628d9e0](https://github.com/mrcjkb/vimcats/commit/628d9e08d03d7c5416020dcda806117142400413))
* add `--prefix-func` and `--prefix-type` options ([#29](https://github.com/mrcjkb/vimcats/issues/29)) ([2295f7a](https://github.com/mrcjkb/vimcats/commit/2295f7aa9e8c8d67126227e5be174e27f5c605b2))
* add support for (exact) class attributes ([#2](https://github.com/mrcjkb/vimcats/issues/2)) ([587fa69](https://github.com/mrcjkb/vimcats/commit/587fa693bcba7068f0e29960f71c5cdad77af7ba))
* added scope support for `---[@field](https://github.com/field)` ([#15](https://github.com/mrcjkb/vimcats/issues/15)) ([529e5c3](https://github.com/mrcjkb/vimcats/commit/529e5c33cb373799609a4a371490bfa8aa4eeb19))
* allow desc in `---[@alias](https://github.com/alias)` enum ([#18](https://github.com/mrcjkb/vimcats/issues/18)) ([d902592](https://github.com/mrcjkb/vimcats/commit/d902592d881140ec86789ef50c888a2c920b429c))
* allow function with no params or no returns ([490f30e](https://github.com/mrcjkb/vimcats/commit/490f30e10650394a7f72974e729cb9f429b7fd86))
* allow multiple `---[@mod](https://github.com/mod)` per file ([3880311](https://github.com/mrcjkb/vimcats/commit/38803110f3df2577938c2ab459858633f59b2617))
* assume `&lt;table&gt;.<field> =` as function if `---[@param](https://github.com/param)` is found ([#50](https://github.com/mrcjkb/vimcats/issues/50)) ([3fc909e](https://github.com/mrcjkb/vimcats/commit/3fc909ee00aded845f822ae011579b2d59709238)), closes [#31](https://github.com/mrcjkb/vimcats/issues/31)
* basic support for `---[@field](https://github.com/field) [&lt;type&gt;] <type>` ([8c05402](https://github.com/mrcjkb/vimcats/commit/8c054023347c9aa577dd3bcbed26e6e0a88900ec))
* better type parsing ([#47](https://github.com/mrcjkb/vimcats/issues/47)) ([7e46731](https://github.com/mrcjkb/vimcats/commit/7e46731f5bef19969f4a46585eaf16aa6674761a)), closes [#27](https://github.com/mrcjkb/vimcats/issues/27) [#48](https://github.com/mrcjkb/vimcats/issues/48)
* **cli:** add `--no-modeline` option ([3a05261](https://github.com/mrcjkb/vimcats/commit/3a05261df66483f1f268bffaf06943739f004deb))
* **cli:** add `--prefix-alias` option ([db4ddcf](https://github.com/mrcjkb/vimcats/commit/db4ddcf0d873519544ecddf186339a484d0d5ae1))
* **cli:** add `--prefix-class` option ([7c1eb1e](https://github.com/mrcjkb/vimcats/commit/7c1eb1e3fab0ea8c4e644c00b6d1976f252e2880))
* convert func to vim help ([de253db](https://github.com/mrcjkb/vimcats/commit/de253db728974597b3b607a77068daef29962039))
* enum support via `---[@alias](https://github.com/alias)` ([#16](https://github.com/mrcjkb/vimcats/issues/16)) ([c251679](https://github.com/mrcjkb/vimcats/commit/c251679a7e4a921a85da3f85b48953265a47439d))
* **func:** allow mutliple description comments ([053c6f0](https://github.com/mrcjkb/vimcats/commit/053c6f086c7b44fdd4209b5e00aaa627188f33fa))
* generate help for `---[@alias](https://github.com/alias)` ([5a8baf0](https://github.com/mrcjkb/vimcats/commit/5a8baf05b9953391a6acf33df87ce6d2637ea4f5))
* generate help for `---[@brief](https://github.com/brief)` ([f3595bd](https://github.com/mrcjkb/vimcats/commit/f3595bd9210a759d82e31665b1be893b4092b819))
* generate help for `---[@tag](https://github.com/tag)` ([89ea313](https://github.com/mrcjkb/vimcats/commit/89ea31383efc349070b976fd9325227eadab1545))
* generate help for `---[@type](https://github.com/type)` ([a0ded5b](https://github.com/mrcjkb/vimcats/commit/a0ded5b32f366ab3ae281d75a543ea71ec99eced))
* gpl2+ license ([b57371a](https://github.com/mrcjkb/vimcats/commit/b57371a8a9cb4d8034555f410571528f7b2af3aa))
* initial CLI ([beb4aa1](https://github.com/mrcjkb/vimcats/commit/beb4aa1085f9fa43847ca83a9e13a16af33124d9))
* language support in `---[@usage](https://github.com/usage)` ([#65](https://github.com/mrcjkb/vimcats/issues/65)) ([5dba98a](https://github.com/mrcjkb/vimcats/commit/5dba98a10033d5dbfc94c41f5b9368ac5a0a1d10))
* make `name` optional in `---[@return](https://github.com/return)` ([5959dd1](https://github.com/mrcjkb/vimcats/commit/5959dd130aad12e0885a5c8857b2dede4df64e34))
* migrate cargo project ([040f6fc](https://github.com/mrcjkb/vimcats/commit/040f6fc4e5a93033fc221071d63ad3836df14846))
* multiline `---[@field](https://github.com/field)` description ([#24](https://github.com/mrcjkb/vimcats/issues/24)) ([9577770](https://github.com/mrcjkb/vimcats/commit/957777012e9797554d8274701dd1498a9b85b9b0))
* multiline `---[@param](https://github.com/param)` description ([#22](https://github.com/mrcjkb/vimcats/issues/22)) ([2ad346a](https://github.com/mrcjkb/vimcats/commit/2ad346a52dc14daddd12c84592919c77410b57de))
* multiline `---[@return](https://github.com/return)` description ([#33](https://github.com/mrcjkb/vimcats/issues/33)) ([8b40d03](https://github.com/mrcjkb/vimcats/commit/8b40d03c7ea217b1ddadae9ef94b38a714b57130))
* multiline `---[@usage](https://github.com/usage)` ([#35](https://github.com/mrcjkb/vimcats/issues/35)) ([96a8e26](https://github.com/mrcjkb/vimcats/commit/96a8e262309785b2d50ba643df3d4019929bfb02))
* only document exported module ([7265285](https://github.com/mrcjkb/vimcats/commit/7265285fb709043ac8adb5b2e29c04a74f663db1))
* optional `---[@field](https://github.com/field)` ([#55](https://github.com/mrcjkb/vimcats/issues/55)) ([e78c359](https://github.com/mrcjkb/vimcats/commit/e78c359eaebbfbe2d5344de6ba3f9f60abe1cfb2)), closes [#54](https://github.com/mrcjkb/vimcats/issues/54)
* parse `---[@expr](https://github.com/expr)` tag ([cd8c66a](https://github.com/mrcjkb/vimcats/commit/cd8c66ab8be1600f3fae0d318fafe3315b2c7735))
* parse `---[@usage](https://github.com/usage)` ([fad57e0](https://github.com/mrcjkb/vimcats/commit/fad57e08f5890989304f6e6fa56b10be39cdc698))
* parse local functions and assignments ([e1894d5](https://github.com/mrcjkb/vimcats/commit/e1894d5fe312ad7457f17425495eecd1246fb70c))
* parse simple comments ([6f49a41](https://github.com/mrcjkb/vimcats/commit/6f49a41ac9bb8d7c7e13e8f203e19974b0a0738d))
* rewrite ([18ac08a](https://github.com/mrcjkb/vimcats/commit/18ac08a897297afccf0f4ddf5213d670abcfeb0f))
* skip over extra nodes ([698f7f5](https://github.com/mrcjkb/vimcats/commit/698f7f55bad76e654984a945a74b7223a71078d6))
* static binary on windows ([1a28a7b](https://github.com/mrcjkb/vimcats/commit/1a28a7b7632024f925d15ab0f48703fa8b71a041))
* support `---[@class](https://github.com/class) &lt;name&gt;[: <parent>]` ([#68](https://github.com/mrcjkb/vimcats/issues/68)) ([3a180f8](https://github.com/mrcjkb/vimcats/commit/3a180f827d4e4159060c87cc64ecd36593133431))
* support `---[@see](https://github.com/see)` in func and class ([c368735](https://github.com/mrcjkb/vimcats/commit/c3687359de3c6b5f2fe146efc969978908b0cb71))
* support `---@{protected,package,public}` ([#71](https://github.com/mrcjkb/vimcats/issues/71)) ([bcb0a38](https://github.com/mrcjkb/vimcats/commit/bcb0a38f2e46b26fc440a80ff355f591f2a03dc9))
* support `table.field = function` style functions ([b6d4693](https://github.com/mrcjkb/vimcats/commit/b6d4693733715fd97cc9e644ebbefdcfe3a05c96)), closes [#2](https://github.com/mrcjkb/vimcats/issues/2)
* support deep access ([#67](https://github.com/mrcjkb/vimcats/issues/67)) ([8894074](https://github.com/mrcjkb/vimcats/commit/88940742e87c4ce79c7d8cd2f9e7cf584c73713b))
* support identifier enum variant ([#70](https://github.com/mrcjkb/vimcats/issues/70)) ([4353a66](https://github.com/mrcjkb/vimcats/commit/4353a668e277ab77034689c752d438411d1b4cdd))
* support optional `---[@param](https://github.com/param)` ([27dbafa](https://github.com/mrcjkb/vimcats/commit/27dbafad8d9ee682f22ee18582b4ce82a976b451))
* support vararg (`...`) in `---[@param](https://github.com/param)` ([#62](https://github.com/mrcjkb/vimcats/issues/62)) ([3a2f125](https://github.com/mrcjkb/vimcats/commit/3a2f125acd656ebd57a946567154bdb75da8fa5d)), closes [#61](https://github.com/mrcjkb/vimcats/issues/61)
* visitor pattern for vimdoc ([#74](https://github.com/mrcjkb/vimcats/issues/74)) ([5b04225](https://github.com/mrcjkb/vimcats/commit/5b04225dfc6838036f9cabff16c4641af149d645))
* wrapped and formatted ([308ef64](https://github.com/mrcjkb/vimcats/commit/308ef64ee706970b310c3bfba465c99dcce36946))


### Bug Fixes

* `---[@mod](https://github.com/mod)` without description ([c906714](https://github.com/mrcjkb/vimcats/commit/c906714b9acaa791ad0c6486041ef468c11d8578))
* `---[@param](https://github.com/param)` with no description ([bc9eb17](https://github.com/mrcjkb/vimcats/commit/bc9eb17c5a76744319525d49658e62deb00c7788))
* `---[@return](https://github.com/return)` name can be empty ([dc1ca50](https://github.com/mrcjkb/vimcats/commit/dc1ca505da889b41b83a6c1229c3c0d82fdb5b04))
* bring back `[desc]` support in `---[@type](https://github.com/type)` ([#43](https://github.com/mrcjkb/vimcats/issues/43)) ([d65f2a5](https://github.com/mrcjkb/vimcats/commit/d65f2a506192c20ba985d767f1b20eb3a485a0ea))
* **build:** rust installation on publish ([d052cb8](https://github.com/mrcjkb/vimcats/commit/d052cb828a82e39e711b8597d41e036567427c2c))
* **ci:** `cross` installation ([0bad359](https://github.com/mrcjkb/vimcats/commit/0bad3594e88eb4c91eb8cc5928fb042427cc6841))
* don't panic when `---[@class](https://github.com/class)` has no `---[@field](https://github.com/field)` ([97350ce](https://github.com/mrcjkb/vimcats/commit/97350cee216f244db22b3c73b5233958fc884df9)), closes [#5](https://github.com/mrcjkb/vimcats/issues/5)
* properly render long names ([d2bc1e4](https://github.com/mrcjkb/vimcats/commit/d2bc1e4d1f1b1df9f126d4509ca775a9de41f5d8))
* **toc:** tags are not rendered properly ([c42d58c](https://github.com/mrcjkb/vimcats/commit/c42d58c52d8cdc23461843a8baad47902cd85792))
* typo in --help doc ([78b9d10](https://github.com/mrcjkb/vimcats/commit/78b9d1014fb05f87b28b30aecf7eda8a6364e025))
* use of undeclared crate or module `lexopt` ([43fcda3](https://github.com/mrcjkb/vimcats/commit/43fcda3b94dda8ee24c021fd58e73896738a736a))
* **vimdoc:** wrap `---[@return](https://github.com/return)` type in parenthesis instead of curly brackets ([59e9fa8](https://github.com/mrcjkb/vimcats/commit/59e9fa87758f64cb0b504d42d5cd7cc4f4a6b25f))


### Performance Improvements

* reuse LemmyHelp struct while generating help ([7e4c42c](https://github.com/mrcjkb/vimcats/commit/7e4c42c9fac9e59b42bbb0c33b4cc2cce34fe3cf))


### Miscellaneous Chores

* better emmylua grammar support ([#37](https://github.com/mrcjkb/vimcats/issues/37)) ([c430652](https://github.com/mrcjkb/vimcats/commit/c430652ff788eb305cf4e51d631616db5c72cb3d))
* proper types ([#11](https://github.com/mrcjkb/vimcats/issues/11)) ([0b50ce8](https://github.com/mrcjkb/vimcats/commit/0b50ce8e835c7e09b0ca3e23f8d4474a04fc701e))
* replace `clap` with `lexopt` ([402efa3](https://github.com/mrcjkb/vimcats/commit/402efa3464ebde9089686f5ffa8f31f1ecd2290e))
* replace `tabled` with `tabular` + `textwrap` ([9d8ef91](https://github.com/mrcjkb/vimcats/commit/9d8ef91c37a3004be45aaf0aecceeff163663dd3))
* separate vimdoc from parser ([#42](https://github.com/mrcjkb/vimcats/issues/42)) ([f98322f](https://github.com/mrcjkb/vimcats/commit/f98322fa7920482c82872d443cea63448a47f0c8)), closes [#41](https://github.com/mrcjkb/vimcats/issues/41)
* tokenize file in one step ([#14](https://github.com/mrcjkb/vimcats/issues/14)) ([dd3abfe](https://github.com/mrcjkb/vimcats/commit/dd3abfe86fdcffae71e92f9ebcd442f807a1079c))
