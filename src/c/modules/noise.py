import math
import random
from PIL import Image

def vec2_length2(v):
    return v[0] * v[0] + v[1] * v[1]

def vec2_distance(a, b):
    dx = a[0] - b[0]
    dy = a[1] - b[1]
    return math.sqrt(dx * dx + dy * dy)

def energy(w, h, buffer, x, y):
    total_energy = 0.0
    for dy in range(-4, 5):
        for dx in range(-4, 5):
            if dx == 0 and dy == 0:
                continue

            x2 = (x + dx) % w
            y2 = (y + dy) % h

            dist = vec2_distance(buffer[y][x], buffer[y2][x2])

            length = vec2_length2([dx, dy])

            total_energy += math.exp(-length / (2.1 * 2.1) - dist)

    return total_energy

def minimize_energy(w, h, buffer, iterations=1000):
    for _ in range(iterations):
        x0 = random.randint(0, w - 1)
        y0 = random.randint(0, h - 1)
        x1 = random.randint(0, w - 1)
        y1 = random.randint(0, h - 1)

        if x0 == x1 and y0 == y1:
            continue

        start_energy = energy(w, h, buffer, x0, y0) + energy(w, h, buffer, x1, y1)

        buffer[y0][x0], buffer[y1][x1] = buffer[y1][x1], buffer[y0][x0]

        end_energy = energy(w, h, buffer, x0, y0) + energy(w, h, buffer, x1, y1)

        if start_energy < end_energy:
            buffer[y0][x0], buffer[y1][x1] = buffer[y1][x1], buffer[y0][x0]

def generate_blue_noise_array(w, h, iterations):
    buffer = [[[random.random(), random.random()] for _ in range(w)] for _ in range(h)]

    minimize_energy(w, h, buffer, iterations)
    return buffer

def format_as_c_array(w, h, buffer, array_name="blue_noise"):
    lines = [
        "#include <stdint.h>",
        f"#define BLUE_NOISE_W {w}",
        f"#define BLUE_NOISE_H {h}",
        f"static const uint8_t {array_name}[{w * h // 8}] = {{"
    ]

    for y in range(h):
        for x in range(w // 8):
            bit = []
            for x_bit in range(8):
                vx, vy = noise_data[y][x * 8 + x_bit]
                b = (vx + vy) > 1.0
                bit.append(b > 0.5)
            lines.append(" 0b{},".format("".join("1" if z else "0" for z in bit )))

    lines.append("};")
    return "\n".join(lines)

if __name__ == "__main__":
    WIDTH = 64
    HEIGHT = 64
    ITERATIONS = 100000

    noise_data = generate_blue_noise_array(WIDTH, HEIGHT, ITERATIONS)
    c_code = format_as_c_array(WIDTH, HEIGHT, noise_data)

    img = Image.new('RGB', (WIDTH, HEIGHT), "black")
    pixels = img.load()

    for y in range(HEIGHT):
        for x in range(WIDTH // 8):
            bit = []
            for x_bit in range(8):
                vx, vy = noise_data[y][x * 8 + x_bit]
                b = (vx + vy) > 1.0
                pixels[y, (x * 8 + x_bit)] = (255 * b,)*3

    img.show()

    # You can print it to standard output or redirect it into a file
    print(c_code)
