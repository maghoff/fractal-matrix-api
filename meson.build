project(
  'fractal', 'rust',
  version: '0.1.0',
  license: 'GPLv3',
)

fractal_version = meson.project_version()
version_array = fractal_version.split('.')
fractal_major_version = version_array[0].to_int()
fractal_minor_version = version_array[1].to_int()
fractal_version_micro = version_array[2].to_int()

fractal_prefix = get_option('prefix')
fractal_datadir = join_paths(fractal_prefix, get_option('datadir'), 'fractal')
fractal_bindir = join_paths(fractal_prefix, get_option('bindir'))

install_data('res/app.css', install_dir : fractal_datadir)
install_data('res/fractal.svg', install_dir : fractal_datadir)
install_data('res/main_window.glade', install_dir : fractal_datadir)

install_data('res/fractal.svg', install_dir : get_option('datadir') + '/icons')
install_data('res/org.gnome.Fractal.desktop', install_dir : get_option('datadir') + '/applications')

cargo = find_program('cargo')
cargo_release = custom_target('cargo-build',
                        output: ['fractal'],
                        install: false,
                        command: ['FRACTAL_RES=' + fractal_datadir, cargo, 'build', '--release'])
run_target('cargo', command: 'scripts/cargo.sh', depends: [cargo_release])
install_data('target/release/fractal', install_dir : fractal_bindir)
