import math
import sys

PI = 3.141592653589793
SOLAR_MASS = 4 * PI * PI
DAYS_PER_YEAR = 365.24


class Body:
    def __init__(self, x, y, z, vx, vy, vz, mass):
        self.x = x
        self.y = y
        self.z = z
        self.vx = vx
        self.vy = vy
        self.vz = vz
        self.mass = mass


def make_bodies():
    jupiter = Body(4.84143144246472090, -1.16032004402742839, -0.103622044471123109, 0.00166007664274403694 * DAYS_PER_YEAR, 0.00769901118419740425 * DAYS_PER_YEAR, -0.0000690460016972063023 * DAYS_PER_YEAR, 0.000954791938424326609 * SOLAR_MASS)
    saturn = Body(8.34336671824457987, 4.12479856412430479, -0.403523417114321381, -0.00276742510726862411 * DAYS_PER_YEAR, 0.00499852801234917238 * DAYS_PER_YEAR, 0.0000230417297573763929 * DAYS_PER_YEAR, 0.000285885980666130812 * SOLAR_MASS)
    uranus = Body(12.8943695621391310, -15.1111514016986312, -0.223307578892655734, 0.00296460137564761618 * DAYS_PER_YEAR, 0.00237847173959480950 * DAYS_PER_YEAR, -0.0000296589568540237556 * DAYS_PER_YEAR, 0.0000436624404335156298 * SOLAR_MASS)
    neptune = Body(15.3796971148509165, -25.9193146099879641, 0.179258772950371181, 0.00268067772490389322 * DAYS_PER_YEAR, 0.00162824170038242295 * DAYS_PER_YEAR, -0.0000951592254519715870 * DAYS_PER_YEAR, 0.0000515138902046611451 * SOLAR_MASS)
    return [Body(0, 0, 0, 0, 0, 0, SOLAR_MASS), jupiter, saturn, uranus, neptune]


def offset_momentum(bodies):
    px = 0
    py = 0
    pz = 0
    for b in bodies:
        px = px + b.vx * b.mass
        py = py + b.vy * b.mass
        pz = pz + b.vz * b.mass
    sun = bodies[0]
    sun.vx = -px / SOLAR_MASS
    sun.vy = -py / SOLAR_MASS
    sun.vz = -pz / SOLAR_MASS


def advance(bodies, dt):
    count = len(bodies)
    i = 0
    while i < count:
        bi = bodies[i]
        j = i + 1
        while j < count:
            bj = bodies[j]
            dx = bi.x - bj.x
            dy = bi.y - bj.y
            dz = bi.z - bj.z
            d2 = dx * dx + dy * dy + dz * dz
            mag = dt / (d2 * math.sqrt(d2))
            bi.vx = bi.vx - dx * bj.mass * mag
            bi.vy = bi.vy - dy * bj.mass * mag
            bi.vz = bi.vz - dz * bj.mass * mag
            bj.vx = bj.vx + dx * bi.mass * mag
            bj.vy = bj.vy + dy * bi.mass * mag
            bj.vz = bj.vz + dz * bi.mass * mag
            j = j + 1
        i = i + 1
    i = 0
    while i < count:
        b = bodies[i]
        b.x = b.x + dt * b.vx
        b.y = b.y + dt * b.vy
        b.z = b.z + dt * b.vz
        i = i + 1


def energy(bodies):
    e = 0
    count = len(bodies)
    i = 0
    while i < count:
        bi = bodies[i]
        e = e + 0.5 * bi.mass * (bi.vx * bi.vx + bi.vy * bi.vy + bi.vz * bi.vz)
        j = i + 1
        while j < count:
            bj = bodies[j]
            dx = bi.x - bj.x
            dy = bi.y - bj.y
            dz = bi.z - bj.z
            distance = math.sqrt(dx * dx + dy * dy + dz * dz)
            e = e - (bi.mass * bj.mass) / distance
            j = j + 1
        i = i + 1
    return e


n = int(sys.argv[1]) if len(sys.argv) > 1 else 200

bodies = make_bodies()
offset_momentum(bodies)

step = 0
while step < n:
    advance(bodies, 0.01)
    step = step + 1

print(math.floor(energy(bodies) * 1000000000))
