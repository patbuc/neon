import sys


class Random:
    def __init__(self):
        self.seed = 74755

    def next(self):
        self.seed = ((self.seed * 1309) + 13849) & 65535
        return self.seed


class Ball:
    def __init__(self, x, y, x_vel, y_vel):
        self.x = x
        self.y = y
        self.x_vel = x_vel
        self.y_vel = y_vel

    def bounce(self):
        x_limit = 500
        y_limit = 500
        bounced = False

        self.x += self.x_vel
        self.y += self.y_vel

        if self.x > x_limit:
            self.x = x_limit
            self.x_vel = -abs(self.x_vel)
            bounced = True

        if self.x < 0:
            self.x = 0
            self.x_vel = abs(self.x_vel)
            bounced = True

        if self.y > y_limit:
            self.y = y_limit
            self.y_vel = -abs(self.y_vel)
            bounced = True

        if self.y < 0:
            self.y = 0
            self.y_vel = abs(self.y_vel)
            bounced = True

        return bounced


def new_ball(random):
    x = random.next() % 500
    y = random.next() % 500
    x_vel = random.next() % 300 - 150
    y_vel = random.next() % 300 - 150
    return Ball(x, y, x_vel, y_vel)


def benchmark():
    random = Random()
    ball_count = 100
    bounces = 0
    balls = [None] * ball_count

    for i in range(ball_count):
        balls[i] = new_ball(random)

    for _ in range(50):
        for ball in balls:
            if ball.bounce():
                bounces += 1

    return bounces


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

checksum = 0
for _ in range(iterations):
    result = benchmark()
    if result != 1331:
        raise Exception("Bounce verification failed")
    checksum += result
print(checksum)
