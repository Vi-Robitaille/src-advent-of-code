import numpy as np
import numpy.typing as npt
import pyvista as pv
from pyvista import CellType
import re

sqrt2 = np.sqrt(2)

def get_rotation_vector(theta=np.pi/2):
    return np.array([
            [np.cos(theta),-np.sin(theta), 0],
            [np.sin(theta),np.cos(theta), 0],
            [0, 0, 1]
        ])


cell_origin= [0, 0, 0]
origins = [
[[2, 18, 0], 7],
[[9, 16, 0], 1],
[[13, 2, 0], 3],
[[12, 14, 0], 4],
[[10, 20, 0], 4],
[[14, 17, 0], 5],
[[8, 7, 0], 9],
[[2, 0, 0], 10],
[[0, 11, 0], 3],
[[20, 14, 0], 8],
[[17, 20, 0], 6],
[[16, 7, 0], 5],
[[14, 3, 0], 1],
[[20, 1, 0], 7],
]

real_shit = [
[[3482210, 422224, 0], 1832939],
[[3679395, 2737332, 0], 668222],
[[3173475, 3948494, 0], 714748],
[[27235, 3642190, 0], 224785],
[[3851721, 1754784, 0], 1119102],
[[327074, 3250656, 0], 902828],
[[3499970, 3186179, 0], 374062],
[[150736, 2522778, 0], 759320],
[[3000768, 3333983, 0], 607054],
[[1751302, 1660540, 0], 1901495],
[[2591068, 2923079, 0], 267552],
[[48946, 3999178, 0], 603484],
[[3695475, 3863101, 0], 509805],
[[1504031, 2760, 0], 975102],
[[3021186, 2667125, 0], 623774],
[[1514629, 3771171, 0], 1656979],
[[234064, 616106, 0], 1703764],
[[3990843, 3393575, 0], 526209],
[[768875, 2665271, 0], 1519952],
]
855911
cell_xy_length = sqrt2
cell_z_length = 0.1

bounds = 4000000

rect_bounds = pv.Cube(center=[bounds / 2, bounds / 2, 0], x_length=bounds, y_length=bounds, z_length=1)
rect_bounds.triangulate(True)
rect_bounds.subdivide(6, progress_bar=True, inplace=True)
pl = pv.Plotter()

for pos, size in real_shit:
    s = size * sqrt2
    rect = pv.Cube(center=cell_origin, x_length=s, y_length=s, z_length=1)
    rect.triangulate(True).rotate_z(45, inplace=True)
    rect.translate(pos, inplace=True)
    rect.subdivide(6, progress_bar=True, inplace=True)
    # _ = pl.add_mesh(rect, color='r', line_width=3)
    _ = pl.add_mesh(rect, color='r', style='wireframe', line_width=1)
    # diff = rect_bounds.boolean_difference(rect)
    # if diff.is_all_triangles:
    #     rect_bounds = diff
    
    # rect_bounds.triangulate(True)


_ = pl.add_mesh(rect_bounds, color='b', style='wireframe', line_width=3)

pl.camera_position = 'xy'
pl.show_grid(bounds=[-1, bounds +1 , -1, bounds +1, -1, 1])
pl.show()