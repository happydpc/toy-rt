import trt

red = (1, 0, 0)
green = (0, 1, 0)
blue = (0, 0, 1)

def scene():
    spheres = [
        trt.Sphere(
            center=(-50 + i * 50, 0, 0),
            radius=20,
            material=trt.Material.matte(color),
        )
        for i, color in enumerate((red, green, blue))
    ]

    ground = trt.Sphere(
        center=(0, -5050, 0),
        radius=5000,
        material=trt.Material.matte((1, 1, 1)),
    )

    ceiling = trt.Sphere(
        center=(0, +5200, 0),
        radius=5000,
        material=trt.Material.matte((1, 1, 1)),
    )

    return trt.Scene(**{
        "world": spheres + [ground],
        "width": 300,
        "height": 300,
        "samples_per_px": 20,
        "rays_per_sample": 10,
        "camera": trt.Camera(
            look_at=(-25, 0, 0),
            look_from=(-25, 0, 200),
        )
    })
