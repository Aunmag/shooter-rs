import os
from PIL import Image, ImageDraw, ExifTags


TEST = False
COLOR_STEP = 16


def main():
    for image_path in iter_images():
        if TEST and not image_path.endswith("my_test.png"):
            continue

        image = Image.open(image_path)

        if image.mode not in ["RGBA", "P"]:
            print("Image {} has unsupported mode ({}). Skipping".format(image_path, image.mode))

        size_h = image.size[0]
        size_v = image.size[1]

        image_converted = image.convert("RGBA")
        padding_l = find_first_non_transparent_pixel(image_converted, 0, 1)
        padding_t = find_first_non_transparent_pixel(image_converted, 1, 1)
        padding_r = size_h - 1 - find_first_non_transparent_pixel(image_converted, 0, -1)
        padding_b = size_v - 1 - find_first_non_transparent_pixel(image_converted, 1, -1)

        padding_h_min = min(padding_l, padding_r)
        padding_v_min = min(padding_t, padding_b)

        image = image.crop((
            padding_h_min,
            padding_v_min,
            size_h - padding_h_min,
            size_v - padding_v_min,
        ))

        if image.size[0] == size_h and image.size[1] == size_v:
            # print("Image {} hasn't changed. Skipping".format(image_path))
            continue

        compress_colors(image, image_path)

        if TEST:
            # image.show()
            image.save("../assets/my_test_crop.png")
            return
        else:
            image.save(image_path)


def iter_images():
    for root, directory, files in os.walk("../assets"):
        for file in files:
            if file.endswith(".png"):
                # TODO: return as path
                # TODO: do join
                yield "{}/{}".format(root, file)


def find_first_non_transparent_pixel(image, axis, direction):
    assert axis in [0, 1]
    assert direction in [-1, 1]

    r = range(image.size[axis])

    if direction < 0:
        r = reversed(r)

    for a in r:
        for b in range(image.size[(axis + 1) % 2]):
            if axis == 0:
                i = (a, b)
            else:
                i = (b, a)

            if image.getpixel(i)[3] >= COLOR_STEP:
                return a

    return a


def compress_colors(image, image_path):
    if image.mode != "RGBA":
        return

    for x in range(image.size[0]):
        for y in range(image.size[1]):
            i = (x, y)
            (r, g, b, a) = image.getpixel(i)
            r = compress_color_value(r)
            g = compress_color_value(g)
            b = compress_color_value(b)
            a = compress_color_value(a)  # TODO: check
            image.putpixel(i, (r, g, b, a))


# TODO: test
def compress_color_value(n):
    return min(round(n / COLOR_STEP) * COLOR_STEP, 255)


def test_compress_color_value():
    assert compress_color_value(0) == 0
    assert compress_color_value(8) == 0
    assert compress_color_value(9) == 16
    assert compress_color_value(250) == 255
    assert compress_color_value(255) == 255


if __name__ == "__main__":
    # test_compress_color_value()
    main()
