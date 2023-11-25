import turtle


class Pawn:
    def __init__(self, x, y, case):
        self.x = x
        self.y = y
        self.taille = 75 / 100 * case
        self.case = case

    def draw(self):
        turtle.penup()
        turtle.goto(self.x, self.y)
        turtle.fillcolor("black")
        turtle.begin_fill()
        turtle.forward(self.case * self.y - self.case / 2)
        turtle.right(90)
        turtle.forward(self.case * self.x - self.case / 2)
        turtle.circle(self.taille)
        turtle.end_fill()
