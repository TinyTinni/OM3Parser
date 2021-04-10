# Reverse Engineered Specification

Each file consists of:

- Header
- [optional] Face List
- Vertex List
- Footer

## Header

Starts with "HOME3DF\n"

## Face List Section

- starts with tag "face_polygon"
- 1 little endian binary unsigned integer 32-bit : # faces
- 1 little endian binary unsigned integer 32-bit : # vertices per face
- #faces * #vertices-per-faces little endian binary unsigend integers 32-bit : vertex indices referencing coords in Point Coord Section
- some unknown stuff, looks like little endian binary unsigned integers 32-bit, some sort of indices. Unsure what it is, maybe for alignment. Needs to be checked wit different files.

## Vertex List Section

This section has the xyz coordinate information of the vertices.
- starting tag "point_coord"
- 1 little endian binary unsigned integer 32-bit : # point coordinates which follow (n)
- 3 x n little endian binary floats 32-bit. the coordinates are continious per dimension, so the 3 x n are ordered as follow:
    - n : x coordinates
    - n : y coordinates
    - n : z coordinates

## Footer

Ends with "FD3EMOH\x2E"
