# Shapes

Polygons and the Platonic Solids.

The derivation of vertex positions requires only elementary trigonometry and algebra.
All angles for the Platonic Solids have simple [exact trigonometric values](https://en.wikipedia.org/wiki/Exact_trigonometric_values).

## Polygons

Regular polygons with unit length. The faces of the polyhedra.

### Trigon

An equilateral triangle has $60^\circ$ vertex angles, so the height is $h = { \sqrt 3 \over 2 }$

<img src="https://github.com/justincredible/Demos/assets/3183562/d1da94a8-8314-47f0-81cb-6e5ae63ed4a2" height="200" width="200" >

### Tetragon

A square has $90^\circ$ vertex angles, and the distance between non-adjacent vertices is $\sqrt 2$

<img src="https://github.com/justincredible/Demos/assets/3183562/375048da-4714-43d7-9a2f-d6b0cd5bacbe" height="200" width="200" >

### Pentagon

A regular pentagon has $108^\circ$ vertex angles, leading to several facts:
- the distance between non-adjacent vertices is $\phi = { 1 + \sqrt 5 \over 2 }$
- the angles adjacent to lengths top and span are $54^\circ$ and $36^\circ$ respectively
- the angle adjacent to length base is $18^\circ$
- the height of this polygon is $h = \cos { \pi \over 10 } + \cos { 3\pi \over 10 }$

<img src="https://github.com/justincredible/Demos/assets/3183562/85945636-84f9-492e-84a6-d25b501313a6" height="200" width="200" >

### Circumscribing circle

The circumcircle's center, relative to the bottom edge of the polygon,
and the radius can be determined by positioning the bottom of the face on the x-axis
as well as symmetrically with respect to the y-axis.
With this positioning we know three vertices of the polygon,
$(-{ 1 \over 2 }, 0)$ and $({ 1 \over 2 }, 0)$, $({ 1 \over 2 }, 1)$ for the square, and $(0, h)$ for the other two.

Solving for $(x - c_x)^2 + (y - c_y)^2 = r^2$ provides us with
center $c = (0, { h \over 2 } - { 1 \over 8h })$ and
radius $r = { h \over 2 } + { 1 \over 8h }$

The vertices are centred by subtracting $c$

<img src="https://github.com/justincredible/Demos/assets/3183562/1bed5bcd-f34e-4ece-84be-e254bfde83dd" height="200" width="200" >

Also notice how a face can be reflected in the circumcircle.

### Remaining polygons

No other polygon is the face of a Platonic Solid, but all are simple to generate.
The techniques used thus far will continue to determine vertex positions for higher-sided polygons.
However, this gets tedious quickly,
so we plot the vertices in a more convenient manner.

## Platonic Solids

Finite regular convex polyhedra with unit length edges.

### Tetrahedron

The relative positions of three vertices are known from the triangle.
The last vertex is collinear with the center and orthogonal to the circle.
Since the edges have unit length, the distance $z$
from the last vertex to the center satisfies $(h - c_y)^2 + z^2 = 1 \implies z = \sqrt { 1 - r^2 }$

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

<img src="https://github.com/justincredible/Demos/assets/3183562/0707ca75-8411-44eb-8480-94ffb8576a9b" height="200" width="200" >

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
From this we have our first ten vertices (the pentagon and its xy-plane reflection).
The distance betwen these vertices is $z_{inner} = \sqrt { 1 - 4(base - c)^2 }  = \sqrt { 1 - 4(r - top)^2 }$

The final two vertices are collinear with the center and orthogonal to the circle.
The distance from the center is $z_{outer} = \sqrt { 1 - r^2 }$
