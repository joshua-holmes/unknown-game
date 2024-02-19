import re, sys


def print_help():
    example = "5fe3aa"
    r, g, b = hex_to_floats(example)
    print(
        f"""
    -- HELP --

    Run this script with a 6-digit hex as the first argument, then this script
    will print the `vec3` constructure with the correct normalized float values
    as it's arguments. For example this:
        python3 {sys.argv[0]} {example}
    will produce this result:
        vec3({r}, {g}, {b});

    This script is a shortcut to quickly generate color values in the fragment
    shader for different materials.

    Run this script with --help for this help message.
    """
    )


def parse_args(args: list[str]):
    if len(args) < 2:
        print_help()
        raise Exception("This script requires 1 argument, a 6-digit hexidecimal value, and 0 were given.")
    input_arg = args[1]
    if input_arg == "--help":
        print_help()
        exit(0)
    re_match = re.search("[^a-f0-9]", input_arg)
    if re_match:
        print_help()
        raise Exception("Found non-hex character:", input_arg[re_match.start()])
    if len(input_arg) != 6:
        print_help()
        raise Exception(f"Length of input should be exactly 6 hexidecimal values. {len(input_arg)} were provided")
    return input_arg


def hex_to_floats(hex_digits: str) -> tuple[float, float, float]:
    r_hex = hex_digits[:2]
    g_hex = hex_digits[2:4]
    b_hex = hex_digits[4:]
    r = int(r_hex, 16) / 255
    g = int(g_hex, 16) / 255
    b = int(b_hex, 16) / 255
    return (r, g, b)


def main():
    hex_value = parse_args(sys.argv)
    r, g, b = hex_to_floats(hex_value)
    print(f"vec({r}, {g}, {b});")


if __name__ == "__main__":
    main()
