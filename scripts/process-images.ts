#!/usr/bin/env tsx

/**
 * Image Processing Script for Syndicode Assets
 * 
 * Converts source images to optimized responsive variants for web use.
 * Generates WebP formats in multiple sizes for different viewports.
 */

import sharp from 'sharp';
import { promises as fs } from 'fs';
import path from 'path';

// Responsive breakpoints based on syndicode-web viewport sizes
const SIZES = [
  { width: 320, suffix: '320w', quality: 80 },   // Mobile
  { width: 768, suffix: '768w', quality: 85 },   // Tablet
  { width: 1920, suffix: '1920w', quality: 85 }, // Desktop
  { width: 3840, suffix: '3840w', quality: 90 }  // 4K/Retina
] as const;

interface ProcessingOptions {
  quality?: number;
  effort?: number;
  withoutEnlargement?: boolean;
}

/**
 * Process a single image into multiple responsive variants
 */
async function processImage(
  inputPath: string, 
  outputDir: string, 
  basename: string,
  options: ProcessingOptions = {}
): Promise<void> {
  const { quality = 95, effort = 6, withoutEnlargement = true } = options;
  
  console.log(`Processing: ${inputPath}`);
  
  // Ensure output directory exists
  await fs.mkdir(outputDir, { recursive: true });
  
  // Get original image metadata
  const metadata = await sharp(inputPath).metadata();
  console.log(`Original: ${metadata.width}x${metadata.height} (${metadata.format})`);
  
  // Save original as WebP (full size)
  await sharp(inputPath)
    .webp({ quality, effort })
    .toFile(path.join(outputDir, `${basename}-original.webp`));
  
  console.log(`✓ Saved original as WebP`);
  
  // Generate responsive variants
  for (const size of SIZES) {
    const outputPath = path.join(outputDir, `${basename}-${size.suffix}.webp`);
    
    await sharp(inputPath)
      .resize(size.width, null, { 
        withoutEnlargement,
        fit: 'inside'
      })
      .webp({ 
        quality: size.quality, 
        effort 
      })
      .toFile(outputPath);
    
    console.log(`✓ Generated ${size.suffix} variant`);
  }
}

/**
 * Process building images for game assets
 */
async function processBuildingImages(): Promise<void> {
  const assetsDir = path.join(__dirname, '..', 'assets');
  const outputBaseDir = path.join(__dirname, '..', 'processed-assets', 'syndicode', 'images');
  
  try {
    // Check if assets directory exists
    await fs.access(assetsDir);
    
    // Find all building images
    const files = await fs.readdir(assetsDir);
    const buildingFiles = files.filter(file => 
      file.startsWith('building-') && file.endsWith('.png')
    );
    
    if (buildingFiles.length === 0) {
      console.log('No building images found in assets directory');
      return;
    }
    
    console.log(`Found ${buildingFiles.length} building images to process`);
    
    for (const file of buildingFiles) {
      const inputPath = path.join(assetsDir, file);
      const basename = path.parse(file).name;
      const outputDir = path.join(outputBaseDir, 'game', 'buildings', basename);
      
      await processImage(inputPath, outputDir, basename);
      console.log(`✅ Completed processing: ${file}\n`);
    }
    
    console.log('🎉 All building images processed successfully!');
    console.log(`📁 Output directory: ${outputBaseDir}`);
    
  } catch (error) {
    console.error('❌ Error processing images:', (error as Error).message);
    process.exit(1);
  }
}

/**
 * Process market images for game assets
 */
async function processMarketImages(): Promise<void> {
  const marketsDir = path.join(__dirname, '..', 'assets', 'markets');
  const outputBaseDir = path.join(__dirname, '..', 'processed-assets', 'syndicode', 'images', 'game', 'markets');
  
  try {
    // Check if markets directory exists
    await fs.access(marketsDir);
    
    // Find all subdirectories (market groups)
    const marketGroups = await fs.readdir(marketsDir, { withFileTypes: true });
    const marketGroupDirs = marketGroups.filter(dirent => dirent.isDirectory()).map(dirent => dirent.name);
    
    if (marketGroupDirs.length === 0) {
      console.log('No market group directories found');
      return;
    }
    
    console.log(`Found ${marketGroupDirs.length} market groups to process`);
    
    let totalProcessed = 0;
    
    for (const groupDir of marketGroupDirs) {
      const groupInputDir = path.join(marketsDir, groupDir);
      const groupOutputDir = path.join(outputBaseDir, groupDir);
      
      console.log(`\n📁 Processing market group: ${groupDir}`);
      
      // Find all PNG files in this group
      const files = await fs.readdir(groupInputDir);
      const pngFiles = files.filter(file => file.endsWith('.png'));
      
      console.log(`Found ${pngFiles.length} PNG files in group ${groupDir}`);
      
      for (const file of pngFiles) {
        const inputPath = path.join(groupInputDir, file);
        const basename = path.parse(file).name;
        const outputDir = path.join(groupOutputDir, basename);
        
        await processImage(inputPath, outputDir, basename);
        console.log(`✅ Completed processing: ${groupDir}/${file}`);
        totalProcessed++;
      }
    }
    
    console.log(`\n🎉 All market images processed successfully! (${totalProcessed} total images)`);
    console.log(`📁 Output directory: ${outputBaseDir}`);
    
  } catch (error) {
    console.error('❌ Error processing market images:', (error as Error).message);
    process.exit(1);
  }
}

// CLI usage
if (import.meta.url === `file://${process.argv[1]}`) {
  const mode = process.argv[2] || 'buildings';
  
  if (mode === 'markets') {
    processMarketImages();
  } else {
    processBuildingImages();
  }
}

export { processImage, processBuildingImages, processMarketImages };