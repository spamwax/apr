# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.4] - 2021-08-29
### Added
- Add -e flag to search command to find pins with exact tags

## [0.16.3] - 2021-08-28
### Added
- Use --query-as-item global flag to always add an Alfred item based on user's exact entry.
- Alwyas show a tag that exactly matches user's input at the top of list.
### Fixed
- search urls in lowercase

## [0.16.0] - 2021-08-23
Bumped minor version since new fuzzy search engine may produce different search results.

### Added
- Use a new fuzzy search engine.
- Support tag renaming using a keyword.
- Add tag renaming and bookmark deletion to Universal Actions.

### Changed
- Add Urls to default search when tag_only is false
- Don't show 'you have latest version' unless user checks for update.
- Use conditional objects in workflow's canvas.

## [0.15.14] - 2020-07-19
### Added
Improve error messages dusing post/delete/search operations.

## [0.15.13] - 2020-07-19
### Fixed
- Use rusty-pin 0.5.3 to fix #78 (empty tag list on Pinboard)

## [0.15.12] - 2020-04-03
### Fixed
- Use rusty-pin 0.5.1 to fix #46 (empty tag list on Pinboard)

## [0.15.11] - 2020-03-22
### Added
- Add basic support for tag renaming.

## [0.15.10] - 2020-03-12
### Fixed
- Trying to address issue [#47](https://github.com/spamwax/alfred-pinboard-rs/issues/47) (Catalina osascript premissions)
### Added
- Suport Microsoft Edge Browser
### Changed
- Don't use `sed` hack to set username for url search on [pinboard](https://pinboard.in). A `username` environment variable is now passed to Alfred.

## [0.15.8] - 2019-08-29
### Changed
- Holding `Control`/`Option` keys while posting a bookmark will now momentarily toggle `toread`/`shared` settings. ([Closes #38](https://github.com/spamwax/alfred-pinboard-rs/issues/38)) 

## [0.15.7] - 2019-07-14
- Preserve upper/lowercase of titles/urls/description.

## [0.15.6] - 2019-07-11
### Added
- Holding CMD in search results now correctly shows either tags or URL based on users' settings.
### Fixed
- Fix appveyor CI issue with directory names.

## [0.15.4] - 2019-06-17
### Added
- Add option to either show TAGs or URLs in search results.
- Add a combo modifier for search result to copy URL to clipboard.
### Fixed
- Fix multiple issues related to release of Alfred 4
- `pcheck` should now force a network call regardless of when last update check was done.
- Fix: deleting a bookmark was not working.

## [0.14.9] - 2019-02-13
### Added
- Add settings for notifying if page is already bookmarked.

## [0.14.8] - 2019-02-13
### Fixed
- Workaround for Firefox ([Fixes #25](https://github.com/spamwax/alfred-pinboard-rs/issues/25))

## [0.14.7] - 2019-01-30
### Added
- Support [Brave Browser](brave.com)

## [0.14.6] - 2019-01-22
### Added
- Minor improvements

## [0.14.5] - 2019-01-15
### Added
- Show whether current page is already bookmarked.

## [0.14.4] - 2018-11-22
### Fixed
- Fixes issue [#21](https://github.com/spamwax/alfred-pinboard-rs/issues/21)

## [0.14.1 - 0.14.3] - 2018-08-27 - 2018-10-31
### Fixed
- Re-enable auto cache update
- Using `;` to add description was broken
- Recompile binary to fix an upstream bug

### Added
- Add Opera support

## [0.14.0] - 2018-06-04
### Added
- Workflow can notify and auto update itself.

## [0.13.3] - 2018-05-29
### Fixed
- Fixes issue [#7](https://github.com/spamwax/alfred-pinboard-rs/issues/7)
