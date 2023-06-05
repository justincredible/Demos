# Shapes

The Platonic Solids and necessary polygons.

The derivation of vertex positions requires only elementary trigonometry and algebra.
All angles used have [simple exact solutions](https://en.wikipedia.org/wiki/Exact_trigonometric_values).

## Polygons

These are regular polygons with unit length.

### Triangle

An equilateral triangle has $60^\circ$ vertex angles, so the height is $h = { \sqrt 3 \over 2 }$

<img src="https://github.com/justincredible/Playground/assets/3183562/53c5a295-349c-4f4a-9e8f-61946b857a40" height="200" width="200" >

### Square

A square has $90^\circ$ vertex angles, and the distance between non-adjacent vertices is $\sqrt 2$

<img src="https://github.com/justincredible/Playground/assets/3183562/acc7e7ef-74dc-49bb-b149-c8d5acd0fc96" height="200" width="200" >

### Pentagon

A regular pentagon has $108^\circ$ vertex angles, leading to several facts:
- the distance between non-adjacent vertices is $\phi = { 1 + \sqrt 5 \over 2 }$
- the angles adjacent to lengths top and span are $54^\circ$ and $36^\circ$ respectively
- the angle adjacent to length base is $18^\circ$
- the height of this polygon is $h = \cos { \pi \over 10 } + \cos { 3\pi \over 10 }$

<img src="https://github.com/justincredible/Playground/assets/3183562/96e6389a-37de-4180-bde6-b9fd77ca1206" height="200" width="200" >

### Circumscribing circle

The circle's center, relative to the bottom edge of the polygon,
and the radius can be determined by positioning the bottom of the face on the x-axis
as well as symmetrically with respect to the y-axis.
With this positioning we know three vertices of the polygon,
$(-{ 1 \over 2 }, 0)$ and $({ 1 \over 2 }, 0)$, $({ 1 \over 2 }, 1)$ for the square, and $(0, h)$ for the other two.

Solving for $(x - c_x)^2 + (y - c_y)^2 = r^2$ provides us with
center $c = (0, { h \over 2 } - { 1 \over 8h })$ and
radius $r = { h \over 2 } + { 1 \over 8h }$

The vertices are centred by subtracting $c$

<img src="https://github.com/justincredible/Playground/assets/3183562/4c8909b1-5b86-4328-adff-10eec300eaf0" height="200" width="200" >

Also notice how a reflected face can be symmetrically positioned in the circle.

## Platonic Solids

The derivations are based on an assumption that the polyhedra can be circumscribed by spheres.
The intuition behind this claim is that the uniform regularity of these polyhedra requires containment within a sphere.
I do not have a proof for this premise, but let the program itself serve as evidence.

### Tetrahedron

The relative positions of three vertices are known from the triangle,
and we assume the last vertex is orthogonal to the circumscribing circle's center.
Since the edges have unit length, the shortest distance from the last vertex to the center, $z$,
satisfies $(h - c_y)^2 + z^2 = 1 \implies z = \sqrt { 1 - r^2 }$

The polyhedron is centred by solving for $a, b$ in $a^2 = b^2 + r^2$ such that $a - b = z$

### Hexahedron

Better known as the cube, the vertices are immediately known for specific orientations.

### Octahedron

The bottom edges of the triangles form a square perimeter.
Two unit length edges connect non-adjacent vertices on this square over a distance of $\sqrt 2$,
so the angle between those two edges is $90^\circ$
and the two remaining vertices have a distance of $1 \over \sqrt 2$ from the square's center.

### Dodecahedron

The first five relative positions are known from the pentagon,
and from the diagram below we see a conceptual pentagon with side length $\phi$

<img src="https://github.com/justincredible/Playground/assets/3183562/7f563d60-20c0-4136-931d-6424ffd2005a" height="200" width="200" >

Positioning this larger pentagon similarly shows the next five vertices have x- and y-coordinates $\phi$ times larger than the first five.
The distance between the first two sets of vertices, $z_{outer}$,
must satisfy ${\Delta x}^2 + {\Delta y}^2 + z_{outer}^2 = 1$ for any neighbouring pair.
Using the top vertices we see that $z_{outer} = \sqrt { 1 - (2 - \phi)r^2 }$,
and from the exact values this simplifies to $z_{outer} = r$

The next five vertices are the previous five reflected about the x-axis.
The distance between these two sets, $z_{inner}$,
must also satisfy ${\Delta x}^2 + {\Delta y}^2 + z_{inner}^2 = 1$ for neighbouring vertices.
Using the middle vertices on one side we see $z_{inner} = \sqrt { 1 - 4(\phi + 1)(base - c)^2 }  = \sqrt { 1 - 4(\phi + 1)(r - top)^2 }$

The final five vertices are the first five reflected about the x-axis,
and the distance between this and the previous set is again $z_{outer} = r$

### Icosahedron

The bottom edges of the triangles form a regular pentagon perimeter.
From this we have our first ten vertices (the pentagon and its x-axis reflection).
The distance betwen these vertices is $z_{inner} = \sqrt { 1 - 4(base - c)^2 }  = \sqrt { 1 - 4(r - top)^2 }$

The final two vertices are assumed to be orthogonal to the circumscribing circle's center.
The distance from the center is $z_{outer} = \sqrt { 1 - r^2 }$
