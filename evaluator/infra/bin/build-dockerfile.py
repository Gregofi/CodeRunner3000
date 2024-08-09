#!/usr/bin/env python3

import sys
import tomllib

PATH = "./languages"
LANG_CONFIG = "lang-config.toml"

result = "FROM debian:bookworm\n\n"

def create_subdockerfile(name: str, config: dict) -> str:
    return f"""
########## {name} ##########

FROM {config['image']} AS {name}-builder-cr3000

WORKDIR /tmp/{name}

COPY {PATH}/{name}/ .

RUN ./setup.sh
"""


def build_dockerfile(languages: list[str] | None = None) -> None:
    with open(f"{PATH}/config.toml", "rb") as f:
        config = tomllib.load(f)

    result = ""

    used_languages = []

    for k, v in config.items():
        if languages is not None and k not in languages:
            continue
        result += create_subdockerfile(k, v)
        used_languages.append(k)

    result += "\n\n"
    result += "#################################\n"
    result += "########## Final image ##########\n\n"
    result += "FROM debian:bookworm\n\n"

    for lang in used_languages:
        result += f"COPY --from={lang}-builder-cr3000 /opt/evaluator/compilers/{lang} /opt/evaluator/compilers/{lang}\n"


    print(result)


def main() -> None:
    args = sys.argv[1:]
    if len(args) == 0:
        build_dockerfile()
    elif args[0] == "-h" or args[0] == "--help":
        print(HELP_STRING)
    else:
        build_dockerfile(args)


HELP_STRING = """
Usage: build-dockerfile.py [LANGUAGE] ...

Options:
    -h, --help      Show this help message and exit

Creates a Dockerfile that contains the build instructions for compilers and
interpreters for the specified languages. The Dockerfile is printed to stdout.

For example, to create a Dockerfile for Lua and Rust, run:

    build-dockerfile.py lua rust

The languages specified needs to be in the infra/languages directory, with
existing config matching the name.

If no languages are specified, the Dockerfile for all languages will be created.
"""


if __name__ == "__main__":
    main()
