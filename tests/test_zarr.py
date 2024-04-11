import json
from pathlib import Path
from tempfile import mkdtemp
from typing import Optional

import numpy as np
import pytest
import zarr
from dask import array as da
from numcodecs import blosc as blosc
from ome_zarr.io import parse_url
from ome_zarr.reader import Reader
from skimage.transform import downscale_local_mean

import acquire
from acquire import Runtime, DeviceKind


# FIXME (aliddell): this should be module scoped, but the runtime is leaky
@pytest.fixture(scope="function")
def runtime():
    yield acquire.Runtime()


def test_set_acquisition_dimensions(
    runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    props = runtime.get_configuration()
    props.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, ".*empty.*"
    )
    props.video[0].camera.settings.shape = (64, 48)

    props.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    props.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    props.video[0].max_frame_count = 32

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=64, chunk_size_px=64
    )
    assert dimension_x.shard_size_chunks == 0

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=48, chunk_size_px=48
    )
    assert dimension_y.shard_size_chunks == 0

    dimension_t = acquire.StorageDimension(
        name="t", kind="Time", array_size_px=32, chunk_size_px=32
    )
    assert dimension_t.shard_size_chunks == 0

    props.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_t,
    ]
    assert len(props.video[0].storage.settings.acquisition_dimensions) == 3

    # sleep(10)

    # set and test
    props = runtime.set_configuration(props)
    assert len(props.video[0].storage.settings.acquisition_dimensions) == 3

    assert (
        props.video[0].storage.settings.acquisition_dimensions[0].name
        == dimension_x.name
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[0].kind
        == dimension_x.kind
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[0].array_size_px
        == dimension_x.array_size_px
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[0].chunk_size_px
        == dimension_x.chunk_size_px
    )

    assert (
        props.video[0].storage.settings.acquisition_dimensions[2].name
        == dimension_t.name
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[2].kind
        == dimension_t.kind
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[2].array_size_px
        == dimension_t.array_size_px
    )
    assert (
        props.video[0].storage.settings.acquisition_dimensions[2].chunk_size_px
        == dimension_t.chunk_size_px
    )


def test_write_external_metadata_to_zarr(
    runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin.*"
    )
    p.video[0].camera.settings.shape = (33, 47)
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    p.video[0].max_frame_count = 4
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    metadata = {"hello": "world"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    p.video[0].storage.settings.pixel_scale_um = (0.5, 4)

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=33, chunk_size_px=33
    )
    assert dimension_x.shard_size_chunks == 0

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=47, chunk_size_px=47
    )
    assert dimension_y.shard_size_chunks == 0

    dimension_z = acquire.StorageDimension(
        name="z", kind="Space", array_size_px=0, chunk_size_px=4
    )
    assert dimension_z.shard_size_chunks == 0

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_z,
    ]

    p = runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    while nframes < p.video[0].max_frame_count:
        with runtime.get_available_data(0) as packet:
            nframes += packet.get_frame_count()
    runtime.stop()

    assert p.video[0].storage.settings.filename
    store = parse_url(p.video[0].storage.settings.filename)
    assert store
    reader = Reader(store)
    nodes = list(reader())

    # ome-ngff supports multiple images, in separate directories but we only
    # wrote one.
    multi_scale_image_node = nodes[0]

    # ome-ngff always stores multi-scale images, but we only have a single
    # scale/level.
    image_data = multi_scale_image_node.data[0]
    assert image_data.shape == (
        p.video[0].max_frame_count,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )

    multi_scale_image_metadata = multi_scale_image_node.metadata

    axes = multi_scale_image_metadata["axes"]
    axis_names = tuple(a["name"] for a in axes)
    assert axis_names == ("z", "y", "x")

    axis_types = tuple(a["type"] for a in axes)
    assert axis_types == ("space", "space", "space")

    axis_units = tuple(a.get("unit") for a in axes)
    assert axis_units == (None, "micrometer", "micrometer")

    # We only have one multi-scale level and one transform.
    transform = multi_scale_image_metadata["coordinateTransformations"][0][0]
    pixel_scale_um = tuple(
        transform["scale"][axis_names.index(axis)] for axis in ("x", "y")
    )
    assert pixel_scale_um == p.video[0].storage.settings.pixel_scale_um

    # ome-zarr only reads attributes it recognizes, so use a plain zarr reader
    # to read external metadata instead.
    group = zarr.open(p.video[0].storage.settings.filename)
    assert group["0"].attrs.asdict() == metadata


@pytest.mark.parametrize(
    ("compressor_name",),
    [
        ("zstd",),
        ("lz4",),
    ],
)
def test_write_compressed_zarr(
    runtime: Runtime, request: pytest.FixtureRequest, compressor_name
):
    filename = f"{request.node.name}.zarr"
    filename = filename.replace("[", "_").replace("]", "_")

    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (64, 48)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        f"ZarrBlosc1{compressor_name.capitalize()}ByteShuffle",
    )
    p.video[0].max_frame_count = 70
    p.video[0].storage.settings.filename = filename
    metadata = {"foo": "bar"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=64, chunk_size_px=64
    )
    assert dimension_x.shard_size_chunks == 0

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=48, chunk_size_px=48
    )
    assert dimension_y.shard_size_chunks == 0

    dimension_c = acquire.StorageDimension(
        name="c", kind="Channel", array_size_px=1, chunk_size_px=1
    )
    assert dimension_c.shard_size_chunks == 0

    dimension_t = acquire.StorageDimension(
        name="t", kind="Time", array_size_px=0, chunk_size_px=70
    )
    assert dimension_t.shard_size_chunks == 0

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_c,
        dimension_t,
    ]

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    # load from Zarr
    group = zarr.open(p.video[0].storage.settings.filename)
    data = group["0"]

    assert data.compressor.cname == compressor_name
    assert data.compressor.clevel == 1
    assert data.compressor.shuffle == blosc.SHUFFLE

    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.attrs.asdict() == metadata

    # load from Dask
    data = da.from_zarr(p.video[0].storage.settings.filename, component="0")
    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )


@pytest.mark.parametrize(
    ("number_of_frames", "expected_number_of_chunks", "compression"),
    [
        (64, 4, None),
        (64, 4, {"codec": "zstd", "clevel": 1, "shuffle": 1}),
        (65, 8, None),  # rollover
        (65, 8, {"codec": "blosclz", "clevel": 2, "shuffle": 2}),  # rollover
    ],
)
def test_write_zarr_with_chunking(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
    number_of_frames: int,
    expected_number_of_chunks: int,
    compression: Optional[dict],
):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        "Zarr",
    )
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    p.video[0].max_frame_count = number_of_frames

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=1920, chunk_size_px=960
    )
    assert dimension_x.shard_size_chunks == 0

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=1080, chunk_size_px=540
    )
    assert dimension_y.shard_size_chunks == 0

    dimension_t = acquire.StorageDimension(
        name="t", kind="Time", array_size_px=0, chunk_size_px=64
    )
    assert dimension_t.shard_size_chunks == 0

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_t,
    ]

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    group = zarr.open(p.video[0].storage.settings.filename)
    data = group["0"]

    assert data.chunks == (64, 540, 960)

    assert data.shape == (
        number_of_frames,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.nchunks == expected_number_of_chunks


def test_write_zarr_multiscale(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
):
    filename = f"{request.node.name}.zarr"
    filename = filename.replace("[", "_").replace("]", "_")

    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        "Zarr",
    )
    p.video[0].storage.settings.filename = filename
    p.video[0].storage.settings.pixel_scale_um = (1, 1)
    p.video[0].max_frame_count = 100

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=1920, chunk_size_px=640
    )
    assert dimension_x.shard_size_chunks == 0

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=1080, chunk_size_px=360
    )
    assert dimension_y.shard_size_chunks == 0

    dimension_t = acquire.StorageDimension(
        name="t", kind="Time", array_size_px=0, chunk_size_px=64
    )
    assert dimension_t.shard_size_chunks == 0

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_t,
    ]
    p.video[0].storage.settings.enable_multiscale = True

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    reader = Reader(parse_url(filename))
    zgroup = list(reader())[0]
    # loads each layer as a dask array from the Zarr dataset
    data = [
        da.from_zarr(filename, component=str(i))
        for i in range(len(zgroup.data))
    ]
    assert len(data) == 3

    image = data[0][0, :, :].compute()  # convert dask array to numpy array

    for d in data:
        assert (
            np.linalg.norm(image - d[0, :, :].compute()) == 0
        )  # validate against the same method from scikit-image
        image = downscale_local_mean(image, (2, 2)).astype(np.uint8)


@pytest.mark.parametrize(
    ("number_of_frames", "expected_number_of_chunks", "codec"),
    [
        (64, 4, None),
        (64, 4, "zstd"),
        (65, 8, None),  # rollover
        (65, 8, "lz4"),  # rollover
    ],
)
def test_write_zarr_v3(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
    number_of_frames: int,
    expected_number_of_chunks: int,
    codec: Optional[str],
):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )

    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        f"ZarrV3Blosc1{codec.capitalize()}ByteShuffle" if codec else "ZarrV3",
    )
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    p.video[0].max_frame_count = number_of_frames

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x",
        kind="Space",
        array_size_px=1920,
        chunk_size_px=960,
        shard_size_chunks=2,
    )

    dimension_y = acquire.StorageDimension(
        name="y",
        kind="Space",
        array_size_px=1080,
        chunk_size_px=540,
        shard_size_chunks=2,
    )

    dimension_t = acquire.StorageDimension(
        name="t",
        kind="Time",
        array_size_px=0,
        chunk_size_px=64,
        shard_size_chunks=1,
    )

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_t,
    ]

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    store = zarr.DirectoryStoreV3(p.video[0].storage.settings.filename)
    group = zarr.open(store=store, mode="r")
    data = group["0"]

    assert data.chunks == (64, 540, 960)

    assert data.shape == (
        number_of_frames,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.nchunks == expected_number_of_chunks


def test_metadata_with_trailing_whitespace(
    runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (64, 48)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    p.video[0].max_frame_count = 70
    p.video[0].storage.settings.filename = str(
        Path(mkdtemp()) / f"{request.node.name}.zarr"
    )
    metadata = {"foo": "bar"}
    p.video[0].storage.settings.external_metadata_json = (
        json.dumps(metadata) + "   "
    )

    # configure storage dimensions
    dimension_x = acquire.StorageDimension(
        name="x", kind="Space", array_size_px=64, chunk_size_px=64
    )

    dimension_y = acquire.StorageDimension(
        name="y", kind="Space", array_size_px=48, chunk_size_px=48
    )

    dimension_t = acquire.StorageDimension(
        name="t", kind="Time", array_size_px=0, chunk_size_px=64
    )

    p.video[0].storage.settings.acquisition_dimensions = [
        dimension_x,
        dimension_y,
        dimension_t,
    ]

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    # load from Zarr
    group = zarr.open(p.video[0].storage.settings.filename)
    data = group["0"]

    assert data.attrs.asdict() == metadata
