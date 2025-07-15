#!/usr/bin/env python3
"""
Generate an improved boundary polygon from building data in parquet files.
This script loads building footprints, samples their corner points, and then
creates a concave hull (alpha shape) to define a tight-fitting boundary.
"""

import os
import argparse
import sys
import logging
import pandas as pd
import numpy as np
import geopandas as gpd
from shapely.geometry import Polygon
from scipy.spatial import ConvexHull
from alphashape import alphashape

# --- Setup ---
# Configure logging to provide clear, informative output.
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)


def load_all_parquet_files(directory):
    """
    Load all parquet files from a directory, extracting building coordinates
    and counting the total number of valid buildings.
    """
    logging.info(f"Scanning for parquet files in '{directory}'...")
    all_coords = []
    total_buildings = 0

    try:
        parquet_files = [f for f in os.listdir(directory) if f.endswith('.parquet')]
    except FileNotFoundError:
        logging.error(f"Input directory not found: {directory}")
        return np.array([]).reshape(0, 2), 0

    if not parquet_files:
        logging.warning(f"No .parquet files found in '{directory}'.")
        return np.array([]).reshape(0, 2), 0

    logging.info(f"Found {len(parquet_files)} parquet files to process.")

    for i, filename in enumerate(parquet_files):
        filepath = os.path.join(directory, filename)
        logging.info(f"--> Processing {filename} ({i+1}/{len(parquet_files)})")

        try:
            df = pd.read_parquet(filepath, columns=['cal_xmin', 'cal_xmax', 'cal_ymin', 'cal_ymax'])
            
            # Filter out rows with any missing coordinate data
            valid_mask = df[['cal_xmin', 'cal_xmax', 'cal_ymin', 'cal_ymax']].notna().all(axis=1)
            valid_df = df[valid_mask]

            num_valid_buildings = len(valid_df)
            total_buildings += num_valid_buildings
            logging.info(f"    Found {num_valid_buildings} valid buildings.")

            if num_valid_buildings == 0:
                continue
            
            # Vectorized creation of corner points for all buildings at once
            x_coords = np.column_stack([valid_df['cal_xmin'], valid_df['cal_xmax'], valid_df['cal_xmax'], valid_df['cal_xmin']]).flatten()
            y_coords = np.column_stack([valid_df['cal_ymin'], valid_df['cal_ymin'], valid_df['cal_ymax'], valid_df['cal_ymax']]).flatten()
            
            coords = np.column_stack([x_coords, y_coords])
            all_coords.append(coords)

        except Exception as e:
            logging.warning(f"    Could not process {filename}: {e}")
            continue

    if not all_coords:
        logging.warning("No valid coordinates were loaded from any file.")
        return np.array([]).reshape(0, 2), 0

    final_coords = np.vstack(all_coords)
    logging.info(f"Finished loading data. Total buildings: {total_buildings}, Total corner points: {len(final_coords)}")
    return final_coords, total_buildings


def grid_based_sampling(coords, cell_size):
    """
    Performs fast grid-based sampling to reduce point density while preserving shape.
    This is critical for avoiding performance issues and numerical instability with alphashape.
    """
    if cell_size <= 0:
        logging.info("Cell size is 0 or less, skipping sampling.")
        return coords

    logging.info(f"Performing grid-based sampling with cell size {cell_size}...")
    logging.info(f"  Original number of points: {len(coords)}")

    # Create a non-unique integer ID for each cell by dividing by cell size
    # This is a highly efficient vector-based operation
    cell_indices = np.floor(coords / cell_size)

    # Use pandas to find the first index of each unique cell ID
    # This is orders of magnitude faster than other methods for large datasets
    _, unique_indices = np.unique(cell_indices, axis=0, return_index=True)

    sampled_coords = coords[unique_indices]
    
    logging.info(f"  Sampled down to {len(sampled_coords)} points.")
    return sampled_coords


def create_concave_hull(coords, alpha):
    """
    Creates a concave hull (alpha shape) from a set of points.
    Falls back to a convex hull if the alpha shape algorithm fails.
    """
    logging.info(f"Creating concave hull with alpha={alpha} from {len(coords)} points...")

    if len(coords) < 3:
        logging.error("Cannot create a polygon: need at least 3 points.")
        return None

    try:
        # The alphashape function can be slow, this is the main computation
        alpha_shape = alphashape(coords, alpha)

        if alpha_shape.is_empty:
            raise ValueError("Alpha shape result is empty.")

        # If alphashape returns a collection of polygons, select the largest one
        if alpha_shape.geom_type == 'MultiPolygon':
            logging.warning("Alpha shape resulted in a MultiPolygon. Selecting the largest part by area.")
            return max(alpha_shape.geoms, key=lambda p: p.area)
        
        logging.info("Successfully generated alpha shape.")
        return alpha_shape

    except Exception as e:
        logging.warning(f"Alpha shape creation failed ({e}). Falling back to Convex Hull.")
        try:
            # Fallback to a simple convex hull if alphashape has issues
            hull = ConvexHull(coords)
            return Polygon(coords[hull.vertices])
        except Exception as e2:
            logging.error(f"Convex hull fallback also failed: {e2}")
            return None


def main():
    """Main execution function."""
    parser = argparse.ArgumentParser(
        description='Generate a boundary polygon from building footprint data.',
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )
    parser.add_argument(
        'input_dir',
        help='Directory containing the input .parquet files.'
    )
    parser.add_argument(
        'output_file',
        help='Path for the output .geojson file.'
    )
    parser.add_argument(
        '--alpha',
        type=float,
        default=0.05,
        help='Alpha parameter for the concave hull. Smaller values produce a more generalized shape.'
    )
    parser.add_argument(
        '--cell-size',
        type=float,
        default=100.0,
        help='Grid cell size for point sampling to improve performance. The unit should match your data (e.g., meters). Set to 0 to disable.'
    )
    parser.add_argument(
        '--smooth',
        type=float,
        default=0.0,
        help='Simplification tolerance for smoothing the final polygon. A small value like 10 (if in meters) can clean up artifacts. Set to 0 to disable.'
    )
    parser.add_argument(
        '--buffer',
        type=float,
        default=0.0,
        help='Buffer distance to expand the boundary polygon. The unit should match your data (e.g., meters). Set to 0 to disable.'
    )
    parser.add_argument(
        '--crs',
        default='EPSG:4326',
        help='The Coordinate Reference System (CRS) for the output file. For deck.gl compatibility, use EPSG:4326 (default).'
    )
    args = parser.parse_args()

    # --- Step 1: Load all data ---
    coords, total_buildings = load_all_parquet_files(args.input_dir)
    if coords.size == 0:
        logging.error("No coordinates found. Aborting.")
        sys.exit(1)

    # --- Step 2: Sample points for performance ---
    sampled_coords = grid_based_sampling(coords, args.cell_size)

    # --- Step 3: Generate the boundary polygon ---
    polygon = create_concave_hull(sampled_coords, args.alpha)
    if polygon is None:
        logging.error("Failed to generate any boundary polygon. Aborting.")
        sys.exit(1)

    # --- Step 4: (Optional) Smooth the polygon ---
    if args.smooth > 0:
        logging.info(f"Smoothing polygon with tolerance {args.smooth}...")
        original_verts = len(polygon.exterior.coords)
        polygon = polygon.simplify(args.smooth, preserve_topology=True)
        smoothed_verts = len(polygon.exterior.coords)
        logging.info(f"  Vertices reduced from {original_verts} to {smoothed_verts}.")

    # --- Step 5: (Optional) Apply buffer to expand the boundary ---
    if args.buffer > 0:
        logging.info(f"Applying buffer of {args.buffer} meters to expand the boundary...")
        try:
            # Create GeoDataFrame with the polygon in geographic coordinates
            temp_gdf = gpd.GeoDataFrame([{}], geometry=[polygon], crs='EPSG:4326')
            
            # For accurate buffering in meters, transform to a suitable projected CRS
            # Japan Plane Rectangular CS Zone IX (EPSG:6677) is ideal for Tokyo
            projected_gdf = temp_gdf.to_crs('EPSG:6677')
            
            # Apply buffer in meters
            buffered_projected = projected_gdf.buffer(args.buffer)
            
            # Transform back to geographic coordinates for deck.gl compatibility
            buffered_gdf = gpd.GeoDataFrame([{}], geometry=buffered_projected, crs='EPSG:6677')
            final_gdf = buffered_gdf.to_crs('EPSG:4326')
            
            polygon = final_gdf.geometry.iloc[0]
            logging.info("  Buffer applied successfully with proper coordinate transformation.")
        except Exception as e:
            logging.warning(f"  Buffer application failed: {e}")
            logging.info("  Falling back to direct polygon buffering...")
            try:
                # This will be inaccurate but might work for small buffers
                polygon = polygon.buffer(args.buffer)
                logging.info("  Direct buffer applied (note: units may be inaccurate).")
            except Exception as e2:
                logging.error(f"  Direct buffer also failed: {e2}")

    # --- Step 6: Save the output file ---
    properties = {
        "buildings_analyzed": total_buildings,
        "total_corner_points": len(coords),
        "points_used_for_hull": len(sampled_coords),
        "alpha_value": args.alpha,
        "sampling_cell_size": args.cell_size,
        "smoothing_tolerance": args.smooth,
        "buffer_distance": args.buffer
    }

    try:
        logging.info(f"Saving boundary to '{args.output_file}'...")
        gdf = gpd.GeoDataFrame([properties], geometry=[polygon], crs=args.crs)
        gdf.to_file(args.output_file, driver='GeoJSON')
        logging.info("---")
        logging.info("✅ Success! Boundary file created.")
        logging.info("---")
    except Exception as e:
        logging.error(f"Failed to save GeoJSON file: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
