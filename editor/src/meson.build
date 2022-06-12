pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')


blueprints = custom_target('blueprints',
  input: files(
    join_paths('components', 'editor', 'editor.blp'),
    join_paths('components', 'window', 'window.blp'),
    join_paths('components', 'window', 'menu.blp'),
    join_paths('components', 'workspace', 'workspace.blp')
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

gnome.compile_resources('echidna',
  'echidna.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,

  dependencies: blueprints,

)


rust_sources = [
  'main.rs',
  'config.rs.in',
  'components/editor/imp.rs',
  'components/editor/mod.rs',
  'components/sidebar/imp.rs',
  'components/sidebar/mod.rs',
  'components/window/file.rs',
   'components/window/imp.rs',
   'components/window/mod.rs',
   'components/window/menubar.rs',
   'components/window/sidebar.rs',
]

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'echidna')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'editor', 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'editor', 'src', 'config.rs'),
  check: true
)


cargo_script = find_program(join_paths(meson.project_source_root(), 'editor', 'build-aux', 'cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: rust_sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_build_root(),
    meson.project_source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)
