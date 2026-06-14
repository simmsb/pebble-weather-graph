#
# This file is the default set of rules to compile a Pebble application.
#
# Feel free to customize this to your needs.
#
import os.path

from waflib import Errors
from waflib.Task import Task
from waflib.TaskGen import feature, after_method

top = '.'
out = 'build'

RUST_TARGET = 'thumbv7m-pebble-eabi/release/libpebble_weather_graph.a'
MESSAGE_KEYS_C = 'src/message_keys.auto.c'
MESSAGE_KEYS_OBJ = 'src/message_keys.auto.o'
MESSAGE_KEYS_LIB = 'src/libmessage_keys.auto.a'


def _build_message_keys_lib(task):
    cc = task.env.CC[0] if task.env.CC else 'arm-none-eabi-gcc'
    ar = task.env.AR[0] if task.env.AR else 'arm-none-eabi-ar'
    src = task.inputs[0].abspath()
    obj = task.outputs[0].abspath()
    lib = task.outputs[1].abspath()
    task.exec_command([
        cc, '-std=c99', '-mcpu=cortex-m3', '-mthumb', '-ffunction-sections',
        '-fdata-sections', '-Os', '-c', src, '-o', obj,
    ])
    task.exec_command([ar, 'rcs', lib, obj])


@feature('pebble_cprogram')
@after_method('apply_link')
def link_rust_staticlib(self):
    try:
        cargo_tg = self.bld.get_tgen_by_name(RUST_TARGET)
    except Errors.WafError:
        return

    rust_lib = self.bld.bldnode.find_node(RUST_TARGET)
    if rust_lib is None:
        return

    for task in cargo_tg.tasks:
        self.link_task.set_run_after(task)
    if rust_lib not in self.link_task.inputs:
        self.link_task.inputs.append(rust_lib)


def options(ctx):
    ctx.load('pebble_sdk')


def configure(ctx):
    """
    This method is used to configure your build. ctx.load(`pebble_sdk`) automatically configures
    a build for each valid platform in `targetPlatforms`. Platform-specific configuration: add your
    change after calling ctx.load('pebble_sdk') and make sure to set the correct environment first.
    Universal configuration: add your change prior to calling ctx.load('pebble_sdk').
    """
    ctx.load('pebble_sdk')
    ctx.load('clang_compilation_database', tooldir="./tools")


def build(ctx):
    ctx.exec_command('npm run build')

    ctx.load('pebble_sdk')

    message_keys_c = ctx.path.get_bld().make_node(MESSAGE_KEYS_C)
    message_keys_obj = ctx.path.get_bld().make_node(MESSAGE_KEYS_OBJ)
    message_keys_lib = ctx.path.get_bld().make_node(MESSAGE_KEYS_LIB)
    ctx(
        rule=_build_message_keys_lib,
        source=message_keys_c,
        target=[message_keys_obj, message_keys_lib],
        env=ctx.all_envs[ctx.env.TARGET_PLATFORMS[0]],
    )

    message_keys_lib_dir = message_keys_lib.parent.abspath()

    rust_sources = (
        ['Cargo.toml', 'Cargo.lock', '.cargo/config.toml', 'rust-toolchain.toml', 'thumbv7m-pebble-eabi.json',
         'gravel/gravel/Cargo.toml', 'gravel/gravel-sys/Cargo.toml']
        + ctx.path.ant_glob('src/**/*.rs')
        + ctx.path.ant_glob('gravel/gravel/src/**/*.rs')
        + ctx.path.ant_glob('gravel/gravel-sys/src/**/*.rs')
    )
    rust_target = RUST_TARGET
    ctx(
        rule='cargo build --release --target ../thumbv7m-pebble-eabi.json -Zjson-target-spec --target-dir .',
        source=rust_sources + [message_keys_lib],
        target=rust_target,
        always=True,
    )

    binaries = []

    cached_env = ctx.env
    for platform in ctx.env.TARGET_PLATFORMS:
        ctx.env = ctx.all_envs[platform]
        ctx.set_group(ctx.env.PLATFORM_NAME)
        app_elf = '{}/pebble-app.elf'.format(ctx.env.BUILD_DIR)
        ctx.env.LINKFLAGS += ['-z', 'noexecstack']
        ctx.env.LINKFLAGS += ['-Wl,--no-warn-rwx-segments']

        app_elf = ctx.pbl_program(
            target=f"{ctx.env.BUILD_DIR}/pebble-app.elf",
        )

        binaries.append({'platform': platform, 'app_elf': app_elf.target})
    ctx.env = cached_env

    ctx.set_group('bundle')
    ctx.pbl_bundle(binaries=binaries,
                   js=ctx.path.ant_glob(['src/ts-build/**/*.js',
                                         'src/pkjs/**/*.json',
                                         'src/common/**/*.js']),
                   js_entry_file='src/ts-build/index.js')

orig_exec_command = Task.exec_command

def custom_exec_command(self, cmd, **kw):
    if isinstance(cmd, list) and len(cmd) > 0 and ('gcc' in cmd[0] or 'cc' in cmd[0]):
        new_cmd = []
        for arg in cmd:
            if arg.startswith('-I') and ('sdk' in arg.lower() or 'pebble' in arg.lower()):
                new_cmd.append('-isystem' + arg[2:])
            else:
                new_cmd.append(arg)
        cmd = new_cmd
    return orig_exec_command(self, cmd, **kw)

Task.exec_command = custom_exec_command
