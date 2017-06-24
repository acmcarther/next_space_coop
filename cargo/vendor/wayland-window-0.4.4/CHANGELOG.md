# Change Log

## 0.4.3 -- 2016-12-24

- Bugfix suface damaging on wl-surfaces of version <= 3 (kudos to @emilio for finding the bug,
  details on https://github.com/vberger/wayland-client-rs/issues/75 )

## 0.4.2 -- 2016-10-08

- Better handling of cursor theming
- Ability to diable decorations & go fullscreen

## 0.4.1 -- 2016-10-08

DecoratedSurface is now Send and require a Send handler.

## 0.4.0 -- 2016-10-03

Update to wayland-client-0.7

## 0.3.0 -- 2016-05-29

Update to wayland-client-0.6

## 0.2.3 -- 2016-04-11

Update dependencies.

## 0.2.2 -- 2015-12-13

### Added

- `set_title` for decorated surfaces
- `set_class` for decorated surfaces
- `add_borders` free helper function

## 0.2.1 -- 2015-12-09

Update to wayland-client 0.5.

## 0.2.0 -- 2015-11-30

Update the lib to new wayland-client API.
