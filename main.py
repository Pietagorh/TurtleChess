import turtle

tile_size = 32

turtle.reset()
turtle.speed(0)
turtle.hideturtle()
turtle.tracer(0)
turtle.title("chess")
turtle.screensize(tile_size * 8, tile_size * 8, "#000000")


def draw_board():
    turtle.setpos(0, 0)

    for row in range(8):
        for tile in range(8):
            turtle.color("#26611a" if (row + tile) % 2 == 0 else "#9cff81")
            turtle.begin_fill()
            for _ in range(4):
                turtle.forward(tile_size)
                turtle.left(90)
            turtle.forward(tile_size + 1)
            turtle.end_fill()
        turtle.penup()
        turtle.back((tile_size + 1) * 8)
        turtle.right(90)
        turtle.forward(tile_size + 1)
        turtle.left(90)
        turtle.pendown()

    turtle.done()


draw_board()
