
<a name="0.2.0"></a>
## [0.2.0](https://github.com/bdarcus/csln/compare/0.1.0...0.2.0) (2023-08-01)

### Added

* basic conditional ([1ca55bb](https://github.com/bdarcus/csln/commit/1ca55bb))
* **bib:** identifiers ([664808c](https://github.com/bdarcus/csln/commit/664808c))
* **bib:** contributor and, et al ([452123c](https://github.com/bdarcus/csln/commit/452123c))
* **bib:** date methods ([42846fa](https://github.com/bdarcus/csln/commit/42846fa))
* **bib:** structured and multilingual titles ([eec9f89](https://github.com/bdarcus/csln/commit/eec9f89))
* **citation:** the model ([2274c3d](https://github.com/bdarcus/csln/commit/2274c3d))
* **cli:** clapify ([7a1bf74](https://github.com/bdarcus/csln/commit/7a1bf74))
* **proc:** add refs_to_string placeholder ([5a6c114](https://github.com/bdarcus/csln/commit/5a6c114))
* **proc:** titles renderin ([826a72a](https://github.com/bdarcus/csln/commit/826a72a))
* **proc:** numbers ([0361c5c](https://github.com/bdarcus/csln/commit/0361c5c))
* **proc:** publisher ([4b46098](https://github.com/bdarcus/csln/commit/4b46098))
* **proc:** template rendering ([c82aa8b](https://github.com/bdarcus/csln/commit/c82aa8b))
* **proc:** verb and standard role forms ([c98d368](https://github.com/bdarcus/csln/commit/c98d368))
* **proc:** author substitution ([a6bf2b8](https://github.com/bdarcus/csln/commit/a6bf2b8))
* **proc:** get_cited_references, etc ([0804344](https://github.com/bdarcus/csln/commit/0804344))
* **proc:** contributor roles ([57e87e9](https://github.com/bdarcus/csln/commit/57e87e9))
* **style:** Titles options ([4c951b3](https://github.com/bdarcus/csln/commit/4c951b3))
* **style:** simple string variables ([0ca7200](https://github.com/bdarcus/csln/commit/0ca7200))
* **style:** locale model, example ([c0d5c74](https://github.com/bdarcus/csln/commit/c0d5c74))

### Fixed

* **bib:** editor, reference component ([ea65bd9](https://github.com/bdarcus/csln/commit/ea65bd9))
* **bib:** import warning ([af5b71c](https://github.com/bdarcus/csln/commit/af5b71c))
* **proc:** clippy warnings ([1247955](https://github.com/bdarcus/csln/commit/1247955))
* **proc:** check config before adding year suffix ([530e1d2](https://github.com/bdarcus/csln/commit/530e1d2))
* **proc:** correct year suffix ([2c8f780](https://github.com/bdarcus/csln/commit/2c8f780))
* **proc:** sorting ([318aac9](https://github.com/bdarcus/csln/commit/318aac9))
* **style:** add quote, make fields public ([9d4c7bc](https://github.com/bdarcus/csln/commit/9d4c7bc))
* **style:** remove sort, group from top ([d85e1e0](https://github.com/bdarcus/csln/commit/d85e1e0))

### Changed

* add csln-types crate ([ef35de2](https://github.com/bdarcus/csln/commit/ef35de2))
* add csln-types crate ([8a2afde](https://github.com/bdarcus/csln/commit/8a2afde))
*  option definitions ([f0cff31](https://github.com/bdarcus/csln/commit/f0cff31))
* comment out types ([cddf018](https://github.com/bdarcus/csln/commit/cddf018))
* move logic to InputReference, etc. ([a19dc30](https://github.com/bdarcus/csln/commit/a19dc30))
* types -> core ([b3ed80b](https://github.com/bdarcus/csln/commit/b3ed80b))
* **bib:** enrich contributor model ([5002757](https://github.com/bdarcus/csln/commit/5002757))
* **bib:** SimpleName, string -> struct ([6e02648](https://github.com/bdarcus/csln/commit/6e02648))
* **bib:** allow string subtitle ([9ec91f6](https://github.com/bdarcus/csln/commit/9ec91f6))
* **citation:** clean up, etc ([686646f](https://github.com/bdarcus/csln/commit/686646f))
* **proc:** consolidate Render traits ([01d7739](https://github.com/bdarcus/csln/commit/01d7739))
* **proc:** substitution, suppression ([90ba768](https://github.com/bdarcus/csln/commit/90ba768))
* **proc:** ProcTemplate from type to struct ([cb26c1c](https://github.com/bdarcus/csln/commit/cb26c1c))
* **proc:** remove string_for_key ([316c866](https://github.com/bdarcus/csln/commit/316c866))
* **proc:** add process_template method ([6e3992c](https://github.com/bdarcus/csln/commit/6e3992c))
* **style:** StyleTemplate* -> Template* ([488f755](https://github.com/bdarcus/csln/commit/488f755))
* **style:** disamb -> processing ([bed20c1](https://github.com/bdarcus/csln/commit/bed20c1))
* **style:** option adjustments, docs ([423a703](https://github.com/bdarcus/csln/commit/423a703))
* **style:** make contrib config optional ([f66c50e](https://github.com/bdarcus/csln/commit/f66c50e))
* **style:** title -> primary ([27cf738](https://github.com/bdarcus/csln/commit/27cf738))
* **style:** remove template conditional ([e9f6c75](https://github.com/bdarcus/csln/commit/e9f6c75))
* **types:** remove ([a22dae8](https://github.com/bdarcus/csln/commit/a22dae8))


<a name="0.1.0"></a>
## 0.1.0 (2023-06-06)

### Added

* **citation:** add the model ([7e586e3](https://github.com/bdarcus/csln/commit/7e586e3))
* **cli:** use render_references ([9368dc2](https://github.com/bdarcus/csln/commit/9368dc2))
* **proc:** options, dates ([4a2a813](https://github.com/bdarcus/csln/commit/4a2a813))
* **proc:** set disabm_condition ([438e484](https://github.com/bdarcus/csln/commit/438e484))
* **proc:** add start of disambiguation ([3b36cf5](https://github.com/bdarcus/csln/commit/3b36cf5))
* **proc:** render_references, render_renderence ([2d4f3f7](https://github.com/bdarcus/csln/commit/2d4f3f7))
* **proc:** grouping, etc. ([e9d8740](https://github.com/bdarcus/csln/commit/e9d8740))

### Fixed

* **proc:** suffix is a letter ([1650d36](https://github.com/bdarcus/csln/commit/1650d36))
* **proc:** missing id field ([eb068e4](https://github.com/bdarcus/csln/commit/eb068e4))
* **proc:** render_references return type ([ae4f13c](https://github.com/bdarcus/csln/commit/ae4f13c))
* **proc:** clippy warning ([31b855f](https://github.com/bdarcus/csln/commit/31b855f))
* **proc:** start at 1 for group index ([172e2f7](https://github.com/bdarcus/csln/commit/172e2f7))
* **proc:** sorting ([d840a3f](https://github.com/bdarcus/csln/commit/d840a3f))
* **test:** update ([bc87a59](https://github.com/bdarcus/csln/commit/bc87a59))

### Changed

* **bib:** use edtf for date parsing ([f73cb7c](https://github.com/bdarcus/csln/commit/f73cb7c))
* **proc:** move file loading to style, bib ([197fbee](https://github.com/bdarcus/csln/commit/197fbee))
* **proc:** more -> iter/map ([d0d5308](https://github.com/bdarcus/csln/commit/d0d5308))
* **proc:** switch to map, group_by ([228918c](https://github.com/bdarcus/csln/commit/228918c))
* **proc:** impl render traits ([7ad2c3a](https://github.com/bdarcus/csln/commit/7ad2c3a))
* **proc:** ProcTemplate/Component, docstrings ([b6d5504](https://github.com/bdarcus/csln/commit/b6d5504))
* **proc:** remove ProcReference ([04d37e7](https://github.com/bdarcus/csln/commit/04d37e7))
* **proc:** split proc hints ([12c60e5](https://github.com/bdarcus/csln/commit/12c60e5))

