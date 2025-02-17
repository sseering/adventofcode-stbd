import png


def convert(c: chr) -> int:
    return (ord(c) - ord('a'))


def color(v) -> tuple[int, int, int]:
    if v >= 0:
        c = min(v * 255//ord('z'), 255)
        return (c, c, c)
    if v == -14:
        return (255, 0, 0)
    if v == -28:
        return (0, 255, 0)


def main() -> None:
    with open("input.txt", "r") as f:
        data = [[convert(c) for c in line.strip()] for line in f.readlines()]

    height = len(data)
    width = len(data[0])

    print(f"{width=} {height=}")

    img = []
    for y in range(height):
        row = ()
        for x in range(width):
            row = row + color(data[y][x])
        img.append(row)
    with open("map.png", "wb") as f:
        w = png.Writer(width, height, greyscale=False)
        w.write(f, img)


if __name__ == "__main__":
    main()
