import sys


def mandelbrot(size):
    _sum = 0
    byte_acc = 0
    bit_num = 0

    y = 0
    while y < size:
        ci = (2.0 * y / size) - 1.0
        x = 0

        while x < size:
            zrzr = 0.0
            zi = 0.0
            zizi = 0.0
            cr = (2.0 * x / size) - 1.5

            z = 0
            not_done = True
            escape = 0
            while not_done and z < 50:
                zr = zrzr - zizi + cr
                zi = 2.0 * zr * zi + ci

                zrzr = zr * zr
                zizi = zi * zi

                if zrzr + zizi > 4.0:
                    not_done = False
                    escape = 1
                z += 1

            byte_acc = (byte_acc << 1) + escape
            bit_num += 1

            if bit_num == 8:
                _sum ^= byte_acc
                byte_acc = 0
                bit_num = 0
            elif x == size - 1:
                byte_acc <<= 8 - bit_num
                _sum ^= byte_acc
                byte_acc = 0
                bit_num = 0
            x += 1
        y += 1

    return _sum


size = int(sys.argv[1]) if len(sys.argv) > 1 else 1

result = mandelbrot(size)
if size == 1 and result != 128:
    raise Exception("Mandelbrot verification failed")
if size == 500 and result != 191:
    raise Exception("Mandelbrot verification failed")
if size == 750 and result != 50:
    raise Exception("Mandelbrot verification failed")
print(result)
