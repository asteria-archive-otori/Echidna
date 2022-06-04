project('echidna', 'rust', 
          version: '0.1.0',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2',
                   ],
)


dependency('gtk4')
dependency('libadwaita-1')
dependency('gio-2.0', version: '>=2.66')
dependency('glib-2.0', version: '>=2.66')
dependency('gtksourceview-5')

i18n = import('i18n')

gnome = import('gnome')


subdir(join_paths('editor', 'data'))
subdir(join_paths('editor', 'src'))
subdir(join_paths('editor', 'po'))

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
