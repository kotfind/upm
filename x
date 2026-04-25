#!/usr/bin/env python

from os import chdir
from typing import Callable
from click import argument, echo, style, secho, group
from threading import Thread
from pathlib import Path
import click
from duct import cmd
from serial.tools.list_ports_common import ListPortInfo
from serial.tools.list_ports import comports
from serial import Serial, SerialException
from time import sleep


def has_bootsel_pico() -> bool:
    out = cmd("picotool", "info").stdout_null().unchecked().run()
    return out.status == 0


def try_open_serial(
    prompt: str, filter: Callable[[ListPortInfo], bool], num: int, baud: int = 115200
) -> tuple[str, Serial] | None:
    ports = comports()
    ports = [port for port in ports if filter(port)]

    if num < len(ports):
        ports.sort(key=lambda p: p.device)
        port = ports[num].device
        try:
            return (port, Serial(port, baud))
        except SerialException as e:
            echo(
                prompt + style(f" Failed to open port {port}: {e}", bold=True, fg="red")
            )

    return None


def read_serial(prompt: str, port: str, serial: Serial) -> None:
    echo(prompt + style(f" Reading {port}...", bold=True))

    try:
        while True:
            line = serial.read_until().decode("utf-8")

            echo(prompt + f" {line}", nl=False)
    except SerialException:
        echo(prompt + style(" Serial closed...\n", bold=True))


def loop_serial(
    prompt: str, filter: Callable[[ListPortInfo], bool], num: int, baud: int = 115200
):
    while True:
        res = try_open_serial(prompt, filter, num, baud)
        if res is None:
            sleep(0.1)
            continue
        (port, serial) = res

        read_serial(prompt, port, serial)


def autoload():
    while True:
        if not has_bootsel_pico():
            sleep(0.1)
            continue

        secho(f"{"-" * 50}\n", bold=True, fg="blue")

        out = cmd("cargo", "run", "--release").dir("./upm-device").unchecked().run()

        secho(f"\n{"-" * 50}\n", bold=True, fg="blue")

        if out.status != 0:
            secho("Loading failed!", bold=True, fg="red")
            secho("Press enter to continue...", bold=True, fg="red")
            input()


def run_device():
    Thread(target=autoload).start()

    Thread(
        target=loop_serial,
        args=[
            style("LOG:", fg="yellow", bold=True),
            lambda p: p.vid == 0xC0DE and p.pid == 0xCAFE,
            0,
        ],
    ).start()

    Thread(
        target=loop_serial,
        args=[
            style("PNC:", fg="red", bold=True),
            lambda p: p.vid == 0x1A86 and p.pid == 0x7523,
            0,
        ],
    ).start()


def run_cli(args: list[str]):
    secho("Running cli...", bold=True)

    out = cmd("cargo", "run", "--", *args).dir("./upm-cli").unchecked().run()
    exit(out.status)


# -------------------- Cli --------------------


@group(context_settings=dict(help_option_names=["-h", "--help"]))
def x():
    pass


@x.command("device")
def x_device():
    run_device()


@x.command("cli")
@argument("args", nargs=-1, type=click.UNPROCESSED)
def x_cli(args):
    args = [str(arg) for arg in args]
    run_cli(args)


if __name__ == "__main__":
    chdir(Path(__file__).parent)
    x()
